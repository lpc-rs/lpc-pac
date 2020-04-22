#[doc = r"Register block"]
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
#[doc = "DMA Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "DMA Interrupt Terminal Count Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttcstat](inttcstat) module"]
pub type INTTCSTAT = crate::Reg<u32, _INTTCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTCSTAT;
#[doc = "`read()` method returns [inttcstat::R](inttcstat::R) reader structure"]
impl crate::Readable for INTTCSTAT {}
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "DMA Interrupt Terminal Count Request Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inttcclear](inttcclear) module"]
pub type INTTCCLEAR = crate::Reg<u32, _INTTCCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTTCCLEAR;
#[doc = "`write(|w| ..)` method takes [inttcclear::W](inttcclear::W) writer structure"]
impl crate::Writable for INTTCCLEAR {}
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "DMA Interrupt Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrstat](interrstat) module"]
pub type INTERRSTAT = crate::Reg<u32, _INTERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRSTAT;
#[doc = "`read()` method returns [interrstat::R](interrstat::R) reader structure"]
impl crate::Readable for INTERRSTAT {}
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "DMA Interrupt Error Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interrclr](interrclr) module"]
pub type INTERRCLR = crate::Reg<u32, _INTERRCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTERRCLR;
#[doc = "`write(|w| ..)` method takes [interrclr::W](interrclr::W) writer structure"]
impl crate::Writable for INTERRCLR {}
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "DMA Raw Interrupt Terminal Count Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawinttcstat](rawinttcstat) module"]
pub type RAWINTTCSTAT = crate::Reg<u32, _RAWINTTCSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWINTTCSTAT;
#[doc = "`read()` method returns [rawinttcstat::R](rawinttcstat::R) reader structure"]
impl crate::Readable for RAWINTTCSTAT {}
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "DMA Raw Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rawinterrstat](rawinterrstat) module"]
pub type RAWINTERRSTAT = crate::Reg<u32, _RAWINTERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAWINTERRSTAT;
#[doc = "`read()` method returns [rawinterrstat::R](rawinterrstat::R) reader structure"]
impl crate::Readable for RAWINTERRSTAT {}
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "DMA Enabled Channel Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enbldchns](enbldchns) module"]
pub type ENBLDCHNS = crate::Reg<u32, _ENBLDCHNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENBLDCHNS;
#[doc = "`read()` method returns [enbldchns::R](enbldchns::R) reader structure"]
impl crate::Readable for ENBLDCHNS {}
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "DMA Software Burst Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softbreq](softbreq) module"]
pub type SOFTBREQ = crate::Reg<u32, _SOFTBREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTBREQ;
#[doc = "`read()` method returns [softbreq::R](softbreq::R) reader structure"]
impl crate::Readable for SOFTBREQ {}
#[doc = "`write(|w| ..)` method takes [softbreq::W](softbreq::W) writer structure"]
impl crate::Writable for SOFTBREQ {}
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "DMA Software Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softsreq](softsreq) module"]
pub type SOFTSREQ = crate::Reg<u32, _SOFTSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTSREQ;
#[doc = "`read()` method returns [softsreq::R](softsreq::R) reader structure"]
impl crate::Readable for SOFTSREQ {}
#[doc = "`write(|w| ..)` method takes [softsreq::W](softsreq::W) writer structure"]
impl crate::Writable for SOFTSREQ {}
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "DMA Software Last Burst Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softlbreq](softlbreq) module"]
pub type SOFTLBREQ = crate::Reg<u32, _SOFTLBREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTLBREQ;
#[doc = "`read()` method returns [softlbreq::R](softlbreq::R) reader structure"]
impl crate::Readable for SOFTLBREQ {}
#[doc = "`write(|w| ..)` method takes [softlbreq::W](softlbreq::W) writer structure"]
impl crate::Writable for SOFTLBREQ {}
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "DMA Software Last Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softlsreq](softlsreq) module"]
pub type SOFTLSREQ = crate::Reg<u32, _SOFTLSREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOFTLSREQ;
#[doc = "`read()` method returns [softlsreq::R](softlsreq::R) reader structure"]
impl crate::Readable for SOFTLSREQ {}
#[doc = "`write(|w| ..)` method takes [softlsreq::W](softlsreq::W) writer structure"]
impl crate::Writable for SOFTLSREQ {}
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "DMA Synchronization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "DMA Channel 0 Source Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcaddr](srcaddr) module"]
pub type SRCADDR = crate::Reg<u32, _SRCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCADDR;
#[doc = "`read()` method returns [srcaddr::R](srcaddr::R) reader structure"]
impl crate::Readable for SRCADDR {}
#[doc = "`write(|w| ..)` method takes [srcaddr::W](srcaddr::W) writer structure"]
impl crate::Writable for SRCADDR {}
#[doc = "DMA Channel 0 Source Address Register"]
pub mod srcaddr;
#[doc = "DMA Channel 0 Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destaddr](destaddr) module"]
pub type DESTADDR = crate::Reg<u32, _DESTADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DESTADDR;
#[doc = "`read()` method returns [destaddr::R](destaddr::R) reader structure"]
impl crate::Readable for DESTADDR {}
#[doc = "`write(|w| ..)` method takes [destaddr::W](destaddr::W) writer structure"]
impl crate::Writable for DESTADDR {}
#[doc = "DMA Channel 0 Destination Address Register"]
pub mod destaddr;
#[doc = "DMA Channel 0 Linked List Item Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lli](lli) module"]
pub type LLI = crate::Reg<u32, _LLI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LLI;
#[doc = "`read()` method returns [lli::R](lli::R) reader structure"]
impl crate::Readable for LLI {}
#[doc = "`write(|w| ..)` method takes [lli::W](lli::W) writer structure"]
impl crate::Writable for LLI {}
#[doc = "DMA Channel 0 Linked List Item Register"]
pub mod lli;
#[doc = "DMA Channel 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "DMA Channel 0 Control Register"]
pub mod control;
#[doc = "DMA Channel 0 Configuration Register\\[1\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub mod config;
