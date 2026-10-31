//! SigmaOS Display Management
//! Inspired by Linux Mint and Omarchy Linux
//! Handles display configuration and output management.

/// DisplayMode represents the current display resolution and refresh rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    R640x480,
    R800x600,
    R1024x768,
    R1920x1080,
    R2560x1440,
    R3840x2160,
}

impl DisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R640x480 => "640x480",
            Self::R800x600 => "800x600",
            Self::R1024x768 => "1024x768",
            Self::R1920x1080 => "1920x1080",
            Self::R2560x1440 => "2560x1440",
            Self::R3840x2160 => "3840x2160",
        }
    }
}

/// DisplayProvider manages display configuration and output.
pub struct DisplayProvider {
    mode: DisplayMode,
    initialized: bool,
}

impl Default for DisplayProvider {
    fn default() -> Self {
        Self {
            mode: DisplayMode::R1920x1080,
            initialized: false,
        }
    }
}

impl DisplayProvider {
    pub fn new(mode: DisplayMode) -> Self {
        Self {
            mode,
            initialized: false,
        }
    }

    pub fn mode(&self) -> DisplayMode {
        self.mode
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize display subsystem
        self.initialized = true;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: DisplayMode) -> Result<(), String> {
        self.mode = mode;
        Ok(())
    }
}