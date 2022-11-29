mod lib;
use lib::*;

fn main() {
    use Instruction::*;
    
    let prog = vec![
        INP,
        STA(99), // 'count'
        INP,
        STA(98), // 'step'
        LDA(99),
        BRZ(10),
        SUB(98),
        OUT,
        STA(99),
        BRA(5),
        HLT,
    ];

    let mut lmc = LMC::new();
    
    println!("Loading the program into memory..");
    if let Err(err) = lmc.load(prog) {
        println!("An error occured whilst loading the program: {}", err);
    }    

    println!("Running..");
    if let Err(err) = lmc.run() {
        println!("An error occured whilst running the program: {}", err);
    }

    println!("Program finished.")
}
