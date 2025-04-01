// TODO: REMOVE THIS ALLOW DEAD CODE
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
        // NOTE: These four lines are just to test if the code is valid and will be removed.
        // They will not be used here. Just a syntax/build check to see if 'test' exists
        // on the FluxValue<bool>.
        let x = FluxValue::new(2u32, None);
        let y = FluxEnable::new(true, None);
        //let z = x.test();
        let g = y.test();
        Self {
            chain_channel,
            historical_moments,
        }
    }

    /// Retrieves the value of a FluxValue at a specific height.
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
        todo!()
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

pub type FluxEnable = FluxValue<bool>;
impl FluxValue<bool> {
    pub fn test() -> bool {
        true
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
        self.historical_moment.clone()
    }

    /// Get the value.
    pub fn get_new_value(&self) -> T {
        self.new_value.clone()
    }
}
