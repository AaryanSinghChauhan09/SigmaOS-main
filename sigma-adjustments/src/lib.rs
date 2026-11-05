/// SigmaOS System Adjustments
/// Inspired by Linux Mint mintsystem
/// Manages CPU governors, sysctl memory tuning, and hardware optimization.

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

/// ConfigSchema represents a validation schema for system configuration.
#[derive(Debug, Clone)]
pub struct ConfigSchema {
    /// Schema version
    version: String,
    /// Required fields with their expected types
    required_fields: std::collections::BTreeMap<String, String>,
    /// Optional fields with their expected types
    optional_fields: std::collections::BTreeMap<String, String>,
}

impl Default for ConfigSchema {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            required_fields: std::collections::BTreeMap::new(),
            optional_fields: std::collections::BTreeMap::new(),
        }
    }
}

impl ConfigSchema {
    /// Creates a new ConfigSchema with the given version.
    pub fn new(version: &str) -> Self {
        Self {
            version: version.to_string(),
            required_fields: std::collections::BTreeMap::new(),
            optional_fields: std::collections::BTreeMap::new(),
        }
    }

    /// Adds a required field to the schema.
    pub fn add_required_field(&mut self, name: &str, expected_type: &str) {
        self.required_fields
            .insert(name.to_string(), expected_type.to_string());
    }

    /// Adds an optional field to the schema.
    pub fn add_optional_field(&mut self, name: &str, expected_type: &str) {
        self.optional_fields
            .insert(name.to_string(), expected_type.to_string());
    }

    /// Validates configuration data against the schema.
    ///
    /// Returns `Ok(())` if the config is valid, or `Err` with a description of the validation failure.
    pub fn validate(&self, config: &std::collections::BTreeMap<String, String>) -> Result<(), String> {
        // Check that all required fields are present
        for (field_name, _) in &self.required_fields {
            if !config.contains_key(field_name) {
                return Err(format!("Missing required field: {}", field_name));
            }
        }

        // Check that optional fields have correct types (if present)
        for (field_name, expected_type) in &self.optional_fields {
            if let Some(value) = config.get(field_name) {
                // Simple type checking: if expected_type is "u32", value should be parseable as u32
                if *expected_type == "u32" {
                    let _: u32 = value.parse().map_err(|_| {
                        format!("Field {} expected type u32, got {}", field_name, value)
                    })?;
                }
            }
        }

        Ok(())
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

    /// Sets the swappiness value (clamped to 0-100).
    pub fn set_swappiness(&mut self, swappiness: u32) {
        self.swappiness = swappiness.min(100);
    }

    /// Sets the VFS cache pressure value.
    pub fn set_vfs_cache_pressure(&mut self, pressure: u32) {
        self.vfs_cache_pressure = pressure.min(200);
    }

    /// Sets the governor mode.
    pub fn set_governor(&mut self, governor: GovernorMode) {
        self.governor = governor;
    }

    /// Validates the current adjustment settings against a schema.
    ///
    /// # Arguments
    ///
    /// * `schema` - The config schema to validate against
    /// * `config` - The configuration map to validate
    ///
    /// Returns `Ok(())` if valid, or `Err` with a description of the validation failure.
    pub fn validate_config(schema: &ConfigSchema, config: &std::collections::BTreeMap<String, String>) -> Result<(), String> {
        schema.validate(config)
    }

    /// Migrates configuration from an old version to the current version.
    ///
    /// This handles schema changes between versions, such as renaming fields,
    /// changing types, or adding/removing defaults.
    ///
    /// # Arguments
    ///
    /// * `old_version` - The previous configuration version
    /// * `old_config` - The configuration from the previous version
    ///
    /// Returns the migrated configuration suitable for the current schema.
    pub fn migrate_config(old_version: &str, old_config: &std::collections::BTreeMap<String, String>) -> std::collections::BTreeMap<String, String> {
        let mut new_config = std::collections::BTreeMap::new();

        match old_version {
            "0.1" => {
                // Migrate from v0.1 to v1.0
                // Example: rename old_field to new_field
                if let Some(val) = old_config.get("old_field") {
                    new_config.insert("new_field".to_string(), val.clone());
                }
                // Add defaults for new fields
                new_config.insert("vm.swappiness".to_string(), "10".to_string());
                new_config.insert("vm.vfs_cache_pressure".to_string(), "50".to_string());
            }
            "0.2" => {
                // Migrate from v0.2 to v1.0
                // Example: change format of existing fields
                if let Some(val) = old_config.get("vm.swappiness") {
                    // Ensure value is clamped to 0-100
                    let swappiness: u32 = val.parse().unwrap_or(10).min(100);
                    new_config.insert("vm.swappiness".to_string(), swappiness.to_string());
                }
                if let Some(val) = old_config.get("vm.vfs_cache_pressure") {
                    let pressure: u32 = val.parse().unwrap_or(50).min(200);
                    new_config.insert("vm.vfs_cache_pressure".to_string(), pressure.to_string());
                }
                // Add new fields with defaults
                new_config.insert("cpu.governor".to_string(), "schedutil".to_string());
            }
            _ => {
                // No migration needed, copy as-is
                for (k, v) in old_config {
                    new_config.insert(k.clone(), v.clone());
                }
            }
        }

        new_config
    }
}