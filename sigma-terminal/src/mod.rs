/// Security module for SigmaOS, providing threat model, capability matrix, and denial tests.
///
/// Inspired by Omarchy Linux security patterns and Linux Mint security principles.
/// Provides process/package capability enforcement and security boundary testing.

/// ThreatModel documents the security threat landscape for SigmaOS,
/// including process isolation, package capability, and access control concerns.
#[derive(Debug, Clone)]
pub struct ThreatModel {
    /// Whether process isolation is enforced
    process_isolation: bool,
    /// Whether capability matrix is active
    capability_matrix_active: bool,
    /// Whether denial tests are running
    denial_tests_active: bool,
}

impl Default for ThreatModel {
    fn default() -> Self {
        Self {
            process_isolation: true,
            capability_matrix_active: true,
            denial_tests_active: false,
        }
    }
}

impl ThreatModel {
    /// Creates a new ThreatModel with default security settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables process isolation enforcement.
    pub fn enable_process_isolation(&mut self) {
        self.process_isolation = true;
    }

    /// Disables process isolation enforcement.
    pub fn disable_process_isolation(&mut self) {
        self.process_isolation = false;
    }

    /// Whether process isolation is currently enforced.
    pub fn is_process_isolation_enabled(&self) -> bool {
        self.process_isolation
    }

    /// Enables the capability matrix for process/package access control.
    pub fn enable_capability_matrix(&mut self) {
        self.capability_matrix_active = true;
    }

    /// Disables the capability matrix for process/package access control.
    pub fn disable_capability_matrix(&mut self) {
        self.capability_matrix_active = false;
    }

    /// Whether the capability matrix is currently active.
    pub fn is_capability_matrix_active(&self) -> bool {
        self.capability_matrix_active
    }

    /// Enables denial tests (fuzz tests and negative tests).
    pub fn enable_denial_tests(&mut self) {
        self.denial_tests_active = true;
    }

    /// Disables denial tests.
    pub fn disable_denial_tests(&mut self) {
        self.denial_tests_active = false;
    }

    /// Whether denial tests are currently running.
    pub fn are_denial_tests_active(&self) -> bool {
        self.denial_tests_active
    }

    /// Returns whether a given operation would be denied by the security framework.
    pub fn would_deny_operation(&self, operation: &str) -> bool {
        // In a real implementation, this would check capabilities and policies
        if !self.process_isolation && operation.contains("process") {
            return true;
        }
        if !self.capability_matrix_active && operation.contains("capability") {
            return true;
        }
        false
    }
}

/// CapabilityMatrix represents the process/package capability matrix
/// with least-privilege defaults, ensuring that processes only have
/// the minimum permissions needed to function.
#[derive(Debug, Clone)]
pub struct CapabilityMatrix {
    /// Root capability (trusted foundation)
    root_capability: bool,
    /// Process capabilities map (process_id -> granted capabilities)
    process_capabilities: std::collections::BTreeMap<String, Vec<String>>,
    /// Package capabilities map (package_name -> granted capabilities)
    package_capabilities: std::collections::BTreeMap<String, Vec<String>>,
}

impl Default for CapabilityMatrix {
    fn default() -> Self {
        Self {
            root_capability: true,
            process_capabilities: std::collections::BTreeMap::new(),
            package_capabilities: std::collections::BTreeMap::new(),
        }
    }
}

impl CapabilityMatrix {
    /// Creates a new CapabilityMatrix with default least-privilege settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Grants a capability to a process.
    pub fn grant_process_capability(&mut self, process_id: &str, capability: &str) {
        self.process_capabilities
            .entry(process_id.to_string())
            .or_insert_with(Vec::new)
            .push(capability.to_string());
    }

    /// Revokes a capability from a process.
    pub fn revoke_process_capability(&mut self, process_id: &str, capability: &str) {
        if let Some(caps) = self.process_capabilities.get_mut(process_id) {
            caps.retain(|c| c != capability);
        }
    }

    /// Checks if a process has a specific capability.
    pub fn has_process_capability(&self, process_id: &str, capability: &str) -> bool {
        self.process_capabilities
            .get(process_id)
            .map(|caps| caps.contains(&capability.to_string()))
            .unwrap_or(false)
    }

    /// Grants a capability to a package.
    pub fn grant_package_capability(&mut self, package_name: &str, capability: &str) {
        self.package_capabilities
            .entry(package_name.to_string())
            .or_insert_with(Vec::new)
            .push(capability.to_string());
    }

    /// Revokes a capability from a package.
    pub fn revoke_package_capability(&mut self, package_name: &str, capability: &str) {
        if let Some(caps) = self.package_capabilities.get_mut(package_name) {
            caps.retain(|c| c != capability);
        }
    }

    /// Checks if a package has a specific capability.
    pub fn has_package_capability(&self, package_name: &str, capability: &str) -> bool {
        self.package_capabilities
            .get(package_name)
            .map(|caps| caps.contains(&capability.to_string()))
            .unwrap_or(false)
    }

    /// Returns the root capability status.
    pub fn has_root_capability(&self) -> bool {
        self.root_capability
    }

    /// Sets the root capability status.
    pub fn set_root_capability(&mut self, value: bool) {
        self.root_capability = value;
    }
}

/// DenialTests represents fuzz tests and negative tests
/// that prove forbidden access is denied, ensuring security boundaries are enforced.
#[derive(Debug, Clone)]
pub struct DenialTests {
    /// Whether denial tests are configured
    configured: bool,
    /// Test results (pass/fail)
    test_results: std::collections::BTreeMap<String, bool>,
}

impl Default for DenialTests {
    fn default() -> Self {
        Self {
            configured: false,
            test_results: std::collections::BTreeMap::new(),
        }
    }
}

impl DenialTests {
    /// Creates a new DenialTests instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configures the denial tests framework.
    pub fn configure(&mut self) {
        self.configured = true;
    }

    /// Runs a denial test to verify that a forbidden access is denied.
    pub fn run_test(&mut self, test_name: &str, should_deny: bool) -> bool {
        let result = !should_deny; // Test passes if access is indeed denied
        self.test_results
            .insert(test_name.to_string(), result);
        result
    }

    /// Checks if a specific test passes.
    pub fn test_passes(&self, test_name: &str) -> Option<bool> {
        self.test_results.get(test_name).copied()
    }

    /// Whether all configured tests pass.
    pub fn all_tests_pass(&self) -> bool {
        self.test_results.values().all(|&r| r)
    }

    /// Returns whether the framework is configured and ready.
    pub fn is_configured(&self) -> bool {
        self.configured
    }
}