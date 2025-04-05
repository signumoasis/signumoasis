// FIX: REMOVE THIS ALLOW
#![allow(dead_code)]
use num_bigint::BigUint;
use serde::Deserialize;

use super::Transaction;

pub enum Block {
    /// The genesis block of Signum. This variant is needed to parse the genesis block and provide
    /// it to the client API but should never be serialized to the peer-to-peer transfers. Every
    /// node will generate it deterministically, instead.
    Genesis {
        /// Seconds since Signum's Genesis Block
        timestamp: u32,
        /// The id of the previous block
        previous_block_id: u64,
        /// The total amount of Signa in this block expressed as NQT
        total_amount_nqt: u32,
        /// The total amount of fees in this block expressed as NQT
        total_fee_nqt: u32,
        /// The total length of the block payload
        payload_length: u32,
        /// The hash of the payload
        payload_hash: Vec<u8>,
        /// The public key of the account that forged this block
        generator_public_key: Vec<u8>,
        /// The generation signature used to forge this block
        generation_signature: Vec<u8>,
        /// The hash of the block immediately prior to this one (unless this one is Genesis)
        previous_block_hash: Vec<u8>,
        /// The signature of this block
        block_signature: Vec<u8>,
        /// A list of transactions contained in this block
        transactions: Vec<Transaction>,
        /// The nonce used to forge this block
        nonce: u64,
        /// The raw bytes of the code of smart transactions in this block
        block_ats: Option<Vec<u8>>,
        /// The height of this block on the chain
        height: u32,
        /// The base target of this block
        base_target: u64,
        /// The cumulative difficulty when this block was forged
        cumulative_difficulty: BigUint,
        /// The id of the block following this one
        next_block_id: u64, // TODO: Do we care about this? SRS does for some reason
        /// This block's numerical id
        id: u64,
        /// The numerical id of the account that forged this block
        generator_id: u64,
    },
    /// Version 3 of the block. This is the oldest version in the Signum chain.
    V3 {
        /// Seconds since Signum's Genesis Block
        timestamp: u32,
        /// The id of the previous block
        previous_block_id: u64,
        /// The total amount of Signa in this block expressed as NQT
        total_amount_nqt: u64,
        /// The total amount of fees in this block expressed as NQT
        total_fee_nqt: u64,
        /// The total length of the block payload
        payload_length: u32,
        /// The hash of the payload
        payload_hash: Vec<u8>,
        /// The public key of the account that forged this block
        generator_public_key: Vec<u8>,
        /// The generation signature used to forge this block
        generation_signature: Vec<u8>,
        /// The hash of the block immediately prior to this one (unless this one is Genesis)
        previous_block_hash: Vec<u8>,
        /// The signature of this block
        block_signature: Vec<u8>,
        /// A list of transactions contained in this block
        transactions: Vec<Transaction>,
        /// The nonce used to forge this block
        nonce: u64,
        /// The raw bytes of the code of smart transactions in this block
        block_ats: Option<Vec<u8>>,
        /// The height of this block on the chain
        height: u32,
        /// The base target of this block
        base_target: u64,
        /// The cumulative difficulty when this block was forged
        cumulative_difficulty: BigUint,
        /// The id of the block following this one
        next_block_id: u64, // TODO: Do we care about this? SRS does for some reason
        /// This block's numerical id
        id: u64,
        /// The numerical id of the account that forged this block
        generator_id: u64,
    },
    /// Version 4 of the block. Adds the fields `total_fee_cashback_nqt` and `total_fee_burnt_nqt`.
    V4 {
        /// Seconds since Signum's Genesis Block
        timestamp: u32,
        /// The id of the previous block
        previous_block_id: u64,
        /// The total amount of Signa in this block expressed as NQT
        total_amount_nqt: u64,
        /// The total amount of fees in this block expressed as NQT
        total_fee_nqt: u64,
        /// The total amount of Signa going to cashback in this block expressed as NQT
        total_fee_cashback_nqt: u64,
        /// The total amount of Signa burned in this block expressed as NQT
        total_fee_burnt_nqt: u64,
        /// The total length of the block payload
        payload_length: u32,
        /// The hash of the payload
        payload_hash: Vec<u8>,
        /// The public key of the account that forged this block
        generator_public_key: Vec<u8>,
        /// The generation signature used to forge this block
        generation_signature: Vec<u8>,
        /// The hash of the block immediately prior to this one (unless this one is Genesis)
        previous_block_hash: Vec<u8>,
        /// The signature of this block
        block_signature: Vec<u8>,
        /// A list of transactions contained in this block
        transactions: Vec<Transaction>,
        /// The nonce used to forge this block
        nonce: u64,
        /// The raw bytes of the code of smart transactions in this block
        block_ats: Option<Vec<u8>>,
        /// The height of this block on the chain
        height: u32,
        /// The base target of this block
        base_target: u64,
        /// The cumulative difficulty when this block was forged
        cumulative_difficulty: BigUint,
        /// The id of the block following this one
        next_block_id: u64, // TODO: Do we care about this? SRS does for some reason
        /// This block's numerical id
        id: u64,
        /// The numerical id of the account that forged this block
        generator_id: u64,
    },
}

impl<'de> Deserialize<'de> for Block {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};

        // Define a visitor for the enum
        struct BlockVisitor;

        impl<'de> Visitor<'de> for BlockVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a map with a 'version' field")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut version: Option<i32> = None;
                let mut timestamp: Option<u32> = None;
                let mut previous_block_id: Option<u64> = None;
                let mut total_amount_nqt_u32: Option<u32> = None;
                let mut total_amount_nqt_u64: Option<u64> = None;
                let mut total_fee_nqt_u32: Option<u32> = None;
                let mut total_fee_nqt_u64: Option<u64> = None;
                let mut total_fee_cashback_nqt: Option<u64> = None;
                let mut total_fee_burnt_nqt: Option<u64> = None;
                let mut payload_length: Option<u32> = None;
                let mut payload_hash: Option<Vec<u8>> = None;
                let mut generator_public_key: Option<Vec<u8>> = None;
                let mut generation_signature: Option<Vec<u8>> = None;
                let mut previous_block_hash: Option<Vec<u8>> = None;
                let mut block_signature: Option<Vec<u8>> = None;
                let mut transactions: Option<Vec<Transaction>> = None;
                let mut nonce: Option<u64> = None;
                let mut block_ats: Option<Option<Vec<u8>>> = None;
                let mut height: Option<u32> = None;
                let mut base_target: Option<u64> = None;
                let mut cumulative_difficulty: Option<BigUint> = None;
                let mut next_block_id: Option<u64> = None; // TODO: Do we care about this? SRS does for some reason
                let mut id: Option<u64> = None;
                let mut generator_id: Option<u64> = None;

                // Iterate over the map entries, deserialize into Option<T> for each one
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "version" => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(map.next_value()?);
                        }
                        "timestamp" => {
                            if timestamp.is_some() {
                                return Err(de::Error::duplicate_field("timestamp"));
                            }
                            timestamp = Some(map.next_value()?);
                        }
                        "previous_block_id" => {
                            if previous_block_id.is_some() {
                                return Err(de::Error::duplicate_field("previous_block_id"));
                            }
                            previous_block_id = Some(map.next_value()?);
                        }
                        "total_amount_nqt" => {
                            // Have to deserialize to both u32 and u64 because of the differing type
                            // requirements in each version. Final struct will only use appropriate type.
                            if total_amount_nqt_u32.is_some() || total_amount_nqt_u64.is_some() {
                                return Err(de::Error::duplicate_field("total_amount_nqt"));
                            }

                            let total_amount_string: Option<String> = Some(map.next_value()?);

                            if let Some(total_amount_string) = total_amount_string {
                                total_amount_nqt_u64 = total_amount_string.parse().ok();
                                total_amount_nqt_u32 = total_amount_string.parse().ok();
                            }
                        }
                        "total_fee_nqt" => {
                            // Have to deserialize to both u32 and u64 because of the differing type
                            // requirements in each version. Final struct will only use appropriate type.
                            if total_fee_nqt_u32.is_some() || total_fee_nqt_u64.is_some() {
                                return Err(de::Error::duplicate_field("total_fee_nqt"));
                            }

                            let total_fee_string: Option<String> = Some(map.next_value()?);

                            if let Some(total_fee_string) = total_fee_string {
                                total_fee_nqt_u64 = total_fee_string.parse().ok();
                                total_fee_nqt_u32 = total_fee_string.parse().ok();
                            }
                        }
                        "total_fee_cashback_nqt" => {
                            if total_fee_cashback_nqt.is_some() {
                                return Err(de::Error::duplicate_field("total_fee_cashback_nqt"));
                            }
                            total_fee_cashback_nqt = Some(map.next_value()?);
                        }
                        "total_fee_burnt_nqt" => {
                            if total_fee_burnt_nqt.is_some() {
                                return Err(de::Error::duplicate_field("total_fee_burnt_nqt"));
                            }
                            total_fee_burnt_nqt = Some(map.next_value()?);
                        }
                        "payload_length" => {
                            if payload_length.is_some() {
                                return Err(de::Error::duplicate_field("payload_length"));
                            }
                            payload_length = Some(map.next_value()?);
                        }
                        "payload_hash" => {
                            if payload_hash.is_some() {
                                return Err(de::Error::duplicate_field("payload_hash"));
                            }
                            payload_hash = Some(map.next_value()?);
                        }
                        "generator_public_key" => {
                            if generator_public_key.is_some() {
                                return Err(de::Error::duplicate_field("generator_public_key"));
                            }
                            generator_public_key = Some(map.next_value()?);
                        }
                        "generation_signature" => {
                            if generation_signature.is_some() {
                                return Err(de::Error::duplicate_field("generation_signature"));
                            }
                            generation_signature = Some(map.next_value()?);
                        }
                        "previous_block_hash" => {
                            if previous_block_hash.is_some() {
                                return Err(de::Error::duplicate_field("previous_block_hash"));
                            }
                            previous_block_hash = Some(map.next_value()?);
                        }
                        "block_signature" => {
                            if block_signature.is_some() {
                                return Err(de::Error::duplicate_field("block_signature"));
                            }
                            block_signature = Some(map.next_value()?);
                        }
                        // TODO: Parse this too after implementing deserialize for Transaction
                        // "transactions" => {
                        //     if transactions.is_some() {
                        //         return Err(de::Error::duplicate_field("transactions"));
                        //     }
                        //     transactions = Some(map.next_value()?);
                        // }
                        "nonce" => {
                            if nonce.is_some() {
                                return Err(de::Error::duplicate_field("nonce"));
                            }
                            nonce = Some(map.next_value()?);
                        }
                        "block_ats" => {
                            if block_ats.is_some() {
                                return Err(de::Error::duplicate_field("block_ats"));
                            }
                            block_ats = Some(map.next_value()?);
                        }
                        "height" => {
                            if height.is_some() {
                                return Err(de::Error::duplicate_field("height"));
                            }
                            height = Some(map.next_value()?);
                        }
                        "base_target" => {
                            if base_target.is_some() {
                                return Err(de::Error::duplicate_field("base_target"));
                            }
                            base_target = Some(map.next_value()?);
                        }
                        "cumulative_difficulty" => {
                            if cumulative_difficulty.is_some() {
                                return Err(de::Error::duplicate_field("cumulative_difficulty"));
                            }
                            cumulative_difficulty = Some(map.next_value()?);
                        }
                        "next_block_id" => {
                            if next_block_id.is_some() {
                                return Err(de::Error::duplicate_field("next_block_id"));
                            }
                            next_block_id = Some(map.next_value()?);
                        }
                        "id" => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        "generator_id" => {
                            if generator_id.is_some() {
                                return Err(de::Error::duplicate_field("generator_id"));
                            }
                            generator_id = Some(map.next_value()?);
                        }

                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                // Determine the variant based on the version and build the final output
                let version = version.ok_or_else(|| de::Error::missing_field("version"))?;
                match version {
                    -1 => Ok(Block::Genesis {
                        timestamp: timestamp
                            .ok_or_else(|| de::Error::missing_field("timestamp"))?,
                        previous_block_id: previous_block_id
                            .ok_or_else(|| de::Error::missing_field("previous_block_id"))?,
                        total_amount_nqt: total_amount_nqt_u32
                            .ok_or_else(|| de::Error::missing_field("total_amount_nqt"))?,
                        total_fee_nqt: total_fee_nqt_u32
                            .ok_or_else(|| de::Error::missing_field("total_fee_nqt"))?,
                        payload_length: payload_length
                            .ok_or_else(|| de::Error::missing_field("payload_length"))?,
                        payload_hash: payload_hash
                            .ok_or_else(|| de::Error::missing_field("payload_hash"))?,
                        generator_public_key: generator_public_key
                            .ok_or_else(|| de::Error::missing_field("generator_public_key"))?,
                        generation_signature: generation_signature
                            .ok_or_else(|| de::Error::missing_field("generation_signature"))?,
                        previous_block_hash: previous_block_hash
                            .ok_or_else(|| de::Error::missing_field("previous_block_hash"))?,
                        block_signature: block_signature
                            .ok_or_else(|| de::Error::missing_field("block_signature"))?,
                        transactions: transactions
                            .ok_or_else(|| de::Error::missing_field("transactions"))?,
                        nonce: nonce.ok_or_else(|| de::Error::missing_field("nonce"))?,
                        block_ats: block_ats
                            .ok_or_else(|| de::Error::missing_field("block_ats"))?,
                        height: height.ok_or_else(|| de::Error::missing_field("height"))?,
                        base_target: base_target
                            .ok_or_else(|| de::Error::missing_field("base_target"))?,
                        cumulative_difficulty: cumulative_difficulty
                            .ok_or_else(|| de::Error::missing_field("cumulative_difficulty"))?,
                        next_block_id: next_block_id
                            .ok_or_else(|| de::Error::missing_field("next_block_id"))?,
                        id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                        generator_id: generator_id
                            .ok_or_else(|| de::Error::missing_field("generator_id"))?,
                    }),
                    3 => Ok(Block::V3 {
                        timestamp: timestamp
                            .ok_or_else(|| de::Error::missing_field("timestamp"))?,
                        previous_block_id: previous_block_id
                            .ok_or_else(|| de::Error::missing_field("previous_block_id"))?,
                        total_amount_nqt: total_amount_nqt_u64
                            .ok_or_else(|| de::Error::missing_field("total_amount_nqt"))?,
                        total_fee_nqt: total_fee_nqt_u64
                            .ok_or_else(|| de::Error::missing_field("total_fee_nqt"))?,
                        payload_length: payload_length
                            .ok_or_else(|| de::Error::missing_field("payload_length"))?,
                        payload_hash: payload_hash
                            .ok_or_else(|| de::Error::missing_field("payload_hash"))?,
                        generator_public_key: generator_public_key
                            .ok_or_else(|| de::Error::missing_field("generator_public_key"))?,
                        generation_signature: generation_signature
                            .ok_or_else(|| de::Error::missing_field("generation_signature"))?,
                        previous_block_hash: previous_block_hash
                            .ok_or_else(|| de::Error::missing_field("previous_block_hash"))?,
                        block_signature: block_signature
                            .ok_or_else(|| de::Error::missing_field("block_signature"))?,
                        transactions: transactions
                            .ok_or_else(|| de::Error::missing_field("transactions"))?,
                        nonce: nonce.ok_or_else(|| de::Error::missing_field("nonce"))?,
                        block_ats: block_ats
                            .ok_or_else(|| de::Error::missing_field("block_ats"))?,
                        height: height.ok_or_else(|| de::Error::missing_field("height"))?,
                        base_target: base_target
                            .ok_or_else(|| de::Error::missing_field("base_target"))?,
                        cumulative_difficulty: cumulative_difficulty
                            .ok_or_else(|| de::Error::missing_field("cumulative_difficulty"))?,
                        next_block_id: next_block_id
                            .ok_or_else(|| de::Error::missing_field("next_block_id"))?,
                        id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                        generator_id: generator_id
                            .ok_or_else(|| de::Error::missing_field("generator_id"))?,
                    }),
                    4 => Ok(Block::V4 {
                        timestamp: timestamp
                            .ok_or_else(|| de::Error::missing_field("timestamp"))?,
                        previous_block_id: previous_block_id
                            .ok_or_else(|| de::Error::missing_field("previous_block_id"))?,
                        total_amount_nqt: total_amount_nqt_u64
                            .ok_or_else(|| de::Error::missing_field("total_amount_nqt"))?,
                        total_fee_nqt: total_fee_nqt_u64
                            .ok_or_else(|| de::Error::missing_field("total_fee_nqt"))?,
                        total_fee_cashback_nqt: total_fee_cashback_nqt
                            .ok_or_else(|| de::Error::missing_field("total_fee_cashback_nqt"))?,
                        total_fee_burnt_nqt: total_fee_burnt_nqt
                            .ok_or_else(|| de::Error::missing_field("total_fee_burnt_nqt"))?,
                        payload_length: payload_length
                            .ok_or_else(|| de::Error::missing_field("payload_length"))?,
                        payload_hash: payload_hash
                            .ok_or_else(|| de::Error::missing_field("payload_hash"))?,
                        generator_public_key: generator_public_key
                            .ok_or_else(|| de::Error::missing_field("generator_public_key"))?,
                        generation_signature: generation_signature
                            .ok_or_else(|| de::Error::missing_field("generation_signature"))?,
                        previous_block_hash: previous_block_hash
                            .ok_or_else(|| de::Error::missing_field("previous_block_hash"))?,
                        block_signature: block_signature
                            .ok_or_else(|| de::Error::missing_field("block_signature"))?,
                        transactions: transactions
                            .ok_or_else(|| de::Error::missing_field("transactions"))?,
                        nonce: nonce.ok_or_else(|| de::Error::missing_field("nonce"))?,
                        block_ats: block_ats
                            .ok_or_else(|| de::Error::missing_field("block_ats"))?,
                        height: height.ok_or_else(|| de::Error::missing_field("height"))?,
                        base_target: base_target
                            .ok_or_else(|| de::Error::missing_field("base_target"))?,
                        cumulative_difficulty: cumulative_difficulty
                            .ok_or_else(|| de::Error::missing_field("cumulative_difficulty"))?,
                        next_block_id: next_block_id
                            .ok_or_else(|| de::Error::missing_field("next_block_id"))?,
                        id: id.ok_or_else(|| de::Error::missing_field("id"))?,
                        generator_id: generator_id
                            .ok_or_else(|| de::Error::missing_field("generator_id"))?,
                    }),
                    _ => Err(de::Error::custom(format!("unknown version: {}", version))),
                }
            }
        }
        deserializer.deserialize_map(BlockVisitor)
    }
}
