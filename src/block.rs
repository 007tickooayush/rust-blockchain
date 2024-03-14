use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use log::info;
use sha2::{Digest, Sha256};
use hex;
// use failure::Error;

// Define a Custom Type for Result<T>
pub type Result<T> = std::result::Result<T,failure::Error>;
const TARGET_HEX: usize = 4;
#[derive(Debug,Clone)]
pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
}

impl Block {

    pub fn new_genesis_block() -> Block {
        Block::new(String::from("Genesis Block"),String::new(),0).unwrap()
    }

    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    ///
    ///
    /// # Arguments
    ///
    /// * `data`:
    /// * `prev_block_hash`:
    /// * `height`:
    ///
    /// returns: Result<Block, <unknown>>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn new(data: String, prev_block_hash: String, height: usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time calculation Error")
            .as_millis();
        let mut block = Block {
            timestamp,
            transactions: data,
            prev_block_hash,
            hash: String::new(),
            height,
            nonce: 0,
        };

        // Calculating nonce and work
        block.run_proof_of_work()?;
        Ok(block)
    }

    
    pub fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the Block");
        while !self.validate()? {
            self.nonce +=1
        }
        let data = self.prepare_hash_data()?;
        let mut hasher_sha256 = Sha256::new();
        hasher_sha256.write(&data)?;

        self.hash = hex::encode(hasher_sha256.finalize());

        Ok(())
    }

    fn validate(&self) -> Result<bool> {
        let data = self.prepare_hash_data()?;
        let mut hasher_sha256 = Sha256::new();
        hasher_sha256.write(&data)?;

        let mut vector = vec![];
        vector.resize(TARGET_HEX,'0' as u8);

        let hash_result = hasher_sha256.finalize();
        let hash_slice = &hash_result[0..TARGET_HEX];

        // Ok(&hasher_sha256.finalize()[0..TARGET_HEX] == String::from(vector)?)
        Ok(hash_slice == vector.as_slice())
    }

    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
        let block = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEX,
            self.nonce
        );
        let bytes = bincode::serialize(&block)?;
        Ok(bytes)
    }
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            chain: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data: String) -> Result<()> {
        let prev = self.chain.last().unwrap();
        let next_block = Block::new(data,prev.get_hash(),TARGET_HEX)?;

        self.chain.push(next_block);

        Ok(())
    }
}

#[cfg(test)]
mod test_blockchain {
    use super::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new();

        b.add_block(String::from("Block 1 data")).unwrap();
        b.add_block("Block 2 data".to_string()).unwrap();

        dbg!(b);
    }
}