#[macro_use]
extern crate clap;
extern crate chrono;

use chrono::{DateTime, Utc};
use clap::App;
use std::fs::{self , OpenOptions, File};
use std::path::Path;
use std::process::{Command, Stdio}; // self may be nedded
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() -> std::io::Result<()>  {
    // YAML Method for CLI OPTS
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    // get values for option supplied from command line or fall to default
    if matches.is_present("process") {
      let process = matches.value_of("process").unwrap_or("default");
      create_prosess(process.to_string())?;
    }
    if matches.is_present("file") {
      let file = matches.value_of("file").unwrap_or("default.txt");
      if matches.is_present("create") {
        create_file(file.to_string())?;
      }else if matches.is_present("delete") {
        remove_file(file.to_string())?;
      }else if matches.is_present("modify") {
        modify_file(file.to_string())?;
      }else {
        println!("Nothing Done about {}" , file.to_string());
      }
    }
    //Collect the Net conection information
    if matches.is_present("net") {
      let net: Vec<&str> = matches.values_of("net").unwrap().collect();
      println!("Connection IP: {} PORT: {} File: {} ", net[0], net[1], net[2] );
    }
    Ok(())
} // end main

fn create_file(path: String) -> std::io::Result<()> {
    println!("Created file: {}", path);
    let _file = File::create(path)?;
    Ok(())
}

fn remove_file(path: String) -> std::io::Result<()> {
    println!("Deleted file: {}", path);
    fs::remove_file(path)?;
    Ok(())
}

fn modify_file(path: String) -> std::io::Result<()> {
    use std::io::Write;
    let now: DateTime<Utc> = Utc::now();
    let mut file_check = Path::new(&path.clone()).exists();
    if file_check == false {
        create_file(path.clone())?;
        file_check = true;
    }
    if file_check == true {
        let mut _file = OpenOptions::new().append(true).open(path.clone()).unwrap();;
        writeln!(_file,  "hello world")? ;
        println!("UTC now is: {}", now);
        println!("Modified file: {}", path);
    }
    Ok(())
}

fn create_prosess (path: String) -> std::io::Result<()> {
    println!("Running command: {}", path);
    let process_output = Command::new(path.clone())
        //.arg("--help") // todo:play with args() instead to pass from cli interface
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other,"Error capturing standard output."))?;
    let read_cmd_output = BufReader::new(process_output);
    read_cmd_output.lines().filter_map(|line| line.ok()).for_each(|line| println!("{}", line));
    Ok(())
}
