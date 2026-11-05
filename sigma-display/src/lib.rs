// SigmaOS Display Management
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

/// SystemPersona represents a desktop environment persona/sovereign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPersona {
    SovereignOmarchy,
    LinuxMintClassic,
    ArchMinimal,
}

impl SystemPersona {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SovereignOmarchy => "Omarchy Linux Sovereign",
            Self::LinuxMintClassic => "Linux Mint Cinnamon Classic",
            Self::ArchMinimal => "Arch Linux Minimalist",
        }
    }
}

/// Application represents a desktop application with launch capabilities.
#[derive(Debug, Clone)]
pub struct Application {
    /// Application name
    name: String,
    /// Command to execute
    command: String,
    /// Whether the application is favorited
    favorited: bool,
    /// Category (e.g., "Graphics", "Office", "System")
    category: String,
}

impl Application {
    /// Creates a new Application instance.
    pub fn new(name: &str, command: &str, category: &str) -> Self {
        Self {
            name: name.to_string(),
            command: command.to_string(),
            favorited: false,
            category: category.to_string(),
        }
    }

    /// Returns the application name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the launch command.
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Whether the application is favorited.
    pub fn is_favorited(&self) -> bool {
        self.favorited
    }

    /// Mark/Unmark as favorite.
    pub fn set_favorited(&mut self, favorited: bool) {
        self.favorited = favorited;
    }

    /// Returns the category.
    pub fn category(&self) -> &str {
        &self.category
    }
}

/// MainMenu represents the desktop main menu (Start menu), inspired by Linux Mint's menu.
#[derive(Debug, Clone)]
pub struct MainMenu {
    /// Available categories with their applications
    categories: std::collections::BTreeMap<String, Vec<Application>>,
    /// Currently selected category
    selected_category: String,
    /// Whether the menu is open
    open: bool,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            categories: std::collections::BTreeMap::new(),
            selected_category: "".to_string(),
            open: false,
        }
    }
}

impl MainMenu {
    /// Creates a new MainMenu instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an application to a category.
    pub fn add_application(&mut self, category: &str, application: Application) {
        self.categories
            .entry(category.to_string())
            .or_insert_with(Vec::new)
            .push(application);
    }

    /// Get applications in a specific category.
    pub fn get_applications(&self, category: &str) -> Vec<&Application> {
        self.categories
            .get(category)
            .map(|apps| apps.iter().collect())
            .unwrap_or_default()
    }

    /// Set the selected category.
    pub fn set_selected_category(&mut self, category: &str) {
        self.selected_category = category.to_string();
    }

    /// Get the selected category.
    pub fn selected_category(&self) -> &str {
        &self.selected_category
    }

    /// Open the menu.
    pub fn open_menu(&mut self) {
        self.open = true;
    }

    /// Close the menu.
    pub fn close_menu(&mut self) {
        self.open = false;
    }

    /// Whether the menu is open.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Launch the application at the given index in the selected category.
    pub fn launch_application(&mut self, index: usize) -> Result<String, String> {
        if !self.open {
            return Err("Menu is not open".to_string());
        }

        let apps = match self.categories.get(&self.selected_category) {
            Some(apps) => apps,
            None => return Err("No applications in selected category".to_string()),
        };

        if index >= apps.len() {
            return Err("Application index out of range".to_string());
        }

        let app = &apps[index];
        // In a real implementation, this would execute the command
        // For now, return the command string
        Ok(app.command.clone())
    }

    /// Toggle the menu open/closed.
    pub fn toggle_menu(&mut self) {
        if self.open {
            self.close_menu();
        } else {
            self.open_menu();
        }
    }
}

/// LoginFlow represents the user authentication and session management flow.
///
/// Inspired by Linux Mint's login screen and Omarchy Linux's session management.
/// Supports authentication, lockout protection, and session persistence.
pub struct LoginFlow {
    /// Whether the user is authenticated
    authenticated: bool,
    /// Current user session persona
    session: SystemPersona,
    /// Failed login attempt counter
    failed_attempts: u32,
    /// Maximum allowed failed attempts before lockout
    max_attempts: u32,
    /// Whether the session is locked
    locked: bool,
    /// Lockout timeout in seconds
    lockout_timeout: u64,
    /// Last failed attempt timestamp
    last_failed_at: Option<u64>,
}

impl Default for LoginFlow {
    fn default() -> Self {
        Self {
            authenticated: false,
            session: SystemPersona::SovereignOmarchy,
            failed_attempts: 0,
            max_attempts: 3,
            locked: false,
            lockout_timeout: 300,
            last_failed_at: None,
        }
    }
}

impl LoginFlow {
    /// Creates a new LoginFlow instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Authenticate the user with given credentials.
    ///
    /// Returns `true` if authentication succeeds, `false` otherwise.
    /// Implements lockout protection after max_attempts failed tries.
    pub fn authenticate(&mut self, username: &str, password: &str) -> bool {
        // Check if session is locked
        if self.locked {
            // Check if lockout timeout has expired
            if let Some(last) = self.last_failed_at {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                if now - last > self.lockout_timeout {
                    self.locked = false;
                    self.failed_attempts = 0;
                }
            }
            return false;
        }

        // Simple authentication check (in real system, verify against user database)
        if username == "user" && password == "password" {
            self.authenticated = true;
            self.session = SystemPersona::SovereignOmarchy;
            self.failed_attempts = 0;
            self.locked = false;
            self.last_failed_at = None;
            true
        } else {
            self.failed_attempts += 1;
            self.last_failed_at = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            );

            // Lock out after max_attempts
            if self.failed_attempts >= self.max_attempts {
                self.locked = true;
            }
            false
        }
    }

    /// Whether the user is authenticated.
    pub fn is_authenticated(&self) -> bool {
        self.authenticated
    }

    /// Get the current session persona.
    pub fn session(&self) -> SystemPersona {
        self.session
    }

    /// Reset the login flow (e.g., after reboot or session reset).
    pub fn reset(&mut self) {
        self.authenticated = false;
        self.session = SystemPersona::SovereignOmarchy;
        self.failed_attempts = 0;
        self.locked = false;
        self.last_failed_at = None;
    }

    /// Whether the session is currently locked due to too many failed attempts.
    pub fn is_locked(&self) -> bool {
        self.locked
    }

    /// Get the number of failed login attempts.
    pub fn failed_attempts(&self) -> u32 {
        self.failed_attempts
    }

    /// Get the maximum allowed failed attempts before lockout.
    pub fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    /// Get the lockout timeout in seconds.
    pub fn lockout_timeout(&self) -> u64 {
        self.lockout_timeout
    }
}

/// DesktopLauncher launches applications and manages the desktop session.
///
/// Inspired by Linux Mint's desktop launchers and Omarchy Linux's application framework.
pub struct DesktopLauncher {
    /// Whether the user is logged in
    logged_in: bool,
    /// Currently selected session persona
    session: SystemPersona,
    /// Main menu instance
    main_menu: MainMenu,
    /// Current display mode
    display_mode: DisplayMode,
    /// Whether the desktop is initialized
    initialized: bool,
    /// Login flow for authentication
    login_flow: LoginFlow,
}

impl Default for DesktopLauncher {
    fn default() -> Self {
        Self {
            logged_in: false,
            session: SystemPersona::SovereignOmarchy,
            main_menu: MainMenu::new(),
            display_mode: DisplayMode::R1920x1080,
            initialized: false,
            login_flow: LoginFlow::new(),
        }
    }
}

impl DesktopLauncher {
    /// Creates a new DesktopLauncher instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Log in the user with given credentials.
    pub fn login(&mut self, username: &str, password: &str) -> bool {
        let result = self.login_flow.authenticate(username, password);
        if result {
            self.logged_in = true;
        }
        result
    }

    /// Whether the user is logged in.
    pub fn is_logged_in(&self) -> bool {
        self.logged_in
    }

    /// Set the session persona.
    pub fn set_session(&mut self, persona: SystemPersona) {
        self.session = persona;
    }

    /// Get the current session persona.
    pub fn session(&self) -> SystemPersona {
        self.session
    }

    /// Add an application to the main menu.
    pub fn add_menu_application(&mut self, category: &str, app: Application) {
        self.main_menu.add_application(category, app);
    }

    /// Open the main menu.
    pub fn open_main_menu(&mut self) {
        self.main_menu.open_menu();
    }

    /// Close the main menu.
    pub fn close_main_menu(&mut self) {
        self.main_menu.close_menu();
    }

    /// Toggle the main menu.
    pub fn toggle_main_menu(&mut self) {
        self.main_menu.toggle_menu();
    }

    /// Get applications in a category from the main menu.
    pub fn get_menu_applications(&self, category: &str) -> Vec<&Application> {
        self.main_menu.get_applications(category)
    }

    /// Launch an application from the main menu by category and index.
    pub fn launch_menu_application(&mut self, _category: &str, index: usize) -> Result<String, String> {
        self.main_menu.launch_application(index)
    }

    /// Set the display mode.
    pub fn set_display_mode(&mut self, mode: DisplayMode) {
        self.display_mode = mode;
    }

    /// Get the current display mode.
    pub fn display_mode(&self) -> DisplayMode {
        self.display_mode
    }

    /// Initialize the desktop launcher.
    pub fn initialize(&mut self) -> Result<(), String> {
        self.initialized = true;
        Ok(())
    }

    /// Whether the desktop launcher is initialized.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}