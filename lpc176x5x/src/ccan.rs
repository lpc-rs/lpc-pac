#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN Central Transmit Status Register"]
    pub txsr: TXSR,
    #[doc = "0x04 - CAN Central Receive Status Register"]
    pub rxsr: RXSR,
    #[doc = "0x08 - CAN Central Miscellaneous Register"]
    pub msr: MSR,
}
#[doc = "CAN Central Transmit Status Register"]
pub struct TXSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Central Transmit Status Register"]
pub mod txsr;
#[doc = "CAN Central Receive Status Register"]
pub struct RXSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Central Receive Status Register"]
pub mod rxsr;
#[doc = "CAN Central Miscellaneous Register"]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN Central Miscellaneous Register"]
pub mod msr;
