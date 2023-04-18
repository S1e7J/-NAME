use std::error::Error;
use std::fs::File;
// use std::io::prelude::*;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    // Abre el archivo CSV
    // let mut file = File::open("datos.csv")?;
    let file = File::open("datos.csv")?;

    // Lee los datos del archivo CSV
    let mut reader = Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}
