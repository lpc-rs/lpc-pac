#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power control register"]
    pub pcon: PCON,
    #[doc = "0x04 - General purpose register N"]
    pub gpreg: [GPREG; 4],
    #[doc = "0x14 - Deep power-down control register. Also includes bits for general purpose storage."]
    pub dpdctrl: DPDCTRL,
}
#[doc = "Power control register"]
pub struct PCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power control register"]
pub mod pcon;
#[doc = "General purpose register N"]
pub struct GPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General purpose register N"]
pub mod gpreg;
#[doc = "Deep power-down control register. Also includes bits for general purpose storage."]
pub struct DPDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deep power-down control register. Also includes bits for general purpose storage."]
pub mod dpdctrl;
