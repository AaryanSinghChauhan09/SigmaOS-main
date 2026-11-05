//! SigmaOS Kernel Core
//! Inspired by Linux Mint and Omarchy Linux
//! Provides fundamental kernel services and abstractions.
//! 
//! Aligned with Omarchy Linux kernel-backend patterns for interoperability.

/// KernelMode represents the current kernel operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelMode {
    User,
    Kernel,
    Hypervisor,
}

impl KernelMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Kernel => "kernel",
            Self::Hypervisor => "hypervisor",
        }
    }

    pub fn is_supervisor(&self) -> bool {
        matches!(self, Self::Kernel | Self::Hypervisor)
    }

    pub fn is_user(&self) -> bool {
        matches!(self, Self::User)
    }
}

/// KernelProvider provides fundamental kernel services.
pub struct KernelProvider {
    mode: KernelMode,
    initialized: bool,
}

impl Default for KernelProvider {
    fn default() -> Self {
        Self { mode: KernelMode::User, initialized: false }
    }
}

impl KernelProvider {
    pub fn new(mode: KernelMode) -> Self {
        Self { mode, initialized: false }
    }

    pub fn mode(&self) -> KernelMode {
        self.mode
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), SigmaError> {
        self.initialized = true;
        Ok(())
    }

    pub fn switch_mode(&mut self, new_mode: KernelMode) -> Result<(), SigmaError> {
        if new_mode == KernelMode::Hypervisor && !self.mode.is_supervisor() {
            return Err(SigmaError::PrivilegeViolation);
        }
        self.mode = new_mode;
        Ok(())
    }
}

/// CPU information structure
#[derive(Debug, Clone, Default)]
pub struct CpuInfo {
    pub vendor: String,
    pub model: String,
    pub features: CpuFeatures,
    pub cores: u32,
    pub topology: CpuTopology,
}

#[derive(Debug, Clone, Default)]
pub struct CpuFeatures {
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub aes: bool,
    pub clflush: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CpuTopology {
    pub physical_cores: u32,
    pub logical_cores: u32,
    pub numa_nodes: u32,
}

/// Memory information structure
#[derive(Debug, Clone, Default)]
pub struct MemoryInfo {
    pub total_pages: usize,
    pub free_pages: usize,
    pub page_size: usize,
    pub available_pages: usize,
    pub reserved: usize,
}

impl MemoryInfo {
    fn init(cpu: &CpuInfo) -> Result<Self, SigmaError> {
        Ok(Self {
            total_pages: 0,
            free_pages: 0,
            page_size: 4096,
            available_pages: 0,
            reserved: 0,
        })
    }
}

/// Interrupt entry structure
#[derive(Debug, Clone, Default)]
pub struct InterruptEntry {
    pub handler: InterruptHandler,
    pub present: bool,
    pub reserved: bool,
    pub descriptor: InterruptDescriptor,
}

impl InterruptEntry {
    pub fn new(handler: InterruptHandler) -> Self {
        Self {
            handler,
            present: false,
            reserved: false,
            descriptor: InterruptDescriptor::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct InterruptHandler {
    pub function: usize,
    pub stack_index: u8,
    pub reserved: [u8; 6],
}

#[derive(Debug, Clone, Default)]
pub struct InterruptDescriptor {
    pub gate_type: u8,
    pub dpl: u8,
    pub present: bool,
    pub offset_low: u16,
    pub selector: u16,
    pub flags: u16,
    pub offset_high: u32,
}


/// Boot information structure passed from bootloader
#[derive(Debug, Clone)]
pub struct BootInfo {
    pub command_line: String,
    pub memory_map_size: usize,
    pub memory_map: Vec<u8>,  // Would be [MemoryMapEntry] in full impl
    pub framebuffer: Option<FramebufferInfo>,
    pub reserved: usize,
}

/// Framebuffer information
#[derive(Debug, Clone)]
pub struct FramebufferInfo {
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub bytes_per_pixel: u32,
    pub format: FramebufferFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum FramebufferFormat {
    Rgb,
    Bgr,
    Bgra,
}

/// Kernel entry point signature expected by bootloader
pub type KernelEntry = fn(bootinfo: &BootInfo) -> !;

/// Error types for kernel operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigmaError {
    InvalidVector,
    PrivilegeViolation,
    NotInitialized,
    OutOfMemory,
    HardwareError,
    KernelPanic,
}

impl std::fmt::Display for SigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigmaError::InvalidVector => write!(f, "Interrupt vector out of range"),
            SigmaError::PrivilegeViolation => write!(f, "Privilege violation for mode switch"),
            SigmaError::NotInitialized => write!(f, "Kernel not initialized"),
            SigmaError::OutOfMemory => write!(f, "Out of memory"),
            SigmaError::HardwareError => write!(f, "Hardware error"),
            SigmaError::KernelPanic => write!(f, "Kernel panic"),
        }
    }
}

impl std::fmt::Display for CpuInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CPU: {} model {} ({} cores, features: SSE={}, AVX={})",
            self.vendor,
            self.model,
            self.cores,
            self.features.sse,
            self.features.avx
        )
    }
}

impl std::fmt::Display for MemoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Memory: {} total pages, {} free, {} page size",
            self.total_pages, self.free_pages, self.page_size
        )
    }
}

// Core kernel types exported at crate level

pub mod config {
    //! Kernel configuration subsystem

    #[derive(Debug, Clone, Default)]
    pub struct KernelCmdline {
        pub params: Vec<(String, String)>,
    }

    impl KernelCmdline {
        pub fn new() -> Self {
            Self { params: Vec::new() }
        }

        pub fn add(&mut self, key: String, value: String) {
            self.params.push((key, value));
        }

        pub fn get(&self, key: &str) -> Option<&String> {
            self.params.iter().find(|(k, _)| k == key).map(|(_, v)| v)
        }
    }
}

pub mod memory {
    //! Kernel memory management subsystem

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PageTableFlags {
        Present = 1 << 0,
        Write = 1 << 1,
        User = 1 << 2,
        WriteThrough = 1 << 3,
        CacheDisabled = 1 << 4,
        Accessed = 1 << 5,
        Dirty = 1 << 6,
        HugePage = 1 << 7,
        Global = 1 << 8,
        Available = 1 << 9,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct PageTableEntry {
        pub bits: u64,
    }

    impl PageTableEntry {
        pub fn new(flags: PageTableFlags) -> Self {
            Self { bits: flags as u64 }
        }

        pub fn flags_raw(pub fn flags_raw(&self) -> u32 {self) -> u32 {
            ((self.bits pub fn flags_raw(&self) -> u32 { 0xFFF) as u32)
            self.bits & 0xFFF
        }

        pub fn is_present(&self) -> bool {
            (self.bits & (1 << 0)) != 0
        }

        pub fn is_writable(&self) -> bool {
            (self.bits & (1 << 1)) != 0
        }

        pub fn is_user(&self) -> bool {
            (self.bits & (1 << 2)) != 0
        }
    }
}

pub mod interrupt {
    //! Kernel interrupt handling subsystem

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IrqType {
        Timer,
        Keyboard,
        Mouse,
        Disk,
        Network,
        Serial,
        Exception,
        Software,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum IrqState {
        Disabled,
        Enabled,
        Masked,
    }

    #[derive(Debug, Clone)]
    pub struct IrqDescriptor {
        pub irq: u32,
        pub handler: usize,
        pub type_: IrqType,
        pub state: IrqState,
        pub handler_data: usize,
    }

    impl IrqDescriptor {
        pub fn new(irq: u32, handler: usize, type_: IrqType) -> Self {
            Self {
                irq,
                handler,
                type_,
                state: IrqState::Enabled,
                handler_data: 0,
            }
        }
    }
}

pub mod time {
    //! Kernel time subsystem

    pub const TICK_RATE: u32 = 1000; // 1kHz

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SystemTime {
        pub seconds: u64,
        pub nanoseconds: u32,
    }

    impl SystemTime {
        pub fn now() -> Self {
            Self { seconds: 0, nanoseconds: 0 }
        }

        pub fn duration_since(&self, start: Self) -> Self {
            Self {
                seconds: self.seconds.saturating_sub(start.seconds),
                nanoseconds: self.nanoseconds.saturating_sub(start.nanoseconds),
            }
        }
    }
}

pub mod console {
    //! Kernel console I/O subsystem

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Color {
        Black, Blue, Green, Cyan, Red, Magenta, Brown,
        LightGray, DarkGray, LightBlue, LightGreen, LightCyan,
        LightRed, LightMagenta, LightBrown, White,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ColorPair {
        pub foreground: Color,
        pub background: Color,
    }

    impl ColorPair {
        pub const fn new(foreground: Color, background: Color) -> Self {
            Self { foreground, background }
        }
    }
}


// Kernel entry point called by bootloader after hardware initialization
// Returns ! (never type) indicating this function does not return
// Bootloader should present a shell-ready state after this if initialization succeeds
pub extern "C" fn kernel_main(bootinfo: *const core::ffi::c_void) -> ! {
    // Initialize what we can of the subsystems
    // The bootloader should continue toward a shell-ready state
    
    loop {
        core::hint::spin_loop();
    }
}

