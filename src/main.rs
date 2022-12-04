pub mod emulator;
pub mod errors;
pub mod sasm;

use emulator::LMC;

fn main() {
    let mut lmc = LMC::new();
    
    let compiled = sasm::process("./test.sasm").expect("Expand on this error handling!");

    println!("Loading the program into memory..");
    if let Err(err) = lmc.load(compiled) {
        println!("An error occured whilst loading the program: {}", err);
    }    

    println!("Running..");
    if let Err(err) = lmc.run() {
        println!("An error occured whilst running the program: {}", err);
    }

    println!("Program finished.")
}
