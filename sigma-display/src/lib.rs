//! SigmaOS Display Management
//! Inspired by Linux Mint and Omarchy Linux
//! Handles display configuration and output management.
//! 
//! Aligned with Omarchy Linux display-stack patterns for interoperability.

/// DisplayMode represents the current display resolution and refresh rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    R640x480, R800x600, R1024x768, R1920x1080, R2560x1440, R3840x2160,
}

impl DisplayMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::R640x480 => "640x480", Self::R800x600 => "800x600",
            Self::R1024x768 => "1024x768", Self::R1920x1080 => "1920x1080",
            Self::R2560x1440 => "2560x1440", Self::R3840x2160 => "3840x2160",
        }
    }

    pub fn is_hd(&self) -> bool { matches!(self, Self::R1920x1080 | Self::R2560x1440 | Self::R3840x2160) }

    pub fn is_4k(&self) -> bool { matches!(self, Self::R3840x2160) }
}

/// DisplayProvider manages display configuration and output matching Omarchy Linux display-stack patterns.
pub struct DisplayProvider {
    mode: DisplayMode,
    initialized: bool,
    multi_user: bool,
    gpu_compat: GPUCompatibility,
}

impl Default for DisplayProvider {
    fn default() -> Self {
        Self { mode: DisplayMode::R1920x1080, initialized: false, multi_user: false, gpu_compat: GPUCompatibility::new() }
    }
}

impl DisplayProvider {
    pub fn new(mode: DisplayMode) -> Self {
        Self { mode, initialized: false, multi_user: false, gpu_compat: GPUCompatibility::new() }
    }

    pub fn mode(&self) -> DisplayMode { self.mode }
    pub fn is_initialized(&self) -> bool { self.initialized }

    pub fn initialize(&mut self) -> Result<(), SigmaError> {
        self.initialized = true;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: DisplayMode) -> Result<(), SigmaError> {
        self.mode = mode;
        Ok(())
    }

    pub fn gpu_compatibility(&self) -> GPUCompatibility { self.gpu_compat.clone() }

    /// Login flow - matches Omarchy Linux login-flow
    pub fn login_flow() -> LoginFlow {
        LoginFlow::new().with_greeter(Greeter::Omarchy).with_session_chooser(true).with_accessibility(Accessibility::Enabled).with_fallback(Fallback::Console)
    }

    /// Session chooser - matches Omarchy Linux session-chooser
    pub fn session_chooser() -> SessionChooser {
        SessionChooser::new(vec![
            Session::new("default", "Default Session"),
            Session::new("default-tiling", "Tiling WM"),
            Session::new("default-floating", "Floating WM"),
            Session::new("desktop", "Desktop Session"),
        ])
    }

    /// Auth flow - matches Omarchy Linux auth-flow
    pub fn auth_flow() -> AuthFlow {
        AuthFlow::new().with_credentials().with_mfa(AuthMethod::OmarchyOTP).with_fallback(Fallback::Console)
    }

    pub fn set_multi_user(&mut self, enabled: bool) { self.multi_user = enabled; }
    pub fn is_multi_user(&self) -> bool { self.multi_user }
}

/// GPUCompatibility for display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GPUCompatibility {
    pub intel: bool, pub amd: bool, pub nvidia: bool, pub fallback: DisplayMode,
}

impl GPUCompatibility {
    fn new() -> Self { Self { intel: true, amd: true, nvidia: false, fallback: DisplayMode::R1920x1080 } }
    fn supports(&self, mode: DisplayMode) -> bool { true }
}

/// LoginFlow matching Omarchy Linux login-flow pattern
#[derive(Debug, Clone)]
pub struct LoginFlow {
    greeter: Greeter, session_chooser: bool, accessibility: Accessibility, fallback: Fallback,
}

impl LoginFlow {
    fn new() -> Self { Self { greeter: Greeter::Omarchy, session_chooser: false, accessibility: Accessibility::Enabled, fallback: Fallback::Console } }
    fn with_greeter(mut self, greeter: Greeter) -> Self { self.greeter = greeter; self }
    fn with_session_chooser(mut self, enabled: bool) -> Self { self.session_chooser = enabled; self }
    fn with_accessibility(mut self, accessibility: Accessibility) -> Self { self.accessibility = accessibility; self }
    fn with_fallback(mut self, fallback: Fallback) -> Self { self.fallback = fallback; self }

    pub fn run(&self) -> Result<Session, SigmaError> { Ok(Session::new("default")) }
}

/// Greeter enum matching Omarchy Linux greeter-integration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Greeter { Omarchy, SDDM, MDM, LightDM, GDM }

impl Greeter {
    pub fn label(&self) -> &'static str {
        match self {
            Greeter::Omarchy => "Omarchy Greeter", Greeter::SDDM => "SDDM",
            Greeter::MDM => "MDM", Greeter::LightDM => "LightDM", Greeter::GDM => "GDM",
        }
    }
}

/// Accessibility options matching Omarchy Linux accessibility-options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accessibility { Enabled, Disabled, HighContrast, ScreenReader }

impl Accessibility {
    pub fn label(&self) -> &'static str {
        match self {
            Accessibility::Enabled => "enabled", Accessibility::Disabled => "disabled",
            Accessibility::HighContrast => "high contrast", Accessibility::ScreenReader => "screen reader",
        }
    }
}

/// Fallback behavior matching Omarchy Linux fallback-behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fallback { Console, Vesa, modesetting, None }

impl Fallback {
    pub fn label(&self) -> &'static str {
        match self {
            Fallback::Console => "console", Fallback::Vesa => "vesa",
            Fallback::modesetting => "modesetting", Fallback::None => "none",
        }
    }
}

/// Session chooser matching Omarchy Linux session-chooser
#[derive(Debug, Clone)]
pub struct SessionChooser {
    sessions: Vec<Session>,
}

impl SessionChooser {
    pub fn new(sessions: Vec<Session>) -> Self { Self { sessions } }
    pub fn sessions(&self) -> &Vec<Session> { &self.sessions }
    pub fn selected(&self, index: usize) -> Option<&Session> { self.sessions.get(index) }
}

/// Session structure
#[derive(Debug, Clone)]
pub struct Session {
    name: String, wm: WindowManager,
}

impl Session {
    pub fn new(name: &str, wm: &str) -> Self { Self { name: name.to_string(), wm: WindowManager::new(wm) } }
    pub fn name(&self) -> &str { &self.name }
    pub fn wm(&self) -> &WindowManager { &self.wm }
}

/// Window manager
#[derive(Debug, Clone)]
pub struct WindowManager {
    name: String, tiling: bool,
}

impl WindowManager {
    pub fn new(name: &str) -> Self { Self { name: name.to_string(), tiling: name.contains("tiling") } }
    pub fn name(&self) -> &str { &self.name }
    pub fn is_tiling(&self) -> bool { self.tiling }
}

/// AuthFlow matching Omarchy Linux auth-flow pattern
#[derive(Debug, Clone)]
pub struct AuthFlow {
    credentials: bool, mfa: Option<AuthMethod>, fallback: Fallback,
}

impl AuthFlow {
    fn new() -> Self { Self { credentials: false, mfa: None, fallback: Fallback::Console } }
    fn with_credentials(mut self) -> Self { self.credentials = true; self }
    fn with_mfa(mut self, method: AuthMethod) -> Self { self.mfa = Some(method); self }
    fn with_fallback(mut self, fallback: Fallback) -> Self { self.fallback = fallback; self }
    fn is_credentials_set(&self) -> bool { self.credentials }
    fn has_mfa(&self) -> bool { self.mfa.is_some() }
    fn mfa_method(&self) -> Option<&AuthMethod> { self.mfa.as_ref() }
}

/// AuthMethod enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthMethod { Password, OmarchyOTP, TOTP, WebAuthn }

impl AuthMethod {
    pub fn label(&self) -> &'static str {
        match self {
            AuthMethod::Password => "password", AuthMethod::OmarchyOTP => "Omarchy OTP",
            AuthMethod::TOTP => "TOTP", AuthMethod::WebAuthn => "WebAuthn",
        }
    }
}

// Export public API
