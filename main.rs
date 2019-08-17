#[macro_use]
extern crate clap;
extern crate chrono;

#[cfg(target_os = "windows")]
extern crate winreg;

use chrono::{DateTime, Utc};
use clap::App;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::TcpStream;
use std::path::Path;
use std::process::{Command, Stdio}; // self may be nedded

// [NTS] remember can make use more compact
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

fn main() -> std::io::Result<()> {
    // YAML Method for CLI OPTS to have diffrent options
    // note the targeted compile fells wierd  but this works atm
    // have to figure out a method to add the win file to nix to avoid double edit
    #[cfg(target_os = "windows")]
    let yaml = load_yaml!("cli_win.yml");
    #[cfg(not(target_os = "windows"))]
    let yaml = load_yaml!("cli_nix.yml");

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
        } else if matches.is_present("delete") {
            remove_file(file.to_string())?;
        } else if matches.is_present("modify") {
            modify_file(file.to_string())?;
        } else {
            println!("Nothing Done about {}", file.to_string());
        }
    }
    //Collect the Net conection information
    if matches.is_present("net") {
        let net: Vec<&str> = matches.values_of("net").unwrap().collect();
        tcp_send(net[0].to_string(), net[1].to_string(), net[2].to_string())?;
    }
    if cfg!(target_os = "windows") {
        if matches.is_present("reg") {
            let reg_path = matches.value_of("reg").unwrap_or("Software\\default");
            if matches.is_present("create") {
                create_winreg_key(reg_path.to_string())?;
            } else if matches.is_present("delete") {
                if !matches.is_present("value") {
                    remove_winreg_key(reg_path.to_string())?;
                } else {
                    // bit clunky on this overridden delete but scopeing needs some work
                    let reg_value: Vec<&str> = matches.values_of("value").unwrap().collect();
                    delete_winreg_value(reg_path.to_string(), reg_value[0].to_string())?;
                }
            } else if matches.is_present("modify") {
                if matches.is_present("value") {
                    let reg_value: Vec<&str> = matches.values_of("value").unwrap().collect();
                    modify_winreg_value(
                        reg_path.to_string(),
                        reg_value[0].to_string(),
                        reg_value[1].to_string(),
                    )?;
                } else {
                    println!(
                        "error: The argument --REGEDIT with flag -M requires -V
                         <value>  and -N <name> to create the value"
                    );
                }
            } else {
                println!("Nothing Done about {}", reg_path.to_string());
            }
        }
    }
    Ok(())
} // end main

fn check_file(path: String) -> bool {
    Path::new(&path.clone()).exists()
}

fn create_file(path: String) -> std::io::Result<()> {
    if check_file(path.clone()) {
        println!("Created file: {}", path);
        let _file = File::create(path)?;
    } else {
        println!("File alrady exsits");
    }
    Ok(())
}

fn remove_file(path: String) -> std::io::Result<()> {
    if check_file(path.clone()) {
        println!("Deleted file: {}", path);
        fs::remove_file(path)?;
    } else {
        println!("{} is not a file dooing nothing", path);
    }
    Ok(())
}

fn modify_file(path: String) -> std::io::Result<()> {
    let now: DateTime<Utc> = Utc::now();
    let mut file_check = check_file(path.clone()); //Path::new(&path.clone()).exists();
    if !file_check {
        create_file(path.clone())?;
        file_check = true;
    }
    if file_check {
        let mut _file = OpenOptions::new().append(true).open(path.clone()).unwrap();;
        writeln!(_file, "hello world")?;
        println!("UTC now is: {}", now);
        println!("Modified file: {}", path);
    }
    Ok(())
}

fn create_prosess(path: String) -> std::io::Result<()> {
    println!("Running command: {}", path);
    let process_output = Command::new(path.clone())
        //.arg("--help") // todo:play with args() instead to pass from cli interface
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Error capturing standard output."))?;
    let read_cmd_output = BufReader::new(process_output);
    read_cmd_output
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));
    Ok(())
}

fn tcp_send(ip_address: String, port: String, file: String) -> std::io::Result<()> {
    println!(
        "Connection Information IP: {} PORT: {} File: {} ",
        ip_address, port, file
    );
    let connection = format!("{}:{}", ip_address, port);
    if let Ok(mut tcp_stream) = TcpStream::connect(connection.clone()) {
        println!("Connection to server @ : {}", connection);
        tcp_stream.write(b"hello world")?;
    } else {
        println!("Error No server @: {}", connection);
    }
    Ok(())
}

// not implemneted yet, modify a key name appears to not be built into the winap
// would need to read and create all values in a key and any sub keys in new key
// then delet old key  [SHOULD ADD LATER]

#[cfg(target_os = "windows")]
fn create_winreg_key(path: String) -> std::io::Result<()> {
    // only going to use HKEY_CURRENT_USER atm for simpliity and premmisions
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_path = Path::new(&path);
    let (_key, disp) = hkcu.create_subkey(&reg_path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("Created Key {}", &path),
        REG_OPENED_EXISTING_KEY => println!("Key already exists {}", &path),
    }
    Ok(())
}

// study passing variable more to create a fn of open key
// will break if key is not there but need to look into how to check for this condition
// the panic! from the crate exsample does not work as gracefull as I would like

#[cfg(target_os = "windows")]
fn remove_winreg_key(path: String) -> std::io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_path = Path::new(&path);
    hkcu.open_subkey_with_flags(&reg_path, KEY_WRITE)?;
    hkcu.delete_subkey_all(&reg_path)?;
    Ok(())
}

// Currently only working with REG_SZ

#[cfg(target_os = "windows")]
fn modify_winreg_value(path: String, name: String, value: String) -> std::io::Result<()> {
    println!("test");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_path = Path::new(&path);
    let key = hkcu.open_subkey_with_flags(&reg_path, KEY_WRITE)?;

    key.set_value(name, &value)?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn delete_winreg_value(path: String, name: String) -> std::io::Result<()> {
    println!("test");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let reg_path = Path::new(&path);
    let key = hkcu.open_subkey_with_flags(&reg_path, KEY_WRITE)?;

    key.delete_value(&name)?;
    Ok(())
}
