#[derive(Debug, Clone)]
pub enum DebugMode {
    EmuDebug,
    InfoDebug,
    NoDebug,
}

impl DebugMode {
    // General log method
    pub fn log<F>(&self, message_fn: F)
    where
        F: Fn() -> String,
    {
        match self {
            DebugMode::EmuDebug => println!("{}", message_fn()),
            DebugMode::InfoDebug => println!("{}", message_fn()),
            DebugMode::NoDebug => {}
        }
    }

    // Specific InfoLog method
    pub fn info_log<F>(&self, message_fn: F)
    where
        F: Fn() -> String,
    {
        if let DebugMode::InfoDebug = self {
            println!("{}", message_fn());
        }
    }

    // Specific EmuLog method
    pub fn emu_log<F>(&self, message_fn: F)
    where
        F: Fn() -> String,
    {
        if let DebugMode::EmuDebug = self {
            println!("{}", message_fn());
        }
    }
}
