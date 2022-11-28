mod lib;
use lib::*;

fn main() {
    use Instruction::*;
    
    let prog = vec![
        INP,
        STA(99),
        INP,
        ADD(99),
        OUT,
        HALT,
    ];

    let mut lmc = LMC::new();
    if let Err(err) = lmc.run(prog) {
        println!("An error occured: {}", err);
    }
    println!("Programming finished.")
}
