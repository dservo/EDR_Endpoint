#[macro_use]
extern crate clap;
extern crate chrono;
extern crate whoami;
extern crate which;

#[cfg(target_os = "windows")]
extern crate winreg;

use chrono::Utc;
use clap::App;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::process::{self, Command};
use std::{env, net::TcpStream, path::Path};
use whoami::*;
use which::which;

static LOG_FILE: &str = "edr_endpoint_log.tsv";

// [NTS] remember can make use more compact
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

fn main() -> std::io::Result<()> {
    // YAML Method for CLI OPTS to have diffrent options
    // note the targeted compile fells wierd  but this works atm
    // have to figure out a method to add the win file to nix to avoid double edit
    let args: Vec<String> = env::args().collect();
    #[cfg(target_os = "windows")]
    let yaml = load_yaml!("cli_win.yml");
    #[cfg(not(target_os = "windows"))]
    let yaml = load_yaml!("cli_nix.yml");
    let matches = App::from_yaml(yaml).get_matches();
    log_entry_start()?;
    log_entry_append(format!("{:?}", args))?;
    // get values for option supplied from command line or fall to default
    if matches.is_present("process") {
        log_entry_append("Process".to_string())?;
        let process: Vec<&str> = matches.values_of("process").unwrap().collect();
        create_prosess(process[0].to_string(), process[1].to_string())?;
    }
    if matches.is_present("file") {
        log_entry_append("File".to_string())?;
        let file = matches.value_of("file").unwrap_or("default.txt");
        if matches.is_present("create") {
            create_file(file.to_string())?;
            log_entry_append("Create".to_string())?;
        } else if matches.is_present("delete") {
            remove_file(file.to_string())?;
            log_entry_append("Delete".to_string())?;
        } else if matches.is_present("modify") {
            modify_file(file.to_string(), "hello world".to_string())?;
            log_entry_append("Modify".to_string())?;
        } else {
            log_entry_append("Nothing_Done".to_string())?;
        }
    }
    //Collect the Net conection information
    if matches.is_present("net") {
        log_entry_append("Network".to_string())?;
        let net: Vec<&str> = matches.values_of("net").unwrap().collect();
        tcp_send(net[0].to_string(), net[1].to_string(), net[2].to_string())?;
    }
    #[cfg(target_os = "windows")]
    win_cli(matches)?;
    log_entry_finish()?;
    Ok(())
} // end main

#[cfg(target_os = "windows")]
fn win_cli(matches: clap::ArgMatches) -> std::io::Result<()> {
    if matches.is_present("reg") {
        log_entry_append(format!("Regristry"))?;
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
            log_entry_append(format!("Nothing_Done"))?;
        }
    }
    Ok(())
}

fn check_file(path: String) -> bool {
    Path::new(&path.clone()).exists()
}
fn log_file_path(path: String) -> std::io::Result<()> {
    let file = File::open(path.clone())?;
    log_entry_append(format!("{:?}", file))?;
    Ok(())
}

fn create_file(path: String) -> std::io::Result<()> {
    if !check_file(path.clone()) {
        log_entry_append("Create".to_string())?;
        let file = File::create(path)?;
        //till printing elements of a strucures is fully understoon debug drived formant will do
        log_entry_append(format!("{:?}", file))?;
    } else {
        log_entry_append("Exists".to_string())?;
        log_file_path(path)?;
    }
    Ok(())
}

fn remove_file(path: String) -> std::io::Result<()> {
    if check_file(path.clone()) {
        log_entry_append("Exists".to_string())?;
        log_file_path(path.clone())?;
        fs::remove_file(path)?;
    } else {
        log_entry_append("No_File".to_string())?;
    }
    Ok(())
}

fn modify_file(path: String, data: String) -> std::io::Result<()> {
    let mut file_check = check_file(path.clone());
    let was_file_created = if !file_check {
        create_file(path.clone())?;
        file_check = true;
        true
    } else {
        false
    };
    if file_check {
        let mut file = OpenOptions::new().append(true).open(path.clone()).unwrap();
        if !was_file_created {
            log_entry_append("Exists".to_string())?;
            log_entry_append(format!("{:?}", file))?;
        }
        writeln!(file, "{}", data)?;
    }
    Ok(())
}

// since prgram only runs one command at a time the program will generate log info as it runs
fn log_entry_start() -> std::io::Result<()> {
    let log = LOG_FILE.to_string();
    let mut file_check = Path::new(&log.clone()).exists();
    if !file_check {
        File::create(log.clone())?;
        file_check = true;
    }
    if file_check {
        let mut file = OpenOptions::new().append(true).open(log.clone()).unwrap();
        write!(
            file,
            "[{}]\t{}\t{}\t{:#?}\t",
            Utc::now(),
            username(),
            process::id(),
            std::env::current_exe().unwrap()
        )?;
    }
    Ok(())
}

fn log_entry_append(log_data: String) -> std::io::Result<()> {
    let log = LOG_FILE.to_string();
    let mut file = OpenOptions::new().append(true).open(log.clone()).unwrap();
    write!(file, "{}\t", log_data)?;
    Ok(())
}

fn log_entry_finish() -> std::io::Result<()> {
    let log = LOG_FILE.to_string();
    let mut file = OpenOptions::new().append(true).open(log.clone()).unwrap();
    writeln!(file, "EOE")?;
    Ok(())
}

fn create_prosess(path: String, args: String) -> std::io::Result<()> {
    let args = args.replace(&['\"', ':'][..], "");
     //clippy gives error here or ok() expect() reaserch furthur to fix
    let process_output = Command::new(path.clone())
        .args(&[args])
        .spawn()
        .ok()
        .expect("Failed to execute.");
    let result = which(path).unwrap();
    log_entry_append(format!("{:?}", result))?;
    log_entry_append(format!("{}", process_output.id()))?;
    Ok(())
}

fn tcp_send(ip_address: String, port: String, data: String) -> std::io::Result<()> {
    if let Ok(mut tcp_stream) = TcpStream::connect(format!("{}:{}", ip_address, port)) {
        log_entry_append(format!("[{:?}]", tcp_stream))?;
        let sent = tcp_stream.write(data.as_bytes())?;
        log_entry_append(format!("BYTES:{}", sent))?;
    } else {
        log_entry_append("NO_SERVER".to_string())?;
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
