/// SigmaOS Display Management
/// Inspired by Linux Mint and Omarchy Linux
/// Handles display configuration and output management.

/// DisplayMode represents the current display resolution and refresh rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    R640x480,
    R800x600,
    R1024x768,
    R1920x1080,
    R2560x1440,
    R3840x2160,
}

impl DisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R640x480 => "640x480",
            Self::R800x600 => "800x600",
            Self::R1024x768 => "1024x768",
            Self::R1920x1080 => "1920x1080",
            Self::R2560x1440 => "2560x1440",
            Self::R3840x2160 => "3840x2160",
        }
    }
}

/// LoginResult represents the result of a login attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoginResult {
    Success,
    Failed,
    Cancelled,
}

/// LoginFlow manages the user login sequence.
pub struct LoginFlow {
    /// Whether the user is logged in
    logged_in: bool,
    /// Number of failed login attempts
    failed_attempts: u32,
    /// Maximum allowed failed attempts before lockout
    max_attempts: u32,
    /// Current display mode
    mode: DisplayMode,
    /// Whether the display is initialized
    initialized: bool,
}

impl Default for LoginFlow {
    fn default() -> Self {
        Self {
            logged_in: false,
            failed_attempts: 0,
            max_attempts: 3,
            mode: DisplayMode::R1920x1080,
            initialized: false,
        }
    }
}

impl LoginFlow {
    /// Creates a new LoginFlow instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempts to authenticate with the given credentials.
    ///
    /// # Arguments
    ///
    /// * `username` - The username to authenticate
    /// * `password` - The password to authenticate
    ///
    /// Returns `LoginResult::Success` if authentication succeeds,
    /// `LoginResult::Failed` if credentials are invalid, or
    /// `LoginResult::Cancelled` if the login is cancelled.
    pub fn attempt_login(&mut self, username: &str, password: &str) -> LoginResult {
        // In a real implementation, this would verify credentials
        // against a user database or authentication service
        if username == "user" && password == "password" {
            self.logged_in = true;
            self.failed_attempts = 0;
            LoginResult::Success
        } else {
            self.failed_attempts += 1;
            if self.failed_attempts >= self.max_attempts {
                // Lockout after max attempts
                LoginResult::Failed
            } else {
                LoginResult::Failed
            }
        }
    }

    /// Whether the user is currently logged in.
    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    /// Returns the number of failed login attempts.
    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    /// Returns the maximum allowed failed attempts before lockout.
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Resets the failed attempt counter.
    pub fn reset_failed_attempts(&mut self) {
        self.failed_attempts = 0;
    }

    /// Sets the display mode.
    pub fn set_mode(&mut self, mode: DisplayMode) {
        self.mode = mode;
    }

    /// Returns the current display mode.
    pub fn mode(&self) -> DisplayMode {
        self.mode
    }

    /// Initializes the login flow.
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    /// Whether the login flow is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

/// SessionChooser manages selecting a desktop session persona.
pub struct SessionChooser {
    /// Available personas/sessions
    available_personas: Vec<String>,
    /// Currently selected persona index
    selected_index: usize,
    /// Whether a session has been selected
    selected: bool,
}

impl Default for SessionChooser {
    fn default() -> Self {
        Self {
            available_personas: vec![
                "Omarchy Linux Sovereign".to_string(),
                "Linux Mint Cinnamon Classic".to_string(),
                "Arch Linux Minimalist".to_string(),
            ],
            selected_index: 0,
            selected: false,
        }
    }
}

impl SessionChooser {
    /// Creates a new SessionChooser instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the list of available personas/sessions.
    pub fn available_personas(&self) -> &[String] {
        &self.available_personas
    }

    /// Returns the currently selected persona.
    pub fn selected_persona(&self) -> Option<&str> {
        self.available_personas.get(self.selected_index).map(|s| s.as_str())
    }

    /// Selects the next persona in the list.
    pub fn select_next(&mut self) {
        if self.available_personas.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1) % self.available_personas.len();
    }

    /// Selects the previous persona in the list.
    pub fn select_previous(&mut self) {
        if self.available_personas.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + self.available_personas.len() - 1) % self.available_personas.len();
    }

    /// Selects a persona by index.
    pub fn select_index(&mut self, index: usize) {
        if index < self.available_personas.len() {
            self.selected_index = index;
        }
    }

    /// Whether a session has been selected.
    pub fn is_selected(&self) -> bool {
        self.selected
    }

    /// Marks a session as selected.
    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }
}

/// DisplayProvider manages display configuration and output.
pub struct DisplayProvider {
    mode: DisplayMode,
    initialized: bool,
}

impl Default for DisplayProvider {
    fn default() -> Self {
        Self {
            mode: DisplayMode::R1920x1080,
            initialized: false,
        }
    }
}

impl DisplayProvider {
    pub fn new(mode: DisplayMode) -> Self {
        Self {
            mode,
            initialized: false,
        }
    }

    pub fn mode(&self) -> DisplayMode {
        self.mode
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        // Initialize display subsystem
        self.initialized = true;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: DisplayMode) -> Result<(), String> {
        self.mode = mode;
        Ok(())
    }
}