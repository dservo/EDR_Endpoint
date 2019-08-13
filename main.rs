#[macro_use]
extern crate clap;
extern crate chrono;

use chrono::{DateTime, Utc};
use clap::App;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()>  {
    // YAML Method for CLI OPTS
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // get values for option supplied from command line or fall to default
    if matches.is_present("process") {
      let process = matches.value_of("process").unwrap_or("default");
      println!("process: {}", process);
    }
    if matches.is_present("file") {
        let file = matches.value_of("file").unwrap_or("default.txt");
        let file_path = file.to_string();
      if matches.is_present("create") {
        create_file(file_path)?;
      }else if matches.is_present("delete") {
        remove_file(file_path)?;
      }else if matches.is_present("modify") {
        modify_file(file_path)?;
      }else {
        println!("Nothing Done about {}" , file_path);
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

fn modify_file(path: String) -> std::io::Result<()>{
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
        //file.write_all("ehello")?;
        println!("UTC now is: {}", now);
        println!("Modified file: {}", path);
    }

    //println!("Modified file: {}", path);
    //fs::remove_file(path)?;
    Ok(())
}
