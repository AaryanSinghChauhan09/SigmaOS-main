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
}

impl Default for KernelProvider {
    fn default() -> Self {
        Self {
            mode: KernelMode::User,
            initialized: false,
        }
    }
}

impl KernelProvider {
    pub fn new(mode: KernelMode) -> Self {
        Self {
            mode,
            initialized: false,
        }
    }

    pub fn mode(&self) -> KernelMode {
        self.mode
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize kernel subsystems
        self.initialized = true;
        Ok(())
    }

    pub fn switch_mode(&mut self, new_mode: KernelMode) -> Result<(), String> {
        // Switch kernel mode
        self.mode = new_mode;
        Ok(())
    }
}