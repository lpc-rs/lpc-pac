#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Port Direction control register."]
    pub dir0: DIR,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Mask register for Port."]
    pub mask0: MASK,
    #[doc = "0x14 - Port Pin value register using FIOMASK."]
    pub pin0: PIN,
    #[doc = "0x18 - Port Output Set register using FIOMASK."]
    pub set0: SET,
    #[doc = "0x1c - Port Output Clear register using FIOMASK."]
    pub clr0: CLR,
    #[doc = "0x20 - GPIO Port Direction control register."]
    pub dir1: DIR,
    _reserved6: [u8; 12usize],
    #[doc = "0x30 - Mask register for Port."]
    pub mask1: MASK,
    #[doc = "0x34 - Port Pin value register using FIOMASK."]
    pub pin1: PIN,
    #[doc = "0x38 - Port Output Set register using FIOMASK."]
    pub set1: SET,
    #[doc = "0x3c - Port Output Clear register using FIOMASK."]
    pub clr1: CLR,
    #[doc = "0x40 - GPIO Port Direction control register."]
    pub dir2: DIR,
    _reserved11: [u8; 12usize],
    #[doc = "0x50 - Mask register for Port."]
    pub mask2: MASK,
    #[doc = "0x54 - Port Pin value register using FIOMASK."]
    pub pin2: PIN,
    #[doc = "0x58 - Port Output Set register using FIOMASK."]
    pub set2: SET,
    #[doc = "0x5c - Port Output Clear register using FIOMASK."]
    pub clr2: CLR,
    #[doc = "0x60 - GPIO Port Direction control register."]
    pub dir3: DIR,
    _reserved16: [u8; 12usize],
    #[doc = "0x70 - Mask register for Port."]
    pub mask3: MASK,
    #[doc = "0x74 - Port Pin value register using FIOMASK."]
    pub pin3: PIN,
    #[doc = "0x78 - Port Output Set register using FIOMASK."]
    pub set3: SET,
    #[doc = "0x7c - Port Output Clear register using FIOMASK."]
    pub clr3: CLR,
    #[doc = "0x80 - GPIO Port Direction control register."]
    pub dir4: DIR,
    _reserved21: [u8; 12usize],
    #[doc = "0x90 - Mask register for Port."]
    pub mask4: MASK,
    #[doc = "0x94 - Port Pin value register using FIOMASK."]
    pub pin4: PIN,
    #[doc = "0x98 - Port Output Set register using FIOMASK."]
    pub set4: SET,
    #[doc = "0x9c - Port Output Clear register using FIOMASK."]
    pub clr4: CLR,
}
#[doc = "GPIO Port Direction control register."]
pub struct DIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Port Direction control register."]
pub mod dir;
#[doc = "Mask register for Port."]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register for Port."]
pub mod mask;
#[doc = "Port Pin value register using FIOMASK."]
pub struct PIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Pin value register using FIOMASK."]
pub mod pin;
#[doc = "Port Output Set register using FIOMASK."]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Output Set register using FIOMASK."]
pub mod set;
#[doc = "Port Output Clear register using FIOMASK."]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Port Output Clear register using FIOMASK."]
pub mod clr;
