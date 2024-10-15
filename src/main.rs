use error::MainError;
use log::LevelFilter;
use std::process::ExitCode;
use system::System;
use tudelft_nes_ppu::{run_cpu, Mirroring};
use tudelft_nes_test::TestableCpu;
use tudelft_nes_test::ROM_NROM_TEST;

mod cpu;
mod error;
mod memory;
mod system;

fn run() -> Result<(), MainError> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = System::get_cpu(ROM_NROM_TEST)?;

    log::info!("running cpu");
    run_cpu(cpu, Mirroring::Horizontal);
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(a) => {
            eprintln!("{}", a);
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system::System;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    /// This test fails in the template, since you didn't implement the cpu yet.
    #[ignore]
    #[test]
    fn test_all() {
        env_logger::builder().filter_level(LevelFilter::Info).init();
        let result = run_tests::<System>(TestSelector::DEFAULT);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }
}
