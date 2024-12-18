use clap::Parser;
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error, Write},
    path::Path,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    note: Option<String>,

    #[arg(short, long)]
    date: Option<String>,

    #[arg(short, long)]
    clear: bool,

    #[arg(short, long)]
    important: Option<u8>,
}

fn main() -> Result<(), Error> {
    #[allow(deprecated)] // STUPID "WINDOWS WARNING" I HATE IT!!!!!
    let home_dir_path = match env::home_dir() {
        Some(path) => path.to_string_lossy().into_owned(),
        None => "/".to_string(),
    };

    let data_path_format = format!("{}/.config/sn.txt", home_dir_path);
    let data_path = Path::new(&data_path_format);
    let data = data_path.display().to_string();

    create_file(&data);

    match get_args(&data) {
        Ok(()) => print!(""),
        Err(_) => print!(""),
    }
    Ok(())
}

fn get_args(data: &str) -> Result<(), Error> {
    let args = Args::parse();

    match (args.note, args.date, args.important) {
        (Some(note), Some(date), None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{} -- {}", note, date)?;
        }
        (Some(note), None, None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{}", note)?;
        }
        (None, Some(date), None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{}", date)?;
        }
        (None, None, None) => {
            let to_read = File::open(data)?;
            let read = BufReader::new(to_read);

            for line in read.lines() {
                println!("{}", line?);
            }
        }
        (Some(note), None, Some(important)) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "<== {} | {} ==>", important, note)?;
        }
        (Some(note), Some(date), Some(important)) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "<== {} -- {} | {} ==>", date, note, important)?;
        }
        _ => todo!(),
    }

    if args.clear {
        match clear_file(data) {
            Ok(data) => data,
            Err(e) => println!("{e}"),
        }
    }
    Ok(())
}

fn create_file(data: &str) {
    if !std::path::Path::new(data).exists() {
        match File::create(data) {
            Ok(_) => println!("{} was created", data),
            Err(_) => println!("Failed to create file"),
        }
    } else {
        print!("");
    }
}

fn clear_file(data: &str) -> Result<(), Error> {
    let _file = OpenOptions::new().write(true).truncate(true).open(data)?;

    println!("==Cleared==");
    Ok(())
}
