#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
    pub b0: [B0; 32],
    #[doc = "0x20 - Byte pin registers port 1"]
    pub b132: B1,
    #[doc = "0x21 - Byte pin registers port 1"]
    pub b133: B1,
    #[doc = "0x22 - Byte pin registers port 1"]
    pub b134: B1,
    #[doc = "0x23 - Byte pin registers port 1"]
    pub b135: B1,
    #[doc = "0x24 - Byte pin registers port 1"]
    pub b136: B1,
    #[doc = "0x25 - Byte pin registers port 1"]
    pub b137: B1,
    #[doc = "0x26 - Byte pin registers port 1"]
    pub b138: B1,
    #[doc = "0x27 - Byte pin registers port 1"]
    pub b139: B1,
    #[doc = "0x28 - Byte pin registers port 1"]
    pub b140: B1,
    #[doc = "0x29 - Byte pin registers port 1"]
    pub b141: B1,
    #[doc = "0x2a - Byte pin registers port 1"]
    pub b142: B1,
    #[doc = "0x2b - Byte pin registers port 1"]
    pub b143: B1,
    #[doc = "0x2c - Byte pin registers port 1"]
    pub b144: B1,
    #[doc = "0x2d - Byte pin registers port 1"]
    pub b145: B1,
    #[doc = "0x2e - Byte pin registers port 1"]
    pub b146: B1,
    #[doc = "0x2f - Byte pin registers port 1"]
    pub b147: B1,
    #[doc = "0x30 - Byte pin registers port 1"]
    pub b148: B1,
    #[doc = "0x31 - Byte pin registers port 1"]
    pub b149: B1,
    #[doc = "0x32 - Byte pin registers port 1"]
    pub b150: B1,
    #[doc = "0x33 - Byte pin registers port 1"]
    pub b151: B1,
    #[doc = "0x34 - Byte pin registers port 1"]
    pub b152: B1,
    #[doc = "0x35 - Byte pin registers port 1"]
    pub b153: B1,
    #[doc = "0x36 - Byte pin registers port 1"]
    pub b154: B1,
    #[doc = "0x37 - Byte pin registers port 1"]
    pub b155: B1,
    #[doc = "0x38 - Byte pin registers port 1"]
    pub b156: B1,
    #[doc = "0x39 - Byte pin registers port 1"]
    pub b157: B1,
    #[doc = "0x3a - Byte pin registers port 1"]
    pub b158: B1,
    #[doc = "0x3b - Byte pin registers port 1"]
    pub b159: B1,
    #[doc = "0x3c - Byte pin registers port 1"]
    pub b160: B1,
    #[doc = "0x3d - Byte pin registers port 1"]
    pub b161: B1,
    #[doc = "0x3e - Byte pin registers port 1"]
    pub b162: B1,
    #[doc = "0x3f - Byte pin registers port 1"]
    pub b163: B1,
    _reserved33: [u8; 4032usize],
    #[doc = "0x1000 - Word pin registers port 0"]
    pub w_0: [W_0; 32],
    #[doc = "0x1080 - Word pin registers port 1"]
    pub w_132: W_1,
    #[doc = "0x1084 - Word pin registers port 1"]
    pub w_133: W_1,
    #[doc = "0x1088 - Word pin registers port 1"]
    pub w_134: W_1,
    #[doc = "0x108c - Word pin registers port 1"]
    pub w_135: W_1,
    #[doc = "0x1090 - Word pin registers port 1"]
    pub w_136: W_1,
    #[doc = "0x1094 - Word pin registers port 1"]
    pub w_137: W_1,
    #[doc = "0x1098 - Word pin registers port 1"]
    pub w_138: W_1,
    #[doc = "0x109c - Word pin registers port 1"]
    pub w_139: W_1,
    #[doc = "0x10a0 - Word pin registers port 1"]
    pub w_140: W_1,
    #[doc = "0x10a4 - Word pin registers port 1"]
    pub w_141: W_1,
    #[doc = "0x10a8 - Word pin registers port 1"]
    pub w_142: W_1,
    #[doc = "0x10ac - Word pin registers port 1"]
    pub w_143: W_1,
    #[doc = "0x10b0 - Word pin registers port 1"]
    pub w_144: W_1,
    #[doc = "0x10b4 - Word pin registers port 1"]
    pub w_145: W_1,
    #[doc = "0x10b8 - Word pin registers port 1"]
    pub w_146: W_1,
    #[doc = "0x10bc - Word pin registers port 1"]
    pub w_147: W_1,
    #[doc = "0x10c0 - Word pin registers port 1"]
    pub w_148: W_1,
    #[doc = "0x10c4 - Word pin registers port 1"]
    pub w_149: W_1,
    #[doc = "0x10c8 - Word pin registers port 1"]
    pub w_150: W_1,
    #[doc = "0x10cc - Word pin registers port 1"]
    pub w_151: W_1,
    #[doc = "0x10d0 - Word pin registers port 1"]
    pub w_152: W_1,
    #[doc = "0x10d4 - Word pin registers port 1"]
    pub w_153: W_1,
    #[doc = "0x10d8 - Word pin registers port 1"]
    pub w_154: W_1,
    #[doc = "0x10dc - Word pin registers port 1"]
    pub w_155: W_1,
    #[doc = "0x10e0 - Word pin registers port 1"]
    pub w_156: W_1,
    #[doc = "0x10e4 - Word pin registers port 1"]
    pub w_157: W_1,
    #[doc = "0x10e8 - Word pin registers port 1"]
    pub w_158: W_1,
    #[doc = "0x10ec - Word pin registers port 1"]
    pub w_159: W_1,
    #[doc = "0x10f0 - Word pin registers port 1"]
    pub w_160: W_1,
    #[doc = "0x10f4 - Word pin registers port 1"]
    pub w_161: W_1,
    #[doc = "0x10f8 - Word pin registers port 1"]
    pub w_162: W_1,
    #[doc = "0x10fc - Word pin registers port 1"]
    pub w_163: W_1,
    _reserved66: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers port 0/1"]
    pub dir: [DIR; 2],
    _reserved67: [u8; 120usize],
    #[doc = "0x2080 - Mask register port 0/1"]
    pub mask: [MASK; 2],
    _reserved68: [u8; 120usize],
    #[doc = "0x2100 - Portpin register port 0"]
    pub pin: [PIN; 2],
    _reserved69: [u8; 120usize],
    #[doc = "0x2180 - Masked port register port 0/1"]
    pub mpin: [MPIN; 2],
    _reserved70: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port 0/1 Read: output bits for port 0/1"]
    pub set: [SET; 2],
    _reserved71: [u8; 120usize],
    #[doc = "0x2280 - Clear port 0/1"]
    pub clr: [CLR; 2],
    _reserved72: [u8; 120usize],
    #[doc = "0x2300 - Toggle port 0/1"]
    pub not: [NOT; 2],
}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
pub struct B0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
pub mod b0;
#[doc = "Byte pin registers port 1"]
pub struct B1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Byte pin registers port 1"]
pub mod b1;
#[doc = "Word pin registers port 0"]
pub struct W_0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers port 0"]
pub mod w_0;
#[doc = "Word pin registers port 1"]
pub struct W_1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Word pin registers port 1"]
pub mod w_1;
#[doc = "Direction registers port 0/1"]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Direction registers port 0/1"]
pub mod dir;
#[doc = "Mask register port 0/1"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register port 0/1"]
pub mod mask;
#[doc = "Portpin register port 0"]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Portpin register port 0"]
pub mod pin;
#[doc = "Masked port register port 0/1"]
pub struct MPIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Masked port register port 0/1"]
pub mod mpin;
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub mod set;
#[doc = "Clear port 0/1"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clear port 0/1"]
pub mod clr;
#[doc = "Toggle port 0/1"]
pub struct NOT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Toggle port 0/1"]
pub mod not;
