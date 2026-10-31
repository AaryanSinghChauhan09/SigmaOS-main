//! SigmaOS Power Management
//! Inspired by Linux Mint and Omarchy Linux
//! Manages power settings, CPU governors, and power state transitions.

/// PowerMode represents the current power mode of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    Performance,
    Powersave,
    Balanced,
    Auto,
}

impl PowerMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Balanced => "balanced",
            Self::Auto => "auto",
        }
    }
}

/// PowerManager manages system power settings and transitions.
pub struct PowerManager {
    mode: PowerMode,
    swappiness: u32,
    initialized: bool,
}

impl Default for PowerManager {
    fn default() -> Self {
        Self {
            mode: PowerMode::Balanced,
            swappiness: 10,
            initialized: false,
        }
    }
}

impl PowerManager {
    pub fn new(mode: PowerMode, swappiness: u32) -> Self {
        Self {
            mode,
            swappiness,
            initialized: false,
        }
    }

    pub fn mode(&self) -> PowerMode {
        self.mode
    }

    pub fn swappiness(&self) -> u32 {
        self.swappiness
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize power management
        self.initialized = true;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: PowerMode) -> Result<(), String> {
        self.mode = mode;
        Ok(())
    }

    pub fn set_swappiness(&mut self, swappiness: u32) {
        self.swappiness = swappiness.min(100);
    }
}