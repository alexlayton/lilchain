#[derive(Debug, Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending: Vec<Transaction>,
    nonce: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            chain: vec![Block::genesis()],
            pending: Vec::new(),
            nonce: 0,
        }
    }

    pub fn current_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    pub fn submit_tx(&mut self, payload: String) {
        let tx = Transaction::new(payload, self.nonce);
        self.pending.push(tx);
        self.nonce += 1;
    }

    pub fn new_block(&mut self) {
        let pending = self.pending.to_owned();
        let new_block = Block::new(self.current_block(), pending);
        self.chain.push(new_block);
        self.pending = Vec::new();
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: String,
    pub previous: String,
    pub index: u32,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
}

impl Block {
    fn genesis() -> Self {
        Block {
            id: utils::hash_str(b"genesis"),
            previous: "".to_string(),
            index: 0,
            timestamp: utils::current_time(),
            transactions: vec![],
        }
    }

    fn new(previous: &Block, transactions: Vec<Transaction>) -> Self {
        let mut id_str = transactions
            .iter()
            .map(|tx| tx.id.clone())
            .fold("".to_string(), |cur, next| cur + &next);
        id_str += &previous.id;
        let id = utils::hash_str(id_str.as_bytes());

        Block {
            id: id,
            previous: previous.id.clone(),
            index: previous.index + 1,
            timestamp: utils::current_time(),
            transactions,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: String,
    pub payload: String,
    pub nonce: u32,
    pub timestamp: u64,
}

impl Transaction {
    fn new(payload: String, nonce: u32) -> Self {
        let id_str = format!("{}{}", payload, nonce);
        let id = utils::hash_str(id_str.as_bytes());
        Transaction {
            id: id,
            payload: payload,
            nonce: nonce,
            timestamp: utils::current_time(),
        }
    }
}

mod utils {
    use sha2::{Digest, Sha256};
    use std::time::SystemTime;

    pub(crate) fn current_time() -> u64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("error getting current time")
            .as_secs()
    }

    pub(crate) fn hash_str(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

mod tests {
    use crate::Blockchain;
    use crate::Transaction;

    #[test]
    fn test_new_tx() {
        let tx = Transaction::new("hello".to_string(), 0);
        assert_eq!(
            tx.id,
            "5a936ee19a0cf3c70d8cb0006111b7a52f45ec01703e0af8cdc8c6d81ac5850c"
        );
    }

    #[test]
    fn test_genesis_block() {
        let bc = Blockchain::new();
        assert_eq!(bc.chain.len(), 1);
        let block = bc.current_block();
        assert_eq!(block.index, 0);
        assert_eq!(
            block.id,
            "aeebad4a796fcc2e15dc4c6061b45ed9b373f26adfc798ca7d2d8cc58182718e"
        );
    }

    #[test]
    fn testing() {
        let mut bc = Blockchain::new();
        bc.submit_tx("apple".to_string());
        bc.submit_tx("orange".to_string());
        bc.submit_tx("banana".to_string());
        println!("before - {:?}", bc);

        bc.new_block();

        println!("after - {:?}", bc);
    }
}
