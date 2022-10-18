# Solidity Decompiler

In it's current state bytecode is processed in ``contracts/contract.evm``, in hexadecimal without the prefix. I just copied the bytecode from etherscan and pasted it in the above file.

## Helpful links

Everything Opcodes: [EVM Playground](https://www.evm.codes/)

Great substack gudie: [Noxx Substack](https://noxx.substack.com/p/evm-deep-dives-the-path-to-shadowy)

Ethereum Engineering Group [Solidity to Bytecode](https://www.youtube.com/watch?v=RxL_1AfV7N4&t=2s&themeRefresh=1)

Everything EVM: [EVM Handbook](https://noxx3xxon.notion.site/noxx3xxon/The-EVM-Handbook-bb38e175cc404111a391907c4975426d)

EVM Tools: [EVM Tools](https://github.com/CoinCulture/evm-tools)


## Spec (laundry list of desired features for minimum viable product) 
- Program accepts some configuration as input e.g. EL client endpoint, contract address to fetch bytecode at, potential data about the bytecode (swarm hash, if constructor is included) 
- Program fetches and performs necessary parsing / formatting of the bytecode 
- Representation of the instruction set e.g. `Let Instruction1 = Instruction{0x01, "ADD", 2, 1, 0, "Addition operation"`};`
- Parse bytecode into all opcodes with names and instruction title 

