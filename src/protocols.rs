use std::{
    fmt::{Debug, Display},
    sync::mpsc,
};

#[cfg(feature = "server")]
use tokio::task::JoinError;

pub mod b1;
pub mod traits;

/// Messages to the Chain
pub enum ChainMessage {
    RegisterPlugin(mpsc::Sender<PluginMessage>),
    GetBlock,
    ProcessBlocks,
}

/// Messages to the Plugins
pub enum PluginMessage {
    GetMoreBlocks,
    BadBlock,
}

#[cfg(feature = "server")]
pub fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} task failed to complete",
                task_name
            )
        }
    }
}
