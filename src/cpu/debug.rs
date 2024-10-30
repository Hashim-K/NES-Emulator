#[derive(Debug, Clone, PartialEq)]
pub enum DebugMode {
    EmuDebug,
    InfoDebug,
    NoDebug,
}

impl DebugMode {
    // General log method
    pub fn log(&self, message: String) {
        match self {
            DebugMode::EmuDebug => println!("{}", message),
            DebugMode::InfoDebug => println!("{}", message),
            DebugMode::NoDebug => {}
        }
    }

    // Specific InfoLog method
    pub fn info_log(&self, message: String) {
        if let DebugMode::InfoDebug = self {
            println!("{}", message);
        }
    }

    // Specific EmuLog method
    pub fn emu_log(&self, message: String) {
        if let DebugMode::EmuDebug = self {
            println!("{}", message);
        }
    }
}
