#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DebugMode {
    Emu,
    Info,
    No,
}

impl DebugMode {
    // Specific InfoLog method
    pub fn info_log(&self, message: String) {
        if let DebugMode::Info = self {
            println!("{}", message);
        }
    }

    // Specific EmuLog method
    pub fn emu_log(&self, message: String) {
        if let DebugMode::Emu = self {
            println!("{}", message);
        }
    }
}
