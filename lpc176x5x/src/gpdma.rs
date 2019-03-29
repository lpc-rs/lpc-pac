#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Status Register"]
    pub intstat: INTSTAT,
    #[doc = "0x04 - DMA Interrupt Terminal Count Request Status Register"]
    pub inttcstat: INTTCSTAT,
    #[doc = "0x08 - DMA Interrupt Terminal Count Request Clear Register"]
    pub inttcclear: INTTCCLEAR,
    #[doc = "0x0c - DMA Interrupt Error Status Register"]
    pub interrstat: INTERRSTAT,
    #[doc = "0x10 - DMA Interrupt Error Clear Register"]
    pub interrclr: INTERRCLR,
    #[doc = "0x14 - DMA Raw Interrupt Terminal Count Status Register"]
    pub rawinttcstat: RAWINTTCSTAT,
    #[doc = "0x18 - DMA Raw Error Interrupt Status Register"]
    pub rawinterrstat: RAWINTERRSTAT,
    #[doc = "0x1c - DMA Enabled Channel Register"]
    pub enbldchns: ENBLDCHNS,
    #[doc = "0x20 - DMA Software Burst Request Register"]
    pub softbreq: SOFTBREQ,
    #[doc = "0x24 - DMA Software Single Request Register"]
    pub softsreq: SOFTSREQ,
    #[doc = "0x28 - DMA Software Last Burst Request Register"]
    pub softlbreq: SOFTLBREQ,
    #[doc = "0x2c - DMA Software Last Single Request Register"]
    pub softlsreq: SOFTLSREQ,
    #[doc = "0x30 - DMA Configuration Register"]
    pub config: CONFIG,
    #[doc = "0x34 - DMA Synchronization Register"]
    pub sync: SYNC,
    _reserved14: [u8; 200usize],
    #[doc = "0x100 - DMA Channel 0 Source Address Register"]
    pub srcaddr0: SRCADDR,
    #[doc = "0x104 - DMA Channel 0 Destination Address Register"]
    pub destaddr0: DESTADDR,
    #[doc = "0x108 - DMA Channel 0 Linked List Item Register"]
    pub lli0: LLI,
    #[doc = "0x10c - DMA Channel 0 Control Register"]
    pub control0: CONTROL,
    #[doc = "0x110 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config0: CONFIG,
    _reserved19: [u8; 12usize],
    #[doc = "0x120 - DMA Channel 0 Source Address Register"]
    pub srcaddr1: SRCADDR,
    #[doc = "0x124 - DMA Channel 0 Destination Address Register"]
    pub destaddr1: DESTADDR,
    #[doc = "0x128 - DMA Channel 0 Linked List Item Register"]
    pub lli1: LLI,
    #[doc = "0x12c - DMA Channel 0 Control Register"]
    pub control1: CONTROL,
    #[doc = "0x130 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config1: CONFIG,
    _reserved24: [u8; 12usize],
    #[doc = "0x140 - DMA Channel 0 Source Address Register"]
    pub srcaddr2: SRCADDR,
    #[doc = "0x144 - DMA Channel 0 Destination Address Register"]
    pub destaddr2: DESTADDR,
    #[doc = "0x148 - DMA Channel 0 Linked List Item Register"]
    pub lli2: LLI,
    #[doc = "0x14c - DMA Channel 0 Control Register"]
    pub control2: CONTROL,
    #[doc = "0x150 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config2: CONFIG,
    _reserved29: [u8; 12usize],
    #[doc = "0x160 - DMA Channel 0 Source Address Register"]
    pub srcaddr3: SRCADDR,
    #[doc = "0x164 - DMA Channel 0 Destination Address Register"]
    pub destaddr3: DESTADDR,
    #[doc = "0x168 - DMA Channel 0 Linked List Item Register"]
    pub lli3: LLI,
    #[doc = "0x16c - DMA Channel 0 Control Register"]
    pub control3: CONTROL,
    #[doc = "0x170 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config3: CONFIG,
    _reserved34: [u8; 12usize],
    #[doc = "0x180 - DMA Channel 0 Source Address Register"]
    pub srcaddr4: SRCADDR,
    #[doc = "0x184 - DMA Channel 0 Destination Address Register"]
    pub destaddr4: DESTADDR,
    #[doc = "0x188 - DMA Channel 0 Linked List Item Register"]
    pub lli4: LLI,
    #[doc = "0x18c - DMA Channel 0 Control Register"]
    pub control4: CONTROL,
    #[doc = "0x190 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config4: CONFIG,
    _reserved39: [u8; 12usize],
    #[doc = "0x1a0 - DMA Channel 0 Source Address Register"]
    pub srcaddr5: SRCADDR,
    #[doc = "0x1a4 - DMA Channel 0 Destination Address Register"]
    pub destaddr5: DESTADDR,
    #[doc = "0x1a8 - DMA Channel 0 Linked List Item Register"]
    pub lli5: LLI,
    #[doc = "0x1ac - DMA Channel 0 Control Register"]
    pub control5: CONTROL,
    #[doc = "0x1b0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config5: CONFIG,
    _reserved44: [u8; 12usize],
    #[doc = "0x1c0 - DMA Channel 0 Source Address Register"]
    pub srcaddr6: SRCADDR,
    #[doc = "0x1c4 - DMA Channel 0 Destination Address Register"]
    pub destaddr6: DESTADDR,
    #[doc = "0x1c8 - DMA Channel 0 Linked List Item Register"]
    pub lli6: LLI,
    #[doc = "0x1cc - DMA Channel 0 Control Register"]
    pub control6: CONTROL,
    #[doc = "0x1d0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config6: CONFIG,
    _reserved49: [u8; 12usize],
    #[doc = "0x1e0 - DMA Channel 0 Source Address Register"]
    pub srcaddr7: SRCADDR,
    #[doc = "0x1e4 - DMA Channel 0 Destination Address Register"]
    pub destaddr7: DESTADDR,
    #[doc = "0x1e8 - DMA Channel 0 Linked List Item Register"]
    pub lli7: LLI,
    #[doc = "0x1ec - DMA Channel 0 Control Register"]
    pub control7: CONTROL,
    #[doc = "0x1f0 - DMA Channel 0 Configuration Register\\[1\\]"]
    pub config7: CONFIG,
}
#[doc = "DMA Interrupt Status Register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub struct INTTCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub struct INTTCCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "DMA Interrupt Error Status Register"]
pub struct INTERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "DMA Interrupt Error Clear Register"]
pub struct INTERRCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub struct RAWINTTCSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "DMA Raw Error Interrupt Status Register"]
pub struct RAWINTERRSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "DMA Enabled Channel Register"]
pub struct ENBLDCHNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "DMA Software Burst Request Register"]
pub struct SOFTBREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "DMA Software Single Request Register"]
pub struct SOFTSREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "DMA Software Last Burst Request Register"]
pub struct SOFTLBREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "DMA Software Last Single Request Register"]
pub struct SOFTLSREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "DMA Configuration Register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "DMA Synchronization Register"]
pub struct SYNC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "DMA Channel 0 Source Address Register"]
pub struct SRCADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Source Address Register"]
pub mod srcaddr;
#[doc = "DMA Channel 0 Destination Address Register"]
pub struct DESTADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Destination Address Register"]
pub mod destaddr;
#[doc = "DMA Channel 0 Linked List Item Register"]
pub struct LLI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Linked List Item Register"]
pub mod lli;
#[doc = "DMA Channel 0 Control Register"]
pub struct CONTROL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Control Register"]
pub mod control;
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub mod config;
