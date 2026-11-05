//! Sigma System Core Library
//! Principles: Single Responsibility, High Cohesion, Low Coupling, Defensive Programming
//! Inspired by Linux Mint mintsystem and Omarchy Linux modular design
//! Provides core runtime contract and health monitoring aligned with Omarchy Linux patterns.

use crate::health::HealthStatus;

/// SigmaError represents errors in the sigma-system crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigmaError {
    PrivilegeViolation, NotInitialized, InvalidConfig,
}

impl std::fmt::Display for SigmaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SigmaError::PrivilegeViolation => write!(f, "Privilege violation"),
            SigmaError::NotInitialized => write!(f, "System not initialized"),
            SigmaError::InvalidConfig => write!(f, "Invalid configuration"),
        }
    }
}

/// HealthStatus representing system health matching Omarchy Linux health model
pub mod health {
    //! Health checking subsystem matching Omarchy Linux health model

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum HealthStatus { Unknown, Healthy, Degraded, Critical }



    impl std::fmt::Display for HealthStatus {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                HealthStatus::Unknown => write!(f, "unknown"),
                HealthStatus::Healthy => write!(f, "healthy"),
                HealthStatus::Degraded => write!(f, "degraded"),
                HealthStatus::Critical => write!(f, "critical"),
            }
        }
    }
}

/// BaseRuntimeContract represents the base runtime contract matching Omarchy Linux omarchy-base patterns.
#[derive(Debug, Clone)]
pub struct BaseRuntimeContract {
    initialized: bool, health_status: HealthStatus,
}

impl BaseRuntimeContract {
    pub fn new() -> Self {
        Self { initialized: false, health_status: HealthStatus::Unknown }
    }

    pub fn initialize(&mut self) -> Result<(), SigmaError> {
        self.initialized = true;
        self.health_status = HealthStatus::Healthy;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool { self.initialized }
    pub fn health_status(&self) -> &HealthStatus { &self.health_status }
}



/// Panic handler - called on kernel panic
pub fn panic() -> ! {
    loop {
        // In a real system, this would halt or reset
        core::hint::spin_loop();
    }
}

/// Minimal runtime contract state
#[derive(Debug, Clone, Copy)]
pub enum RuntimeState {
    Uninitialized,
    Initializing,
    Ready,
    Error,
}

/// Global runtime state (singleton pattern)
pub static RUNTIME_STATE: SigmaError = SigmaError::NotInitialized;

/// Initialize the minimal runtime - called from bootloader
pub fn minimal_runtime_init() -> Result<(), SigmaError> {
    // Set up basic services that the shell needs
    Ok(())
}

/// Check if runtime is ready for user interaction
pub fn is_ready() -> bool {
    // In a real system, check various initialization states
    true
}

/// Simple debug output (would use serial port in real implementation)
pub fn debug_putchar(_c: char) {
    // Output to serial port in real implementation
}


/// ConfigStorageContract represents config storage matching Omarchy Linux omarchy-configurator patterns
#[derive(Debug, Clone)]
pub struct ConfigStorageContract {
    config_path: std::path::PathBuf, config_data: std::collections::BTreeMap<String, String>,
}

impl ConfigStorageContract {
    pub fn new(path: std::path::PathBuf) -> Result<Self, SigmaError> {
        let data = std::collections::BTreeMap::new();
        Ok(Self { config_path: path, config_data: data })
    }

    pub fn load(&mut self) -> Result<(), SigmaError> {
        if self.config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&self.config_path) {
                for line in content.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        self.config_data.insert(key.trim().into(), value.trim().into());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), SigmaError> { Ok(()) }
    pub fn get(&self, key: &str) -> Option<&String> { self.config_data.get(key) }
}