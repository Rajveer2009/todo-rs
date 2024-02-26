use std::env;
use std::error::Error;
use std::fs::{write, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];

    let file_name = env::var("TODO_PATH").expect("not found!");

    match command.as_str() {
        "mark" => mark(args[2].parse::<usize>().unwrap(), &file_name),
        "listC" => listC(&file_name),
        "list" => listS(&file_name),
        "reset" => reset(&file_name),
        "remove" => remove(args[2].parse::<usize>().unwrap(), &file_name),
        "add" => {
            let tasks: Vec<String> = args[2..].to_vec();
            add(&tasks, &file_name)?;
            Ok(())
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid command: use 'add', 'list', 'listC','remove', 'mark', 'reset'",
        ))),
    }
}

fn add(tasks: &[String], file_name: &str) -> Result<(), Box<dyn Error>> {
    let timestamp = chrono::offset::Local::now();
    let formatted_time = timestamp.format("%d/%m/%Y %H:%M:%S");

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(file_name)
        .expect("Failed to open file for writing");

    for task in tasks {
        writeln!(data_file, "{} ― [ ] {}", formatted_time, task)?;
        println!("Task added: {}", task);
    }

    Ok(())
}

fn listC(file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("TODO");

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut line_number = 1;

    for line in reader.lines() {
        let line = line?;
        println!("{}. {}", line_number, line);
        line_number += 1;
    }

    Ok(())
}

fn listS(file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("TODO");

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", chop(line));
    }

    Ok(())
}

fn reset(file_name: &str) -> Result<(), Box<dyn Error>> {
    write(file_name, "")?;
    Ok(())
}

fn remove(line_number: usize, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    {
        let file = File::open(file_name)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            line_count += 1;
            if line_count != line_number {
                lines.push(line);
            }
        }
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn mark(line_number: usize, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line_count += 1;

        if line_count == line_number {
            let status_marker_index = line.find("― [ ]").unwrap_or(0) + 4;
            if status_marker_index > 0 {
                line.replace_range(status_marker_index..status_marker_index + 3, "[×]");
            } else {
                println!("Task already marked.");
            }
        }

        lines.push(line);
    }

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn chop(mut line: String) -> String {
    if line.len() > 20 {
        line = line.chars().skip(20).collect();
    } else {
        line.clear();
    }
    line
}
