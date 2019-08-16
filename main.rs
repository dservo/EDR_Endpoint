#[macro_use]
extern crate clap;
extern crate chrono;

#[cfg(target_os = "windows")]
    extern crate winreg;

use chrono::{DateTime, Utc};
use clap::App;
use std::fs::{self , OpenOptions, File};
use std::path::Path;
use std::process::{Command, Stdio}; // self may be nedded
use std::io::{self, BufRead, BufReader, Write, Error, ErrorKind}; // read notused at this time
use std::net::TcpStream;

// confused if #[cfg(target_os = "windows")] is single line only for this type of implenetation
// but being safe atm
#[cfg(target_os = "windows")]
    use winreg::enums::*;
#[cfg(target_os = "windows")]
    use winreg::RegKey;

fn main() -> std::io::Result<()>  {
    // YAML Method for CLI OPTS
    // note the targeted compile fells wierd  but this works atm
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
      tcp_send(net[0].to_string(), net[1].to_string(), net[2].to_string())?;
    }
    if cfg!(target_os = "windows") {
      if matches.is_present("reg") {
        let reg_key = matches.value_of("reg").unwrap_or("default.txt");
        if matches.is_present("create") {
          win_reg_read()?;
        }else if matches.is_present("delete") {
        win_reg_read()?;
        }else if matches.is_present("modify") {
        win_reg_read()?;
        }else {
          println!("Nothing Done about {}" , reg_key.to_string());
        }
      }
    }
    Ok(())
} // end main

fn create_file(path: String) -> std::io::Result<()> {
    println!("Created file: {}", path);
    let _file = File::create(path)?;
    Ok(())
}

fn remove_file(path: String) -> std::io::Result<()> {
    let file_check = Path::new(&path.clone()).exists();
    if file_check == true {
      println!("Deleted file: {}", path);
      fs::remove_file(path)?;
    } else {
      println!("{} is not a file dooning nothing" , path);
    }
    Ok(())
}

fn modify_file(path: String) -> std::io::Result<()> {
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

fn tcp_send (ip_address: String, port: String, file: String) -> std::io::Result<()> {
    println!("Connection Information IP: {} PORT: {} File: {} ", ip_address, port, file);
    let connection = format!("{}:{}",ip_address , port);
    if let Ok(mut tcp_stream) = TcpStream::connect (connection.clone()) {
        println!("Connection to server @ : {}", connection);
        tcp_stream.write(b"hello world")?;
    } else {
        println!("Error No server @: {}", connection);
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn win_reg_read() -> std::io::Result<()>{

    println!("Reading some system info...");
      let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
      let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
      let pf: String = cur_ver.get_value("ProgramFilesDir")?;
      let dp: String = cur_ver.get_value("DevicePath")?;
      println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
      let info = cur_ver.query_info()?;
      println!("info = {:?}", info);
      let mt = info.get_last_write_time_system();
      println!(
          "last_write_time as winapi::um::minwinbase::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
          mt.wYear, mt.wMonth, mt.wDay, mt.wHour, mt.wMinute, mt.wSecond
      );

      // enable `chrono` feature on `winreg` to make this work
      // println!(
      //     "last_write_time as chrono::NaiveDateTime = {}",
      //     info.get_last_write_time_chrono()
      // );

      println!("And now lets write something...");
      let hkcu = RegKey::predef(HKEY_CURRENT_USER);
      let path = Path::new("Software").join("WinregRsExample1");
      let (key, disp) = hkcu.create_subkey(&path)?;

      match disp {
          REG_CREATED_NEW_KEY => println!("A new key has been created"),
          REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
      }

      key.set_value("TestSZ", &"written by Rust")?;
      let sz_val: String = key.get_value("TestSZ")?;
      key.delete_value("TestSZ")?;
      println!("TestSZ = {}", sz_val);

      key.set_value("TestDWORD", &1234567890u32)?;
      let dword_val: u32 = key.get_value("TestDWORD")?;
      println!("TestDWORD = {}", dword_val);

      key.set_value("TestQWORD", &1234567891011121314u64)?;
      let qword_val: u64 = key.get_value("TestQWORD")?;
      println!("TestQWORD = {}", qword_val);

      key.create_subkey("sub\\key")?;
      hkcu.delete_subkey_all(&path)?;

      println!("Trying to open nonexistent key...");
      hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
          io::ErrorKind::NotFound => panic!("Key doesn't exist"),
          io::ErrorKind::PermissionDenied => panic!("Access denied"),
          _ => panic!("{:?}", e),
      });
      Ok(())
} // end win_reg_read
