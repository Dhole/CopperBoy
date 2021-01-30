use super::mcu::GPIOPort;

pub const PORT_ARROW: GPIOPort = GPIOPort::F;
pub const PIN_LEFT: u8 = 1 << 5;
pub const PIN_RIGHT: u8 = 1 << 6;
pub const PIN_UP: u8 = 1 << 7;
pub const PIN_DOWN: u8 = 1 << 4;

pub const PORT_A: GPIOPort = GPIOPort::E;
pub const PIN_A: u8 = 1 << 6;
pub const PORT_B: GPIOPort = GPIOPort::B;
pub const PIN_B: u8 = 1 << 4;
