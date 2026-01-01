use crate::domain::tick::Tick;
use crate::domain::sequence::SequenceNumber;
use crate::protocol::error::ProtocolError;

pub const TICK_WIRE_SIZE: usize = 36;
const PAYLOAD_SIZE: usize = 32;

pub fn encode_tick(tick: &Tick, buffer: &mut [u8]) {
    assert!(buffer.len() >= TICK_WIRE_SIZE);

    buffer[0..4].copy_from_slice(&tick.instrument_id.to_le_bytes());
    buffer[4..12].copy_from_slice(&tick.price.to_le_bytes());
    buffer[12..16].copy_from_slice(&tick.quantity.to_le_bytes());
    buffer[16..24].copy_from_slice(&tick.sequence.value().to_le_bytes());
    buffer[24..32].copy_from_slice(&tick.timestamp_ns.to_le_bytes());

    let checksum = compute_checksum(&buffer[0..PAYLOAD_SIZE]);
    buffer[32..36].copy_from_slice(&checksum.to_le_bytes());
}

pub fn decode_tick(buffer: &[u8]) -> Result<Tick, ProtocolError> {
    if buffer.len() < TICK_WIRE_SIZE {
        return Err(ProtocolError::InvalidLength);
    }

    let expected = u32::from_le_bytes(buffer[32..36].try_into().unwrap());
    let actual = compute_checksum(&buffer[0..PAYLOAD_SIZE]);

    if expected != actual {
        return Err(ProtocolError::InvalidChecksum { expected, actual });
    }

    Ok(Tick {
        instrument_id: u32::from_le_bytes(buffer[0..4].try_into().unwrap()),
        price: i64::from_le_bytes(buffer[4..12].try_into().unwrap()),
        quantity: u32::from_le_bytes(buffer[12..16].try_into().unwrap()),
        sequence: SequenceNumber::new(
            u64::from_le_bytes(buffer[16..24].try_into().unwrap()),
        ),
        timestamp_ns: u64::from_le_bytes(buffer[24..32].try_into().unwrap()),
    })
}

fn compute_checksum(bytes: &[u8]) -> u32 {
    let mut sum = 0u32;
    for &b in bytes {
        sum = sum.wrapping_add(b as u32);
    }
    sum
}
