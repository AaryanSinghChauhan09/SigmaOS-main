// SigmaOS Bootloader Integration
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

/// BootPhase represents a phase of the boot process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootPhase {
    Init,
    Fallback,
}

impl BootPhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Init => "init",
            Self::Fallback => "fallback",
        }
    }
}

/// BootloaderManager handles bootloader detection and configuration.
pub struct BootloaderManager {
    bootloader: BootloaderType,
    initialized: bool,
    current_phase: BootPhase,
}

impl Default for BootloaderManager {
    fn default() -> Self {
        Self {
            bootloader: BootloaderType::M1n1,
            initialized: false,
            current_phase: BootPhase::Init,
        }
    }
}

impl BootloaderManager {
    pub fn new(bootloader: BootloaderType) -> Self {
        Self {
            bootloader,
            initialized: false,
            current_phase: BootPhase::Init,
        }
    }

    pub fn bootloader(&self) -> BootloaderType {
        self.bootloader
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Transition to a new boot phase.
    pub fn set_phase(&mut self, phase: BootPhase) {
        self.current_phase = phase;
    }

    /// Get the current boot phase.
    pub fn current_phase(&self) -> BootPhase {
        self.current_phase
    }

    /// Get the current boot phase as a string.
    pub fn phase_str(&self) -> &'static str {
        self.current_phase.as_str()
    }

    /// Initialize the bootloader.
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        self.current_phase = BootPhase::Init;
        Ok(())
    }

    /// Reset the bootloader.
    pub fn reset(&mut self) -> Result<(), String> {
        self.initialized = false;
        self.current_phase = BootPhase::Init;
        Ok(())
    }

    /// Enter fallback boot phase for rescue/fallback scenarios.
    pub fn enter_fallback(&mut self) -> Result<(), String> {
        self.current_phase = BootPhase::Fallback;
        Ok(())
    }

    /// Exit fallback boot phase and return to normal initialization.
    pub fn exit_fallback(&mut self) -> Result<(), String> {
        self.current_phase = BootPhase::Init;
        Ok(())
    }

    /// Whether the current phase is fallback.
    pub fn is_in_fallback(&self) -> bool {
        matches!(self.current_phase, BootPhase::Fallback)
    }
}