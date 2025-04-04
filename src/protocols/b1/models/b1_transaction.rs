use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::chain::models::{
    AccountControlSubtype, AdvancedPaymentSubtype, AutomatedTransactionsSubtype,
    ColoredCoinsSubtype, DigitalGoodsSubtype, MessageSubtype, PaymentSubtype, SignaMiningSubtype,
    Transaction, TransactionType,
};

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct B1Transaction {
    #[serde(rename = "type")]
    pub transaction_type: u8,
    pub subtype: u8,
    pub timestamp: u64,
    pub deadline: u16,
    pub sender_public_key: Vec<u8>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub recipient: Option<u64>, // TODO Why is this an Option? Are transaction recipients optional?
    #[serde(rename = "amountNQT")]
    pub amount_nqt: u64,
    #[serde(rename = "feeNQT")]
    pub fee_nqt: u64,
    //pub referenced_transaction_full_hash: String,
    #[serde(rename = "ecBlockHeight")]
    pub ec_block_height: u32,
    #[serde(rename = "ecBlockId")]
    #[serde_as(as = "DisplayFromStr")]
    pub ec_block_id: u64,
    #[serde(rename = "cashBackId")]
    #[serde_as(as = "DisplayFromStr")]
    pub cash_back_id: u64,
    pub signature: String,
    //pub attachment: Vec<B1TransactionAttachment>,
    pub version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum B1TransactionAttachment {
    Message,
    EncryptedMessage,
    EncryptMessageToSelf,
    PublicKeyAnnouncement,
}

impl TryFrom<B1Transaction> for Transaction {
    type Error = anyhow::Error;

    fn try_from(value: B1Transaction) -> Result<Self, Self::Error> {
        let transaction_type = match value.transaction_type {
            TYPE_PAYMENT if value.subtype == SUBTYPE_PAYMENT_ORDINARY_PAYMENT => {
                TransactionType::Payment(PaymentSubtype::Ordinary)
            }
            TYPE_PAYMENT if value.subtype == SUBTYPE_PAYMENT_ORDINARY_PAYMENT_MULTI_OUT => {
                TransactionType::Payment(PaymentSubtype::MultiOut)
            }
            TYPE_PAYMENT if value.subtype == SUBTYPE_PAYMENT_ORDINARY_PAYMENT_MULTI_SAME_OUT => {
                TransactionType::Payment(PaymentSubtype::MultiSameOut)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_ARBITRARY_MESSAGE => {
                TransactionType::Messaging(MessageSubtype::ArbitraryMessage)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_ALIAS_ASSIGNMENT => {
                TransactionType::Messaging(MessageSubtype::AliasAssignment)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_ACCOUNT_INFO => {
                TransactionType::Messaging(MessageSubtype::AccountInfo)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_ALIAS_SELL => {
                TransactionType::Messaging(MessageSubtype::AliasSell)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_ALIAS_BUY => {
                TransactionType::Messaging(MessageSubtype::AliasBuy)
            }
            TYPE_MESSAGING if value.subtype == SUBTYPE_MESSAGING_TLD_ASSIGNMENT => {
                TransactionType::Messaging(MessageSubtype::TldAssignment)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASSET_ISSUANCE => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::Issuance)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASSET_TRANSFER => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::Transfer)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASK_ORDER_PLACEMENT => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::AskOrderPlacement)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_BID_ORDER_PLACEMENT => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::BidOrderPlacement)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASK_ORDER_CANCELLATION => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::AskOrderCancellation)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_BID_ORDER_CANCELLATION => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::BidOrderCancellation)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASSET_MINT => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::Mint)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ADD_TREASURY_ACCOUNT => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::AddTreasuryAccount)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_DISTRIBUTE_TO_HOLDERS => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::DistributeToHoldes)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_ASSET_MULTI_TRANSFER => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::MultiTransfer)
            }
            TYPE_COLORED_COINS if value.subtype == SUBTYPE_COLORED_COINS_TRANSFER_OWNERSHIP => {
                TransactionType::ColoredCoins(ColoredCoinsSubtype::TransferOwnership)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_LISTING => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Listing)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_DELISTING => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Delisting)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_PRICE_CHANGE => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::PriceChange)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_QUANTITY_CHANGE => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::QuantityChange)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_PURCHASE => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Purchase)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_DELIVERY => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Delivery)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_FEEDBACK => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Feedback)
            }
            TYPE_DIGITAL_GOODS if value.subtype == SUBTYPE_DIGITAL_GOODS_REFUND => {
                TransactionType::DigitalGoods(DigitalGoodsSubtype::Refund)
            }
            TYPE_ACCOUNT_CONTROL
                if value.subtype == SUBTYPE_ACCOUNT_CONTROL_EFFECTIVE_BALANCE_LEASING =>
            {
                TransactionType::AccountControl(AccountControlSubtype::EffectiveBalanceLeasing)
            }
            TYPE_SIGNA_MINING
                if value.subtype == SUBTYPE_SIGNA_MINING_REWARD_RECIPIENT_ASSIGNMENT =>
            {
                TransactionType::SignaMining(SignaMiningSubtype::RewardRecipientAssignment)
            }
            TYPE_SIGNA_MINING if value.subtype == SUBTYPE_SIGNA_MINING_COMMITMENT_ADD => {
                TransactionType::SignaMining(SignaMiningSubtype::CommitmentAdd)
            }
            TYPE_SIGNA_MINING if value.subtype == SUBTYPE_SIGNA_MINING_COMMITMENT_REMOVE => {
                TransactionType::SignaMining(SignaMiningSubtype::CommitmentRemove)
            }
            TYPE_ADVANCED_PAYMENT if value.subtype == SUBTYPE_ADVANCED_PAYMENT_ESCROW_CREATION => {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::EscrowCreation)
            }
            TYPE_ADVANCED_PAYMENT if value.subtype == SUBTYPE_ADVANCED_PAYMENT_ESCROW_SIGN => {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::EscrowSign)
            }
            TYPE_ADVANCED_PAYMENT if value.subtype == SUBTYPE_ADVANCED_PAYMENT_ESCROW_RESULT => {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::EscrowResult)
            }
            TYPE_ADVANCED_PAYMENT
                if value.subtype == SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_SUBSCRIBE =>
            {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::SubscriptionSubscribe)
            }
            TYPE_ADVANCED_PAYMENT
                if value.subtype == SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_CANCEL =>
            {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::SubscriptionCancel)
            }
            TYPE_ADVANCED_PAYMENT
                if value.subtype == SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_PAYMENT =>
            {
                TransactionType::AdvancedPayment(AdvancedPaymentSubtype::SubscriptionPayment)
            }
            TYPE_AUTOMATED_TRANSACTIONS if value.subtype == SUBTYPE_AT_CREATION => {
                TransactionType::AutomatedTransactions(AutomatedTransactionsSubtype::Creation)
            }
            TYPE_AUTOMATED_TRANSACTIONS if value.subtype == SUBTYPE_AT_NXT_PAYMENT => {
                TransactionType::AutomatedTransactions(AutomatedTransactionsSubtype::NxtPayment)
            }
            _ => {
                anyhow::bail!("unable to convert B1Transaction to Transaction");
            }
        };
        let t = Transaction::V1 {
            transaction_type,
            timestamp: value.timestamp,
            deadline: value.deadline,
            sender_public_key: value.sender_public_key,
            recipient: value.recipient,
            recipient_rs: value.recipient_rs,
            amount_nqt: value.amount_nqt,
            fee_nqt: value.fee_nqt,
            signature: value.signature,
            signature_hash: value.signature_hash,
            full_hash: value.full_hash,
            transaction: value.transaction,
            sender: value.sender,
            sender_rs: value.sender_rs,
            height: value.height,
            version: value.version,
            ec_block_height: value.ec_block_height,
            cash_back_id: value.cash_back_id,
            verify: value.verify,
        };
        todo!() // TODO: construct and return a transaction based on type
    }
}

const TYPE_PAYMENT: u8 = 0;
const TYPE_MESSAGING: u8 = 1;
const TYPE_COLORED_COINS: u8 = 2;
const TYPE_DIGITAL_GOODS: u8 = 3;
const TYPE_ACCOUNT_CONTROL: u8 = 4;
const TYPE_SIGNA_MINING: u8 = 20;
const TYPE_ADVANCED_PAYMENT: u8 = 21;
const TYPE_AUTOMATED_TRANSACTIONS: u8 = 22;

const SUBTYPE_PAYMENT_ORDINARY_PAYMENT: u8 = 0;
const SUBTYPE_PAYMENT_ORDINARY_PAYMENT_MULTI_OUT: u8 = 1;
const SUBTYPE_PAYMENT_ORDINARY_PAYMENT_MULTI_SAME_OUT: u8 = 2;

const SUBTYPE_MESSAGING_ARBITRARY_MESSAGE: u8 = 0;
const SUBTYPE_MESSAGING_ALIAS_ASSIGNMENT: u8 = 1;
const SUBTYPE_MESSAGING_ACCOUNT_INFO: u8 = 5;
const SUBTYPE_MESSAGING_ALIAS_SELL: u8 = 6;
const SUBTYPE_MESSAGING_ALIAS_BUY: u8 = 7;
const SUBTYPE_MESSAGING_TLD_ASSIGNMENT: u8 = 8;

const SUBTYPE_COLORED_COINS_ASSET_ISSUANCE: u8 = 0;
const SUBTYPE_COLORED_COINS_ASSET_TRANSFER: u8 = 1;
const SUBTYPE_COLORED_COINS_ASK_ORDER_PLACEMENT: u8 = 2;
const SUBTYPE_COLORED_COINS_BID_ORDER_PLACEMENT: u8 = 3;
const SUBTYPE_COLORED_COINS_ASK_ORDER_CANCELLATION: u8 = 4;
const SUBTYPE_COLORED_COINS_BID_ORDER_CANCELLATION: u8 = 5;
const SUBTYPE_COLORED_COINS_ASSET_MINT: u8 = 6;
const SUBTYPE_COLORED_COINS_ADD_TREASURY_ACCOUNT: u8 = 7;
const SUBTYPE_COLORED_COINS_DISTRIBUTE_TO_HOLDERS: u8 = 8;
const SUBTYPE_COLORED_COINS_ASSET_MULTI_TRANSFER: u8 = 9;
const SUBTYPE_COLORED_COINS_TRANSFER_OWNERSHIP: u8 = 10;

const SUBTYPE_DIGITAL_GOODS_LISTING: u8 = 0;
const SUBTYPE_DIGITAL_GOODS_DELISTING: u8 = 1;
const SUBTYPE_DIGITAL_GOODS_PRICE_CHANGE: u8 = 2;
const SUBTYPE_DIGITAL_GOODS_QUANTITY_CHANGE: u8 = 3;
const SUBTYPE_DIGITAL_GOODS_PURCHASE: u8 = 4;
const SUBTYPE_DIGITAL_GOODS_DELIVERY: u8 = 5;
const SUBTYPE_DIGITAL_GOODS_FEEDBACK: u8 = 6;
const SUBTYPE_DIGITAL_GOODS_REFUND: u8 = 7;

const SUBTYPE_AT_CREATION: u8 = 0;
const SUBTYPE_AT_NXT_PAYMENT: u8 = 1;

const SUBTYPE_ACCOUNT_CONTROL_EFFECTIVE_BALANCE_LEASING: u8 = 0;

const SUBTYPE_SIGNA_MINING_REWARD_RECIPIENT_ASSIGNMENT: u8 = 0;
const SUBTYPE_SIGNA_MINING_COMMITMENT_ADD: u8 = 1;
const SUBTYPE_SIGNA_MINING_COMMITMENT_REMOVE: u8 = 2;

const SUBTYPE_ADVANCED_PAYMENT_ESCROW_CREATION: u8 = 0;
const SUBTYPE_ADVANCED_PAYMENT_ESCROW_SIGN: u8 = 1;
const SUBTYPE_ADVANCED_PAYMENT_ESCROW_RESULT: u8 = 2;
const SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_SUBSCRIBE: u8 = 3;
const SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_CANCEL: u8 = 4;
const SUBTYPE_ADVANCED_PAYMENT_SUBSCRIPTION_PAYMENT: u8 = 5;
