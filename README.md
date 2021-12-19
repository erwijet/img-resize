# img-resize.exe
A standalone Windows executable for bulk renaming / resizing jpg imgaes.

## Usage
`PS> .\img-resize.exe instr.csv`

## Instruction File
`img-resize.exe` requires a csv file to be passed in as an instruction file. Each line in the csv file will be considered one "instruction", or one file that needs to be processed. Although the csv instruction file may have any number of headers, it is required to have the following four:

|header|description|
|---|---|
|cur_name|The relative path of the target file|
|new_name|The relateive path of the new desired file|
|target_w|The size, in pixels, of the new desired width|
|target_h|The size, in pixels, of the new desired height|

For example, given the following file structure:

```text
H:\
|
+---img-resize
|   |   img-resize.exe
|	|
|                               
\---scrape
    |   demo.csv
    |   photoscrape.ps1
    |   RUNME.bat
    |   
    \---imgs
		|	CFK-123-UIG.jpg
		|	JGH-925-KHA.jpg
		|	JSF-861-HAD.jpg
		|
```

a csv instruction file could look like this:

|cur_name|new_name|target_w|target_h|
|---|---|---|---|
|../scrape/imgs/CFK-123-UIG.jpg|resized/CFK-123-UIG.jpg|100|100|
|../scrape/imgs/JGH-925-KHA.jpg|resized/JGH-925-KHA.jpg|100|100|
|../scrape/imgs/JSF-861-HAD.jpg|resized/JSF-861-HAD.jpg|100|100|

It is worth noting, in this example we are not changing the file name, but rather changing the path the files are saved to. In this situation, we are saving to a folder `resized` and, since the paths are relative and we are executing in the folder with the `img-resize.exe` file, the full path of the `resized` folder would be `H:\img-resize\resized`.

Further, however, all parts of a path must exist _prior_ to execution. What this means is since this folder doesn't exit yet, we have to create it.

```powershell
PS> cd H:\img-resize
PS> mkdir resized

    Directory: H:\img-resize


Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
d-----        XX/XX/20XX  XX:XX PM                resized
```

## Technical Description

`img-resize.exe` is written in [Rust](https://www.rust-lang.org/) and can be built from source using the following steps (assuming [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) is installed and configured).

```powershell
PS> git clone https://github.com/erwijet/img-resize ; cd img-resize
PS> cargo build --release
PS> copy .\target\release\img-resize.exe .\img-resize.exe
```
