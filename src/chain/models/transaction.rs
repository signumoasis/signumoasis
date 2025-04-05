use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Transaction {
    None,
    V1 {
        #[serde(rename = "type")]
        transaction_type: TransactionType,
        timestamp: u64,
        deadline: u16,
        sender_public_key: Vec<u8>,
        recipient: (),
        recipient_rs: (),
        amount_nqt: (),
        fee_nqt: (),
        signature: (),
        signature_hash: (),
        full_hash: (),
        transaction: (),
        sender: (),
        sender_rs: (),
        height: (),
        version: (),
        ec_block_height: (),
        cash_back_id: (),
        verify: (),
    },
}

// INFO: Don't worry about the SRS 'type' values or anything here. The From<T> impl can handle that

/// The type of transaction. Cast to u8 for the value.
#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionType {
    AccountControl(AccountControlSubtype),
    AdvancedPayment(AdvancedPaymentSubtype),
    AutomatedTransactions(AutomatedTransactionsSubtype),
    ColoredCoins(ColoredCoinsSubtype),
    DigitalGoods(DigitalGoodsSubtype),
    Messaging(MessageSubtype),
    Payment(PaymentSubtype),
    SignaMining(SignaMiningSubtype),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AccountControlSubtype {
    EffectiveBalanceLeasing,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AdvancedPaymentSubtype {
    EscrowCreation,
    EscrowResult,
    EscrowSign,
    SubscriptionCancel,
    SubscriptionPayment,
    SubscriptionSubscribe,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AutomatedTransactionsSubtype {
    Creation,
    NxtPayment,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ColoredCoinsSubtype {
    Issuance,
    Mint,
    MultiTransfer,
    Transfer,
    AddTreasuryAccount,
    AskOrderCancellation,
    AskOrderPlacement,
    BidOrderCancellation,
    BidOrderPlacement,
    DistributeToHoldes,
    TransferOwnership,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DigitalGoodsSubtype {
    Delisting,
    Delivery,
    Feedback,
    Listing,
    PriceChange,
    Purchase,
    QuantityChange,
    Refund,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum MessageSubtype {
    AccountInfo,
    AliasAssignment,
    AliasSell,
    AliasBuy,
    ArbitraryMessage,
    TldAssignment,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PaymentSubtype {
    MultiOut,
    MultiSameOut,
    Ordinary,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SignaMiningSubtype {
    CommitmentAdd,
    CommitmentRemove,
    RewardRecipientAssignment,
}
