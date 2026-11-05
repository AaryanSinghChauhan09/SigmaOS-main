//! SigmaOS System Adjustments
//! Inspired by Linux Mint mintsystem and Omarchy Linux
//! Manages CPU governors, sysctl memory tuning, and hardware optimization.
//! 
//! Aligned with Omarchy Linux config-management patterns for interoperability.

/// GovernorMode represents the CPU governor mode for system optimization.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernorMode {
    Performance, Powersave, Schedutil,
}

impl GovernorMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Performance => "performance", Self::Powersave => "powersave",
            Self::Schedutil => "schedutil",
        }
    }

    pub fn is_supervisor(&self) -> bool { !matches!(self, GovernorMode::Powersave) }
}

/// ConfigSchema represents the schema for system configuration matching Omarchy Linux config-schema pattern.
#[derive(Debug, Clone)]
pub struct ConfigSchema {
    pub sysctl: SysctlConfig, pub persona: PersonaConfig, pub keyboard: KeyboardConfig,
    pub locale: LocaleConfig, pub power: PowerDefaults,
}

impl Default for ConfigSchema {
    fn default() -> Self {
        Self { sysctl: SysctlConfig::default(), persona: PersonaConfig::default(),
            keyboard: KeyboardConfig::default(), locale: LocaleConfig::default(), power: PowerDefaults::default() }
    }
}

/// SysctlConfig represents sysctl configuration.
#[derive(Debug, Clone, Default)]
pub struct SysctlConfig {
    pub vm_swappiness: u32, pub vm_dirty_ratio: u32, pub cpu_governor: String,
}

impl SysctlConfig {
    fn default() -> Self { Self { vm_swappiness: 10, vm_dirty_ratio: 20, cpu_governor: "performance".into() } }
}

/// PersonaConfig represents persona configuration.
#[derive(Debug, Clone, Default)]
pub struct PersonaConfig {
    pub name: String, pub theme: String, pub wallpaper: String,
}

impl PersonaConfig {
    fn default() -> Self { Self { name: "default".into(), theme: "default".into(), wallpaper: "default".into() } }
}

/// KeyboardConfig represents keyboard configuration.
#[derive(Debug, Clone, Default)]
pub struct KeyboardConfig {
    pub layout: String, pub variant: String,
}

impl KeyboardConfig {
    fn default() -> Self { Self { layout: "us".into(), variant: "default".into() } }
}

/// LocaleConfig represents locale configuration.
#[derive(Debug, Clone, Default)]
pub struct LocaleConfig {
    pub language: String, pub region: String,
}

impl LocaleConfig {
    fn default() -> Self { Self { language: "en".into(), region: "US".into() } }
}

/// PowerDefaults represents power-saving defaults.
#[derive(Debug, Clone, Default)]
pub struct PowerDefaults {
    pub cpu_governor: String, pub screen_brightness: u8,
}

impl PowerDefaults {
    fn default() -> Self { Self { cpu_governor: "performance".into(), screen_brightness: 80 } }
}

/// SystemAdjuster manages system-level adjustments such as swappiness,
/// VFS cache pressure, and CPU governor selection matching Omarchy Linux config-management patterns.
#[derive(Debug, Clone)]
pub struct SystemAdjuster {
    swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode, config: ConfigSchema,
}

impl Default for SystemAdjuster {
    fn default() -> Self {
        Self { swappiness: 10, vfs_cache_pressure: 50, governor: GovernorMode::Schedutil, config: ConfigSchema::default() }
    }
}

impl SystemAdjuster {
    pub fn new(swappiness: u32, vfs_cache_pressure: u32, governor: GovernorMode) -> Self {
        Self { swappiness: swappiness.min(100), vfs_cache_pressure: vfs_cache_pressure.min(200), governor, config: ConfigSchema::default() }
    }

    pub fn swappiness(&self) -> u32 { self.swappiness }
    pub fn vfs_cache_pressure(&self) -> u32 { self.vfs_cache_pressure }
    pub fn governor(&self) -> GovernorMode { self.governor }

    pub fn set_governor(&mut self, governor: GovernorMode) -> Result<(), SigmaError> {
        if governor == GovernorMode::Performance && !self.is_governor_allowed(governor) {
            return Err(SigmaError::PrivilegeViolation);
        }
        self.governor = governor;
        self.config.power.cpu_governor = governor.as_str().into();
        Ok(())
    }

    fn is_governor_allowed(&self, governor: GovernorMode) -> bool { true }

    pub fn generate_sysctl_conf(&self) -> String {
        format!("# SigmaOS System Adjustments\nvm.swappiness = {}\nvm.vfs_cache_pressure = {}\nkernel.perf_stat = {}\n",
            self.swappiness, self.vfs_cache_pressure, self.governor.as_str())
    }

    pub fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        if self.swappiness > 100 { errors.push("swappiness must be between 0 and 100".into()); }
        if self.vfs_cache_pressure > 200 { errors.push("vfs_cache_pressure must be between 0 and 200".into()); }
        if self.config.persona.name.is_empty() { errors.push("persona name is required".into()); }
        if self.config.keyboard.layout.is_empty() { errors.push("keyboard layout is required".into()); }
        if self.config.locale.language.is_empty() { errors.push("locale language is required".into()); }
        ValidationResult::new(errors)
    }

    pub fn migrate(&self, old: &Self) -> MigrationResult {
        let mut preserved = Vec::new();
        if old.swappiness != self.swappiness { preserved.push("sysctl_swappiness".into()); }
        if old.vfs_cache_pressure != self.vfs_cache_pressure { preserved.push("sysctl_vfs_cache_pressure".into()); }
        if old.config.persona.name != self.config.persona.name { preserved.push("persona_name".into()); }
        if old.config.persona.theme != self.config.persona.theme { preserved.push("persona_theme".into()); }
        if old.config.keyboard.layout != self.config.keyboard.layout { preserved.push("keyboard_layout".into()); }
        if old.config.locale.language != self.config.locale.language { preserved.push("locale_language".into()); }
        MigrationResult::new(preserved)
    }

    pub fn config(&self) -> &ConfigSchema { &self.config }
    pub fn reset_to_defaults(&mut self) {
        self.swappiness = 10; self.vfs_cache_pressure = 50; self.governor = GovernorMode::Schedutil; self.config = ConfigSchema::default();
    }
}

/// ValidationResult matching Omarchy Linux validation pattern
#[derive(Debug, Clone)]
pub struct ValidationResult {
    errors: Vec<String>,
}

impl ValidationResult {
    fn new(errors: Vec<String>) -> Self { Self { errors } }
    pub fn is_valid(&self) -> bool { self.errors.is_empty() }
    pub fn errors(&self) -> &[String] { &self.errors }
}

/// MigrationResult matching Omarchy Linux migration pattern
#[derive(Debug, Clone)]
pub struct MigrationResult {
    preserved: Vec<String>, changes: Vec<String>,
}

impl MigrationResult {
    fn new(preserved: Vec<String>) -> Self { Self { preserved, changes: Vec::new() } }
    pub fn preserved(&self) -> &[String] { &self.preserved }
    pub fn changes(&self) -> &[String] { &self.changes }
    pub fn add_change(&mut self, change: String) { self.changes.push(change); }
}

// Export public API
