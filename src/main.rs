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
    pattern: Vec<String>,
}


#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Thing{
    name: String,
    shape: Option<String>,
    key: String,
}

pub fn delete(deletion: Thing, path: &str, table: &mut HashTableMapThing<String, String>) -> Result<(), Box<dyn Error>> {

    let mut print_ar: Vec<Thing> = vec![];

    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();

    let mut reader = csv::Reader::from_reader(file);

    for line in reader.records(){
        let record = line?;
        let thing = Thing{
            name: record[0].to_string(),
            shape: Some(record[1].to_string()),
            key: record[2].to_string()
        };

        if !record[2].to_string().eq(&deletion.key) { //&deletion.Shape.as_ref().unwrap()
            print_ar.push(thing);
        }
    }

    let mut new_file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .unwrap();

    let mut writer = csv::Writer::from_writer(new_file);

    writer.serialize(("Name", "Shape", "Key"));

    for c in print_ar {
        writer.serialize((&c.name, &c.shape, &c.key))?;
    }

    table.delete(deletion.key);

    writer.flush()?;

    Ok(())
}




//I decided that it should save Key in the value since aswell
//Since only value will be saved in CSV file
//And if you want to recreate and load the hashtablemapthing again from the file the keys has to be in the file
//You could probably do this in some smart way
//However the ability to make your own keys for stuff you put in is according to me a good feature
//You could have a small function that takes like the first three letters of all values and makes key
//But that would be hard to remember so I did not do that
pub fn insert(insertion: Thing, file: &File, table: &mut HashTableMapThing<String, String>) -> Result<(), Box<dyn Error>> {


    if !table.contains(&insertion.key){

        let serialized_insertion:String = serde_json::to_string(&insertion).unwrap();

        let mut wtr = csv::Writer::from_writer(io::stdout());   
        let mut cvs_writer = csv::Writer::from_writer(file);
            
        cvs_writer.serialize((&insertion.name, &insertion.shape, &insertion.key))?;
        
        table.insert(insertion.key, serialized_insertion);
    
        cvs_writer.flush()?;
        wtr.flush()?;
    }

    Ok(())
}


pub fn print_values(table: &HashTableMapThing<String, String>) {
    let print_vec = table.print();
    for val in print_vec {
        let print_val:Thing = serde_json::from_str(val).unwrap();
        println!("{:#?}",print_val);
    }
}


pub fn load(file: &File, args: &Cli, table: &mut HashTableMapThing<String, String>) -> Result<(), Box<dyn Error>> {
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
            name: record[0].to_string(),
            shape: Some(record[1].to_string()),
            key: record[2].to_string(),
        };
        //writer.serialize((&thing.Name, &thing.Shape))?;
        let serialized_thing:String = serde_json::to_string(&thing).unwrap();
        table.insert(thing.key, serialized_thing);
    }

    Ok(())
}

pub fn get_value(search: &String, table: &mut HashTableMapThing<String, String>) {


    let value:Thing = serde_json::from_str(table.get(search).unwrap()).unwrap();

    println!("{:#?}", value);

}



//Why am i actually doing a hashtable ???
//Im just saving to the file and closing the program?
//What point does the hashtable have??
//I started doing a loop cause then it would actually serve some purpose, but i guess if you can have a program with the hashtable in the background running then it would be fine??
//But i removed loop :(
//But is quite easy to implement again, since i saved the code

fn main() {
    let args = Cli::parse();   
    let file_path = "data.csv"; 

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&file_path)
        .unwrap();; 

    let mut table: HashTableMapThing<String, String> = HashTableMapThing::new();

    load(&file, &args, &mut table);

    if args.pattern.len() < 1 {
        println!("");
        println!("Error: no valid argument was given");
        println!("");
        process::exit(1);
    }

    //let command: Vec<String> = args.pattern.split_whitespace().map(str::to_string).collect::<Vec<String>>();

    match args.pattern[0].as_str() {
        "load" => {
            
        },"new" => {

            let mut cvs_writer = csv::Writer::from_writer(&file);
        
            cvs_writer.serialize(("Name", "Shape", "Key"));

        },"insert" => {    //Should probably make it so you say for example "insert --name --shape"
            if args.pattern.len() == 4{
                let thing = Thing{
                    name: args.pattern[1].to_string(),
                    shape: Some(args.pattern[2].to_string()),
                    key: args.pattern[3].to_string(),
                };
                insert(thing, &file ,&mut table);
            }else{
                println!("Invalid input...");
                println!("Input should be in format: insert --name --shape --key");
            }   
        },"delete" => {
            if args.pattern.len() == 2 {
                let thing = Thing{
                    name: "".to_string(),
                    shape: None,
                    key: args.pattern[1].to_string(),
                };
                delete(thing, &file_path,&mut table);
            }else{
                println!("Invalid input...");
                println!("Input should be in format: delete --key");
            }
        },"print" => {
                print_values(&table);
        },"get" => {
            if args.pattern.len() == 2 {
                get_value(&args.pattern[1].to_string(), &mut table);
            }else{
                println!("Invalid input...");
                println!("Input should be in format: get --key");
            }
        },
        _ => {
            println!("");
            println!("Error: no valid argument was given");
            println!("");
            process::exit(1);
        },
    }

}
