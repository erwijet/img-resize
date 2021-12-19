/// img-resize - main.rs
/// given an input csv file, resize a batch of images
extern crate csv;
extern crate image;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize)]
struct Record {
    cur_name: String,
    new_name: String,
    target_w: u32,
    target_h: u32,
}

fn get_first_arg() -> Result<OsString> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn process_img(instr: &Record) -> Result<()> {
    let img = image::open(instr.cur_name.as_str()).unwrap();
    img.thumbnail_exact(instr.target_w, instr.target_h)
        .save_with_format(instr.new_name.as_str(), image::ImageFormat::Jpeg)?;
    Ok(())
}

fn run() -> Result<()> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    for csv_val in rdr.deserialize() {
        let record: Record = csv_val?;
        println!(
            "{}",
            match process_img(&record) {
                Ok(()) => format!("{} -> {}", record.cur_name, record.new_name),
                Err(err) => format!("[FAIL] {}", err),
            }
        );
    }

    println!("That's a wrap!");

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
