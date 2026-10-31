//! SigmaOS Persona Management
//! Inspired by Omarchy Linux and Linux Mint
//! Enables sovereign desktop personas and overlay environments.

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

/// PersonaManager manages desktop persona selection and switching.
pub struct PersonaManager {
    current: SystemPersona,
}

impl Default for PersonaManager {
    fn default() -> Self {
        Self {
            current: SystemPersona::SovereignOmarchy,
        }
    }
}

impl PersonaManager {
    pub fn new(initial: SystemPersona) -> Self {
        Self { current: initial }
    }

    pub fn current(&self) -> SystemPersona {
        self.current
    }

    pub fn switch_persona(&mut self, next: SystemPersona) {
        self.current = next;
    }
}