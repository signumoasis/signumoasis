use std::marker::PhantomData;

use super::{Transaction, TransactionType};

#[derive(Default)]
struct TransactionBuilderData {
    amount_nqt: Option<u64>,
    attachment: Vec<()>,
    block_id: Option<u64>,
    block_timestamp: Option<u32>,
    cash_back_id: Option<u64>,
    deadline: Option<u16>,
    ec_block_height: Option<u32>,
    ec_block_id: Option<u64>,
    fee_nqt: Option<u64>,
    full_hash: Option<String>,
    height: Option<u64>,
    id: Option<u64>,
    recipient_id: Option<u64>,
    referenced_transaction_full_hash: Option<String>,
    sender_id: Option<u64>,
    sender_public_key: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
    timestamp: Option<u32>,
    transaction_type: Option<TransactionType>,
    version: Option<u8>,

    message: Option<()>,
    encrypted_message: Option<()>,
    encrypt_to_self_message: Option<()>,
    public_key_announcement: Option<()>,
}
pub trait Version: Default {}

#[derive(Default)]
pub struct NoVersion;
impl Version for NoVersion {}
pub struct VersionSet;
pub struct TypeSet;

#[derive(Default)]
pub struct Version0;
impl Version for Version0 {}
#[derive(Default)]
pub struct Version1;
impl Version for Version1 {}
#[derive(Default)]
pub struct Version2;
impl Version for Version2 {}

#[derive(Default)]
pub struct TransactionBuilder<State, V: Version>(TransactionBuilderData, PhantomData<(State, V)>);

// INFO: Functions allowed on initial creation.
impl TransactionBuilder<NoVersion, NoVersion> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn version0(self) -> TransactionBuilder<VersionSet, Version0> {
        TransactionBuilder(
            TransactionBuilderData {
                version: Some(0),
                ..self.0
            },
            PhantomData,
        )
    }
    pub fn version1(self) -> TransactionBuilder<VersionSet, Version1> {
        TransactionBuilder(
            TransactionBuilderData {
                version: Some(1),
                ..self.0
            },
            PhantomData,
        )
    }
    pub fn version2(self) -> TransactionBuilder<VersionSet, Version2> {
        TransactionBuilder(
            TransactionBuilderData {
                version: Some(2),
                ..self.0
            },
            PhantomData,
        )
    }
}

// INFO: Functions to be used after setting the version but before setting the type.
impl<V: Version> TransactionBuilder<VersionSet, V> {
    pub fn set_type(self, transaction_type: TransactionType) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                transaction_type: Some(transaction_type),
                ..self.0
            },
            PhantomData,
        )
    }
}

// INFO: Functions to be used after setting the type and version.
impl<V: Version> TransactionBuilder<TypeSet, V> {
    /// Set the amount of Signa in NQT.
    pub fn set_amount_nqt(self, amount: u64) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                amount_nqt: Some(amount),
                ..self.0
            },
            PhantomData,
        )
    }

    /// Set the fee in NQT.
    pub fn set_fee_nqt(self, fee: u64) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                fee_nqt: Some(fee),
                ..self.0
            },
            PhantomData,
        )
    }

    /// Set the timestamp.
    pub fn set_timestamp(self, timestamp: u32) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                timestamp: Some(timestamp),
                ..self.0
            },
            PhantomData,
        )
    }

    /// Set the deadline.
    pub fn set_deadline(self, deadline: u16) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                deadline: Some(deadline),
                ..self.0
            },
            PhantomData,
        )
    }

    /// Set the sender's public key.
    pub fn set_sender_public_key(
        self,
        sender_public_key: Vec<u8>,
    ) -> TransactionBuilder<TypeSet, V> {
        TransactionBuilder(
            TransactionBuilderData {
                sender_public_key: Some(sender_public_key),
                ..self.0
            },
            PhantomData,
        )
    }

    /// Add and attachment to the transaction.
    pub fn attach(self, attachment: ()) -> TransactionBuilder<TypeSet, V> {
        let mut a = self.0.attachment;
        a.push(attachment);
        TransactionBuilder(
            TransactionBuilderData {
                attachment: a,
                ..self.0
            },
            PhantomData,
        )
    }
}

// INFO: Functions to be used only after the type is set on Version 0.
impl TransactionBuilder<TypeSet, Version0> {
    pub fn build(self) -> Result<Transaction, anyhow::Error> {
        if self.0.transaction_type.is_none() {
            return Err(anyhow::anyhow!("transaction type must be set"));
        }
        todo!()
    }
}

// INFO: Functions to be used only after the type is set on Version 1.
impl TransactionBuilder<TypeSet, Version1> {
    pub fn build(self) -> Result<Transaction, anyhow::Error> {
        if self.0.transaction_type.is_none() {
            return Err(anyhow::anyhow!("transaction type must be set"));
        }
        todo!()
    }
}

// INFO: Functions to be used only after the type is set on Version 2.
impl TransactionBuilder<TypeSet, Version2> {
    pub fn build(self) -> Result<Transaction, anyhow::Error> {
        if self.0.transaction_type.is_none() {
            return Err(anyhow::anyhow!("transaction type must be set"));
        }
        todo!()
    }
}
