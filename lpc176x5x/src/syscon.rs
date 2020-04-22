#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    pub flashcfg: FLASHCFG,
    _reserved1: [u8; 124usize],
    #[doc = "0x80 - PLL0 Control Register"]
    pub pll0con: PLL0CON,
    #[doc = "0x84 - PLL0 Configuration Register"]
    pub pll0cfg: PLL0CFG,
    #[doc = "0x88 - PLL0 Status Register"]
    pub pll0stat: PLL0STAT,
    #[doc = "0x8c - PLL0 Feed Register"]
    pub pll0feed: PLL0FEED,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - PLL1 Control Register"]
    pub pll1con: PLL1CON,
    #[doc = "0xa4 - PLL1 Configuration Register"]
    pub pll1cfg: PLL1CFG,
    #[doc = "0xa8 - PLL1 Status Register"]
    pub pll1stat: PLL1STAT,
    #[doc = "0xac - PLL1 Feed Register"]
    pub pll1feed: PLL1FEED,
    _reserved9: [u8; 16usize],
    #[doc = "0xc0 - Power Control Register"]
    pub pcon: PCON,
    #[doc = "0xc4 - Power Control for Peripherals Register"]
    pub pconp: PCONP,
    _reserved11: [u8; 60usize],
    #[doc = "0x104 - CPU Clock Configuration Register"]
    pub cclkcfg: CCLKCFG,
    #[doc = "0x108 - USB Clock Configuration Register"]
    pub usbclkcfg: USBCLKCFG,
    #[doc = "0x10c - Clock Source Select Register"]
    pub clksrcsel: CLKSRCSEL,
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    pub cansleepclr: CANSLEEPCLR,
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    pub canwakeflags: CANWAKEFLAGS,
    _reserved16: [u8; 40usize],
    #[doc = "0x140 - External Interrupt Flag Register"]
    pub extint: EXTINT,
    _reserved17: [u8; 4usize],
    #[doc = "0x148 - External Interrupt Mode register"]
    pub extmode: EXTMODE,
    #[doc = "0x14c - External Interrupt Polarity Register"]
    pub extpolar: EXTPOLAR,
    _reserved19: [u8; 48usize],
    #[doc = "0x180 - Reset Source Identification Register"]
    pub rsid: RSID,
    _reserved20: [u8; 28usize],
    #[doc = "0x1a0 - System control and status"]
    pub scs: SCS,
    _reserved21: [u8; 4usize],
    #[doc = "0x1a8 - Peripheral Clock Selection register 0."]
    pub pclksel0: PCLKSEL0,
    #[doc = "0x1ac - Peripheral Clock Selection register 1."]
    pub pclksel1: PCLKSEL1,
    _reserved23: [u8; 16usize],
    #[doc = "0x1c0 - USB Interrupt Status"]
    pub usbintst: USBINTST,
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    pub dmacreqsel: DMACREQSEL,
    #[doc = "0x1c8 - Clock Output Configuration Register"]
    pub clkoutcfg: CLKOUTCFG,
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flashcfg](flashcfg) module"]
pub type FLASHCFG = crate::Reg<u32, _FLASHCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLASHCFG;
#[doc = "`read()` method returns [flashcfg::R](flashcfg::R) reader structure"]
impl crate::Readable for FLASHCFG {}
#[doc = "`write(|w| ..)` method takes [flashcfg::W](flashcfg::W) writer structure"]
impl crate::Writable for FLASHCFG {}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLL0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0con](pll0con) module"]
pub type PLL0CON = crate::Reg<u32, _PLL0CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CON;
#[doc = "`read()` method returns [pll0con::R](pll0con::R) reader structure"]
impl crate::Readable for PLL0CON {}
#[doc = "`write(|w| ..)` method takes [pll0con::W](pll0con::W) writer structure"]
impl crate::Writable for PLL0CON {}
#[doc = "PLL0 Control Register"]
pub mod pll0con;
#[doc = "PLL0 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0cfg](pll0cfg) module"]
pub type PLL0CFG = crate::Reg<u32, _PLL0CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0CFG;
#[doc = "`read()` method returns [pll0cfg::R](pll0cfg::R) reader structure"]
impl crate::Readable for PLL0CFG {}
#[doc = "`write(|w| ..)` method takes [pll0cfg::W](pll0cfg::W) writer structure"]
impl crate::Writable for PLL0CFG {}
#[doc = "PLL0 Configuration Register"]
pub mod pll0cfg;
#[doc = "PLL0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0stat](pll0stat) module"]
pub type PLL0STAT = crate::Reg<u32, _PLL0STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0STAT;
#[doc = "`read()` method returns [pll0stat::R](pll0stat::R) reader structure"]
impl crate::Readable for PLL0STAT {}
#[doc = "PLL0 Status Register"]
pub mod pll0stat;
#[doc = "PLL0 Feed Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll0feed](pll0feed) module"]
pub type PLL0FEED = crate::Reg<u32, _PLL0FEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL0FEED;
#[doc = "`write(|w| ..)` method takes [pll0feed::W](pll0feed::W) writer structure"]
impl crate::Writable for PLL0FEED {}
#[doc = "PLL0 Feed Register"]
pub mod pll0feed;
#[doc = "PLL1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1con](pll1con) module"]
pub type PLL1CON = crate::Reg<u32, _PLL1CON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1CON;
#[doc = "`read()` method returns [pll1con::R](pll1con::R) reader structure"]
impl crate::Readable for PLL1CON {}
#[doc = "`write(|w| ..)` method takes [pll1con::W](pll1con::W) writer structure"]
impl crate::Writable for PLL1CON {}
#[doc = "PLL1 Control Register"]
pub mod pll1con;
#[doc = "PLL1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1cfg](pll1cfg) module"]
pub type PLL1CFG = crate::Reg<u32, _PLL1CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1CFG;
#[doc = "`read()` method returns [pll1cfg::R](pll1cfg::R) reader structure"]
impl crate::Readable for PLL1CFG {}
#[doc = "`write(|w| ..)` method takes [pll1cfg::W](pll1cfg::W) writer structure"]
impl crate::Writable for PLL1CFG {}
#[doc = "PLL1 Configuration Register"]
pub mod pll1cfg;
#[doc = "PLL1 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1stat](pll1stat) module"]
pub type PLL1STAT = crate::Reg<u32, _PLL1STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1STAT;
#[doc = "`read()` method returns [pll1stat::R](pll1stat::R) reader structure"]
impl crate::Readable for PLL1STAT {}
#[doc = "PLL1 Status Register"]
pub mod pll1stat;
#[doc = "PLL1 Feed Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll1feed](pll1feed) module"]
pub type PLL1FEED = crate::Reg<u32, _PLL1FEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLL1FEED;
#[doc = "`write(|w| ..)` method takes [pll1feed::W](pll1feed::W) writer structure"]
impl crate::Writable for PLL1FEED {}
#[doc = "PLL1 Feed Register"]
pub mod pll1feed;
#[doc = "Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](pcon) module"]
pub type PCON = crate::Reg<u32, _PCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCON;
#[doc = "`read()` method returns [pcon::R](pcon::R) reader structure"]
impl crate::Readable for PCON {}
#[doc = "`write(|w| ..)` method takes [pcon::W](pcon::W) writer structure"]
impl crate::Writable for PCON {}
#[doc = "Power Control Register"]
pub mod pcon;
#[doc = "Power Control for Peripherals Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pconp](pconp) module"]
pub type PCONP = crate::Reg<u32, _PCONP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCONP;
#[doc = "`read()` method returns [pconp::R](pconp::R) reader structure"]
impl crate::Readable for PCONP {}
#[doc = "`write(|w| ..)` method takes [pconp::W](pconp::W) writer structure"]
impl crate::Writable for PCONP {}
#[doc = "Power Control for Peripherals Register"]
pub mod pconp;
#[doc = "CPU Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cclkcfg](cclkcfg) module"]
pub type CCLKCFG = crate::Reg<u32, _CCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCLKCFG;
#[doc = "`read()` method returns [cclkcfg::R](cclkcfg::R) reader structure"]
impl crate::Readable for CCLKCFG {}
#[doc = "`write(|w| ..)` method takes [cclkcfg::W](cclkcfg::W) writer structure"]
impl crate::Writable for CCLKCFG {}
#[doc = "CPU Clock Configuration Register"]
pub mod cclkcfg;
#[doc = "USB Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkcfg](usbclkcfg) module"]
pub type USBCLKCFG = crate::Reg<u32, _USBCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKCFG;
#[doc = "`read()` method returns [usbclkcfg::R](usbclkcfg::R) reader structure"]
impl crate::Readable for USBCLKCFG {}
#[doc = "`write(|w| ..)` method takes [usbclkcfg::W](usbclkcfg::W) writer structure"]
impl crate::Writable for USBCLKCFG {}
#[doc = "USB Clock Configuration Register"]
pub mod usbclkcfg;
#[doc = "Clock Source Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clksrcsel](clksrcsel) module"]
pub type CLKSRCSEL = crate::Reg<u32, _CLKSRCSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKSRCSEL;
#[doc = "`read()` method returns [clksrcsel::R](clksrcsel::R) reader structure"]
impl crate::Readable for CLKSRCSEL {}
#[doc = "`write(|w| ..)` method takes [clksrcsel::W](clksrcsel::W) writer structure"]
impl crate::Writable for CLKSRCSEL {}
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cansleepclr](cansleepclr) module"]
pub type CANSLEEPCLR = crate::Reg<u32, _CANSLEEPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANSLEEPCLR;
#[doc = "`read()` method returns [cansleepclr::R](cansleepclr::R) reader structure"]
impl crate::Readable for CANSLEEPCLR {}
#[doc = "`write(|w| ..)` method takes [cansleepclr::W](cansleepclr::W) writer structure"]
impl crate::Writable for CANSLEEPCLR {}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "Allows reading the wake-up state of the CAN channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canwakeflags](canwakeflags) module"]
pub type CANWAKEFLAGS = crate::Reg<u32, _CANWAKEFLAGS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CANWAKEFLAGS;
#[doc = "`read()` method returns [canwakeflags::R](canwakeflags::R) reader structure"]
impl crate::Readable for CANWAKEFLAGS {}
#[doc = "`write(|w| ..)` method takes [canwakeflags::W](canwakeflags::W) writer structure"]
impl crate::Writable for CANWAKEFLAGS {}
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "External Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extint](extint) module"]
pub type EXTINT = crate::Reg<u32, _EXTINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTINT;
#[doc = "`read()` method returns [extint::R](extint::R) reader structure"]
impl crate::Readable for EXTINT {}
#[doc = "`write(|w| ..)` method takes [extint::W](extint::W) writer structure"]
impl crate::Writable for EXTINT {}
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "External Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extmode](extmode) module"]
pub type EXTMODE = crate::Reg<u32, _EXTMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTMODE;
#[doc = "`read()` method returns [extmode::R](extmode::R) reader structure"]
impl crate::Readable for EXTMODE {}
#[doc = "`write(|w| ..)` method takes [extmode::W](extmode::W) writer structure"]
impl crate::Writable for EXTMODE {}
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "External Interrupt Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extpolar](extpolar) module"]
pub type EXTPOLAR = crate::Reg<u32, _EXTPOLAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTPOLAR;
#[doc = "`read()` method returns [extpolar::R](extpolar::R) reader structure"]
impl crate::Readable for EXTPOLAR {}
#[doc = "`write(|w| ..)` method takes [extpolar::W](extpolar::W) writer structure"]
impl crate::Writable for EXTPOLAR {}
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "Reset Source Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsid](rsid) module"]
pub type RSID = crate::Reg<u32, _RSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSID;
#[doc = "`read()` method returns [rsid::R](rsid::R) reader structure"]
impl crate::Readable for RSID {}
#[doc = "`write(|w| ..)` method takes [rsid::W](rsid::W) writer structure"]
impl crate::Writable for RSID {}
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "System control and status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scs](scs) module"]
pub type SCS = crate::Reg<u32, _SCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCS;
#[doc = "`read()` method returns [scs::R](scs::R) reader structure"]
impl crate::Readable for SCS {}
#[doc = "`write(|w| ..)` method takes [scs::W](scs::W) writer structure"]
impl crate::Writable for SCS {}
#[doc = "System control and status"]
pub mod scs;
#[doc = "Peripheral Clock Selection register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksel0](pclksel0) module"]
pub type PCLKSEL0 = crate::Reg<u32, _PCLKSEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSEL0;
#[doc = "`read()` method returns [pclksel0::R](pclksel0::R) reader structure"]
impl crate::Readable for PCLKSEL0 {}
#[doc = "`write(|w| ..)` method takes [pclksel0::W](pclksel0::W) writer structure"]
impl crate::Writable for PCLKSEL0 {}
#[doc = "Peripheral Clock Selection register 0."]
pub mod pclksel0;
#[doc = "Peripheral Clock Selection register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclksel1](pclksel1) module"]
pub type PCLKSEL1 = crate::Reg<u32, _PCLKSEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCLKSEL1;
#[doc = "`read()` method returns [pclksel1::R](pclksel1::R) reader structure"]
impl crate::Readable for PCLKSEL1 {}
#[doc = "`write(|w| ..)` method takes [pclksel1::W](pclksel1::W) writer structure"]
impl crate::Writable for PCLKSEL1 {}
#[doc = "Peripheral Clock Selection register 1."]
pub mod pclksel1;
#[doc = "USB Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintst](usbintst) module"]
pub type USBINTST = crate::Reg<u32, _USBINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBINTST;
#[doc = "`read()` method returns [usbintst::R](usbintst::R) reader structure"]
impl crate::Readable for USBINTST {}
#[doc = "`write(|w| ..)` method takes [usbintst::W](usbintst::W) writer structure"]
impl crate::Writable for USBINTST {}
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmacreqsel](dmacreqsel) module"]
pub type DMACREQSEL = crate::Reg<u32, _DMACREQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACREQSEL;
#[doc = "`read()` method returns [dmacreqsel::R](dmacreqsel::R) reader structure"]
impl crate::Readable for DMACREQSEL {}
#[doc = "`write(|w| ..)` method takes [dmacreqsel::W](dmacreqsel::W) writer structure"]
impl crate::Writable for DMACREQSEL {}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "Clock Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkoutcfg](clkoutcfg) module"]
pub type CLKOUTCFG = crate::Reg<u32, _CLKOUTCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKOUTCFG;
#[doc = "`read()` method returns [clkoutcfg::R](clkoutcfg::R) reader structure"]
impl crate::Readable for CLKOUTCFG {}
#[doc = "`write(|w| ..)` method takes [clkoutcfg::W](clkoutcfg::W) writer structure"]
impl crate::Writable for CLKOUTCFG {}
#[doc = "Clock Output Configuration Register"]
pub mod clkoutcfg;
