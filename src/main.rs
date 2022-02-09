#![allow(unused)]

use std::error::Error;
use std::io;
use std::process;

use std::fs::File;
use std::fs::OpenOptions;

use serde::Deserialize;
use serde::Serialize;

use clap::Parser;

#[derive(Parser)]
pub struct Cli {

    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Serialize)]
pub struct Thing<'a>{
    name: &'a String,
    shape: &'a Option<String>,
}


pub fn insert(insertion: Thing, args: Cli) -> Result<(), Box<dyn Error>> {
    
    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(args.path)
            .unwrap();


    let mut wtr = csv::Writer::from_writer(io::stdout());
    //let mut cvs_writer = csv::Writer::from_writer(file);

    wtr.serialize(insertion)?;

    wtr.flush()?;
    Ok(())
}


fn main() {
    let args = Cli::parse();    
    //let command: Vec<String> = args.pattern.split_whitespace().map(str::to_string).collect::<Vec<String>>();
    println!("{}, {:?}", args.pattern, args.path);

    match args.pattern.as_str() {
        "insert" => {insert(Thing{
            name: &"Oskar".to_string(),
            shape: &Some("Human".to_string()),
        }, args);},
        "delete" => {println!("Coolt du ska deleta något");},
        "select" => {println!("Coolt du ska selecta något");},
        "projection" => {println!("Coolt du ska projectera något ???? hello what does this mean even");},
        _ => (),
    }

}
