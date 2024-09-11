## ID Connect Hackathon DEX - RAGNAR
This is a Decentralised Exchange on the Concordium Blockchain where users gain access by verifying they are above 18 using Concordium ID 2.0 and exchange CCD and Cis2 assets.

## How to set-up

### Concordium-contracts

1. First, make sure you have `Rust` installed, then install `cargo-concordium`
  
       cargo install cargo-concordium@3.0.0
   
3. Clone the repository
   
       git clone https://github.com/josidbobo/IDConcordiumDEX
3. Install dependencies for the contracts
 
       cargo install 
4. Build the wasm module
    
       cargo concordium build --out ./concordium-out/module.wasm.v1 --schema-embed

### Frontend

1. Open the Frontend folder
 
       cd frontend
2. Install dependencies
 
       npm install
3. To test, Start the server
 
       npm start



