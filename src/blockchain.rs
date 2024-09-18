use crate::block::Block;
pub use crate::block::Transaction;
use sha2::Digest;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new(difficulty: u32) -> Self {
        let mut chain = Vec::new();
        chain.push(Blockchain::create_genesis_block());
        Blockchain {
            chain,
            difficulty,
            pending_transactions: Vec::new(),
        }
    }

    fn create_genesis_block() -> Block {
        Block::new(0, String::from("0"), Vec::new(), 0)
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn mine_pending_transactions(&mut self, miner_address: String) {
        let block = self.create_block(self.pending_transactions.clone());
        self.chain.push(block);

        // Reward the miner
        let reward_transaction = Transaction::new(String::from("System"), miner_address, 1.0); // 1 coin reward
        self.pending_transactions = vec![reward_transaction];
    }

    fn create_block(&self, transactions: Vec<Transaction>) -> Block {
        let previous_block = self.chain.last().unwrap();
        let index = previous_block.index + 1;
        let previous_hash = previous_block.calculate_hash();
        let proof = self.proof_of_work(previous_block.proof);
        Block::new(index, previous_hash, transactions, proof)
    }

    fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;
        while !self.is_valid_proof(last_proof, proof) {
            proof += 1;
        }
        proof
    }

    fn is_valid_proof(&self, last_proof: u64, proof: u64) -> bool {
        let guess = format!("{}{}", last_proof, proof);
        let guess_hash = sha2::Sha256::digest(guess.as_bytes());
        let result = guess_hash
            .iter()
            .take(self.difficulty as usize)
            .all(|&val| val == 0);
        result
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.previous_hash != previous_block.calculate_hash() {
                return false;
            }

            if !self.is_valid_proof(previous_block.proof, current_block.proof) {
                return false;
            }
        }
        true
    }
}
