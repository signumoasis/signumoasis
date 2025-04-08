// FIX: REMOVE THIS ALLOW
#![allow(dead_code)]
use dioxus::prelude::history;

use crate::configuration::HistoricalMoments;

use super::HistoricalMoment;

/// A technique to handle values that change over the history of the blockchain.
pub struct FluxCapacitor {
    // TODO: Need to figure out if this is needed.
    // chain_channel: (),
    pub reward_recipient_enable: FluxEnable,
    pub digital_goods_store_enable: FluxEnable,
    pub automated_transaction_enable: FluxEnable,
    pub automated_transaction_fix_1: FluxEnable,
    pub automated_transaction_fix_2: FluxEnable,
    pub automated_transaction_fix_3: FluxEnable,
    pub pre_poc2: FluxEnable,
    pub poc2_enable: FluxEnable,
    pub sodium_enable: FluxEnable,
    pub signum_name_change: FluxEnable,
    pub poc_plus_enable: FluxEnable,
    pub speedway_enable: FluxEnable,
    pub smart_token_enable: FluxEnable,
    pub smart_fees_enable: FluxEnable,
    pub smart_ats_enable: FluxEnable,
    pub automated_transaction_fix_4: FluxEnable,
    pub distribution_fix_enable: FluxEnable,
    pub pk_freeze: FluxEnable,
    pub pk_freeze_2: FluxEnable,
    pub smart_alias_enable: FluxEnable,
    pub next_fork: FluxEnable,
    pub block_time: FluxValue<u32>,
    pub fee_quant: FluxValue<u64>,
    pub automated_transaction_version: FluxValue<u16>,
    pub max_number_transactions: FluxValue<u32>,
    pub max_payload_length: FluxValue<u32>,
    pub min_capacity: FluxValue<u64>,
    pub commitment_wait: FluxValue<u32>,
    pub average_commitment_window: FluxValue<u64>,
    // TODO: See if I need this. Represents changes to the min required peer version in B1.
    // Might be something to set directly in B1?
    // pub min_peer_version: FluxValue<Version>,
}

// TODO: See if we even need a struct flux capacitor. These methods can all exist on FluxValue directly
// and if using the global OnceCell method you grokked, you might not need this at all.
impl FluxCapacitor {
    /// Creates a new FluxCapacitor.
    pub fn new(
        // chain_channel: (),
        historical_moments: HistoricalMoments,
    ) -> Self {
        Self {
            // chain_channel,
            digital_goods_store_enable: FluxEnable::enable(
                historical_moments.digital_goods_store_enable,
            ),
            reward_recipient_enable: FluxEnable::enable(historical_moments.reward_recipient_enable),
            automated_transaction_enable: FluxEnable::enable(
                historical_moments.automated_transaction_enable,
            ),
            automated_transaction_fix_1: FluxEnable::enable(
                historical_moments.automated_transaction_fix_1,
            ),
            automated_transaction_fix_2: FluxEnable::enable(
                historical_moments.automated_transaction_fix_2,
            ),
            automated_transaction_fix_3: FluxEnable::enable(
                historical_moments.automated_transaction_fix_3,
            ),
            pre_poc2: FluxEnable::enable(historical_moments.pre_poc2),
            poc2_enable: FluxEnable::enable(historical_moments.poc2_enable),
            sodium_enable: FluxEnable::enable(historical_moments.sodium_enable),
            signum_name_change: FluxEnable::enable(historical_moments.signum_name_change),
            poc_plus_enable: FluxEnable::enable(historical_moments.poc_plus_enable),
            speedway_enable: FluxEnable::enable(historical_moments.speedway_enable),
            smart_token_enable: FluxEnable::enable(historical_moments.smart_token_enable),
            smart_fees_enable: FluxEnable::enable(historical_moments.smart_fees_enable),
            smart_ats_enable: FluxEnable::enable(historical_moments.smart_ats_enable),
            automated_transaction_fix_4: FluxEnable::enable(
                historical_moments.automated_transaction_fix_4,
            ),
            distribution_fix_enable: FluxEnable::enable(historical_moments.distribution_fix_enable),
            pk_freeze: FluxEnable::enable(historical_moments.pk_freeze),
            pk_freeze_2: FluxEnable::enable(historical_moments.pk_freeze_2),
            smart_alias_enable: FluxEnable::enable(historical_moments.smart_alias_enable),
            next_fork: FluxEnable::enable(historical_moments.next_fork),
            block_time: FluxValue::new(240, None),
            fee_quant: FluxValue::new(
                100_000_000u64,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 735_000u64),
                    ValueAtHeight::new(historical_moments.smart_fees_enable, 1_000_000u64),
                ]),
            ),
            automated_transaction_version: FluxValue::new(
                1,
                Some(vec![
                    ValueAtHeight::new(historical_moments.sodium_enable, 2u16),
                    ValueAtHeight::new(historical_moments.smart_ats_enable, 3u16),
                ]),
            ),
            max_number_transactions: FluxValue::new(
                255,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 255 * 4),
                    ValueAtHeight::new(historical_moments.smart_fees_enable, 255 * 4 * 2),
                ]),
            ),
            max_payload_length: FluxValue::new(
                255 * 176,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 255 * 176 * 4),
                    ValueAtHeight::new(
                        historical_moments.smart_fees_enable,
                        255 * (176 + 8) * 4 * 2,
                    ),
                ]),
            ),
            min_capacity: FluxValue::new(1000u64, None),
            commitment_wait: FluxValue::new(
                60,
                Some(vec![ValueAtHeight::new(
                    historical_moments.smart_ats_enable,
                    1440,
                )]),
            ),
            average_commitment_window: FluxValue::new(
                24u64,
                Some(vec![ValueAtHeight::new(
                    historical_moments.speedway_enable,
                    96u64,
                )]),
            ),
        }
    }

    /// Retrieves the value of a FluxValue at a specific height. Pass the current
    /// chain height to get the latest value.
    pub fn get_value_at<T: Clone>(flux_value: FluxValue<T>, height: u32) -> T {
        if let Some(changes) = flux_value.get_changes() {
            let mut value_at_height = flux_value.get_default_value();
            let mut most_recent_change_height = 0;
            for change in changes {
                let entry_height = change.get_historical_moment().0;

                if entry_height <= height && entry_height >= most_recent_change_height {
                    value_at_height = change.get_new_value();
                    most_recent_change_height = entry_height;
                }
            }
            value_at_height
        } else {
            flux_value.get_default_value()
        }
    }

    pub fn get_starting_height(flux_enable: FluxEnable) -> u32 {
        flux_enable.get_enable_height().0
    }
}

/// Represents a value that changes with time, depending on the current
/// height of the blockchain.
///
/// # Example
///
/// ```
/// let block_time: FluxValue<u32> = FluxValue::new(240u32, None);
///
/// let fee_quant: FluxValue<u64> = FluxValue::new(100_000_000u64, vec![
///     ValueAtHeight::new(historical_moments.pre_poc2, 735_000u64),
///     ValueAtHeight::new(historical_moments.smart_fees, 1_000_000u64),
/// ]);
/// ```
pub struct FluxValue<T> {
    default: T,
    changes: Option<Vec<ValueAtHeight<T>>>,
}

impl<T: Clone> FluxValue<T> {
    /// Creates a new FluxValue.
    ///
    /// Set `changes` to `None` or to a Vec<ValueAtHeight<T>>.
    pub fn new(default: T, changes: Option<Vec<ValueAtHeight<T>>>) -> Self {
        Self { default, changes }
    }

    // TODO: Not sure update_height_values is even needed.
    // SRS uses this to change values in the TestNetwork.java class but SignumOasis may
    // not need it. Seems to be used because some of the values that flux are different instead
    // of just the points where they shift. Would prefer to take a list from a config.

    /// Replace the list of height-activated values. Allows for overriding FluxValue
    /// values.
    pub fn update_height_values(&mut self, changes: Vec<ValueAtHeight<T>>) {
        self.default = changes
            .first()
            .cloned()
            .expect("update_changes got an empty vec and can't set default")
            .get_new_value();
        self.changes = Some(changes);
    }

    pub fn get_default_value(&self) -> T {
        self.default.clone()
    }

    pub fn get_changes(&self) -> Option<Vec<ValueAtHeight<T>>> {
        self.changes.clone()
    }
}

/// A type alias for FluxValue<bool>. This type has additional capability over
/// the generic. Instead of the 'new' method, you should use the 'enable' method for this.
///
/// # Example
///
/// ```
/// let reward_recipient = FluxEnable::enable(historical_moments.reward_recipient_enable);
///
/// let enable_height = reward_recipient.get_enable_height();
/// ```
pub type FluxEnable = FluxValue<bool>;

impl FluxEnable {
    /// Create an new FluxEnable that begins at the specified height..
    pub fn enable(enable_height: HistoricalMoment) -> Self {
        Self::new(false, Some(vec![ValueAtHeight::new(enable_height, true)]))
    }

    /// Gets the height at which this FluxEnable takes effect.
    ///
    /// # Panics
    ///
    /// This method will panic if there is a missing or empty changes vec.
    /// While that case should never happen, it's possible if creating the FluxEnable
    /// using the `new()` method and passing `None` or an empty vec.
    pub fn get_enable_height(&self) -> HistoricalMoment {
        self.changes
            .as_ref()
            .expect("somehow FluxEnable has no 'changes' collection")
            .first()
            .expect("the FluxEnable collection was empty")
            .get_historical_moment()
    }
}

/// A value that exists at and after a specific [`HistoricalMoment`].
/// It is generic over any other type that implements [`Clone`].
#[derive(Clone, Debug)]
pub struct ValueAtHeight<T> {
    historical_moment: HistoricalMoment,
    new_value: T,
}

impl<T: Clone> ValueAtHeight<T> {
    /// Create a new instance of the struct.
    pub fn new(historical_moment: HistoricalMoment, new_value: T) -> Self {
        Self {
            historical_moment,
            new_value,
        }
    }

    /// Get the historical moment at which this value begins being active.
    pub fn get_historical_moment(&self) -> HistoricalMoment {
        self.historical_moment
    }

    /// Get the value.
    pub fn get_new_value(&self) -> T {
        self.new_value.clone()
    }
}
