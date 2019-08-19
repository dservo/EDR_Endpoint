# EDR_Endpoint
A simple program to help make active noise on a machine to test detection methods. program can run from command line or used in a script such as bash.

USAGE:
    edr_endpoint [FLAGS] [OPTIONS]

FLAGS:
    -C               Create File or Reg Key/value
    -D               Delete File or Reg Key/value
    -h, --help       Prints help information
    -M               Modify File or Reg Key/value with random data If no file modify will create one
    -V, --version    Prints version information

OPTIONS:
    -F, --FILE <FILE_PATH>                      Depending on flags will create, delete or modify.
    -N, --NETSEND <IP_ADDRESS> <PORT> <DATA>    Sends string of data to server over TCP.
    -P, --PROCESS <PROCESS_PATH>                Starts an executable.

Network:
  will open a port and send a stream of bytes to a given destination server currently works with simple server from example [bottom of read me]
    example edr_endpoint -N 127.0.0.1 3333 hello_world

File:
 create - will make a file at a given path and do nothing if already exists
   example  edr_endpoint -F foo.txt -C
 modify - will append hello world to the end of a given file if no file will create one at given path
   example  edr_endpoint -F foo.txt -M
 delete - will delete a given file
   example  edr_endpoint -F foo.txt -d

Process:
  will start an external command or exe and forward arguments to new process.
  to forward add ": before the arguments section and a " after
  null arguments simply use ":"
    example  edr_endpoint -P ls ":-a"

[windows only]
Registry:
  will create or delete a registry key in HKEY_CURRENT_USER also can Modify and delete key values
  modify will create a key if one is not present already. values only creates and modifies  REG_SZ values
  create key-
    example  edr_endpoint -R \foo\bar -C
  delete key-
    example  edr_endpoint -R \foo\bar -D
    [use caution program will crash if value is not present]
  modify/create value -
    example  edr_endpoint -R \foo\bar -V Hello world -M
  delete value -
      example  edr_endpoint -R \foo\bar -V Hello * -D  [use wild card as second argument]
      [use caution program will crash if value is not present]



logging: every time the program successfully runs will append a new event
to edr_endpoint_log.tsv in the directory that the program is run from.

every log event begins with
[time stamp] [username] [pid] [full program path] [[cli arguments]] [requested cmd]
every log event ends with EOE - End Of Event


file arguments will add
[file status] == Exists Create and No_File
[file info]
[cmd activity]

network argument will add source and destination connection information  printed out from TcpStream and number of bytes sent
[[tcp stream information]] [BYTES:n]


[todo]
format readme
figure out multiple file source to split things up better
explore yml file (concat) to save on double edits
recursive calling of this program has fun with log event
fix all debug formatters


[rust server example]

use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
