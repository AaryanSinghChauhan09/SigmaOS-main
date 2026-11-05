//! SigmaOS System Adjustments
//! Inspired by Linux Mint mintsystem
//! Manages CPU governors, sysctl memory tuning, and hardware optimization.

/// GovernorMode represents the CPU governor mode for system optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance,
    Powersave,
    Schedutil,
}

impl GovernorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance",
            Self::Powersave => "powersave",
            Self::Schedutil => "schedutil",
        }
    }
}

/// SystemAdjuster manages system-level adjustments such as swappiness,
/// VFS cache pressure, and CPU governor selection.
///
/// This is inspired by Linux Mint's mintsystem approach to system tuning.
#[derive(Debug, Clone)]
pub struct SystemAdjuster {
    swappiness: u32,
    vfs_cache_pressure: u32,
    governor: GovernorMode,
}

impl Default for SystemAdjuster {
    fn default() -> Self {
        Self {
            swappiness: 10,
            vfs_cache_pressure: 50,
            governor: GovernorMode::Schedutil,
        }
    }
}

impl SystemAdjuster {
    pub fn new(swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode) -> Self {
        Self {
            swappiness: swappiness.min(100),
            vfs_cache_pressure: vfs_cache_pressure.min(200),
            governor,
        }
    }

    /// Returns the swappiness value (0-100).
    pub fn swappiness(&self) -> u32 {
        self.swappiness
    }

    /// Returns the VFS cache pressure value.
    pub fn vfs_cache_pressure(&self) -> u32 {
        self.vfs_cache_pressure
    }

    /// Returns the current governor mode.
    pub fn governor(&self) -> GovernorMode {
        self.governor
    }

    /// Generates a sysctl configuration string for the current settings.
    ///
    /// The format is inspired by Linux Mint mintsystem sysctl configurations.
    pub fn generate_sysctl_conf(&self) -> String {
        format!(
            "# SigmaOS System Adjustments (mintsystem inspiration)\nvm.swappiness = {}\nvm.vfs_cache_pressure = {}\n",
            self.swappiness, self.vfs_cache_pressure
        )
    }
}