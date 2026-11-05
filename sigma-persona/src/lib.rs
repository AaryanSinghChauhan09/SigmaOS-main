//! SigmaOS Persona Management
//! Inspired by Omarchy Linux and Linux Mint
//! Enables sovereign desktop personas and overlay environments.
//! 
//! Aligned with Omarchy Linux session-management patterns for interoperability.

/// SystemPersona represents a desktop environment persona/sovereign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemPersona {
    SovereignOmarchy, LinuxMintClassic, ArchMinimal,
}

impl SystemPersona {
    pub fn label(&self) -> &'static str {
        match self {
            Self::SovereignOmarchy => "Omarchy Linux Sovereign",
            Self::LinuxMintClassic => "Linux Mint Cinnamon Classic",
            Self::ArchMinimal => "Arch Linux Minimalist",
        }
    }

    pub fn is_omarchy(&self) -> bool { matches!(self, SystemPersona::SovereignOmarchy) }

    pub fn is_minimal(&self) -> bool { matches!(self, SystemPersona::ArchMinimal) }
}

/// PersonaManager manages desktop persona selection and switching matching Omarchy Linux session-management patterns.
pub struct PersonaManager {
    current: SystemPersona, session_save: SessionSaveRestore, desktop_session: DesktopSession,
    multi_user: bool, login_lifecycle: LoginLifecycle,
}

impl Default for PersonaManager {
    fn default() -> Self {
        Self { current: SystemPersona::SovereignOmarchy, session_save: SessionSaveRestore::default(),
            desktop_session: DesktopSession::default(), multi_user: false, login_lifecycle: LoginLifecycle::default() }
    }
}

impl PersonaManager {
    pub fn new(initial: SystemPersona) -> Self {
        let mut mgr = Self::default(); mgr.current = initial; mgr
    }

    pub fn current(&self) -> SystemPersona { self.current }

    pub fn switch_persona(&mut self, next: SystemPersona) -> Result<SigmaError, SigmaError> {
        self.session_save.save(&self.current)?;
        self.current = next;
        self.session_save.restore(&self.current)?;
        Ok(())
    }

    pub fn session_save_restore(&self) -> &SessionSaveRestore { &self.session_save }
    pub fn desktop_session(&self) -> &DesktopSession { &self.desktop_session }

    pub fn set_multi_user(&mut self, enabled: bool) { self.multi_user = enabled; }
    pub fn is_multi_user(&self) -> bool { self.multi_user }

    pub fn login_lifecycle(&self) -> &LoginLifecycle { &self.login_lifecycle }

    /// Suspend/resume flow - matches Omarchy Linux suspend-resume pattern
    pub fn suspend_resume(&self) -> SuspendResume {
        SuspendResume::new().with_auto_suspend(true).with_auto_resume(true).with_lock_on_suspend(true).with_unlock_on_resume(true)
    }

    /// Lock/unlock flow
    pub fn lock_unlock(&self) -> LockUnlockFlow { LockUnlockFlow::new() }
}

/// SessionSaveRestore matching Omarchy Linux session-save-restore pattern
#[derive(Debug, Clone, Default)]
pub struct SessionSaveRestore {
    pub save_on_suspend: bool, pub restore_on_resume: bool, pub save_state: Vec<String>,
}

impl SessionSaveRestore {
    pub fn save(&self, persona: &SystemPersona) -> Result<(), SigmaError> { Ok(()) }
    pub fn restore(&self, persona: &SystemPersona) -> Result<(), SigmaError> { Ok(()) }
}

/// DesktopSession matching Omarchy Linux desktop-session pattern
#[derive(Debug, Clone, Default)]
pub struct DesktopSession {
    session_manager: String, compositor: String, panel: String, launcher: String,
}

impl DesktopSession {
    pub fn new() -> Self {
        Self { session_manager: "omarchy-session".into(), compositor: "omarchy-compositor".into(),
            panel: "omarchy-panel".into(), launcher: "omarchy-launcher".into() }
    }
    pub fn session_manager(&self) -> &str { &self.session_manager }
    pub fn compositor(&self) -> &str { &self.compositor }
    pub fn panel(&self) -> &str { &self.panel }
    pub fn launcher(&self) -> &str { &self.launcher }
}

/// LoginLifecycle matching Omarchy Linux login-lifecycle pattern
#[derive(Debug, Clone, Default)]
pub struct LoginLifecycle {
    authenticated: bool, session_saved: bool, desktop_configured: bool,
}

impl LoginLifecycle {
    pub fn new() -> Self { Self { authenticated: false, session_saved: false, desktop_configured: false } }
    pub fn with_authenticate(mut self) -> Self { self.authenticated = true; self }
    pub fn with_session_save(mut self) -> Self { self.session_saved = true; self }
    pub fn with_desktop_setup(mut self) -> Self { self.desktop_configured = true; self }
}

/// SuspendResume matching Omarchy Linux suspend-resume pattern
#[derive(Debug, Clone, Default)]
pub struct SuspendResume {
    auto_suspend: bool, auto_resume: bool, lock_on_suspend: bool, unlock_on_resume: bool,
}

impl SuspendResume {
    fn new() -> Self { Self { auto_suspend: false, auto_resume: false, lock_on_suspend: false, unlock_on_resume: false } }
    fn with_auto_suspend(mut self, enabled: bool) -> Self { self.auto_suspend = enabled; self }
    fn with_auto_resume(mut self, enabled: bool) -> Self { self.auto_resume = enabled; self }
    fn with_lock_on_suspend(mut self, enabled: bool) -> Self { self.lock_on_suspend = enabled; self }
    fn with_unlock_on_resume(mut self, enabled: bool) -> Self { self.unlock_on_resume = enabled; self }
}

/// LockUnlockFlow matching Omarchy Linux lock/unlock flow
#[derive(Debug, Clone, Default)]
pub struct LockUnlockFlow {
    locked: bool,
}

impl LockUnlockFlow {
    fn new() -> Self { Self { locked: false } }
    fn lock(&mut self) { self.locked = true; }
    fn unlock(&mut self) { self.locked = false; }
    fn is_locked(&self) -> bool { self.locked }
}

// Export public API
