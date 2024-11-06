#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DebugMode {
    Emu,
    Info,
    No,
}

impl DebugMode {
    // Specific InfoLog method
    // Specific EmuLog method
    pub fn emu_log(&self, message: String) {
        if let DebugMode::Emu = self {
            println!("{}", message);
        }
    }
}
