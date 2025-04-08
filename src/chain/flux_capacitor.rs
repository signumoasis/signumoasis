// FIX: REMOVE THIS ALLOW
#![allow(dead_code)]

use std::sync::OnceLock;

use super::HistoricalMoment;

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
pub struct FluxValueInner<T> {
    default: T,
    changes: Option<Vec<ValueAtHeight<T>>>,
}

impl<T: Clone> FluxValueInner<T> {
    /// Creates a new FluxValue.
    ///
    /// Set `changes` to `None` or to a Vec<ValueAtHeight<T>>.
    pub fn new(default: T, changes: Option<Vec<ValueAtHeight<T>>>) -> Self {
        Self { default, changes }
    }

    /// Returns the list of [`ValueAtHeight`] in this FluxValue.
    pub fn get_changes(&self) -> Option<Vec<ValueAtHeight<T>>> {
        self.changes.clone()
    }

    /// Returns the default value of this FluxValue.
    pub fn get_default_value(&self) -> T {
        self.default.clone()
    }

    /// Retrieves the value of this FluxValue at a specific height. Pass the current
    /// chain height to get the latest value.
    pub fn get_value_at(&self, height: u32) -> T {
        if let Some(changes) = &self.changes {
            let mut value_at_height = self.get_default_value();
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
            self.get_default_value()
        }
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
}

pub type FluxEnableInner = FluxValueInner<bool>;

impl FluxEnableInner {
    /// Create an new FluxEnable that begins at the specified height..
    pub fn enable(enable_height: HistoricalMoment) -> Self {
        Self::new(false, Some(vec![ValueAtHeight::new(enable_height, true)]))
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
    /// Gets the height at which this FluxEnable takes effect.
    ///
    /// # Panics
    ///
    /// This method will panic if there is a missing or empty changes vec.
    /// While that case should never happen, it's possible if creating the FluxEnable
    /// using the `new()` method and passing `None` or an empty vec.
    pub fn get_enable_height(&self) -> HistoricalMoment {
        self.inner
            .get()
            .expect("FluxValue not initialized")
            .changes
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

pub struct FluxValue<T> {
    inner: OnceLock<FluxValueInner<T>>,
}

impl<T: Clone> FluxValue<T> {
    /// Attempts to set the OnceLock. No error value needed, just the error state.
    fn set(&self, value: FluxValueInner<T>) -> Result<(), FluxError> {
        if self.inner.set(value).is_err() {
            return Err(FluxError::SetFailure);
        }

        Ok(())
    }

    /// Gets the value at a specified height
    pub fn get_value(&self, height: u32) -> T {
        self.inner
            .get()
            .expect("FluxValue not initialized")
            .get_value_at(height)
    }
}

impl<T: Clone> FluxValue<T> {
    /// Returns the list of [`ValueAtHeight`] in this FluxValue.
    pub fn get_changes(&self) -> Option<Vec<ValueAtHeight<T>>> {
        self.inner
            .get()
            .expect("FluxValue not initialized")
            .changes
            .clone()
    }

    /// Returns the default value of this FluxValue.
    pub fn get_default_value(&self) -> T {
        self.inner
            .get()
            .expect("FluxValue not initialized")
            .default
            .clone()
    }

    /// Retrieves the value of this FluxValue at a specific height. Pass the current
    /// chain height to get the latest value.
    pub fn get_value_at(&self, height: u32) -> T {
        if let Some(changes) = &self.inner.get().expect("FluxValue not initialized").changes {
            let mut value_at_height = self.get_default_value();
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
            self.get_default_value()
        }
    }

    // TODO: Not sure update_height_values is even needed.
    // SRS uses this to change values in the TestNetwork.java class but SignumOasis may
    // not need it. Seems to be used because some of the values that flux are different instead
    // of just the points where they shift. Would prefer to take a list from a config.

    /// Replace the list of height-activated values. Allows for overriding FluxValue
    /// values.
    pub fn update_height_values(&mut self, changes: Vec<ValueAtHeight<T>>) {
        self.inner
            .get_mut()
            .expect("FluxValue not initialized")
            .default = changes
            .first()
            .cloned()
            .expect("update_changes got an empty vec and can't set default")
            .get_new_value();
        self.inner
            .get_mut()
            .expect("FluxValue not initialized")
            .changes = Some(changes);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FluxError {
    #[error("unable to set value")]
    SetFailure,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub mod flux_values {
    use std::sync::OnceLock;

    use crate::configuration::HistoricalMoments;

    use super::{FluxEnable, FluxEnableInner, FluxValue, FluxValueInner, ValueAtHeight};

    use anyhow::Context;

    pub static REWARD_RECIPIENT_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static DIGITAL_GOODS_STORE_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_FIX_1_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_FIX_2_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_FIX_3_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static PRE_POC2_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static POC2_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SODIUM_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SIGNUM_NAME_CHANGE_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static POC_PLUS_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SPEEDWAY_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SMART_TOKEN_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SMART_FEES_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SMART_ATS_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_FIX_4_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static DISTRIBUTION_FIX_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static PK_FREEZE_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static PK_FREEZE_2_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static SMART_ALIAS_ENABLE: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static NEXT_FORK: FluxEnable = FluxEnable {
        inner: OnceLock::new(),
    };
    pub static BLOCK_TIME: FluxValue<u32> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static FEE_QUANT: FluxValue<u64> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static AUTOMATED_TRANSACTION_VERSION: FluxValue<u16> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static MAX_NUMBER_TRANSACTIONS: FluxValue<u32> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static MAX_PAYLOAD_LENGTH: FluxValue<u32> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static MIN_CAPACITY: FluxValue<u64> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static COMMITMENT_WAIT: FluxValue<u32> = FluxValue {
        inner: OnceLock::new(),
    };
    pub static AVERAGE_COMMITMENT_WINDOW: FluxValue<u64> = FluxValue {
        inner: OnceLock::new(),
    };

    pub fn set_flux_values(historical_moments: HistoricalMoments) -> Result<(), anyhow::Error> {
        DIGITAL_GOODS_STORE_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.digital_goods_store_enable,
            ))
            .context("unable to set DIGITAL_GOODS_STORE_ENABLE")?;

        REWARD_RECIPIENT_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.reward_recipient_enable,
            ))
            .context("unable to set REWARD_RECIPIENT_ENABLE")?;

        AUTOMATED_TRANSACTION_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.automated_transaction_enable,
            ))
            .context("unable to set AUTOMATED_TRANSACTION_ENABLE")?;

        AUTOMATED_TRANSACTION_FIX_1_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.automated_transaction_fix_1,
            ))
            .context("unable to set AUTOMATED_TRANSACTION_FIX_1")?;

        AUTOMATED_TRANSACTION_FIX_2_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.automated_transaction_fix_2,
            ))
            .context("unable to set AUTOMATED_TRANSACTION_FIX_2")?;

        AUTOMATED_TRANSACTION_FIX_3_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.automated_transaction_fix_3,
            ))
            .context("unable to set AUTOMATED_TRANSACTION_FIX_3")?;

        PRE_POC2_ENABLE
            .set(FluxEnableInner::enable(historical_moments.pre_poc2))
            .context("unable to set PRE_POC2")?;

        POC2_ENABLE
            .set(FluxEnableInner::enable(historical_moments.poc2_enable))
            .context("unable to set POC2_ENABLE")?;

        SODIUM_ENABLE
            .set(FluxEnableInner::enable(historical_moments.sodium_enable))
            .context("unable to set SODIUM_ENABLE")?;

        SIGNUM_NAME_CHANGE_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.signum_name_change,
            ))
            .context("unable to set SIGNUM_NAME_CHANGE")?;

        POC_PLUS_ENABLE
            .set(FluxEnableInner::enable(historical_moments.poc_plus_enable))
            .context("unable to set POC_PLUS_ENABLE")?;

        SPEEDWAY_ENABLE
            .set(FluxEnableInner::enable(historical_moments.speedway_enable))
            .context("unable to set SPEEDWAY_ENABLE")?;

        SMART_TOKEN_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.smart_token_enable,
            ))
            .context("unable to set SMART_TOKEN_ENABLE")?;

        SMART_FEES_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.smart_fees_enable,
            ))
            .context("unable to set SMART_FEES_ENABLE")?;

        SMART_ATS_ENABLE
            .set(FluxEnableInner::enable(historical_moments.smart_ats_enable))
            .context("unable to set SMART_ATS_ENABLE")?;

        AUTOMATED_TRANSACTION_FIX_4_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.automated_transaction_fix_4,
            ))
            .context("unable to set AUTOMATED_TRANSACTION_FIX_4")?;

        DISTRIBUTION_FIX_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.distribution_fix_enable,
            ))
            .context("unable to set DISTRIBUTION_FIX_ENABLE")?;

        PK_FREEZE_ENABLE
            .set(FluxEnableInner::enable(historical_moments.pk_freeze))
            .context("unable to set PK_FREEZE")?;

        PK_FREEZE_2_ENABLE
            .set(FluxEnableInner::enable(historical_moments.pk_freeze_2))
            .context("unable to set PK_FREEZE_2")?;

        SMART_ALIAS_ENABLE
            .set(FluxEnableInner::enable(
                historical_moments.smart_alias_enable,
            ))
            .context("unable to set SMART_ALIAS_ENABLE")?;

        NEXT_FORK
            .set(FluxEnableInner::enable(historical_moments.next_fork))
            .context("unable to set NEXT_FORK")?;

        BLOCK_TIME
            .set(FluxValueInner::new(240, None))
            .context("unable to set BLOCK_TIME")?;

        FEE_QUANT
            .set(FluxValueInner::new(
                100_000_000u64,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 735_000u64),
                    ValueAtHeight::new(historical_moments.smart_fees_enable, 1_000_000u64),
                ]),
            ))
            .context("unable to set FEE_QUANT")?;

        AUTOMATED_TRANSACTION_VERSION
            .set(FluxValueInner::new(
                1,
                Some(vec![
                    ValueAtHeight::new(historical_moments.sodium_enable, 2u16),
                    ValueAtHeight::new(historical_moments.smart_ats_enable, 3u16),
                ]),
            ))
            .context("unable to set AUTOMATED_TRANSACTION_VERSION")?;

        MAX_NUMBER_TRANSACTIONS
            .set(FluxValueInner::new(
                255,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 255 * 4),
                    ValueAtHeight::new(historical_moments.smart_fees_enable, 255 * 4 * 2),
                ]),
            ))
            .context("unable to set MAX_NUMBER_TRANSACTIONS")?;

        MAX_PAYLOAD_LENGTH
            .set(FluxValueInner::new(
                255 * 176,
                Some(vec![
                    ValueAtHeight::new(historical_moments.pre_poc2, 255 * 176 * 4),
                    ValueAtHeight::new(
                        historical_moments.smart_fees_enable,
                        255 * (176 + 8) * 4 * 2,
                    ),
                ]),
            ))
            .context("unable to set MAX_PAYLOAD_LENGTH")?;

        MIN_CAPACITY
            .set(FluxValueInner::new(1000u64, None))
            .context("unable to set MIN_CAPACITY")?;

        COMMITMENT_WAIT
            .set(FluxValueInner::new(
                60,
                Some(vec![ValueAtHeight::new(
                    historical_moments.smart_ats_enable,
                    1440,
                )]),
            ))
            .context("unable to set COMMITMENT_WAIT")?;

        AVERAGE_COMMITMENT_WINDOW
            .set(FluxValueInner::new(
                24u64,
                Some(vec![ValueAtHeight::new(
                    historical_moments.speedway_enable,
                    96u64,
                )]),
            ))
            .context("unable to set AVERAGE_COMMITMENT_WINDOW")?;

        Ok(())
    }
}
