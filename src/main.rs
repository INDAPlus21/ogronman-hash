#![allow(unused)]

use std::error::Error;
use std::io::{stdin,stdout,Write};
use std::io;
use std::process;


use hashlib::*;


use std::fs::File;
use std::fs::OpenOptions;

use serde::Deserialize;
use serde::Serialize;

use serde_json::json;
use serde_json::{Value, Error as JError};

use clap::Parser;

#[derive(Parser, Clone)]
pub struct Cli {

    pattern: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}


#[derive(Serialize, Clone, Debug, Default)]
pub struct Thing{
    Name: String,
    Shape: Option<String>,
}

pub fn delete(deletion: Thing, args: &Cli, table: &mut HashTableMapThing<String, Thing>) -> Result<(), Box<dyn Error>> {

    let mut printAr: Vec<Thing> = vec![];

    let mut file = OpenOptions::new()
        .read(true)
        .open(&args.path)
        .unwrap();

    let mut reader = csv::Reader::from_reader(file);

    for line in reader.records(){
        let record = line?;
        let thing = Thing{
            Name: record[0].to_string(),
            Shape: Some(record[1].to_string()),
        };

        if !record[0].to_string().eq(&deletion.Name) && !record[1].to_string().eq(deletion.Shape.as_ref().unwrap()) { //&deletion.Shape.as_ref().unwrap()
            printAr.push(thing);
        }
    }

    let mut new_file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(&args.path)
        .unwrap();

    let mut writer = csv::Writer::from_writer(new_file);

    writer.serialize(("Name", "Shape"));

    for c in printAr {
        writer.serialize((&c.Name, &c.Shape))?;
    }

    let serialized_thing:String = serde_json::to_string(&deletion).unwrap();

    table.delete(serialized_thing);

    Ok(())
}



pub fn insert(insertion: Thing, file: &File, table: &mut HashTableMapThing<String, Thing>) -> Result<(), Box<dyn Error>> {
    
    let serialized_insertion:String = serde_json::to_string(&insertion).unwrap();

    if !table.contains(&serialized_insertion){

        let mut wtr = csv::Writer::from_writer(io::stdout());   
        let mut cvs_writer = csv::Writer::from_writer(file);
            
        cvs_writer.serialize((&insertion.Name, &insertion.Shape))?;
        
        table.insert(serialized_insertion, insertion);
    
        cvs_writer.flush()?;
        wtr.flush()?;
    }

    Ok(())
}


pub fn load(file: &File, args: &Cli, table: &mut HashTableMapThing<String, Thing>) -> Result<(), Box<dyn Error>> {
    /*let mut new_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(args.path)
        .unwrap();*/

    let mut reader = csv::Reader::from_reader(file);

    //writer.serialize(("Name", "Shape"));

    for line in reader.records(){
        let record = line?;
        let thing = Thing{
            Name: record[0].to_string(),
            Shape: Some(record[1].to_string()),
        };
        //writer.serialize((&thing.Name, &thing.Shape))?;
        let serialized_thing:String = serde_json::to_string(&thing).unwrap();
        table.insert(serialized_thing, thing);
    }

    Ok(())
}

pub fn get_user_input() -> Option<Thing>{
    let mut input = String::new();

    println!("Please enter a valid thing");
    println!("--name --shape");
    input = String::new();
    stdin().read_line(&mut input).expect("Invalid input");
    input = input[0..input.len() - 1].to_string();

    let inputVec = input.split_whitespace().map(str::to_string).collect::<Vec<String>>();

    if inputVec.len() > 1 {

        let returnThing = Thing{
            Name: inputVec[0].to_string(),
            Shape: Some(inputVec[1].to_string()),
        };

        return Some(returnThing);
    }else{
        println!("Invalid input");
        return None;
    }

}


fn main() {
    let args = Cli::parse();    

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&args.path)
        .unwrap();; 

    let mut table: HashTableMapThing<String, Thing> = HashTableMapThing::new();

    
  

    //let command: Vec<String> = args.pattern.split_whitespace().map(str::to_string).collect::<Vec<String>>();

    match args.pattern.as_str() {
        "load" => {
            load(&file, &args,&mut table);
            },
        "new" => {


            let mut cvs_writer = csv::Writer::from_writer(&file);
        
            cvs_writer.serialize(("Name", "Shape"));

        },
        _ => {
            println!("");
            println!("Error: no valid argument was given");
            println!("");
            process::exit(1);
        },
    }

    let mut input = String::new();

    while !input.eq("exit"){

        //Get user input
        input = String::new();
        stdin().read_line(&mut input).expect("Invalid input");
        input = input[0..input.len() - 1].to_string();

        //Match input with things
        match input.as_str() {
            "insert" => {   //Should probably make it so you say for example "insert --name --shape"
                let thing = get_user_input().unwrap();
                insert(thing, &file ,&mut table);
            },
            "delete" => {
                let thing = get_user_input().unwrap();
                delete(thing, &args ,&mut table);
            },
            "print" => {
                &table.print();
            }
            _ => (),
        }

        stdout().flush().unwrap();
    }



}
