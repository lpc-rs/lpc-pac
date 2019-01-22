#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO overall Interrupt Status."]
    pub status: STATUS,
    #[doc = "0x04 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr0: STATR0,
    #[doc = "0x08 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf0: STATF0,
    #[doc = "0x0c - GPIO Interrupt Clear."]
    pub clr0: CLR0,
    #[doc = "0x10 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr0: ENR0,
    #[doc = "0x14 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf0: ENF0,
    _reserved6: [u8; 12usize],
    #[doc = "0x24 - GPIO Interrupt Status for Rising edge for Port 0."]
    pub statr2: STATR2,
    #[doc = "0x28 - GPIO Interrupt Status for Falling edge for Port 0."]
    pub statf2: STATF2,
    #[doc = "0x2c - GPIO Interrupt Clear."]
    pub clr2: CLR2,
    #[doc = "0x30 - GPIO Interrupt Enable for Rising edge for Port 0."]
    pub enr2: ENR2,
    #[doc = "0x34 - GPIO Interrupt Enable for Falling edge for Port 0."]
    pub enf2: ENF2,
}
#[doc = "GPIO overall Interrupt Status."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO overall Interrupt Status."]
pub mod status;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub struct STATR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr0;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub struct STATF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf0;
#[doc = "GPIO Interrupt Clear."]
pub struct CLR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Clear."]
pub mod clr0;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub struct ENR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr0;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub struct ENF0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf0;
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub struct STATR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0."]
pub mod statr2;
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub struct STATF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0."]
pub mod statf2;
#[doc = "GPIO Interrupt Clear."]
pub struct CLR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Clear."]
pub mod clr2;
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub struct ENR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Enable for Rising edge for Port 0."]
pub mod enr2;
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub struct ENF2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "GPIO Interrupt Enable for Falling edge for Port 0."]
pub mod enf2;
