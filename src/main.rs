use std::fs;

use solidity_decompiler::opcode::Processor;

fn main() {
    let solidity = fs::read_to_string("contracts/contract.evm").unwrap();
    let bytecode = hex::decode(solidity).unwrap();

    let contract = Processor::new(&bytecode);
    contract.print();
    contract.write("dst/gen.txt");
    let sigs = &contract.func_sigs();
    // print each element in the vector as hexadecimal
    println!("Contracts possible function signatures: ");
    for sig in sigs {
        println!("0x{:x?}", sig);
    }
}
