// SigmaOS File Manager
/// Inspired by Linux Mint's file manager and Omarchy Linux file operations
/// Provides file system browsing and management capabilities.

/// FileRepresents a file or directory in the file system.
#[derive(Debug, Clone, Default)]
pub struct File {
    /// File name
    name: String,
    /// Whether it is a directory
    is_directory: bool,
    /// File size in bytes (0 for directories in simplified model)
    size: u64,
    /// File path
    path: String,
}

impl File {
    /// Creates a new File instance.
    pub fn new(name: &str, is_directory: bool, size: u64, path: &str) -> Self {
        Self {
            name: name.to_string(),
            is_directory,
            size,
            path: path.to_string(),
        }
    }

    /// Returns the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Whether the entry is a directory.
    pub fn is_directory(&self) -> bool {
        self.is_directory
    }

    /// Returns the file size in bytes.
    pub fn size(&self) -> u64 {
        self.size
    }

    /// Returns the file path.
    pub fn path(&self) -> &str {
        &self.path
    }
}

/// FileManager manages file system operations.
///
/// Inspired by Linux Mint's file manager and Omarchy Linux file operations.
/// Provides file browsing, selection, and management.
pub struct FileManager {
    /// Current working directory
    current_dir: File,
    /// History of visited directories
    history: Vec<File>,
    /// Whether the file manager is active
    active: bool,
}

impl Default for FileManager {
    fn default() -> Self {
        Self {
            current_dir: File::default(),
            history: Vec::new(),
            active: false,
        }
    }
}

impl FileManager {
    /// Creates a new FileManager instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the current working directory.
    pub fn set_current_dir(&mut self, dir: File) {
        self.history.push(std::mem::take(&mut self.current_dir));
        self.current_dir = dir;
    }

    /// Goes to the parent directory.
    pub fn go_parent(&mut self) -> Result<(), String> {
        // In a real implementation, would navigate to parent
        // For now, just return OK
        Ok(())
    }

    /// Goes to a specific directory.
    pub fn go_directory(&mut self, _name: &str) -> Result<(), String> {
        // In a real implementation, would navigate to the named directory
        // For now, just return OK
        Ok(())
    }

    /// Returns the current working directory.
    pub fn current_dir(&self) -> &File {
        &self.current_dir
    }

    /// Whether the file manager is active.
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activates the file manager.
    pub fn activate(&mut self) {
        self.active = true;
    }

    /// Deactivates the file manager.
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// Returns the history of visited directories.
    pub fn history(&self) -> &Vec<File> {
        &self.history
    }
}