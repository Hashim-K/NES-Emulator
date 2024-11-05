#[allow(clippy::upper_case_acronyms)]
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub(crate) enum InterruptState {
    NormalOperation,
    IRQ,
    NMI,
    Uninitialized,
    Booting,
}
//
//
//
//
//
//
//
//  ooooooooooooo      oooooooooooo       .oooooo..o      ooooooooooooo       .oooooo..o
//  8'   888   `8      `888'     `8      d8P'    `Y8      8'   888   `8      d8P'    `Y8
//       888            888              Y88bo.                888           Y88bo.
//       888            888oooo8          `"Y8888o.            888            `"Y8888o.
//       888            888    "              `"Y88b           888                `"Y88b
//       888            888       o      oo     .d8P           888           oo     .d8P
//      o888o          o888ooooood8      8""88888P'           o888o          8""88888P'
//
//
//
//
//
//
#[test]
fn test_ordering() {
    assert!(InterruptState::NMI == InterruptState::NMI);
    assert!(InterruptState::NMI > InterruptState::IRQ);
    assert!(InterruptState::NMI > InterruptState::NormalOperation);
    assert!(InterruptState::IRQ < InterruptState::NMI);
    assert!(InterruptState::IRQ == InterruptState::IRQ);
    assert!(InterruptState::IRQ > InterruptState::NormalOperation);
    assert!(InterruptState::NormalOperation < InterruptState::NMI);
    assert!(InterruptState::NormalOperation < InterruptState::IRQ);
    assert!(InterruptState::NormalOperation == InterruptState::NormalOperation);
}
