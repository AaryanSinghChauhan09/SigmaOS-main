//! SigmaOS Power Management
//! Inspired by Linux Mint and Omarchy Linux
//! Manages power settings, CPU governors, and power state transitions.
//! 
//! Aligned with Omarchy Linux power-management patterns for interoperability.

/// PowerMode represents the current power mode of the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerMode {
    Performance, Powersave, Balanced, Auto,
}

impl PowerMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance", Self::Powersave => "powersave",
            Self::Balanced => "balanced", Self::Auto => "auto",
        }
    }

    pub fn is_supervisor(&self) -> bool { !matches!(self, PowerMode::Powersave) }

    pub fn is_user(&self) -> bool { matches!(self, PowerMode::Powersave | PowerMode::Auto) }
}

/// PowerManager manages system power settings and transitions matching Omarchy Linux power-management patterns.
pub struct PowerManager {
    mode: PowerMode, swappiness: u32, brightness: u8, initialized: bool,
    battery_logic: BatteryLogic, sleep_resume: SleepResume, low_power_mode: LowPowerMode,
}

impl Default for PowerManager {
    fn default() -> Self {
        Self { mode: PowerMode::Balanced, swappiness: 10, brightness: 80, initialized: false,
            battery_logic: BatteryLogic::default(), sleep_resume: SleepResume::default(), low_power_mode: LowPowerMode::default() }
    }
}

impl PowerManager {
    pub fn new(mode: PowerMode, swappiness: u32) -> Self {
        Self { mode, swappiness, brightness: 80, initialized: false,
            battery_logic: BatteryLogic::default(), sleep_resume: SleepResume::default(), low_power_mode: LowPowerMode::default() }
    }

    pub fn mode(&self) -> PowerMode { self.mode }
    pub fn swappiness(&self) -> u32 { self.swappiness }
    pub fn brightness(&self) -> u8 { self.brightness }
    pub fn is_initialized(&self) -> bool { self.initialized }

    pub fn initialize(&mut self) -> Result<(), SigmaError> {
        self.swappiness = self.swappiness.min(100);
        self.battery_logic = BatteryLogic::init()?;
        self.sleep_resume = SleepResume::init()?;
        self.low_power_mode = LowPowerMode::init()?;
        self.initialized = true;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: PowerMode) -> Result<(), SigmaError> {
        self.mode = mode;
        self.update_low_power_from_mode();
        Ok(())
    }

    pub fn set_swappiness(&mut self, swappiness: u32) { self.swappiness = swappiness.min(100); }

    pub fn set_brightness(&mut self, brightness: u8) { self.brightness = brightness.clamp(0, 100); }

    fn is_mode_allowed(&self, mode: PowerMode) -> bool { true }

    fn update_low_power_from_mode(&mut self) {
        match self.mode {
            PowerMode::Performance => self.low_power_mode.set_performance(),
            PowerMode::Powersave => self.low_power_mode.set_powersave(),
            PowerMode::Balanced => self.low_power_mode.set_balanced(),
            PowerMode::Auto => self.low_power_mode.set_auto(),
        }
    }
}

/// BatteryLogic matching Omarchy Linux battery-logic pattern
#[derive(Debug, Clone, Default)]
pub struct BatteryLogic {
    pub low_threshold: u8, pub critical_threshold: u8, pub action: PowerAction, pub warn_at: u8,
}

impl BatteryLogic {
    fn init() -> Result<Self, SigmaError> { Ok(Self { low_threshold: 10, critical_threshold: 5, action: PowerAction::Suspend, warn_at: 20 }) }
}

/// PowerAction enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerAction { Suspend, Hibernate, Shutdown, Nothing }

impl PowerAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerAction::Suspend => "suspend", PowerAction::Hibernate => "hibernate",
            PowerAction::Shutdown => "shutdown", PowerAction::Nothing => "nothing",
        }
    }
}

/// SleepResume matching Omarchy Linux sleep-resume pattern
#[derive(Debug, Clone, Default)]
pub struct SleepResume {
    pub suspend_disk: bool, pub hibernate: bool, pub resume_time: std::time::Duration,
}

impl SleepResume {
    fn init() -> Result<Self, SigmaError> { Ok(Self { suspend_disk: true, hibernate: true, resume_time: std::time::Duration::from_secs(2) }) }
}

/// LowPowerMode matching Omarchy Linux low-power-mode pattern
#[derive(Debug, Clone, Default)]
pub struct LowPowerMode {
    pub cpu_governor: String, pub screen_dimming: bool, pub usb_selective_suspend: bool, pub powersave_enabled: bool,
}

impl LowPowerMode {
    fn init() -> Result<Self, SigmaError> { Ok(Self { cpu_governor: "powersave".into(), screen_dimming: true, usb_selective_suspend: true, powersave_enabled: true }) }

    fn set_performance(&mut self) { self.cpu_governor = "performance".into(); self.screen_dimming = false; self.usb_selective_suspend = false; self.powersave_enabled = false; }
    fn set_powersave(&mut self) { self.cpu_governor = "powersave".into(); self.screen_dimming = true; self.usb_selective_suspend = true; self.powersave_enabled = true; }
    fn set_balanced(&mut self) { self.cpu_governor = "balanced".into(); self.screen_dimming = true; self.usb_selective_suspend = true; self.powersave_enabled = true; }
    fn set_auto(&mut self) { self.cpu_governor = "powersave".into(); self.screen_dimming = true; self.usb_selective_suspend = true; self.powersave_enabled = true; }
}

// Export public API
