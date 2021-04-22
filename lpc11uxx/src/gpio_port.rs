#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
    pub b0: [crate::Reg<b0::B0_SPEC>; 32],
    #[doc = "0x20 - Byte pin registers port 1"]
    pub b132: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x21 - Byte pin registers port 1"]
    pub b133: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x22 - Byte pin registers port 1"]
    pub b134: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x23 - Byte pin registers port 1"]
    pub b135: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x24 - Byte pin registers port 1"]
    pub b136: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x25 - Byte pin registers port 1"]
    pub b137: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x26 - Byte pin registers port 1"]
    pub b138: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x27 - Byte pin registers port 1"]
    pub b139: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x28 - Byte pin registers port 1"]
    pub b140: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x29 - Byte pin registers port 1"]
    pub b141: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2a - Byte pin registers port 1"]
    pub b142: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2b - Byte pin registers port 1"]
    pub b143: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2c - Byte pin registers port 1"]
    pub b144: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2d - Byte pin registers port 1"]
    pub b145: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2e - Byte pin registers port 1"]
    pub b146: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x2f - Byte pin registers port 1"]
    pub b147: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x30 - Byte pin registers port 1"]
    pub b148: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x31 - Byte pin registers port 1"]
    pub b149: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x32 - Byte pin registers port 1"]
    pub b150: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x33 - Byte pin registers port 1"]
    pub b151: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x34 - Byte pin registers port 1"]
    pub b152: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x35 - Byte pin registers port 1"]
    pub b153: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x36 - Byte pin registers port 1"]
    pub b154: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x37 - Byte pin registers port 1"]
    pub b155: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x38 - Byte pin registers port 1"]
    pub b156: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x39 - Byte pin registers port 1"]
    pub b157: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3a - Byte pin registers port 1"]
    pub b158: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3b - Byte pin registers port 1"]
    pub b159: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3c - Byte pin registers port 1"]
    pub b160: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3d - Byte pin registers port 1"]
    pub b161: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3e - Byte pin registers port 1"]
    pub b162: crate::Reg<b1::B1_SPEC>,
    #[doc = "0x3f - Byte pin registers port 1"]
    pub b163: crate::Reg<b1::B1_SPEC>,
    _reserved33: [u8; 4032usize],
    #[doc = "0x1000 - Word pin registers port 0"]
    pub w_0: [crate::Reg<w_0::W_0_SPEC>; 32],
    #[doc = "0x1080 - Word pin registers port 1"]
    pub w_132: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x1084 - Word pin registers port 1"]
    pub w_133: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x1088 - Word pin registers port 1"]
    pub w_134: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x108c - Word pin registers port 1"]
    pub w_135: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x1090 - Word pin registers port 1"]
    pub w_136: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x1094 - Word pin registers port 1"]
    pub w_137: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x1098 - Word pin registers port 1"]
    pub w_138: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x109c - Word pin registers port 1"]
    pub w_139: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10a0 - Word pin registers port 1"]
    pub w_140: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10a4 - Word pin registers port 1"]
    pub w_141: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10a8 - Word pin registers port 1"]
    pub w_142: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10ac - Word pin registers port 1"]
    pub w_143: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10b0 - Word pin registers port 1"]
    pub w_144: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10b4 - Word pin registers port 1"]
    pub w_145: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10b8 - Word pin registers port 1"]
    pub w_146: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10bc - Word pin registers port 1"]
    pub w_147: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10c0 - Word pin registers port 1"]
    pub w_148: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10c4 - Word pin registers port 1"]
    pub w_149: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10c8 - Word pin registers port 1"]
    pub w_150: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10cc - Word pin registers port 1"]
    pub w_151: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10d0 - Word pin registers port 1"]
    pub w_152: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10d4 - Word pin registers port 1"]
    pub w_153: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10d8 - Word pin registers port 1"]
    pub w_154: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10dc - Word pin registers port 1"]
    pub w_155: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10e0 - Word pin registers port 1"]
    pub w_156: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10e4 - Word pin registers port 1"]
    pub w_157: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10e8 - Word pin registers port 1"]
    pub w_158: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10ec - Word pin registers port 1"]
    pub w_159: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10f0 - Word pin registers port 1"]
    pub w_160: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10f4 - Word pin registers port 1"]
    pub w_161: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10f8 - Word pin registers port 1"]
    pub w_162: crate::Reg<w_1::W_1_SPEC>,
    #[doc = "0x10fc - Word pin registers port 1"]
    pub w_163: crate::Reg<w_1::W_1_SPEC>,
    _reserved66: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers port 0/1"]
    pub dir: [crate::Reg<dir::DIR_SPEC>; 2],
    _reserved67: [u8; 120usize],
    #[doc = "0x2080 - Mask register port 0/1"]
    pub mask: [crate::Reg<mask::MASK_SPEC>; 2],
    _reserved68: [u8; 120usize],
    #[doc = "0x2100 - Portpin register port 0"]
    pub pin: [crate::Reg<pin::PIN_SPEC>; 2],
    _reserved69: [u8; 120usize],
    #[doc = "0x2180 - Masked port register port 0/1"]
    pub mpin: [crate::Reg<mpin::MPIN_SPEC>; 2],
    _reserved70: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port 0/1 Read: output bits for port 0/1"]
    pub set: [crate::Reg<set::SET_SPEC>; 2],
    _reserved71: [u8; 120usize],
    #[doc = "0x2280 - Clear port 0/1"]
    pub clr: [crate::Reg<clr::CLR_SPEC>; 2],
    _reserved72: [u8; 120usize],
    #[doc = "0x2300 - Toggle port 0/1"]
    pub not: [crate::Reg<not::NOT_SPEC>; 2],
}
#[doc = "B0 register accessor: an alias for `Reg<B0_SPEC>`"]
pub type B0 = crate::Reg<b0::B0_SPEC>;
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
pub mod b0;
#[doc = "B1 register accessor: an alias for `Reg<B1_SPEC>`"]
pub type B1 = crate::Reg<b1::B1_SPEC>;
#[doc = "Byte pin registers port 1"]
pub mod b1;
#[doc = "W_0 register accessor: an alias for `Reg<W_0_SPEC>`"]
pub type W_0 = crate::Reg<w_0::W_0_SPEC>;
#[doc = "Word pin registers port 0"]
pub mod w_0;
#[doc = "W_1 register accessor: an alias for `Reg<W_1_SPEC>`"]
pub type W_1 = crate::Reg<w_1::W_1_SPEC>;
#[doc = "Word pin registers port 1"]
pub mod w_1;
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers port 0/1"]
pub mod dir;
#[doc = "MASK register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register port 0/1"]
pub mod mask;
#[doc = "PIN register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Portpin register port 0"]
pub mod pin;
#[doc = "MPIN register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register port 0/1"]
pub mod mpin;
#[doc = "SET register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub mod set;
#[doc = "CLR register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port 0/1"]
pub mod clr;
#[doc = "NOT register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port 0/1"]
pub mod not;
