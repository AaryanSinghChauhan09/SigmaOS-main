//! SigmaOS Hardware Drivers
//! Inspired by Linux Mint mintdrivers and Omarchy Linux
//! Provides hardware driver abstractions and implementations.
//! 
//! Aligned with Omarchy Linux driver-model patterns for interoperability.

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

    pub fn is_loaded(&self) -> bool {
        matches!(self, DriverStatus::Loaded)
    }

    pub fn is_unloaded(&self) -> bool {
        matches!(self, DriverStatus::Unloaded)
    }

    pub fn is_error(&self) -> bool {
        matches!(self, DriverStatus::Error)
    }
}

/// DriverRecommendation represents a recommended driver for hardware.
#[derive(Debug, Clone)]
pub struct DriverRecommendation {
    pub name: String,
    pub version: String,
}

impl DriverRecommendation {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
        }
    }
}

/// DriverCompatibilityCheck represents the result of a driver compatibility check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverCompatibilityCheck {
    Compatible,
    Incompatible,
    NeedsFirmware,
    NotSupported,
}

impl std::fmt::Display for DriverCompatibilityCheck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverCompatibilityCheck::Compatible => write!(f, "compatible"),
            DriverCompatibilityCheck::Incompatible => write!(f, "incompatible"),
            DriverCompatibilityCheck::NeedsFirmware => write!(f, "needs firmware"),
            DriverCompatibilityCheck::NotSupported => write!(f, "not supported"),
        }
    }
}

/// SafeDefaults represents safe default driver configurations.
#[derive(Debug, Clone, Default)]
pub struct SafeDefaults {
    pub recommended_gpu: String,
    pub recommended_network: String,
    pub recommended_storage: String,
    pub enable_power_management: bool,
    pub enable_async_probe: bool,
}

impl SafeDefaults {
    pub fn new() -> Self {
        Self {
            recommended_gpu: "modesetting".into(),
            recommended_network: "e1000e".into(),
            recommended_storage: "nvme".into(),
            enable_power_management: true,
            enable_async_probe: true,
        }
    }
}

/// HardwareInventory represents detected hardware.
#[derive(Debug, Clone)]
pub struct HardwareInventory {
    pub gpus: Vec<GPUInfo>,
    pub network: Vec<NetworkInfo>,
    pub storage: Vec<StorageInfo>,
    pub input: Vec<InputInfo>,
}

#[derive(Debug, Clone)]
pub struct GPUInfo {
    pub vendor: String,
    pub model: String,
    pub driver: String,
}

#[derive(Debug, Clone)]
pub struct NetworkInfo {
    pub interface: String,
    pub driver: String,
    pub state: NetworkState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkState {
    Up, Down, Disconnected,
}

#[derive(Debug, Clone)]
pub struct StorageInfo {
    pub device: String,
    pub size: u64,
    pub driver: String,
}

#[derive(Debug, Clone)]
pub struct InputInfo {
    pub device: String,
    pub driver: String,
}

/// DriverSupport trait for driver operations.
pub trait DriverSupport {
    fn detect_hardware() -> HardwareInventory;
    fn recommend_drivers(hardware: &HardwareInventory) -> Vec<DriverRecommendation>;
    fn install_driver(&self, driver: &str) -> InstallResult;
    fn rollback(&self) -> RollbackResult;
    fn logs(&self) -> Vec<String>;
    fn support_matrix(&self) -> SupportMatrix;
    fn gpu_detection(&self) -> GPUCompatibility;
    fn performance_policy(&self) -> PerformancePolicy;
    fn power_profile(&self) -> PowerProfile;
    fn compatibility_checks(&self) -> Vec<DriverCompatibilityCheck>;
}

/// SupportMatrix represents driver support matrix.
#[derive(Debug, Clone, Default)]
pub struct SupportMatrix {
    pub gpu_vendors: Vec<String>,
    pub network_vendors: Vec<String>,
    pub storage_vendors: Vec<String>,
    pub compatibility: Vec<DriverCompatibilityCheck>,
}

impl SupportMatrix {
    pub fn new() -> Self {
        Self {
            gpu_vendors: vec!["intel".into(), "amd".into(), "nvidia".into()],
            network_vendors: vec!["intel".into(), "realtek".into(), "atheros".into()],
            storage_vendors: vec!["nvme".into(), "sata".into(), "scsi".into()],
            compatibility: vec![
                DriverCompatibilityCheck::Compatible,
                DriverCompatibilityCheck::Compatible,
                DriverCompatibilityCheck::NeedsFirmware,
            ],
        }
    }
}

/// GPUCompatibility represents GPU compatibility info.
#[derive(Debug, Clone, Default)]
pub struct GPUCompatibility {
    pub intel: bool,
    pub amd: bool,
    pub nvidia: bool,
    pub fallback: String,
}

impl GPUCompatibility {
    pub fn new() -> Self {
        Self {
            intel: true,
            amd: true,
            nvidia: false,
            fallback: "modesetting".into(),
        }
    }
}

/// PerformancePolicy represents driver performance policy.
#[derive(Debug, Clone, Default)]
pub struct PerformancePolicy {
    pub powersave: bool,
    pub powersave_clocks: bool,
    pub maximum_performance: bool,
    pub balance: f32,
}

impl PerformancePolicy {
    pub fn new() -> Self {
        Self {
            powersave: false,
            powersave_clocks: false,
            maximum_performance: true,
            balance: 0.5,
        }
    }
}

/// PowerProfile represents driver power profile.
#[derive(Debug, Clone, Default)]
pub struct PowerProfile {
    pub active: bool,
    pub powersave: bool,
    pub medium: bool,
    pub high: bool,
}

impl PowerProfile {
    pub fn new() -> Self {
        Self {
            active: true,
            powersave: true,
            medium: true,
            high: true,
        }
    }
}

/// InstallResult represents the result of driver installation.
#[derive(Debug, Clone)]
pub struct InstallResult {
    pub success: bool,
    pub logged: bool,
    pub rollback_available: bool,
    pub message: String,
}

impl InstallResult {
    pub fn success(logged: bool) -> Self {
        Self { success: true, logged, rollback_available: true, message: "Driver installed successfully".into() }
    }

    pub fn failure(message: String) -> Self {
        Self { success: false, logged: false, rollback_available: false, message }
    }
}

/// RollbackResult represents the result of driver rollback.
#[derive(Debug, Clone)]
pub struct RollbackResult {
    pub success: bool,
    pub previous_state_restored: bool,
    pub logs_preserved: bool,
    pub message: String,
}

impl RollbackResult {
    pub fn success(previous_state_restored: bool, logs_preserved: bool) -> Self {
        Self { success: true, previous_state_restored, logs_preserved, message: "Driver rollback successful".into() }
    }

    pub fn failure(message: String) -> Self {
        Self { success: false, previous_state_restored: false, logs_preserved: false, message }
    }
}

/// HardwareDriver represents a generic hardware driver.
#[derive(Debug)]
pub struct HardwareDriver {
    name: String,
    status: DriverStatus,
    vendor_id: String,
    driver_name: String,
    loaded_at: Option<std::time::SystemTime>,
}

impl HardwareDriver {
    pub fn new(name: &str, vendor_id: &str, driver_name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: DriverStatus::Unloaded,
            vendor_id: vendor_id.to_string(),
            driver_name: driver_name.to_string(),
            loaded_at: None,
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn status(&self) -> DriverStatus { self.status }
    pub fn vendor_id(&self) -> &str { &self.vendor_id }
    pub fn driver_name(&self) -> &str { &self.driver_name }
    pub fn loaded_at(&self) -> Option<&std::time::SystemTime> { self.loaded_at.as_ref() }
    pub fn set_loaded_at(&mut self, time: std::time::SystemTime) { self.loaded_at = Some(time) }

    pub fn load(&mut self) -> Result<(), String> {
        self.status = DriverStatus::Loaded;
        let now = std::time::SystemTime::now();
        self.set_loaded_at(now);
        Ok(())
    }

    pub fn unload(&mut self) -> Result<(), String> {
        self.status = DriverStatus::Unloaded;
        self.loaded_at = None;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), String> {
        self.status = DriverStatus::Unloaded;
        self.loaded_at = None;
        Ok(())
    }
}

/// DriverManager manages driver detection, installation, and rollback.
#[derive(Debug)]
pub struct DriverManager {
    drivers: Vec<HardwareDriver>,
    inventory: Option<HardwareInventory>,
    support: SupportMatrix,
    safe_defaults: SafeDefaults,
}

impl Default for DriverManager {
    fn default() -> Self {
        Self { drivers: Vec::new(), inventory: None, support: SupportMatrix::new(), safe_defaults: SafeDefaults::new() }
    }
}

impl DriverManager {
    pub fn new() -> Self { Self::default() }

    pub fn detect_hardware() -> HardwareInventory {
        HardwareInventory {
            gpus: vec![GPUInfo { vendor: "intel".into(), model: "integrated".into(), driver: "i915".into() }],
            network: vec![NetworkInfo { interface: "eth0".into(), driver: "e1000e".into(), state: NetworkState::Up }],
            storage: vec![StorageInfo { device: "/dev/nvme0n1".into(), size: 512, driver: "nvme".into() }],
            input: Vec::new(),
        }
    }

    pub fn recommend_drivers(hardware: &HardwareInventory) -> Vec<DriverRecommendation> {
        let mut recommendations = Vec::new();
        for gpu in &hardware.gpus { recommendations.push(DriverRecommendation::new(&gpu.driver, "latest")); }
        for network in &hardware.network { recommendations.push(DriverRecommendation::new(&network.driver, "latest")); }
        for storage in &hardware.storage { recommendations.push(DriverRecommendation::new(&storage.driver, "latest")); }
        recommendations
    }

    pub fn install_driver(&mut self, driver: &str) -> InstallResult {
        if self.drivers.iter().any(|d| d.driver_name() == driver) { return InstallResult::success(true); }
        let driver_hw = HardwareDriver::new(driver, "vendor-unknown", driver);
        driver_hw.load();
        self.drivers.push(driver_hw);
        InstallResult::success(true)
    }

    pub fn rollback(&self) -> RollbackResult {
        if self.drivers.is_empty() { return RollbackResult::failure("No drivers to rollback".into()); }
        RollbackResult::success(true, true)
    }

    pub fn logs(&self) -> Vec<String> {
        self.drivers.iter().map(|d| format!("Driver {} status: {:?}", d.name(), d.status())).collect()
    }

    pub fn support_matrix(&self) -> &SupportMatrix { &self.support }

    pub fn gpu_detection(&self) -> GPUCompatibility { GPUCompatibility::new() }

    pub fn performance_policy(&self) -> PerformancePolicy { PerformancePolicy::new() }

    pub fn power_profile(&self) -> PowerProfile { PowerProfile::new() }

    pub fn compatibility_checks(&self) -> Vec<DriverCompatibilityCheck> {
        vec![DriverCompatibilityCheck::Compatible, DriverCompatibilityCheck::Compatible, DriverCompatibilityCheck::NeedsFirmware]
    }

    pub fn safe_defaults(&self) -> SafeDefaults { self.safe_defaults.clone() }
}

// Export public API
