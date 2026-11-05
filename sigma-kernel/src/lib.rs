//! SigmaOS Kernel Core
//! Inspired by Linux Mint and Omarchy Linux
//! Provides fundamental kernel services and abstractions.

/// KernelMode represents the current kernel operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

/// KernelProvider provides fundamental kernel services.
pub struct KernelProvider {
    mode: KernelMode,
    initialized: bool,
    memory_info: Option<String>,
    cpu_info: Option<String>,
}

impl Default for KernelProvider {
    fn default() -> Self {
        Self {
            mode: KernelMode::User,
            initialized: false,
            memory_info: None,
            cpu_info: None,
        }
    }
}

impl KernelProvider {
    pub fn new(mode: KernelMode) -> Self {
        Self {
            mode,
            initialized: false,
            memory_info: None,
            cpu_info: None,
        }
    }

    pub fn mode(&self) -> KernelMode {
        self.mode
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Initialize kernel subsystems.
    /// 
    /// This performs fundamental kernel initialization including:
    /// - Setting up memory management basics
    /// - Initializing CPU feature detection
    /// - Establishing basic data structures
    pub fn initialize(&mut self) -> Result<(), String> {
        // Mark as initialized
        self.initialized = true;
        
        // Initialize basic CPU information (simulated)
        self.cpu_info = Some("detected:x86_64".to_string());
        
        // Initialize basic memory information (simulated)
        self.memory_info = Some("memory: 512MiB".to_string());
        
        Ok(())
    }

    /// Switch kernel mode.
    pub fn switch_mode(&mut self, new_mode: KernelMode) -> Result<(), String> {
        self.mode = new_mode;
        Ok(())
    }

    /// Get CPU information (simulated).
    pub fn cpu_info(&self) -> Option<&str> {
        self.cpu_info.as_deref()
    }

    /// Get memory information (simulated).
    pub fn memory_info(&self) -> Option<&str> {
        self.memory_info.as_deref()
    }
}