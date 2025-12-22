#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    /// Create a new sequence number
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    /// Get the raw value
    pub fn value(self) -> u64 {
        self.0
    }

    /// Check if this sequence comes immediately after another
    pub fn is_next_after(self, previous: SequenceNumber) -> bool {
        self.0 == previous.0 + 1
    }
}
