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

/// DMA direction.
pub trait Direction {
    /// Returns the `DmaDirection` of the type.
    fn direction() -> DmaDirection;
}

/// Mark a target that the DMA can use. This is a peripheral (PeripheralToMemory
/// or MemoryToPeripheral) or a memory (MemoryToMemory)
///
/// This trait is generic over transfer direction, so a given target can
/// implement this trait multiple times for different directions.
///
/// Each implementation has an associated memory size (u32/u16/u8) and
/// optionally an associated request line in the DMA's DMAMUX.
///
/// # Safety
///
/// Both the memory size and the address must be correct for the memory region
/// and for the DMA.
pub unsafe trait TargetAddress<D: Direction> {
    /// Memory size of the target address
    type MemSize;

    /// The address to be used by the DMA stream
    fn address(&self) -> u32;

    /// An optional associated request line
    const REQUEST_LINE: Option<u8> = None;
}

/// Possible DMA's directions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DmaDirection {
    /// Memory to Memory transfer.
    MemoryToMemory,
    /// Peripheral to Memory transfer.
    PeripheralToMemory,
    /// Memory to Peripheral transfer.
    MemoryToPeripheral,
}

/// DMA from a peripheral to a memory location.
#[derive(Debug, Clone, Copy)]
pub struct PeripheralToMemory;

impl PeripheralToMemory {
    pub fn new() -> Self {
        PeripheralToMemory
    }
}

impl Default for PeripheralToMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl Direction for PeripheralToMemory {
    #[inline(always)]
    fn direction() -> DmaDirection {
        DmaDirection::PeripheralToMemory
    }
}

/// DMA from one memory location to another memory location.
#[derive(Debug, Clone, Copy)]
pub struct MemoryToMemory<T>
where
    T: Into<u32>,
{
    data: T,
}

impl<T> MemoryToMemory<T>
where
    T: Into<u32>,
{
    pub fn new(t: T) -> Self {
        Self { data: t }
    }
}

impl<T> Direction for MemoryToMemory<T>
where
    T: Into<u32>,
{
    #[inline(always)]
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToMemory
    }
}

/// DMA from a memory location to a peripheral.
#[derive(Debug, Clone, Copy)]
pub struct MemoryToPeripheral;

impl MemoryToPeripheral {
    pub fn new() -> Self {
        MemoryToPeripheral
    }
}

impl Default for MemoryToPeripheral {
    fn default() -> Self {
        Self::new()
    }
}

impl Direction for MemoryToPeripheral {
    fn direction() -> DmaDirection {
        DmaDirection::MemoryToPeripheral
    }
}

unsafe impl TargetAddress<Self> for MemoryToMemory<u8> {
    fn address(&self) -> u32 {
        self.data.into()
    }
    type MemSize = u8;
}

unsafe impl TargetAddress<Self> for MemoryToMemory<u16> {
    fn address(&self) -> u32 {
        self.data.into()
    }
    type MemSize = u16;
}

unsafe impl TargetAddress<Self> for MemoryToMemory<u32> {
    fn address(&self) -> u32 {
        self.data
    }
    type MemSize = u32;
}
