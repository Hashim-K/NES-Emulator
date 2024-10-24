use cpu::Cpu;
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

    let cpu = Cpu::get_cpu(file_bytes)?;

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
    use crate::cpu::Cpu;
    use log::LevelFilter;
    use tudelft_nes_test::{run_tests, TestSelector};

    #[test]
    fn test_nrom() {
        env_logger::builder().filter_level(LevelFilter::Info).init();
        let result = run_tests::<Cpu>(TestSelector::NROM_TEST);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }

    #[ignore]
    #[test]
    fn test_official_instructions() {
        let result = run_tests::<Cpu>(TestSelector::OFFICIAL_INSTRS);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }

    #[ignore]
    #[test]
    fn test_nestest() {
        let result = run_tests::<Cpu>(TestSelector::NESTEST);
        assert!(result.is_ok(), "TEST FAILED: {}", result.unwrap_err());
    }
}
