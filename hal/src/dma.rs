//! DMA

/// Get an address and memory size the DMA can use.
///
/// # Safety
///
/// Both the memory size and the address must be correct for the specific peripheral and for the
/// DMA.
pub unsafe trait PeriAddress {
    /// Memory size of the peripheral.
    type MemSize;

    /// Returns the address to be used by the DMA stream.
    fn address(&self) -> u32;
}
