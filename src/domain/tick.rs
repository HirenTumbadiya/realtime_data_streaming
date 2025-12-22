use crate::domain::sequence::SequenceNumber;

#[derive(Debug, Copy, Clone)]
pub struct Tick {
    pub instrument_id: u32,
    pub price: i64,
    pub quantity: u32,
    pub sequence: SequenceNumber,
    pub timestamp_ns: u64,
}

impl Tick {
    pub fn new(
        instrument_id: u32,
        price: i64,
        quantity: u32,
        sequence: SequenceNumber,
        timestamp_ns: u64,
    ) -> Self {
        Self {
            instrument_id,
            price,
            quantity,
            sequence,
            timestamp_ns,
        }
    }
}
