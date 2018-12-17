#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CAN AF ram access register"]
    pub mask: [MASK; 512],
}
#[doc = "CAN AF ram access register"]
pub struct MASK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "CAN AF ram access register"]
pub mod mask;
