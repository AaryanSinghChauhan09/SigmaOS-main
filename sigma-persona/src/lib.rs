/// SigmaOS Persona Management
/// Inspired by Omarchy Linux and Linux Mint
/// Enables sovereign desktop personas and overlay environments.

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

/// PersonaSave represents a saved persona/session state.
#[derive(Debug, Clone)]
pub struct PersonaSave {
    /// The persona/session label
    label: String,
    /// The persona type
    persona: SystemPersona,
    /// Timestamp when the save was created
    timestamp: u64,
    /// Whether the save is active
    active: bool,
}

impl Default for PersonaSave {
    fn default() -> Self {
        Self {
            label: "Unnamed Session".to_string(),
            persona: SystemPersona::SovereignOmarchy,
            timestamp: 0,
            active: false,
        }
    }
}

impl PersonaSave {
    /// Creates a new PersonaSave instance.
    pub fn new(label: &str, persona: SystemPersona, timestamp: u64) -> Self {
        Self {
            label: label.to_string(),
            persona,
            timestamp,
            active: true,
        }
    }

    /// Returns the save label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the persona type.
    pub fn persona(&self) -> SystemPersona {
        self.persona
    }

    /// Returns the timestamp.
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    /// Whether the save is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Marks the save as active or inactive.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

/// PersonaManager manages desktop persona selection and switching.
pub struct PersonaManager {
    current: SystemPersona,
    /// Saved persona/session states
    saves: std::collections::BTreeMap<String, PersonaSave>,
    /// Current save slot name
    current_save: Option<String>,
}

impl Default for PersonaManager {
    fn default() -> Self {
        Self {
            current: SystemPersona::SovereignOmarchy,
            saves: std::collections::BTreeMap::new(),
            current_save: None,
        }
    }
}

impl PersonaManager {
    /// Creates a new PersonaManager instance.
    pub fn new(initial: SystemPersona) -> Self {
        Self {
            current: initial,
            saves: std::collections::BTreeMap::new(),
            current_save: None,
        }
    }

    /// Returns the current persona.
    pub fn current(&self) -> SystemPersona {
        self.current
    }

    /// Switches to the next persona.
    pub fn switch_persona(&mut self, next: SystemPersona) {
        self.current = next;
        // Auto-save the persona switch
        self.save_current();
    }

    /// Saves the current persona state.
    pub fn save_current(&mut self) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let save = PersonaSave::new(&format!("session_{}", timestamp), self.current, timestamp);
        self.saves.insert(format!("save_{}", timestamp), save);
        self.current_save = Some(format!("save_{}", timestamp));
    }

    /// Restores a saved persona/session.
    ///
    /// # Arguments
    ///
    /// * `save_name` - The name of the save to restore
    ///
    /// Returns `Ok(())` if the save was restored, or `Err` if the save does not exist.
    pub fn restore_save(&mut self, save_name: &str) -> Result<(), String> {
        if let Some(save) = self.saves.get(save_name) {
            self.current = save.persona();
            self.current_save = Some(save_name.to_string());
            Ok(())
        } else {
            Err(format!("Save not found: {}", save_name))
        }
    }

    /// Returns the list of saved persona/session states.
    pub fn saves(&self) -> &std::collections::BTreeMap<String, PersonaSave> {
        &self.saves
    }

    /// Returns the current save slot name, if any.
    pub fn current_save(&self) -> Option<&str> {
        self.current_save.as_deref()
    }

    /// Deletes a saved persona/session.
    ///
    /// # Arguments
    ///
    /// * `save_name` - The name of the save to delete
    ///
    /// Returns `Ok(())` if the save was deleted, or `Err` if the save does not exist.
    pub fn delete_save(&mut self, save_name: &str) -> Result<(), String> {
        if self.saves.remove(save_name).is_some() {
            Ok(())
        } else {
            Err(format!("Save not found: {}", save_name))
        }
    }

    /// Whether a save with the given name exists.
    pub fn save_exists(&self, save_name: &str) -> bool {
        self.saves.contains_key(save_name)
    }
}