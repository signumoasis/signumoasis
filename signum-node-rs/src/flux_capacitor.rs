pub trait FluxValue<T> {
    fn get_flux_value(height: u32) -> T;
}

pub struct FluxChangeValue<T> {
    pub(crate) height: u32,
    pub(crate) new_value: T,
}
impl<T> FluxChangeValue<T> {
    pub fn new(height: u32, new_value: T) -> Self {
        Self { height, new_value }
    }
}

pub struct Flux<T> {
    genesis_value: T,
    /// A vec of tuple of u32 (height) and T (the value)
    changes: Vec<FluxChangeValue<T>>,
}
impl<T> Flux<T> {
    pub const fn new(genesis_value: T, changes: Vec<FluxChangeValue<T>>) -> Self {
        Self {
            genesis_value,
            changes,
        }
    }
    pub fn get_flux_value(height: u32) -> T {
        todo!()
    }
}

pub static MAX_PAYLOAD_LENGTH: Flux<u32> = Flux::<u32>::new(
    255 * 176,
    vec![
        FluxChangeValue::new(historical_moments::PRE_POC2.get().unwrap(), 255 * 176 * 4),
        FluxChangeValue::new(
            historical_moments::SMART_FEES_ENABLE.get().unwrap(),
            255 * (176 + 8) * 4 * 2,
        ),
    ],
);
//pub static MAX_PAYLOAD_LENGTH: Flux<u32> = Flux::<u32>::new(
//    255 * 176,
//    vec![
//        FluxChangeValue::new(historical_moments::PRE_POC2.get().unwrap(), 255 * 176 * 4),
//        FluxChangeValue::new(
//            historical_moments::SMART_FEES_ENABLE.get().unwrap(),
//            255 * (176 + 8) * 4 * 2,
//        ),
//    ],
//);
