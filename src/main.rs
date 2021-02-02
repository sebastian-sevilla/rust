use std::env;
use std::fs;
use csv::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]

//Data type to store info into categories
struct Record {
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
    nombre: String,
    genero: String,
    estado_civil: String,
    fecha_nacimiento: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    telefono: Option<u64>,
    direccion: String,
    email: String,
}

use csv::ReaderBuilder;

fn main() -> Result<(), Error> {
    //read input fron command line
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("Archivo: {}\n", filename);

    //Read file and return a string
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    //Print content of string from csv file
    //println!("\n{}", contents);

    //Read csv from string
    let mut reader = ReaderBuilder::new().delimiter(b';').has_headers(false).from_reader(contents.as_bytes());

    for result in reader.deserialize::<Record>() {
        println!("{:?}", result?);
    }

    Ok(())
}

//Process input from terminal into filename
fn parse_config(args: &[String]) -> String {
    let filename = args[1].clone();

    filename
}