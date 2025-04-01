use num_bigint::BigUint;

pub enum Block {
    Genesis {
        /// Seconds since Signum's Genesis Block
        timestamp: u32,
        previous_block_id: u64,
        total_amount_nqt: u64,
        total_fee_nqt: u64,
        total_fee_cashback_nqt: u64,
        total_fee_burnt_nqt: u64,
        payload_length: u32,
        payload_hash: Vec<u8>,
        generator_public_key: Vec<u8>,
        generation_signature: Vec<u8>,
        previous_block_hash: Vec<u8>,
        block_signature: Vec<u8>,
        transactions: Vec<Transaction>,
        nonce: u64,
        block_ats: Option<Vec<u8>>,
        height: u32,
        base_target: u64,
        cumulative_difficulty: BigUint,
        next_block_id: u64, // TODO: Do we care about this? SRS does for some reason
        id: u64,
        generator_id: u64,
    },
    V3 {},
    V4 {},
}

pub struct Transaction;
