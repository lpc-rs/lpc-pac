#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pcon: PCON,
    #[doc = "0x04 - General purpose register 0"]
    pub gpreg: [GPREG; 4],
    #[doc = "0x14 - General purpose register 4"]
    pub gpreg4: GPREG4,
}
#[doc = "Power control register"]
pub struct PCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register"]
pub mod pcon;
#[doc = "General purpose register 0"]
pub struct GPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose register 0"]
pub mod gpreg;
#[doc = "General purpose register 4"]
pub struct GPREG4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose register 4"]
pub mod gpreg4;
