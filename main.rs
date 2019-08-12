#[macro_use]
extern crate clap;
use clap::App;

fn main() {
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
       println!("File location: {}", file);
    }
    //Collect the Net conection information
    if matches.is_present("net") {
    let net: Vec<&str> = matches.values_of("net").unwrap().collect();
       println!("Connection IP: {} PORT: {} File: {} ", net[0], net[1], net[2] );
    }
} // end main
