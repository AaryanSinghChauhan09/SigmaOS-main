//! SigmaOS Bootloader Integration
//! Inspired by Linux Mint m1n1 and Omarchy Linux
//! Handles bootloader configuration and initialization.

/// BootloaderType represents the type of bootloader used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootloaderType {
    Uefi,
    Uboot,
    Coreboot,
    M1n1,
}

impl BootloaderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uefi => "UEFI",
            Self::Uboot => "U-Boot",
            Self::Coreboot => "Coreboot",
            Self::M1n1 => "m1n1",
        }
    }
}

/// BootloaderManager handles bootloader detection and configuration.
pub struct BootloaderManager {
    bootloader: BootloaderType,
    initialized: bool,
}

impl Default for BootloaderManager {
    fn default() -> Self {
        Self {
            bootloader: BootloaderType::M1n1,
            initialized: false,
        }
    }
}

impl BootloaderManager {
    pub fn new(bootloader: BootloaderType) -> Self {
        Self {
            bootloader,
            initialized: false,
        }
    }

    pub fn bootloader(&self) -> BootloaderType {
        self.bootloader
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize the bootloader
        self.initialized = true;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), String> {
        // Reset the bootloader
        self.initialized = false;
        Ok(())
    }
}