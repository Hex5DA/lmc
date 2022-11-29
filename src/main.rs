use lmc::*;

fn main() {
    use Instruction::*;
    
    let prog = vec![
        INP,
        STA(99),
        INP,
        STA(98),
        LDA(99),
        ADD(98),
        OUT
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
