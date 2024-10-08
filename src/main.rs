use std::process::ExitCode;
use log::LevelFilter;
use tudelft_nes_ppu::{run_cpu, Mirroring};
use tudelft_nes_test::TestableCpu;
use tudelft_nes_test::ROM_NROM_TEST;
use cpu::MyCpu;
use error::MyGetCpuError;

mod cpu;
mod memory;
mod error;

fn run() -> Result<(), MyGetCpuError> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = MyCpu::get_cpu(ROM_NROM_TEST)?;

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
        },
    }
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};
    use crate::cpu::MyCpu;

    /// This test fails in the template, since you didn't implement the cpu yet.
    #[test]
    fn test_all() {
        env_logger::builder().filter_level(LevelFilter::Info).init();
        let result = run_tests::<MyCpu>(TestSelector::DEFAULT);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }
}
