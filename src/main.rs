use std::env;
use ansi_term::Style;
use std::error::Error;
use std::process::exit;
use std::fs::{write, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: [command] *");
        exit(0x0101)
    } 

    let bold = Style::new().bold();
    println!("{}", bold.paint("TODO"));

    let command = &args[1];

    let todo_path = env::var("TODO_PATH").expect("Environment variable 'TODO_PATH' not found!");

    match command.as_str() {
        "mark" => mark_task(args[2].parse::<usize>().unwrap(), &todo_path),
        "listC" => list_complex(&todo_path),
        "list" => list_simple(&todo_path),
        "reset" => reset_todo_file(&todo_path),
        "remove" => remove_task(args[2].parse::<usize>().unwrap(), &todo_path),
        "add" => {
            let tasks: Vec<String> = args[2..].to_vec();
            add_tasks(&tasks, &todo_path)?;
            Ok(())
        }
        _ => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid command: use 'add', 'list', 'listC','remove', 'mark', 'reset'",
        ))),
    }
}

fn add_tasks(tasks: &[String], todo_path: &str) -> Result<(), Box<dyn Error>> {
    let timestamp = chrono::offset::Local::now();
    let formatted_time = timestamp.format("%d/%m/%Y %H:%M:%S");

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(todo_path)
        .expect("Failed to open file for writing");

    for task in tasks {
        writeln!(data_file, "{} ― [ ] {}", formatted_time, task)?;
        println!("Task added: {}", task);
    }

    Ok(())
}

fn list_complex(todo_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(todo_path)?;
    let reader = BufReader::new(file);

    let mut line_number = 1;

    for line in reader.lines() {
        let line = line?;
        println!("{}. {}", line_number, line);
        line_number += 1;
    }

    Ok(())
}

fn list_simple(todo_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(todo_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", truncate(line));
    }

    Ok(())
}

fn reset_todo_file(todo_path: &str) -> Result<(), Box<dyn Error>> {
    write(todo_path, "")?;
    Ok(())
}

fn remove_task(line_number: usize, todo_path: &str) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    {
        let file = File::open(todo_path)?;
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
        .open(todo_path)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn mark_task(line_number: usize, todo_path: &str) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    let file = File::open(todo_path)?;
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
        .open(todo_path)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn truncate(mut line: String) -> String {
    if line.len() > 20 {
        line = line.chars().skip(20).collect();
    } else {
        line.clear();
    }
    line
}
