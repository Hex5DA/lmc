mod emulator;
mod errors;
mod sasm;

use std::process::exit;
use emulator::LMC;

fn main() {
    let mut lmc = LMC::new();
    
    let compiled = sasm::process("./test.sasm").unwrap_or_else(|err| {
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
