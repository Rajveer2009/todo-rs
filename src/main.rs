use chrono::offset::Local;
use chrono::{DateTime, Utc};
use std::time::SystemTime;
use std::fs::{File, OpenOptions, write};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::error::Error;

const FILE_NAME: &str = "/path/to/todo.[txt or any other format]";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let command = &args[1];
    let mut tasks = Vec::new(); // Define tasks vector outside of the match statement

    match command.as_str() {
        "mark" => mark(args[2].parse::<usize>().unwrap()),
        "listC" => listC(),
        "listS" => listS(),
        "reset" => reset(),
        "remove" => remove(args[2].parse::<usize>().unwrap()),
        "add" => {
            tasks.extend_from_slice(&args[2..]); // Populate tasks vector with arguments
            add(&tasks)?; // Pass tasks vector to the add function
            Ok(()) // Return Ok if everything is successful
        },
        _ => Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid command: use 'add', 'listS', 'listC','remove', 'mark', 'reset'"))),
    }
}

fn add(tasks: &[String]) -> Result<(), Box<dyn Error>> {
    let timestamp: SystemTime = SystemTime::now();
    let timestamp: DateTime<Utc> = timestamp.into();
    let local_time: DateTime<Local> = timestamp.with_timezone(&Local);
    let formatted_time = local_time.format("%d/%m/%Y %H:%M:%S");

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(FILE_NAME)
        .expect("cannot open file");

    for task in tasks {
        data_file.write_fmt(format_args!("{} ― [ ] {}\n", formatted_time, task))?;
        println!("Task added: {}", task);
    }

    Ok(())
}

fn listC() -> Result<(), Box<dyn Error>> {
    println!("TODO");

    let file = File::open(FILE_NAME)?;
    let reader = BufReader::new(file);

    let mut line_number = 1; // Initialize a counter for line numbers

    for line in reader.lines() {
        let line = line?;
        println!("{}. {}", line_number, line); // Print line number and line
        line_number += 1; // Increment counter for the next line
    }

    Ok(())
}

fn listS() -> Result<(), Box<dyn Error>> {
    println!("TODO");

    let file = File::open(FILE_NAME)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", chop(line)); // Print line number and line
    }

    Ok(())
}

fn reset() -> Result<(), Box<dyn Error>> {
    write(FILE_NAME, "");
    Ok(())
}

fn remove(line_number: usize) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    {
        let file = File::open(FILE_NAME)?;
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
        .truncate(true) // Clear existing content
        .open(FILE_NAME)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn mark(line_number: usize) -> Result<(), Box<dyn Error>> {
    let mut lines = Vec::new();
    let mut line_count = 0;

    let file = File::open(FILE_NAME)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line = line?;
        line_count += 1;

        if line_count == line_number {
            // Find the starting index of the status marker
            let status_marker_index = line.find("― [ ]").unwrap_or(0) + 4;

            // Check if the task is already marked
            if status_marker_index > 0 {
                // Replace the status marker with the checked mark
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
        .open(FILE_NAME)?;

    let mut writer = BufWriter::new(file);

    for line in lines {
        writeln!(writer, "{}", line)?;
    }

    Ok(())
}

fn chop(mut line: String) -> String {
    // Check if the line length is greater than 20 characters
    if line.len() > 20 {
        // Remove the first 20 characters from the string
        line = line.chars().skip(20).collect();
    } else {
        // If the line length is less than or equal to 20 characters,
        // clear the line
        line.clear();
    }
    line
}
