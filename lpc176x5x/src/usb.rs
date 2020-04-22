#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 220usize],
    #[doc = "0xdc - USB Receive Packet Length"]
    pub rxplen: RXPLEN,
    _reserved1: [u8; 32usize],
    #[doc = "0x100 - OTG Interrupt Status"]
    pub intst: INTST,
    #[doc = "0x104 - OTG Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x108 - OTG Interrupt Set"]
    pub intset: INTSET,
    #[doc = "0x10c - OTG Interrupt Clear"]
    pub intclr: INTCLR,
    #[doc = "0x110 - OTG Status and Control and USB port select"]
    pub stctrl: STCTRL,
    #[doc = "0x114 - OTG Timer"]
    pub tmr: TMR,
    _reserved7: [u8; 232usize],
    #[doc = "0x200 - USB Device Interrupt Status"]
    pub devintst: DEVINTST,
    #[doc = "0x204 - USB Device Interrupt Enable"]
    pub devinten: DEVINTEN,
    #[doc = "0x208 - USB Device Interrupt Clear"]
    pub devintclr: DEVINTCLR,
    #[doc = "0x20c - USB Device Interrupt Set"]
    pub devintset: DEVINTSET,
    #[doc = "0x210 - USB Command Code"]
    pub cmdcode: CMDCODE,
    #[doc = "0x214 - USB Command Data"]
    pub cmddata: CMDDATA,
    #[doc = "0x218 - USB Receive Data"]
    pub rxdata: RXDATA,
    #[doc = "0x21c - USB Transmit Data"]
    pub txdata: TXDATA,
    _reserved15: [u8; 4usize],
    #[doc = "0x224 - USB Transmit Packet Length"]
    pub txplen: TXPLEN,
    #[doc = "0x228 - USB Control"]
    pub ctrl: CTRL,
    #[doc = "0x22c - USB Device Interrupt Priority"]
    pub devintpri: DEVINTPRI,
    #[doc = "0x230 - USB Endpoint Interrupt Status"]
    pub epintst: EPINTST,
    #[doc = "0x234 - USB Endpoint Interrupt Enable"]
    pub epinten: EPINTEN,
    #[doc = "0x238 - USB Endpoint Interrupt Clear"]
    pub epintclr: EPINTCLR,
    #[doc = "0x23c - USB Endpoint Interrupt Set"]
    pub epintset: EPINTSET,
    #[doc = "0x240 - USB Endpoint Priority"]
    pub epintpri: EPINTPRI,
    #[doc = "0x244 - USB Realize Endpoint"]
    pub reep: REEP,
    #[doc = "0x248 - USB Endpoint Index"]
    pub epind: EPIND,
    #[doc = "0x24c - USB MaxPacketSize"]
    pub maxpsize: MAXPSIZE,
    #[doc = "0x250 - USB DMA Request Status"]
    pub dmarst: DMARST,
    #[doc = "0x254 - USB DMA Request Clear"]
    pub dmarclr: DMARCLR,
    #[doc = "0x258 - USB DMA Request Set"]
    pub dmarset: DMARSET,
    _reserved29: [u8; 36usize],
    #[doc = "0x280 - USB UDCA Head"]
    pub udcah: UDCAH,
    #[doc = "0x284 - USB Endpoint DMA Status"]
    pub epdmast: EPDMAST,
    #[doc = "0x288 - USB Endpoint DMA Enable"]
    pub epdmaen: EPDMAEN,
    #[doc = "0x28c - USB Endpoint DMA Disable"]
    pub epdmadis: EPDMADIS,
    #[doc = "0x290 - USB DMA Interrupt Status"]
    pub dmaintst: DMAINTST,
    #[doc = "0x294 - USB DMA Interrupt Enable"]
    pub dmainten: DMAINTEN,
    _reserved35: [u8; 8usize],
    #[doc = "0x2a0 - USB End of Transfer Interrupt Status"]
    pub eotintst: EOTINTST,
    #[doc = "0x2a4 - USB End of Transfer Interrupt Clear"]
    pub eotintclr: EOTINTCLR,
    #[doc = "0x2a8 - USB End of Transfer Interrupt Set"]
    pub eotintset: EOTINTSET,
    #[doc = "0x2ac - USB New DD Request Interrupt Status"]
    pub nddrintst: NDDRINTST,
    #[doc = "0x2b0 - USB New DD Request Interrupt Clear"]
    pub nddrintclr: NDDRINTCLR,
    #[doc = "0x2b4 - USB New DD Request Interrupt Set"]
    pub nddrintset: NDDRINTSET,
    #[doc = "0x2b8 - USB System Error Interrupt Status"]
    pub syserrintst: SYSERRINTST,
    #[doc = "0x2bc - USB System Error Interrupt Clear"]
    pub syserrintclr: SYSERRINTCLR,
    #[doc = "0x2c0 - USB System Error Interrupt Set"]
    pub syserrintset: SYSERRINTSET,
    _reserved44: [u8; 60usize],
    _reserved_44_i2c: [u8; 4usize],
    #[doc = "0x304 - I2C Status"]
    pub i2c_sts: I2C_STS,
    #[doc = "0x308 - I2C Control"]
    pub i2c_ctl: I2C_CTL,
    #[doc = "0x30c - I2C Clock High"]
    pub i2c_clkhi: I2C_CLKHI,
    #[doc = "0x310 - I2C Clock Low"]
    pub i2c_clklo: I2C_CLKLO,
    _reserved49: [u8; 3296usize],
    _reserved_49_otgclkctrl: [u8; 4usize],
    _reserved_50_otgclkst: [u8; 4usize],
}
impl RegisterBlock {
    #[doc = "0x300 - I2C Transmit"]
    #[inline(always)]
    pub fn i2c_wo(&self) -> &I2C_WO {
        unsafe { &*(((self as *const Self) as *const u8).add(768usize) as *const I2C_WO) }
    }
    #[doc = "0x300 - I2C Transmit"]
    #[inline(always)]
    pub fn i2c_wo_mut(&self) -> &mut I2C_WO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(768usize) as *mut I2C_WO) }
    }
    #[doc = "0x300 - I2C Receive"]
    #[inline(always)]
    pub fn i2c_rx(&self) -> &I2C_RX {
        unsafe { &*(((self as *const Self) as *const u8).add(768usize) as *const I2C_RX) }
    }
    #[doc = "0x300 - I2C Receive"]
    #[inline(always)]
    pub fn i2c_rx_mut(&self) -> &mut I2C_RX {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(768usize) as *mut I2C_RX) }
    }
    #[doc = "0xff4 - OTG clock controller"]
    #[inline(always)]
    pub fn otgclkctrl(&self) -> &OTGCLKCTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(4084usize) as *const OTGCLKCTRL) }
    }
    #[doc = "0xff4 - OTG clock controller"]
    #[inline(always)]
    pub fn otgclkctrl_mut(&self) -> &mut OTGCLKCTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4084usize) as *mut OTGCLKCTRL) }
    }
    #[doc = "0xff4 - USB Clock Control"]
    #[inline(always)]
    pub fn usbclkctrl(&self) -> &USBCLKCTRL {
        unsafe { &*(((self as *const Self) as *const u8).add(4084usize) as *const USBCLKCTRL) }
    }
    #[doc = "0xff4 - USB Clock Control"]
    #[inline(always)]
    pub fn usbclkctrl_mut(&self) -> &mut USBCLKCTRL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4084usize) as *mut USBCLKCTRL) }
    }
    #[doc = "0xff8 - OTG clock status"]
    #[inline(always)]
    pub fn otgclkst(&self) -> &OTGCLKST {
        unsafe { &*(((self as *const Self) as *const u8).add(4088usize) as *const OTGCLKST) }
    }
    #[doc = "0xff8 - OTG clock status"]
    #[inline(always)]
    pub fn otgclkst_mut(&self) -> &mut OTGCLKST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4088usize) as *mut OTGCLKST) }
    }
    #[doc = "0xff8 - USB Clock Status"]
    #[inline(always)]
    pub fn usbclkst(&self) -> &USBCLKST {
        unsafe { &*(((self as *const Self) as *const u8).add(4088usize) as *const USBCLKST) }
    }
    #[doc = "0xff8 - USB Clock Status"]
    #[inline(always)]
    pub fn usbclkst_mut(&self) -> &mut USBCLKST {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4088usize) as *mut USBCLKST) }
    }
}
#[doc = "OTG Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intst](intst) module"]
pub type INTST = crate::Reg<u32, _INTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTST;
#[doc = "`read()` method returns [intst::R](intst::R) reader structure"]
impl crate::Readable for INTST {}
#[doc = "OTG Interrupt Status"]
pub mod intst;
#[doc = "OTG Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "OTG Interrupt Enable"]
pub mod inten;
#[doc = "OTG Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "OTG Interrupt Set"]
pub mod intset;
#[doc = "OTG Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](intclr) module"]
pub type INTCLR = crate::Reg<u32, _INTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLR;
#[doc = "`write(|w| ..)` method takes [intclr::W](intclr::W) writer structure"]
impl crate::Writable for INTCLR {}
#[doc = "OTG Interrupt Clear"]
pub mod intclr;
#[doc = "OTG Status and Control and USB port select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stctrl](stctrl) module"]
pub type STCTRL = crate::Reg<u32, _STCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STCTRL;
#[doc = "`read()` method returns [stctrl::R](stctrl::R) reader structure"]
impl crate::Readable for STCTRL {}
#[doc = "`write(|w| ..)` method takes [stctrl::W](stctrl::W) writer structure"]
impl crate::Writable for STCTRL {}
#[doc = "OTG Status and Control and USB port select"]
pub mod stctrl;
#[doc = "OTG Timer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr](tmr) module"]
pub type TMR = crate::Reg<u32, _TMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMR;
#[doc = "`read()` method returns [tmr::R](tmr::R) reader structure"]
impl crate::Readable for TMR {}
#[doc = "`write(|w| ..)` method takes [tmr::W](tmr::W) writer structure"]
impl crate::Writable for TMR {}
#[doc = "OTG Timer"]
pub mod tmr;
#[doc = "USB Device Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintst](devintst) module"]
pub type DEVINTST = crate::Reg<u32, _DEVINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTST;
#[doc = "`read()` method returns [devintst::R](devintst::R) reader structure"]
impl crate::Readable for DEVINTST {}
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "USB Device Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devinten](devinten) module"]
pub type DEVINTEN = crate::Reg<u32, _DEVINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTEN;
#[doc = "`read()` method returns [devinten::R](devinten::R) reader structure"]
impl crate::Readable for DEVINTEN {}
#[doc = "`write(|w| ..)` method takes [devinten::W](devinten::W) writer structure"]
impl crate::Writable for DEVINTEN {}
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "USB Device Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintclr](devintclr) module"]
pub type DEVINTCLR = crate::Reg<u32, _DEVINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTCLR;
#[doc = "`write(|w| ..)` method takes [devintclr::W](devintclr::W) writer structure"]
impl crate::Writable for DEVINTCLR {}
#[doc = "USB Device Interrupt Clear"]
pub mod devintclr;
#[doc = "USB Device Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintset](devintset) module"]
pub type DEVINTSET = crate::Reg<u32, _DEVINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTSET;
#[doc = "`write(|w| ..)` method takes [devintset::W](devintset::W) writer structure"]
impl crate::Writable for DEVINTSET {}
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "USB Command Code\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdcode](cmdcode) module"]
pub type CMDCODE = crate::Reg<u32, _CMDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDCODE;
#[doc = "`write(|w| ..)` method takes [cmdcode::W](cmdcode::W) writer structure"]
impl crate::Writable for CMDCODE {}
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "USB Command Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmddata](cmddata) module"]
pub type CMDDATA = crate::Reg<u32, _CMDDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDDATA;
#[doc = "`read()` method returns [cmddata::R](cmddata::R) reader structure"]
impl crate::Readable for CMDDATA {}
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "USB Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdata](rxdata) module"]
pub type RXDATA = crate::Reg<u32, _RXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDATA;
#[doc = "`read()` method returns [rxdata::R](rxdata::R) reader structure"]
impl crate::Readable for RXDATA {}
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "USB Transmit Data\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdata](txdata) module"]
pub type TXDATA = crate::Reg<u32, _TXDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDATA;
#[doc = "`write(|w| ..)` method takes [txdata::W](txdata::W) writer structure"]
impl crate::Writable for TXDATA {}
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "USB Receive Packet Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxplen](rxplen) module"]
pub type RXPLEN = crate::Reg<u32, _RXPLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPLEN;
#[doc = "`read()` method returns [rxplen::R](rxplen::R) reader structure"]
impl crate::Readable for RXPLEN {}
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "USB Transmit Packet Length\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txplen](txplen) module"]
pub type TXPLEN = crate::Reg<u32, _TXPLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPLEN;
#[doc = "`write(|w| ..)` method takes [txplen::W](txplen::W) writer structure"]
impl crate::Writable for TXPLEN {}
#[doc = "USB Transmit Packet Length"]
pub mod txplen;
#[doc = "USB Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "USB Device Interrupt Priority\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devintpri](devintpri) module"]
pub type DEVINTPRI = crate::Reg<u32, _DEVINTPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVINTPRI;
#[doc = "`write(|w| ..)` method takes [devintpri::W](devintpri::W) writer structure"]
impl crate::Writable for DEVINTPRI {}
#[doc = "USB Device Interrupt Priority"]
pub mod devintpri;
#[doc = "USB Endpoint Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintst](epintst) module"]
pub type EPINTST = crate::Reg<u32, _EPINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTST;
#[doc = "`read()` method returns [epintst::R](epintst::R) reader structure"]
impl crate::Readable for EPINTST {}
#[doc = "USB Endpoint Interrupt Status"]
pub mod epintst;
#[doc = "USB Endpoint Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinten](epinten) module"]
pub type EPINTEN = crate::Reg<u32, _EPINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTEN;
#[doc = "`read()` method returns [epinten::R](epinten::R) reader structure"]
impl crate::Readable for EPINTEN {}
#[doc = "`write(|w| ..)` method takes [epinten::W](epinten::W) writer structure"]
impl crate::Writable for EPINTEN {}
#[doc = "USB Endpoint Interrupt Enable"]
pub mod epinten;
#[doc = "USB Endpoint Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintclr](epintclr) module"]
pub type EPINTCLR = crate::Reg<u32, _EPINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTCLR;
#[doc = "`write(|w| ..)` method takes [epintclr::W](epintclr::W) writer structure"]
impl crate::Writable for EPINTCLR {}
#[doc = "USB Endpoint Interrupt Clear"]
pub mod epintclr;
#[doc = "USB Endpoint Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintset](epintset) module"]
pub type EPINTSET = crate::Reg<u32, _EPINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTSET;
#[doc = "`write(|w| ..)` method takes [epintset::W](epintset::W) writer structure"]
impl crate::Writable for EPINTSET {}
#[doc = "USB Endpoint Interrupt Set"]
pub mod epintset;
#[doc = "USB Endpoint Priority\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epintpri](epintpri) module"]
pub type EPINTPRI = crate::Reg<u32, _EPINTPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINTPRI;
#[doc = "`write(|w| ..)` method takes [epintpri::W](epintpri::W) writer structure"]
impl crate::Writable for EPINTPRI {}
#[doc = "USB Endpoint Priority"]
pub mod epintpri;
#[doc = "USB Realize Endpoint\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reep](reep) module"]
pub type REEP = crate::Reg<u32, _REEP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REEP;
#[doc = "`read()` method returns [reep::R](reep::R) reader structure"]
impl crate::Readable for REEP {}
#[doc = "`write(|w| ..)` method takes [reep::W](reep::W) writer structure"]
impl crate::Writable for REEP {}
#[doc = "USB Realize Endpoint"]
pub mod reep;
#[doc = "USB Endpoint Index\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epind](epind) module"]
pub type EPIND = crate::Reg<u32, _EPIND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPIND;
#[doc = "`write(|w| ..)` method takes [epind::W](epind::W) writer structure"]
impl crate::Writable for EPIND {}
#[doc = "USB Endpoint Index"]
pub mod epind;
#[doc = "USB MaxPacketSize\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxpsize](maxpsize) module"]
pub type MAXPSIZE = crate::Reg<u32, _MAXPSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXPSIZE;
#[doc = "`read()` method returns [maxpsize::R](maxpsize::R) reader structure"]
impl crate::Readable for MAXPSIZE {}
#[doc = "`write(|w| ..)` method takes [maxpsize::W](maxpsize::W) writer structure"]
impl crate::Writable for MAXPSIZE {}
#[doc = "USB MaxPacketSize"]
pub mod maxpsize;
#[doc = "USB DMA Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarst](dmarst) module"]
pub type DMARST = crate::Reg<u32, _DMARST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARST;
#[doc = "`read()` method returns [dmarst::R](dmarst::R) reader structure"]
impl crate::Readable for DMARST {}
#[doc = "USB DMA Request Status"]
pub mod dmarst;
#[doc = "USB DMA Request Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarclr](dmarclr) module"]
pub type DMARCLR = crate::Reg<u32, _DMARCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARCLR;
#[doc = "`write(|w| ..)` method takes [dmarclr::W](dmarclr::W) writer structure"]
impl crate::Writable for DMARCLR {}
#[doc = "USB DMA Request Clear"]
pub mod dmarclr;
#[doc = "USB DMA Request Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmarset](dmarset) module"]
pub type DMARSET = crate::Reg<u32, _DMARSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMARSET;
#[doc = "`write(|w| ..)` method takes [dmarset::W](dmarset::W) writer structure"]
impl crate::Writable for DMARSET {}
#[doc = "USB DMA Request Set"]
pub mod dmarset;
#[doc = "USB UDCA Head\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udcah](udcah) module"]
pub type UDCAH = crate::Reg<u32, _UDCAH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UDCAH;
#[doc = "`read()` method returns [udcah::R](udcah::R) reader structure"]
impl crate::Readable for UDCAH {}
#[doc = "`write(|w| ..)` method takes [udcah::W](udcah::W) writer structure"]
impl crate::Writable for UDCAH {}
#[doc = "USB UDCA Head"]
pub mod udcah;
#[doc = "USB Endpoint DMA Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdmast](epdmast) module"]
pub type EPDMAST = crate::Reg<u32, _EPDMAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPDMAST;
#[doc = "`read()` method returns [epdmast::R](epdmast::R) reader structure"]
impl crate::Readable for EPDMAST {}
#[doc = "USB Endpoint DMA Status"]
pub mod epdmast;
#[doc = "USB Endpoint DMA Enable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdmaen](epdmaen) module"]
pub type EPDMAEN = crate::Reg<u32, _EPDMAEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPDMAEN;
#[doc = "`write(|w| ..)` method takes [epdmaen::W](epdmaen::W) writer structure"]
impl crate::Writable for EPDMAEN {}
#[doc = "USB Endpoint DMA Enable"]
pub mod epdmaen;
#[doc = "USB Endpoint DMA Disable\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdmadis](epdmadis) module"]
pub type EPDMADIS = crate::Reg<u32, _EPDMADIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPDMADIS;
#[doc = "`write(|w| ..)` method takes [epdmadis::W](epdmadis::W) writer structure"]
impl crate::Writable for EPDMADIS {}
#[doc = "USB Endpoint DMA Disable"]
pub mod epdmadis;
#[doc = "USB DMA Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaintst](dmaintst) module"]
pub type DMAINTST = crate::Reg<u32, _DMAINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTST;
#[doc = "`read()` method returns [dmaintst::R](dmaintst::R) reader structure"]
impl crate::Readable for DMAINTST {}
#[doc = "USB DMA Interrupt Status"]
pub mod dmaintst;
#[doc = "USB DMA Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmainten](dmainten) module"]
pub type DMAINTEN = crate::Reg<u32, _DMAINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMAINTEN;
#[doc = "`read()` method returns [dmainten::R](dmainten::R) reader structure"]
impl crate::Readable for DMAINTEN {}
#[doc = "`write(|w| ..)` method takes [dmainten::W](dmainten::W) writer structure"]
impl crate::Writable for DMAINTEN {}
#[doc = "USB DMA Interrupt Enable"]
pub mod dmainten;
#[doc = "USB End of Transfer Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eotintst](eotintst) module"]
pub type EOTINTST = crate::Reg<u32, _EOTINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOTINTST;
#[doc = "`read()` method returns [eotintst::R](eotintst::R) reader structure"]
impl crate::Readable for EOTINTST {}
#[doc = "USB End of Transfer Interrupt Status"]
pub mod eotintst;
#[doc = "USB End of Transfer Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eotintclr](eotintclr) module"]
pub type EOTINTCLR = crate::Reg<u32, _EOTINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOTINTCLR;
#[doc = "`write(|w| ..)` method takes [eotintclr::W](eotintclr::W) writer structure"]
impl crate::Writable for EOTINTCLR {}
#[doc = "USB End of Transfer Interrupt Clear"]
pub mod eotintclr;
#[doc = "USB End of Transfer Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eotintset](eotintset) module"]
pub type EOTINTSET = crate::Reg<u32, _EOTINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EOTINTSET;
#[doc = "`write(|w| ..)` method takes [eotintset::W](eotintset::W) writer structure"]
impl crate::Writable for EOTINTSET {}
#[doc = "USB End of Transfer Interrupt Set"]
pub mod eotintset;
#[doc = "USB New DD Request Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nddrintst](nddrintst) module"]
pub type NDDRINTST = crate::Reg<u32, _NDDRINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDDRINTST;
#[doc = "`read()` method returns [nddrintst::R](nddrintst::R) reader structure"]
impl crate::Readable for NDDRINTST {}
#[doc = "USB New DD Request Interrupt Status"]
pub mod nddrintst;
#[doc = "USB New DD Request Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nddrintclr](nddrintclr) module"]
pub type NDDRINTCLR = crate::Reg<u32, _NDDRINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDDRINTCLR;
#[doc = "`write(|w| ..)` method takes [nddrintclr::W](nddrintclr::W) writer structure"]
impl crate::Writable for NDDRINTCLR {}
#[doc = "USB New DD Request Interrupt Clear"]
pub mod nddrintclr;
#[doc = "USB New DD Request Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nddrintset](nddrintset) module"]
pub type NDDRINTSET = crate::Reg<u32, _NDDRINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NDDRINTSET;
#[doc = "`write(|w| ..)` method takes [nddrintset::W](nddrintset::W) writer structure"]
impl crate::Writable for NDDRINTSET {}
#[doc = "USB New DD Request Interrupt Set"]
pub mod nddrintset;
#[doc = "USB System Error Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syserrintst](syserrintst) module"]
pub type SYSERRINTST = crate::Reg<u32, _SYSERRINTST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSERRINTST;
#[doc = "`read()` method returns [syserrintst::R](syserrintst::R) reader structure"]
impl crate::Readable for SYSERRINTST {}
#[doc = "USB System Error Interrupt Status"]
pub mod syserrintst;
#[doc = "USB System Error Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syserrintclr](syserrintclr) module"]
pub type SYSERRINTCLR = crate::Reg<u32, _SYSERRINTCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSERRINTCLR;
#[doc = "`write(|w| ..)` method takes [syserrintclr::W](syserrintclr::W) writer structure"]
impl crate::Writable for SYSERRINTCLR {}
#[doc = "USB System Error Interrupt Clear"]
pub mod syserrintclr;
#[doc = "USB System Error Interrupt Set\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syserrintset](syserrintset) module"]
pub type SYSERRINTSET = crate::Reg<u32, _SYSERRINTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSERRINTSET;
#[doc = "`write(|w| ..)` method takes [syserrintset::W](syserrintset::W) writer structure"]
impl crate::Writable for SYSERRINTSET {}
#[doc = "USB System Error Interrupt Set"]
pub mod syserrintset;
#[doc = "I2C Receive\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_rx](i2c_rx) module"]
pub type I2C_RX = crate::Reg<u32, _I2C_RX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_RX;
#[doc = "`read()` method returns [i2c_rx::R](i2c_rx::R) reader structure"]
impl crate::Readable for I2C_RX {}
#[doc = "I2C Receive"]
pub mod i2c_rx;
#[doc = "I2C Transmit\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_wo](i2c_wo) module"]
pub type I2C_WO = crate::Reg<u32, _I2C_WO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_WO;
#[doc = "`write(|w| ..)` method takes [i2c_wo::W](i2c_wo::W) writer structure"]
impl crate::Writable for I2C_WO {}
#[doc = "I2C Transmit"]
pub mod i2c_wo;
#[doc = "I2C Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_sts](i2c_sts) module"]
pub type I2C_STS = crate::Reg<u32, _I2C_STS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_STS;
#[doc = "`read()` method returns [i2c_sts::R](i2c_sts::R) reader structure"]
impl crate::Readable for I2C_STS {}
#[doc = "I2C Status"]
pub mod i2c_sts;
#[doc = "I2C Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ctl](i2c_ctl) module"]
pub type I2C_CTL = crate::Reg<u32, _I2C_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CTL;
#[doc = "`read()` method returns [i2c_ctl::R](i2c_ctl::R) reader structure"]
impl crate::Readable for I2C_CTL {}
#[doc = "`write(|w| ..)` method takes [i2c_ctl::W](i2c_ctl::W) writer structure"]
impl crate::Writable for I2C_CTL {}
#[doc = "I2C Control"]
pub mod i2c_ctl;
#[doc = "I2C Clock High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clkhi](i2c_clkhi) module"]
pub type I2C_CLKHI = crate::Reg<u32, _I2C_CLKHI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CLKHI;
#[doc = "`read()` method returns [i2c_clkhi::R](i2c_clkhi::R) reader structure"]
impl crate::Readable for I2C_CLKHI {}
#[doc = "`write(|w| ..)` method takes [i2c_clkhi::W](i2c_clkhi::W) writer structure"]
impl crate::Writable for I2C_CLKHI {}
#[doc = "I2C Clock High"]
pub mod i2c_clkhi;
#[doc = "I2C Clock Low\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_clklo](i2c_clklo) module"]
pub type I2C_CLKLO = crate::Reg<u32, _I2C_CLKLO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2C_CLKLO;
#[doc = "`write(|w| ..)` method takes [i2c_clklo::W](i2c_clklo::W) writer structure"]
impl crate::Writable for I2C_CLKLO {}
#[doc = "I2C Clock Low"]
pub mod i2c_clklo;
#[doc = "USB Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkctrl](usbclkctrl) module"]
pub type USBCLKCTRL = crate::Reg<u32, _USBCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKCTRL;
#[doc = "`read()` method returns [usbclkctrl::R](usbclkctrl::R) reader structure"]
impl crate::Readable for USBCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [usbclkctrl::W](usbclkctrl::W) writer structure"]
impl crate::Writable for USBCLKCTRL {}
#[doc = "USB Clock Control"]
pub mod usbclkctrl;
#[doc = "OTG clock controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgclkctrl](otgclkctrl) module"]
pub type OTGCLKCTRL = crate::Reg<u32, _OTGCLKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGCLKCTRL;
#[doc = "`read()` method returns [otgclkctrl::R](otgclkctrl::R) reader structure"]
impl crate::Readable for OTGCLKCTRL {}
#[doc = "`write(|w| ..)` method takes [otgclkctrl::W](otgclkctrl::W) writer structure"]
impl crate::Writable for OTGCLKCTRL {}
#[doc = "OTG clock controller"]
pub mod otgclkctrl;
#[doc = "USB Clock Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkst](usbclkst) module"]
pub type USBCLKST = crate::Reg<u32, _USBCLKST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBCLKST;
#[doc = "`read()` method returns [usbclkst::R](usbclkst::R) reader structure"]
impl crate::Readable for USBCLKST {}
#[doc = "USB Clock Status"]
pub mod usbclkst;
#[doc = "OTG clock status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgclkst](otgclkst) module"]
pub type OTGCLKST = crate::Reg<u32, _OTGCLKST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTGCLKST;
#[doc = "`read()` method returns [otgclkst::R](otgclkst::R) reader structure"]
impl crate::Readable for OTGCLKST {}
#[doc = "OTG clock status"]
pub mod otgclkst;
