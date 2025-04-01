// FIX: REMOVE THIS ALLOW
#![allow(dead_code)]
use crate::configuration::HistoricalMoments;

use super::HistoricalMoment;

/// A technique to handle values that change over the history of the blockchain.
pub struct FluxCapacitor {
    chain_channel: (),
    historical_moments: HistoricalMoments,
}

impl FluxCapacitor {
    /// Creates a new FluxCapacitor.
    pub fn new(chain_channel: (), historical_moments: HistoricalMoments) -> Self {
        Self {
            chain_channel,
            historical_moments,
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
    // SRS uses this to changev alues in the TestNetwork.java class but SignumOasis may
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
