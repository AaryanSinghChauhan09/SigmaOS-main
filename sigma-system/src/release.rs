/// Release quality report infrastructure for SigmaOS.
///
/// Provides the "Verified Today" daily report and known issues tracking,
/// aligned with Omarchy Linux release verification patterns.

/// VerifiedToday represents a daily quality report covering all system aspects.
#[derive(Debug, Clone)]
pub struct VerifiedToday {
    /// Whether the system booted successfully today
    booted: bool,
    /// Whether the system was installed today
    installed: bool,
    /// Whether the system was updated today
    updated: bool,
    /// Package management status
    packages: PackageStatus,
    /// Desktop session status
    desktop: DesktopStatus,
    /// Settings migration status
    settings: SettingsStatus,
    /// Hardware validation status
    hardware: HardwareStatus,
    /// Security defaults check
    secure_defaults: SecureDefaultsStatus,
}

impl Default for VerifiedToday {
    fn default() -> Self {
        Self {
            booted: false,
            installed: false,
            updated: false,
            packages: PackageStatus::default(),
            desktop: DesktopStatus::default(),
            settings: SettingsStatus::default(),
            hardware: HardwareStatus::default(),
            secure_defaults: SecureDefaultsStatus::default(),
        }
    }
}

impl VerifiedToday {
    /// Creates a new VerifiedToday report with all fields set to default.
    pub fn new() -> Self {
        Self::default()
    }

    /// Mark the system as booted successfully today.
    pub fn set_booted(&mut self) {
        self.booted = true;
    }

    /// Mark the system as installed today.
    pub fn set_installed(&mut self) {
        self.installed = true;
    }

    /// Mark the system as updated today.
    pub fn set_updated(&mut self) {
        self.updated = true;
    }

    /// Whether the system booted successfully today.
    pub fn is_booted(&self) -> bool {
        self.booted
    }

    /// Whether the system was installed today.
    pub fn is_installed(&self) -> bool {
        self.installed
    }

    /// Whether the system was updated today.
    pub fn is_updated(&self) -> bool {
        self.updated
    }

    /// Returns the package management status.
    pub fn packages(&self) -> &PackageStatus {
        &self.packages
    }

    /// Returns the desktop session status.
    pub fn desktop(&self) -> &DesktopStatus {
        &self.desktop
    }

    /// Returns the settings migration status.
    pub fn settings(&self) -> &SettingsStatus {
        &self.settings
    }

    /// Returns the hardware validation status.
    pub fn hardware(&self) -> &HardwareStatus {
        &self.hardware
    }

    /// Returns the secure defaults check status.
    pub fn secure_defaults(&self) -> &SecureDefaultsStatus {
        &self.secure_defaults
    }

    /// Generates a human-readable summary of the day's status.
    pub fn summary(&self) -> String {
        format!(
            "SigmaOS Verified Today:\n\
             Booted: {}\n\
             Installed: {}\n\
             Updated: {}\n\
             Packages: {}\n\
             Desktop: {}\n\
             Settings: {}\n\
             Hardware: {}\n\
             Secure Defaults: {}",
            self.booted,
            self.installed,
            self.updated,
            self.packages,
            self.desktop,
            self.settings,
            self.hardware,
            self.secure_defaults
        )
    }
}

/// PackageStatus represents the package management verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageStatus {
    /// All packages up to date with valid signatures
    UpToDate,
    /// Some packages need updating
    UpdatesAvailable,
    /// Package verification failed
    VerificationFailed,
    /// No package database available
    NoDatabase,
}

impl Default for PackageStatus {
    fn default() -> Self {
        Self::UpToDate
    }
}

impl std::fmt::Display for PackageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UpToDate => write!(f, "up-to-date"),
            Self::UpdatesAvailable => write!(f, "updates available"),
            Self::VerificationFailed => write!(f, "verification failed"),
            Self::NoDatabase => write!(f, "no database"),
        }
    }
}

/// DesktopStatus represents the desktop session verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopStatus {
    /// Desktop session running successfully
    Running,
    /// Desktop session failed to start
    Failed,
    /// No desktop session configured
    NotConfigured,
}

impl Default for DesktopStatus {
    fn default() -> Self {
        Self::NotConfigured
    }
}

impl std::fmt::Display for DesktopStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Running => write!(f, "running"),
            Self::Failed => write!(f, "failed"),
            Self::NotConfigured => write!(f, "not configured"),
        }
    }
}

/// SettingsStatus represents the settings migration verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsStatus {
    /// Settings migrated and validated
    Migrated,
    /// Settings migration needed
    MigrationNeeded,
    /// No settings found
    NoneFound,
}

impl Default for SettingsStatus {
    fn default() -> Self {
        Self::NoneFound
    }
}

impl std::fmt::Display for SettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Migrated => write!(f, "migrated"),
            Self::MigrationNeeded => write!(f, "migration needed"),
            Self::NoneFound => write!(f, "none found"),
        }
    }
}

/// HardwareStatus represents the hardware validation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareStatus {
    /// All hardware validated and functional
    Functional,
    /// Some hardware issues detected
    Degraded,
    /// Hardware validation failed
    Failed,
    /// No hardware detected
    NoneDetected,
}

impl Default for HardwareStatus {
    fn default() -> Self {
        Self::NoneDetected
    }
}

impl std::fmt::Display for HardwareStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Functional => write!(f, "functional"),
            Self::Degraded => write!(f, "degraded"),
            Self::Failed => write!(f, "failed"),
            Self::NoneDetected => write!(f, "none detected"),
        }
    }
}

/// SecureDefaultsStatus represents the secure defaults verification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecureDefaultsStatus {
    /// All secure defaults verified
    Verified,
    /// Some secure defaults not met
    Partial,
    /// Secure defaults verification failed
    Failed,
}

impl Default for SecureDefaultsStatus {
    fn default() -> Self {
        Self::Verified
    }
}

impl std::fmt::Display for SecureDefaultsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Verified => write!(f, "verified"),
            Self::Partial => write!(f, "partial"),
            Self::Failed => write!(f, "failed"),
        }
    }
}