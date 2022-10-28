
use std::fs::File;
use std::path::Path;

use clap::Parser;
use walkdir::WalkDir;
use chrono::offset::Utc;
use chrono::{DateTime, NaiveDate};

extern crate exif;

use exif::{Tag, In};

/// Sort your pictures by dates
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root folder containing the images
    #[arg(short = 'i', long = "input")]
    input: String,
    
    /// Where the sorted images are placed
    #[arg(short = 'o', long = "output")]
    output: String,
    
    /// Makes the operation recursive through folders
    #[arg(short = 'r', long = "recursive")]
    recursive: bool,

    /// Moves your files instead of copying them
    #[arg(short = 'm', long = "move")]
    move_files: bool,

    /// The format of the resulting folder structure ('/' will create subfolders) https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    #[arg(short = 'f', long = "format", default_value_t = ("%Y-%m-%d").to_string())]
    format: String,

    /// Removes empty folders after sorting (NOT IMPLEMENTED)
    #[arg(short = 'c', long = "cleanup")]
    cleanup: bool,
}

fn main() {
    let args = Args::parse();

    let in_path = std::path::Path::new(&args.input);
    let out_path = std::path::Path::new(&args.output);

    println!("Successfully {} {} files", if args.move_files {"moved"} else {"copied"},sort(in_path, args.recursive, out_path, &args.format, args.move_files));
    
    if args.cleanup {
        for dir in WalkDir::new(in_path).contents_first(true).into_iter().filter_entry(|e| e.file_type().is_dir()) {
            
            match dir {
                Ok(d) => {
                    if WalkDir::new(d.path()).min_depth(1).into_iter().count() == 0 {
                        match std::fs::remove_dir(d.path()) {
                            Ok(_) => (),
                            Err(e) => println!("Error removing directory: {}", e)
                        }
                    }
                },
                Err(e) => println!("Could not clean directory: {}", e)
            }
        }
    }
}

fn sort(input: &Path, recursive: bool, output: &Path, format_str: &str,move_files: bool) -> i64 {
    let mut counter = 0;
    let mut walker = WalkDir::new(input);
    if !recursive {
        walker = walker.max_depth(1);
    }
    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            match process_file(&entry, output, format_str, move_files) {
                Ok(_) => counter+=1,
                Err(e) => println!("Error processing file <{:?}>: {}", &entry.path(), e)
            }
        }
    }
    return counter;
}

fn process_file(entry: &walkdir::DirEntry, output: &Path, format_str: &str, move_files: bool) -> Result<(),std::io::Error> {
    let file = std::fs::File::open(entry.path())?;
    let oldest_date: DateTime<Utc> = match get_exif(file) {
        Some(date) => date,
        None => match entry.metadata()?.created() {
            Ok(date_match) => date_match.into(),
            Err(_) => entry.metadata()?.modified()?.into()
        }
    };

    let dir_path = output.join(oldest_date.format(format_str).to_string());
    let file_name: &str;

    match entry.file_name().to_str() {
        Some(fname) => file_name = fname,
        None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Filename could not be converted"))
    }

    let mut file_path = dir_path.join(file_name);

    if file_path.exists() {
        match change_file_name(file_path) {
            Some(fpath) => file_path = fpath,
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Filepath could not be changed to avoid collisions"))
        }
    }

    std::fs::create_dir_all(&dir_path)?;
    if move_files {
        std::fs::rename(entry.path(), file_path)?;
    } else {
        std::fs::copy(entry.path(), file_path)?;
    };

    return Ok(());
}

fn change_file_name(path: impl AsRef<Path>) -> Option<std::path::PathBuf> {
    let path = path.as_ref();
    println!("File already exists: {:?}", path.file_name()?);
    let mut result = path.to_owned();
    let mut counter = 1;
    let original_name = path.file_stem()?.to_str()?;

    while result.exists() {
        let name = format!("{} ({})", original_name, counter);
        result.set_file_name(name);
        if let Some(ext) = path.extension() {
            result.set_extension(ext);
        }
        counter += 1;
    }

    println!("renaming: {:?}", result.file_name()?);

    return Some(result);
}

fn get_exif(file: File) -> Option<DateTime<Utc>>{
    
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif;

    match exifreader.read_from_container(&mut bufreader) {
        Ok(exif_v) => exif = exif_v,
        Err(_) => return None
    }

    let mut c: Vec<DateTime<Utc>> = vec![];

    let tag_list = [Tag::DateTime,Tag::DateTimeOriginal,Tag::DateTimeDigitized];

    for &tag in tag_list.iter() {
        if let Some(field) = exif.get_field(tag, In::PRIMARY) {
            match field.value {
                exif::Value::Ascii(ref vec) if !vec.is_empty() => {
                    if let Ok(datetime) = exif::DateTime::from_ascii(&vec[0]) {
                        let naivedatetime_utc = NaiveDate::from_ymd(
                            datetime.year.into(),
                            datetime.month.into(),
                            datetime.day.into()
                        ).and_hms(
                            datetime.hour.into(), 
                            datetime.minute.into(),
                            datetime.second.into());
                        
                        let datetime_utc = DateTime::<Utc>::from_utc(naivedatetime_utc, Utc);
    
                        c.push(datetime_utc);
                    }
                },
                _ => {},
            }
        }
    }

    match c.iter().min(){
        Some(min) => return Some(min.to_owned()),
        None => return None
    }
}
