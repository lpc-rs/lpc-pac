#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB Device Command/Status register"]
    pub devcmdstat: DEVCMDSTAT,
    #[doc = "0x04 - USB Info register"]
    pub info: INFO,
    #[doc = "0x08 - USB EP Command/Status List start address"]
    pub epliststart: EPLISTSTART,
    #[doc = "0x0c - USB Data buffer start address"]
    pub databufstart: DATABUFSTART,
    #[doc = "0x10 - Link Power Management register"]
    pub lpm: LPM,
    #[doc = "0x14 - USB Endpoint skip"]
    pub epskip: EPSKIP,
    #[doc = "0x18 - USB Endpoint Buffer in use"]
    pub epinuse: EPINUSE,
    #[doc = "0x1c - USB Endpoint Buffer Configuration register"]
    pub epbufcfg: EPBUFCFG,
    #[doc = "0x20 - USB interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0x24 - USB interrupt enable register"]
    pub inten: INTEN,
    #[doc = "0x28 - USB set interrupt status register"]
    pub intsetstat: INTSETSTAT,
    #[doc = "0x2c - USB interrupt routing register"]
    pub introuting: INTROUTING,
    _reserved12: [u8; 4usize],
    #[doc = "0x34 - USB Endpoint toggle register"]
    pub eptoggle: EPTOGGLE,
}
#[doc = "USB Device Command/Status register"]
pub struct DEVCMDSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Command/Status register"]
pub mod devcmdstat;
#[doc = "USB Info register"]
pub struct INFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Info register"]
pub mod info;
#[doc = "USB EP Command/Status List start address"]
pub struct EPLISTSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB EP Command/Status List start address"]
pub mod epliststart;
#[doc = "USB Data buffer start address"]
pub struct DATABUFSTART {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Data buffer start address"]
pub mod databufstart;
#[doc = "Link Power Management register"]
pub struct LPM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Link Power Management register"]
pub mod lpm;
#[doc = "USB Endpoint skip"]
pub struct EPSKIP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint skip"]
pub mod epskip;
#[doc = "USB Endpoint Buffer in use"]
pub struct EPINUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Buffer in use"]
pub mod epinuse;
#[doc = "USB Endpoint Buffer Configuration register"]
pub struct EPBUFCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Buffer Configuration register"]
pub mod epbufcfg;
#[doc = "USB interrupt status register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB interrupt status register"]
pub mod intstat;
#[doc = "USB interrupt enable register"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB interrupt enable register"]
pub mod inten;
#[doc = "USB set interrupt status register"]
pub struct INTSETSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB set interrupt status register"]
pub mod intsetstat;
#[doc = "USB interrupt routing register"]
pub struct INTROUTING {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB interrupt routing register"]
pub mod introuting;
#[doc = "USB Endpoint toggle register"]
pub struct EPTOGGLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint toggle register"]
pub mod eptoggle;
