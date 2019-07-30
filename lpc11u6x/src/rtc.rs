#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - RTC match register"]
    pub match_: MATCH,
    #[doc = "0x08 - RTC counter register"]
    pub count: COUNT,
    #[doc = "0x0c - RTC high-resolution/wake-up timer control register"]
    pub wake: WAKE,
}
#[doc = "RTC control register"]
pub struct CTRL {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "RTC match register"]
pub struct MATCH {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RTC match register"]
pub mod match_;
#[doc = "RTC counter register"]
pub struct COUNT {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RTC counter register"]
pub mod count;
#[doc = "RTC high-resolution/wake-up timer control register"]
pub struct WAKE {
    register: vcell::VolatileCell<u32>,
}
#[doc = "RTC high-resolution/wake-up timer control register"]
pub mod wake;
