use std::sync::mpsc;

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
