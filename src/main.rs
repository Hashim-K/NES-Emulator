use cpu::MyCpu;
use log::LevelFilter;
use tudelft_nes_ppu::{run_cpu, Mirroring};

mod cartridge;
mod cpu;
mod error;

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let cpu = MyCpu { rom: Vec::new() };

    log::info!("running cpu");
    run_cpu(cpu, Mirroring::Horizontal);
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
