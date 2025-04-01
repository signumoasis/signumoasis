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
        payload_hash: [u8],
        generator_public_key: [u8],
        generation_signature: [u8],
        previous_block_hash: [u8],
        block_signature: [u8],
        transactions: Vec<Transaction>,
        nonce: u64,
        block_ats: Option<[u8]>,
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
