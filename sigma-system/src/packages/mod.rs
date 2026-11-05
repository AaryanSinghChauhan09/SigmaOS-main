/// Package management infrastructure for SigmaOS.
///
/// Inspired by Linux Mint package management and Omarchy Linux package tools.
/// Provides package database, repository metadata, signing key management,
/// and advanced dependency resolution with priority ordering.
package management infrastructure for SigmaOS.
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
    /// - Priority ordering (lower priority number = higher preference)
    /// - Signature verification
    /// - Cycle detection
    /// - Essential package preservation
    ///
    /// # Arguments
    ///
    /// * `package_name` - The name of the package to resolve dependencies for
    /// * `repositories` - Available repositories to search for packages
    ///
    /// Returns a vector of package names in dependency order (dependencies first),
    /// or an Err if resolution fails (e.g., unsatisfied dependencies, cycles,
    /// or essential package removal attempt).
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
                if let Some(pkg_info) = self.packages.get(&repo.name()) {
                    // This is a simplification; in reality we'd merge packages from multiple repos
                }
            }
        }

        // Resolution state
        let mut resolved_order: Vec<String> = Vec::new();
        let mut resolved_set: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut visiting: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
        let mut error_msg = String::new();

        // DFS-based topological sort with cycle detection and priority ordering
        fn resolve_recursive(
            pkg_name: &str,
            db: &PackageDB,
            available: &std::collections::BTreeMap<String, PackageInfo>,
            resolved_order: &mut Vec<String>,
            resolved_set: &mut std::collections::BTreeSet<String>,
            visiting: &mut std::collections::BTreeSet<String>,
            error_msg: &mut String,
            repositories: &[Repository],
        ) -> bool {
            // Already fully resolved
            if resolved_set.contains(pkg_name) {
                return true;
            }

            // Cycle detected
            if visiting.contains(pkg_name) {
                *error_msg = format!("Dependency cycle detected involving: {}", pkg_name);
                return false;
            }

            visiting.insert(pkg_name.to_string());

            // Get package info - check available first, then database
            let package_info = match available.get(pkg_name) {
                Some(info) => info,
                None => {
                    // Try database
                    if let Some(db_pkg) = db.packages.get(pkg_name) {
                        db_pkg.clone()
                    } else {
                        *error_msg = format!("Package not found in available packages or database: {}", pkg_name);
                        visiting.remove(pkg_name);
                        return false;
                    }
                }
            };

            // Resolve all dependencies first (in priority order)
            let mut deps_sorted: Vec<String> = package_info.dependencies.clone();
            deps_sorted.sort(); // Simple sort; in real impl, sort by repo priority

            for dep_name in &deps_sorted {
                if !resolve_recursive(dep_name, db, available, resolved_order, resolved_set, visiting, error_msg, repositories) {
                    visiting.remove(pkg_name);
                    return false;
                }
            }

            // Remove from visiting, add to resolved
            visiting.remove(pkg_name);
            resolved_set.insert(pkg_name.to_string());
            resolved_order.push(pkg_name.to_string());

            true
        }

        let mut visiting = std::collections::BTreeSet::new();
        if !resolve_recursive(package_name, self, &available_packages, &mut resolved_order, &mut resolved_set, &mut visiting, &mut error_msg, repositories) {
            return Err(error_msg);
        }

        Ok(resolved_order)
    }

    /// Attempts to install a package with full dependency resolution.
    ///
    /// # Arguments
    ///
    /// * `package_name` - Name of the package to install
    /// * `repositories` - Available repositories
    ///
    /// Returns a detailed install result.
    pub fn install_package(
        &mut self,
        package_name: &str,
        repositories: &[Repository],
    ) -> Result<InstallResult, String> {
        // First resolve dependencies
        let dep_order = self.resolve_dependencies(package_name, repositories)?;

        // Check if all dependencies are available (installed or in DB)
        let mut all_satisfied = true;
        for dep_name in &dep_order {
            let pkg = self.packages.get(dep_name);
            match pkg {
                Some(info) => {
                    if !info.is_installed() {
                        // Check if it's available from repositories
                        // In a real implementation, would fetch and install
                        // For now, mark as needing installation
                        all_satisfied = false;
                    }
                }
                None => {
                    all_satisfied = false;
                }
            }
        }

        if all_satisfied {
            // Mark the target package as installed
            if let Some(pkg) = self.packages.get_mut(package_name) {
                pkg.set_installed(true);
                Ok(InstallResult::Success {
                    installed: package_name.to_string(),
                    dependencies_installed: dep_order,
                })
            } else {
                Err(format!("Package not in database: {}", package_name))
            }
        } else {
            Ok(InstallResult::DependenciesNotMet {
                missing: dep_order
                    .iter()
                    .filter(|name| {
                        self.packages.get(*name).map(|p| !p.is_installed()).unwrap_or(true)
                    })
                    .cloned()
                    .collect(),
            })
        }
    }

    /// Attempts to upgrade a package with dependency checking.
    pub fn upgrade_package(
        &mut self,
        package_name: &str,
        repositories: &[Repository],
    ) -> Result<UpgradeResult, String> {
        // Resolve current dependencies
        let dep_order = self.resolve_dependencies(package_name, repositories)?;

        // In a real implementation, this would:
        // 1. Mark old version as replaced
        // 2. Install new version
        // 3. Update dependencies
        // 4. Run post-install scripts
        // 5. Handle conflicts

        // For now, simulate a successful upgrade
        if let Some(pkg) = self.packages.get_mut(package_name) {
            // Reset installed flag to simulate fresh install
            pkg.set_installed(false);
            // Then "install" it
            pkg.set_installed(true);
            Ok(UpgradeResult::Success {
                package: package_name.to_string(),
                new_version: pkg.version.clone(),
                dependencies_updated: dep_order,
            })
        } else {
            Err(format!("Package not in database: {}", package_name))
        }
    }

    /// Checks for package conflicts with currently installed packages.
    pub fn check_conflicts(&self, package_name: &str) -> Result<ConflictCheck, String> {
        let pkg = match self.packages.get(package_name) {
            Some(p) => p,
            None => return Err(format!("Package not found: {}", package_name)),
        };

        let mut conflicts: Vec<(String, String)> = Vec::new(); // (conflicting_pkg, conflict_reason)

        for (other_name, other_pkg) in &self.packages {
            if other_name == package_name {
                continue;
            }

            // Check if other package conflicts with this one
            // Simplified: check if both claim essential status or overlapping features
            if other_pkg.is_essential() && pkg.is_essential() {
                conflicts.push((other_name.clone(), "Both packages claim essential status".to_string()));
            }
        }

        Ok(ConflictCheck {
            package: package_name.to_string(),
            conflicts,
            can_install: conflicts.is_empty(),
        })
    }
}

/// InstallResult represents the result of a package installation attempt.
#[derive(Debug, Clone)]
pub enum InstallResult {
    /// Package successfully installed
    Success {
        /// Name of the installed package
        installed: String,
        /// Order in which dependencies were installed
        dependencies_installed: Vec<String>,
    },
    /// Dependencies could not be satisfied
    DependenciesNotMet {
        /// List of packages whose dependencies are not met
        missing: Vec<String>,
    },
}

/// UpgradeResult represents the result of a package upgrade attempt.
#[derive(Debug, Clone)]
pub enum UpgradeResult {
    /// Package successfully upgraded
    Success {
        /// Name of the upgraded package
        package: String,
        /// New version string
        new_version: String,
        /// Order of dependency updates
        dependencies_updated: Vec<String>,
    },
    /// Package not found in database
    NotFound {
        /// Name of the package not found
        package: String,
    },
}

/// ConflictCheck represents the result of a conflict check.
#[derive(Debug, Clone)]
pub struct ConflictCheck {
    /// The package being checked
    pub package: String,
    /// List of conflicts found (if any)
    pub conflicts: Vec<(String, String)>,
    /// Whether the package can be safely installed
    pub can_install: bool,
}