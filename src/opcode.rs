#![allow(unused)]

use std::collections::LinkedList;
use std::io::Write;

pub struct Opcode {
    opcode: u8,
    gas: u32,
    push_bytes: Option<Vec<u8>>,
    name: &'static str,
}

pub struct Processor {
    pub opcodes: Vec<Opcode>,
    pub program_counter: Vec<u16>,
    pub bytecode: Vec<u8>,
}

impl Processor {
    pub fn new(bytecode: &Vec<u8>) -> Processor {
        let mut opcodes = vec![];
        let mut program_cntr: Vec<u16> = vec![];

        let mut pc: u16 = 0;
        let mut i: usize = 0;
        while i < bytecode.len() {
            let opcode = Opcode::from_opcode(bytecode[i]);
            if bytecode[i] >= 0x60 && bytecode[i] <= 0x7f {
                let n = (bytecode[i] - 0x5f) as usize;

                let mut push_bytes = vec![];
                for j in i..(i+n) {
                    push_bytes.push(bytecode[j+1])
                }
                i += n;

                opcodes.push(opcode.push_bytes(push_bytes));
                program_cntr.push(pc);
                pc += 1 + n as u16;

            } else {
                opcodes.push(opcode);
                program_cntr.push(pc);
                pc += 1;
            }
            i += 1;
        }
        Processor { 
            opcodes: opcodes, 
            program_counter: program_cntr,
            bytecode: bytecode.to_vec() 
        }
    }
    pub fn func_sigs(&self) -> Vec<u32> {
        let mut sigs = vec![];
        for opcode in &self.opcodes {
            // check if the opcode is a PUSH4
            if opcode.opcode == 0x63 {
                // change the push_bytes to a u16
                let mut sig: u32 = 0;
                for i in 0..4 {
                    sig += (opcode.push_bytes.as_ref().unwrap()[3-i] as u32) << (8 * i);
                }
                // do not push the value if it is the max for u32
                if sig != 0xffffffff {
                    sigs.push(sig);
                }
            }
        }
        sigs
    }

    // Viewing functions
    pub fn print(&self) {
        // loop over the vector of opcodes
        for (i, opcode) in self.opcodes.iter().enumerate() {
            match &opcode.push_bytes {
                Some(x) => {
                    println!("PC[{:x?}]: {} 0x{}", self.program_counter[i], opcode.name, hex::encode(x));
                },
                None => {
                    println!("PC[{:x?}]: {}", self.program_counter[i], opcode.name);
                },
            }
        }
        println!();
    }
    pub fn write(&self, filename: &str) {
        let mut file = std::fs::File::create(filename).unwrap();
        let mut contents = String::new();
        for (i, opcode) in self.opcodes.iter().enumerate() {
            match &opcode.push_bytes {
                Some(x) => {
                    contents.push_str(&format!("PC[{:x?}]: {} 0x{}\n", self.program_counter[i], opcode.name, hex::encode(x)));
                },
                None => {
                    contents.push_str(&format!("PC[{:x?}]: {}\n", self.program_counter[i], opcode.name));
                },
            }
        }
        file.write_all(contents.as_bytes()).unwrap();
    }
}

impl Opcode {
    pub const fn new(opcode: u8, gas: u32, name: &'static str) -> Opcode {
        Opcode { opcode, gas, name, push_bytes: None }
    }

    pub fn get_opcode(&self) -> u8 {
        self.opcode
    }

    pub fn get_gas(&self) -> u32 {
        self.gas
    }

    pub fn push_bytes(&self, bytes: Vec<u8>) -> Self {
        // assert that the opcode is within the range of 0x60 to 0x7f
        match self.opcode {
            0x60..=0x7f => Self { opcode: self.opcode, gas: self.gas, name: self.name, push_bytes: Some(bytes) },
            _ => panic!(),
        }
    }

    pub fn from_opcode(opcode: u8) -> Opcode {
        match opcode {
            0x00 => STOP,
            0x01 => ADD,
            0x02 => MUL,
            0x03 => SUB,
            0x04 => DIV,
            0x05 => SDIV,
            0x06 => MOD,
            0x07 => SMOD,
            0x08 => ADDMOD,
            0x09 => MULMOD,
            0x0a => EXP,
            0x0b => SIGNEXTEND,

            0x10 => LT,
            0x11 => GT,
            0x12 => SLT,
            0x13 => SGT,
            0x14 => EQ,
            0x15 => ISZERO,
            0x16 => AND,
            0x17 => OR,
            0x18 => XOR,
            0x19 => NOT,
            0x1a => BYTE,
            0x1b => SHL,
            0x1c => SHR,
            0x1d => SAR,

            0x20 => SHA3,

            0x30 => ADDRESS,
            0x31 => BALANCE,
            0x32 => ORIGIN,
            0x33 => CALLER,
            0x34 => CALLVALUE,
            0x35 => CALLDATALOAD,
            0x36 => CALLDATASIZE,
            0x37 => CALLDATACOPY,
            0x38 => CODESIZE,
            0x39 => CODECOPY,
            0x3a => GASPRICE,
            0x3b => EXTCODESIZE,
            0x3c => EXTCODECOPY,
            0x3d => RETURNDATASIZE,
            0x3e => RETURNDATACOPY,
            0x3f => EXTCODEHASH,

            0x40 => BLOCKHASH,
            0x41 => COINBASE,
            0x42 => TIMESTAMP,
            0x43 => NUMBER,
            0x44 => DIFFICULTY,
            0x45 => GASLIMIT,
            0x46 => CHAINID,
            0x47 => SELFBALANCE,
            0x48 => BASEFEE,

            0x50 => POP,
            0x51 => MLOAD,
            0x52 => MSTORE,
            0x53 => MSTORE8,
            0x54 => SLOAD,
            0x55 => SSTORE,
            0x56 => JUMP,
            0x57 => JUMPI,
            0x58 => PC,
            0x59 => MSIZE,
            0x5a => GAS,
            0x5b => JUMPDEST,

            0x60 => PUSH1,
            0x61 => PUSH2,
            0x62 => PUSH3,
            0x63 => PUSH4,
            0x64 => PUSH5,
            0x65 => PUSH6,
            0x66 => PUSH7,
            0x67 => PUSH8,
            0x68 => PUSH9,
            0x69 => PUSH10,
            0x6a => PUSH11,
            0x6b => PUSH12,
            0x6c => PUSH13,
            0x6d => PUSH14,
            0x6e => PUSH15,
            0x6f => PUSH16,

            0x70 => PUSH17,
            0x71 => PUSH18,
            0x72 => PUSH19,
            0x73 => PUSH20,
            0x74 => PUSH21,
            0x75 => PUSH22,
            0x76 => PUSH23,
            0x77 => PUSH24,
            0x78 => PUSH25,
            0x79 => PUSH26,
            0x7a => PUSH27,
            0x7b => PUSH28,
            0x7c => PUSH29,
            0x7d => PUSH30,
            0x7e => PUSH31,
            0x7f => PUSH32,

            0x80 => DUP1,
            0x81 => DUP2,
            0x82 => DUP3,
            0x83 => DUP4,
            0x84 => DUP5,
            0x85 => DUP6,
            0x86 => DUP7,
            0x87 => DUP8,
            0x88 => DUP9,
            0x89 => DUP10,
            0x8a => DUP11,
            0x8b => DUP12,
            0x8c => DUP13,
            0x8d => DUP14,
            0x8e => DUP15,
            0x8f => DUP16,

            0x90 => SWAP1,
            0x91 => SWAP2,
            0x92 => SWAP3,
            0x93 => SWAP4,
            0x94 => SWAP5,
            0x95 => SWAP6,
            0x96 => SWAP7,
            0x97 => SWAP8,
            0x98 => SWAP9,
            0x99 => SWAP10,
            0x9a => SWAP11,
            0x9b => SWAP12,
            0x9c => SWAP13,
            0x9d => SWAP14,
            0x9e => SWAP15,
            0x9f => SWAP16,

            0xa0 => LOG0,
            0xa1 => LOG1,
            0xa2 => LOG2,
            0xa3 => LOG3,
            0xa4 => LOG4,

            0xb0 => CREATE,
            0xb1 => CALL,
            0xb2 => CALLCODE,
            0xb3 => RETURN,

            0xf0 => CREATE2,
            0xf1 => CALL,
            0xf2 => CALLCODE,
            0xf3 => RETURN,
            0xf4 => DELEGATECALL,
            0xf5 => CREATE2,
            0xfa => STATICCALL,
            0xfd => REVERT,
            0xfe => INVALID,
            0xff => SELFDESTRUCT,

            _ => INVALID,
        }
    }
}

const STOP: Opcode = Opcode::new(0x00, 0, "STOP");
const ADD: Opcode = Opcode::new(0x01, 3, "ADD");
const MUL: Opcode = Opcode::new(0x02, 5, "MUL");
const SUB: Opcode = Opcode::new(0x03, 3, "SUB");
const DIV: Opcode = Opcode::new(0x04, 5, "DIV");
const SDIV: Opcode = Opcode::new(0x05, 5, "SDIV");
const MOD: Opcode = Opcode::new(0x06, 5, "MOD");
const SMOD: Opcode = Opcode::new(0x07, 5, "SMOD");
const ADDMOD: Opcode = Opcode::new(0x08, 8, "ADDMOD");
const MULMOD: Opcode = Opcode::new(0x09, 8, "MULMOD");
const EXP: Opcode = Opcode::new(0x0a, 10, "EXP");
const SIGNEXTEND: Opcode = Opcode::new(0x0b, 5, "SIGNEXTEND");

const LT: Opcode = Opcode::new(0x10, 3, "LT");
const GT: Opcode = Opcode::new(0x11, 3, "GT");
const SLT: Opcode = Opcode::new(0x12, 3, "SLT");
const SGT: Opcode = Opcode::new(0x13, 3, "SGT");
const EQ: Opcode = Opcode::new(0x14, 3, "EQ");
const ISZERO: Opcode = Opcode::new(0x15, 3, "ISZERO");
const AND: Opcode = Opcode::new(0x16, 3, "AND");
const OR: Opcode = Opcode::new(0x17, 3, "OR");
const XOR: Opcode = Opcode::new(0x18, 3, "XOR");
const NOT: Opcode = Opcode::new(0x19, 3, "NOT");
const BYTE: Opcode = Opcode::new(0x1a, 3, "BYTE");
const SHL: Opcode = Opcode::new(0x1b, 3, "SHL");
const SHR: Opcode = Opcode::new(0x1c, 3, "SHR");
const SAR: Opcode = Opcode::new(0x1d, 3, "SAR");

const SHA3: Opcode = Opcode::new(0x20, 30, "SHA3");

const ADDRESS: Opcode = Opcode::new(0x30, 2, "ADDRESS");
const BALANCE: Opcode = Opcode::new(0x31, 20, "BALANCE");
const ORIGIN: Opcode = Opcode::new(0x32, 2, "ORIGIN");
const CALLER: Opcode = Opcode::new(0x33, 2, "CALLER");
const CALLVALUE: Opcode = Opcode::new(0x34, 2, "CALLVALUE");
const CALLDATALOAD: Opcode = Opcode::new(0x35, 3, "CALLDATALOAD");
const CALLDATASIZE: Opcode = Opcode::new(0x36, 2, "CALLDATASIZE");
const CALLDATACOPY: Opcode = Opcode::new(0x37, 3, "CALLDATACOPY");
const CODESIZE: Opcode = Opcode::new(0x38, 2, "CODESIZE");
const CODECOPY: Opcode = Opcode::new(0x39, 3, "CODECOPY");
const GASPRICE: Opcode = Opcode::new(0x3a, 2, "GASPRICE");
const EXTCODESIZE: Opcode = Opcode::new(0x3b, 20, "EXTCODESIZE");
const EXTCODECOPY: Opcode = Opcode::new(0x3c, 20, "EXTCODECOPY");
const RETURNDATASIZE: Opcode = Opcode::new(0x3d, 2, "RETURNDATASIZE");
const RETURNDATACOPY: Opcode = Opcode::new(0x3e, 3, "RETURNDATACOPY");
const EXTCODEHASH: Opcode = Opcode::new(0x3f, 100, "EXTCODEHASH");

const BLOCKHASH: Opcode = Opcode::new(0x40, 20, "BLOCKHASH");
const COINBASE: Opcode = Opcode::new(0x41, 2, "COINBASE");
const TIMESTAMP: Opcode = Opcode::new(0x42, 2, "TIMESTAMP");
const NUMBER: Opcode = Opcode::new(0x43, 2, "NUMBER");
const DIFFICULTY: Opcode = Opcode::new(0x44, 2, "DIFFICULTY");
const GASLIMIT: Opcode = Opcode::new(0x45, 2, "GASLIMIT");
const CHAINID: Opcode = Opcode::new(0x46, 2, "CHAINID");
const SELFBALANCE: Opcode = Opcode::new(0x47, 5, "SELFBALANCE");
const BASEFEE: Opcode = Opcode::new(0x48, 2, "BASEFEE");

const POP: Opcode = Opcode::new(0x50, 2, "POP");
const MLOAD: Opcode = Opcode::new(0x51, 3, "MLOAD");
const MSTORE: Opcode = Opcode::new(0x52, 3, "MSTORE");
const MSTORE8: Opcode = Opcode::new(0x53, 3, "MSTORE8");
const SLOAD: Opcode = Opcode::new(0x54, 50, "SLOAD");
const SSTORE: Opcode = Opcode::new(0x55, 0, "SSTORE");
const JUMP: Opcode = Opcode::new(0x56, 8, "JUMP");
const JUMPI: Opcode = Opcode::new(0x57, 10, "JUMPI");
const PC: Opcode = Opcode::new(0x58, 2, "PC");
const MSIZE: Opcode = Opcode::new(0x59, 2, "MSIZE");
const GAS: Opcode = Opcode::new(0x5a, 2, "GAS");
const JUMPDEST: Opcode = Opcode::new(0x5b, 1, "JUMPDEST");

const PUSH1: Opcode = Opcode::new(0x60, 3, "PUSH1");
const PUSH2: Opcode = Opcode::new(0x61, 3, "PUSH2");
const PUSH3: Opcode = Opcode::new(0x62, 3, "PUSH3");
const PUSH4: Opcode = Opcode::new(0x63, 3, "PUSH4");
const PUSH5: Opcode = Opcode::new(0x64, 3, "PUSH5");
const PUSH6: Opcode = Opcode::new(0x65, 3, "PUSH6");
const PUSH7: Opcode = Opcode::new(0x66, 3, "PUSH7");
const PUSH8: Opcode = Opcode::new(0x67, 3, "PUSH8");
const PUSH9: Opcode = Opcode::new(0x68, 3, "PUSH9");
const PUSH10: Opcode = Opcode::new(0x69, 3, "PUSH10");
const PUSH11: Opcode = Opcode::new(0x6a, 3, "PUSH11");
const PUSH12: Opcode = Opcode::new(0x6b, 3, "PUSH12");
const PUSH13: Opcode = Opcode::new(0x6c, 3, "PUSH13");
const PUSH14: Opcode = Opcode::new(0x6d, 3, "PUSH14");
const PUSH15: Opcode = Opcode::new(0x6e, 3, "PUSH15");
const PUSH16: Opcode = Opcode::new(0x6f, 3, "PUSH16");

const PUSH17: Opcode = Opcode::new(0x70, 3, "PUSH17");
const PUSH18: Opcode = Opcode::new(0x71, 3, "PUSH18");
const PUSH19: Opcode = Opcode::new(0x72, 3, "PUSH19");
const PUSH20: Opcode = Opcode::new(0x73, 3, "PUSH20");
const PUSH21: Opcode = Opcode::new(0x74, 3, "PUSH21");
const PUSH22: Opcode = Opcode::new(0x75, 3, "PUSH22");
const PUSH23: Opcode = Opcode::new(0x76, 3, "PUSH23");
const PUSH24: Opcode = Opcode::new(0x77, 3, "PUSH24");
const PUSH25: Opcode = Opcode::new(0x78, 3, "PUSH25");
const PUSH26: Opcode = Opcode::new(0x79, 3, "PUSH26");
const PUSH27: Opcode = Opcode::new(0x7a, 3, "PUSH27");
const PUSH28: Opcode = Opcode::new(0x7b, 3, "PUSH28");
const PUSH29: Opcode = Opcode::new(0x7c, 3, "PUSH29");
const PUSH30: Opcode = Opcode::new(0x7d, 3, "PUSH30");
const PUSH31: Opcode = Opcode::new(0x7e, 3, "PUSH31");
const PUSH32: Opcode = Opcode::new(0x7f, 3, "PUSH32");

const DUP1: Opcode = Opcode::new(0x80, 3, "DUP1");
const DUP2: Opcode = Opcode::new(0x81, 3, "DUP2");
const DUP3: Opcode = Opcode::new(0x82, 3, "DUP3");
const DUP4: Opcode = Opcode::new(0x83, 3, "DUP4");
const DUP5: Opcode = Opcode::new(0x84, 3, "DUP5");
const DUP6: Opcode = Opcode::new(0x85, 3, "DUP6");
const DUP7: Opcode = Opcode::new(0x86, 3, "DUP7");
const DUP8: Opcode = Opcode::new(0x87, 3, "DUP8");
const DUP9: Opcode = Opcode::new(0x88, 3, "DUP9");
const DUP10: Opcode = Opcode::new(0x89, 3, "DUP10");
const DUP11: Opcode = Opcode::new(0x8a, 3, "DUP11");
const DUP12: Opcode = Opcode::new(0x8b, 3, "DUP12");
const DUP13: Opcode = Opcode::new(0x8c, 3, "DUP13");
const DUP14: Opcode = Opcode::new(0x8d, 3, "DUP14");
const DUP15: Opcode = Opcode::new(0x8e, 3, "DUP15");
const DUP16: Opcode = Opcode::new(0x8f, 3, "DUP16");

const SWAP1: Opcode = Opcode::new(0x90, 3, "SWAP1");
const SWAP2: Opcode = Opcode::new(0x91, 3, "SWAP2");
const SWAP3: Opcode = Opcode::new(0x92, 3, "SWAP3");
const SWAP4: Opcode = Opcode::new(0x93, 3, "SWAP4");
const SWAP5: Opcode = Opcode::new(0x94, 3, "SWAP5");
const SWAP6: Opcode = Opcode::new(0x95, 3, "SWAP6");
const SWAP7: Opcode = Opcode::new(0x96, 3, "SWAP7");
const SWAP8: Opcode = Opcode::new(0x97, 3, "SWAP8");
const SWAP9: Opcode = Opcode::new(0x98, 3, "SWAP9");
const SWAP10: Opcode = Opcode::new(0x99, 3, "SWAP10");
const SWAP11: Opcode = Opcode::new(0x9a, 3, "SWAP11");
const SWAP12: Opcode = Opcode::new(0x9b, 3, "SWAP12");
const SWAP13: Opcode = Opcode::new(0x9c, 3, "SWAP13");
const SWAP14: Opcode = Opcode::new(0x9d, 3, "SWAP14");
const SWAP15: Opcode = Opcode::new(0x9e, 3, "SWAP15");
const SWAP16: Opcode = Opcode::new(0x9f, 3, "SWAP16");

const LOG0: Opcode = Opcode::new(0xa0, 375, "LOG0");
const LOG1: Opcode = Opcode::new(0xa1, 750, "LOG1");
const LOG2: Opcode = Opcode::new(0xa2, 1125, "LOG2");
const LOG3: Opcode = Opcode::new(0xa3, 1500, "LOG3");
const LOG4: Opcode = Opcode::new(0xa4, 1875, "LOG4");

const CREATE: Opcode = Opcode::new(0xf0, 32000, "CREATE");
const CALL: Opcode = Opcode::new(0xf1, 40, "CALL");
const CALLCODE: Opcode = Opcode::new(0xf2, 40, "CALLCODE");
const RETURN: Opcode = Opcode::new(0xf3, 0, "RETURN");
const DELEGATECALL: Opcode = Opcode::new(0xf4, 40, "DELEGATECALL");
const CREATE2: Opcode = Opcode::new(0xf5, 32000, "CREATE2");
const STATICCALL: Opcode = Opcode::new(0xfa, 40, "STATICCALL");
const REVERT: Opcode = Opcode::new(0xfd, 0, "REVERT");
const INVALID: Opcode = Opcode::new(0xfe, 0, "INVALID");
const SELFDESTRUCT: Opcode = Opcode::new(0xff, 5000, "SELFDESTRUCT");