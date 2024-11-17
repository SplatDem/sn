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
    notify: Option<String>,

    #[arg(short, long)]
    date: Option<String>,

    #[arg(short, long)]
    clear: Option<u8>,

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

    match (args.notify, args.date, args.clear, args.important) {
        (Some(notify), Some(date), None, None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{} -- {}", notify, date)?;
        }
        (Some(notify), None, None, None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{}", notify)?;
        }
        (None, Some(date), None, None) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "{}", date)?;
        }
        (None, None, None, None) => {
            let to_read = File::open(data)?;
            let read = BufReader::new(to_read);

            for line in read.lines() {
                println!("{}", line?);
            }
        }
        (None, None, Some(_clear), None) => {
            let _ = clear_file(data);
        }
        (Some(notify), None, None, Some(important)) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "<== {} | {} ==>", important, notify)?;
        }
        (Some(notify), Some(date), None, Some(important)) => {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(data)?;

            writeln!(file, "<== {} -- {} | {} ==>", date, notify, important)?;
        }

        // WHAT THE FUCK IS THIS??!?!??!?!?!?!
        (None, Some(_), Some(_), None)
        | (Some(_), None, Some(_), None)
        | (Some(_), Some(_), Some(_), Some(_)) => {
            todo!()
        }
        _ => todo!(),
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
