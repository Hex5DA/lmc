mod emulator;
mod errors;
mod sasm;

use std::{process::exit, env::args};
use emulator::LMC;

fn main() {
    // Too small a use-case to really use clap so this is fine
    if args().len() < 2 {
        eprintln!("Please pass in the path to the sasm program!");
        exit(1);
    }

    let path = args().collect::<Vec<String>>()[1].clone();
    let mut lmc = LMC::new();
    
    let compiled = sasm::process(path).unwrap_or_else(|err| {
        eprintln!("Error while processsing sasm file: {}", err);
        exit(1);
    });

    println!("Loading the program into memory..");
    if let Err(err) = lmc.load(compiled) {
        eprintln!("An error occured whilst loading the program: {}", err);
        exit(1);
    }    

    println!("Running..");
    if let Err(err) = lmc.run() {
        eprintln!("An error occured whilst running the program: {}", err);
        exit(1);
    }

    println!("Program finished.")
}
