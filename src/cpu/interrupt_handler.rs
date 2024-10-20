#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub(crate) enum InterruptState {
    NormalOperation,
    IRQ,
    NMI,
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
    assert_eq!(InterruptState::NMI > InterruptState::NMI, false);
    assert_eq!(InterruptState::NMI > InterruptState::IRQ, true);
    assert_eq!(InterruptState::NMI > InterruptState::NormalOperation, true);
    assert_eq!(InterruptState::IRQ > InterruptState::NMI, false);
    assert_eq!(InterruptState::IRQ > InterruptState::IRQ, false);
    assert_eq!(InterruptState::IRQ > InterruptState::NormalOperation, true);
    assert_eq!(InterruptState::NormalOperation > InterruptState::NMI, false);
    assert_eq!(InterruptState::NormalOperation > InterruptState::IRQ, false);
    assert_eq!(
        InterruptState::NormalOperation > InterruptState::NormalOperation,
        false
    );
}
