#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Self wake-up timer control register."]
    pub ctrl: CTRL,
    _reserved0: [u8; 8usize],
    #[doc = "0x0c - Counter register."]
    pub count: COUNT,
}
#[doc = "Self wake-up timer control register."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Self wake-up timer control register."]
pub mod ctrl;
#[doc = "Counter register."]
pub struct COUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter register."]
pub mod count;
