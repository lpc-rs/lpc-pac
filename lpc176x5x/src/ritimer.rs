#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Compare register"]
    pub compval: COMPVAL,
    #[doc = "0x04 - Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
    pub mask: MASK,
    #[doc = "0x08 - Control register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - 32-bit counter"]
    pub counter: COUNTER,
}
#[doc = "Compare register"]
pub struct COMPVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare register"]
pub mod compval;
#[doc = "Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mask register. This register holds the 32-bit mask value. A 1 written to any bit will force a compare on the corresponding bit of the counter and compare register."]
pub mod mask;
#[doc = "Control register."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register."]
pub mod ctrl;
#[doc = "32-bit counter"]
pub struct COUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "32-bit counter"]
pub mod counter;
