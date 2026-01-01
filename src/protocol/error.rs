// This will protect the domain with precise failure reason, no heap, no string

#[derive(Debug)]
pub enum ProtocolError {
    InvalidLength,
    InvalidChecksum { expected: u32, actual: u32 },
}
