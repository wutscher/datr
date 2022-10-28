# DatR

A simple application to sort files (especially pictures) by their date.

## How it works

DatR will first look if a file includes EXIF data. If so it will choose the earliest among `DateTime`, `DateTimeDigitized` and `DateTimeOriginal`.
If not it will try to find a creation date and as a last resort the modification date. Once a date is aquired, DatR will either copy or move the files
into their new folders according to the specified pattern.

## How to install

1. Clone this repository
2. run `cargo build --release`
3. run `cargo install --path .`

If you havn't already add the `~/.cargo/bin` directory to your PATH.

## How to use

<ins>**Usage**</ins>: datr [OPTIONS] --input \<INPUT\> --output \<OUTPUT\>

<ins>**Options**</ins>:

  -i, --input \<INPUT\>    The root folder containing the images

  -o, --output \<OUTPUT\>  Where the sorted images are placed

  -r, --recursive        Makes the operation recursive through folders

  -m, --move             Moves your files instead of copying them

  -f, --format \<FORMAT\>  The format of the resulting folder structure ('/' will create subfolders) https://docs.rs/chrono/latest/chrono/format/strftime/index.html [default: %Y-%m-%d]

  -c, --cleanup          Removes empty folders after sorting (NOT IMPLEMENTED)

  -h, --help             Print help information

  -V, --version          Print version information

### Example:

> `datr -r -m -c -i ./photos -o ./sorted_photos -f "%Y-%m-%d"`
>
> might create the following structure: 
>```
>photos
>│
>└─ <files>
>
>sorted_photos
>│
>└─ 2020-10-31
>│  │
>│  └─ <files>
>└─ 2020-11-01
>│  │
>│  └─ <files>
>└─ 2020-11-03
>   │
>   └─ <files>
>```

> `datr -r -m -c -i ./photos -o ./sorted_photos -f "%Y/%m/%d"`
>
> might create the following structure: 
>```
>photos
>│
>└─ <files>
>
>sorted_photos
>│
>└─ 2020
>   │
>   └─ 10
>   │  │
>   │  └─ 31
>   │     │
>   │     └─ <files>
>   └─ 11
>      │
>      └─ 1
>      │  │
>      │  └─ <files>
>      │
>      └─ 3
>         │
>         └─ <files>
>```

## Contribution

I made this for my own personal use but I welcome anyone who wants to expand or improve it.
If no major bugs appear, I will most likely not be updating this app.
