//! SigmaOS Hardware Drivers
//! Inspired by Linux Mint mintdrivers and Omarchy Linux
//! Provides hardware driver abstractions and implementations.

/// DriverStatus represents the status of a hardware driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Loaded,
    Unloaded,
    Error,
    Initializing,
}

impl DriverStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Loaded => "loaded",
            Self::Unloaded => "unloaded",
            Self::Error => "error",
            Self::Initializing => "initializing",
        }
    }
}

/// HardwareDriver represents a generic hardware driver.
pub struct HardwareDriver {
    name: String,
    status: DriverStatus,
    vendor_id: String,
}

impl HardwareDriver {
    pub fn new(name: &str, vendor_id: &str) -> Self {
        Self {
            name: name.to_string(),
            status: DriverStatus::Loaded,
            vendor_id: vendor_id.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> DriverStatus {
        self.status
    }

    pub fn vendor_id(&self) -> &str {
        &self.vendor_id
    }

    pub fn load(&mut self) -> Result<(), String> {
        self.status = DriverStatus::Loaded;
        Ok(())
    }

    pub fn unload(&mut self) -> Result<(), String> {
        self.status = DriverStatus::Unloaded;
        Ok(())
    }
}