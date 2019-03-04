#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
pub struct INTVAL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
pub mod intval;
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
pub struct TIMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
pub mod timer;
#[doc = "MRT Control register. This register controls the MRT modes."]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT Control register. This register controls the MRT modes."]
pub mod ctrl;
#[doc = "MRT Status register."]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MRT Status register."]
pub mod stat;
