// SigmaOS Terminal
/// Inspired by Linux Mint's terminal and Omarchy Linux console
/// Provides terminal/shell interaction capabilities.

/// TerminalMode represents the current terminal operating mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalMode {
    Normal,
    Root,
    Privileged,
}

impl TerminalMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Root => "root",
            Self::Privileged => "privileged",
        }
    }
}

/// Terminal represents a terminal/shell interface.
///
/// Provides basic terminal input/output capabilities and mode management.
/// Inspired by Linux Mint's terminal and Omarchy Linux console.
pub struct Terminal {
    /// Current terminal mode
    mode: TerminalMode,
    /// Whether the terminal is active
    active: bool,
    /// Buffer for terminal input
    input_buffer: String,
    /// Buffer for terminal output
    output_buffer: String,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            mode: TerminalMode::Normal,
            active: false,
            input_buffer: String::new(),
            output_buffer: String::new(),
        }
    }
}

impl Terminal {
    /// Creates a new Terminal instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the terminal mode.
    pub fn set_mode(&mut self, mode: TerminalMode) {
        self.mode = mode;
    }

    /// Gets the current terminal mode.
    pub fn mode(&self) -> TerminalMode {
        self.mode
    }

    /// Whether the terminal is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activates the terminal.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivates the terminal.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Adds output to the terminal buffer.
    pub fn add_output(&mut self, text: &str) {
        self.output_buffer.push_str(text);
    }

    /// Gets the terminal output buffer.
    pub fn get_output(&self) -> &str {
        &self.output_buffer
    }

    /// Clears the terminal output buffer.
    pub fn clear_output(&mut self) {
        self.output_buffer.clear();
    }

    /// Appends input to the input buffer.
    pub fn add_input(&mut self, text: &str) {
        self.input_buffer.push_str(text);
    }

    /// Gets the terminal input buffer.
    pub fn get_input(&self) -> &str {
        &self.input_buffer
    }

    /// Clears the terminal input buffer.
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
    }

    /// Types a character into the terminal.
    pub fn type_char(&mut self, c: char) {
        self.input_buffer.push(c);
    }

    /// Submits the current input for processing.
    pub fn submit_input(&mut self) -> String {
        let input = std::mem::take(&mut self.input_buffer);
        self.add_output(&format!("$ {}\n", input));
        input
    }
}