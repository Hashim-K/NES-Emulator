use cpu::MyCpu;
use error::MainError;
use log::LevelFilter;
use std::env;
use std::fs;
use std::process::ExitCode;
use tudelft_nes_ppu::{run_cpu, Mirroring};
use tudelft_nes_test::TestableCpu;
use tudelft_nes_test::ROM_NROM_TEST;

mod cpu;
mod error;
mod memory;

fn run(file_bytes: &[u8]) -> Result<(), MainError> {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = MyCpu::get_cpu(file_bytes)?;

    log::info!("running cpu");
    run_cpu(cpu, Mirroring::Horizontal);
    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Invalid number of arguments");
        return ExitCode::from(2);
    }

    let file_bytes = if args.len() == 2 {
        fs::read(&args[1]).unwrap()
    } else {
        ROM_NROM_TEST.to_vec()
    };

    match run(&file_bytes) {
        Ok(_) => ExitCode::SUCCESS,
        Err(a) => {
            eprintln!("{}", a);
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::MyCpu;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    /// This test fails in the template, since you didn't implement the cpu yet.
    #[ignore] // This test doesn't pass yet
    #[test]
    fn test_all() {
        env_logger::builder().filter_level(LevelFilter::Info).init();
        let result = run_tests::<MyCpu>(TestSelector::DEFAULT);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }
}
