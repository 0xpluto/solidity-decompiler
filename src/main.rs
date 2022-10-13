use std::fs;

use solidity_decompiler::opcode::Processor;

fn main() {
    let solidity = fs::read_to_string("contracts/contract.txt").unwrap();
    // remove the 0x prefix
    let solidity = &solidity[2..];
    let bytecode = hex::decode(solidity).unwrap();

    let contract = Processor::new(&bytecode);
    contract.print();
    contract.write("dst/gen.txt");
}
