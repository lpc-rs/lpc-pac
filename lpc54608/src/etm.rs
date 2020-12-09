#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Configuration Code Register"]
    pub ccr: CCR,
    #[doc = "0x08 - Trigger Event Register"]
    pub trigger: TRIGGER,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - ETM Status Register"]
    pub sr: SR,
    #[doc = "0x14 - System Configuration Register"]
    pub scr: SCR,
    _reserved5: [u8; 8usize],
    #[doc = "0x20 - Trace Enable Event Register"]
    pub eevr: EEVR,
    #[doc = "0x24 - Trace Enable Control 1 Register"]
    pub tecr1: TECR1,
    #[doc = "0x28 - FIFOFULL Level Register"]
    pub fflr: FFLR,
    _reserved8: [u8; 276usize],
    #[doc = "0x140 - Free-running counter reload value"]
    pub cntrldvr1: CNTRLDVR1,
    _reserved9: [u8; 156usize],
    #[doc = "0x1e0 - Synchronization Frequency Register"]
    pub syncfr: SYNCFR,
    #[doc = "0x1e4 - ID Register"]
    pub idr: IDR,
    #[doc = "0x1e8 - Configuration Code Extension Register"]
    pub ccer: CCER,
    _reserved12: [u8; 4usize],
    #[doc = "0x1f0 - TraceEnable Start/Stop EmbeddedICE Control Register"]
    pub tesseicr: TESSEICR,
    _reserved13: [u8; 4usize],
    #[doc = "0x1f8 - Timestamp Event Register"]
    pub tsevr: TSEVR,
    _reserved14: [u8; 4usize],
    #[doc = "0x200 - CoreSight Trace ID Register"]
    pub traceidr: TRACEIDR,
    _reserved15: [u8; 4usize],
    #[doc = "0x208 - ETM ID Register 2"]
    pub idr2: IDR2,
    _reserved16: [u8; 264usize],
    #[doc = "0x314 - Device Power-Down Status Register"]
    pub pdsr: PDSR,
    _reserved17: [u8; 3016usize],
    #[doc = "0xee0 - Integration Test Miscelaneous Inputs Register"]
    pub _itmiscin: _ITMISCIN,
    _reserved18: [u8; 4usize],
    #[doc = "0xee8 - Integration Test Trigger Out Register"]
    pub _ittrigout: _ITTRIGOUT,
    _reserved19: [u8; 4usize],
    #[doc = "0xef0 - ETM Integration Test ATB Control 2 Register"]
    pub _itatbctr2: _ITATBCTR2,
    _reserved20: [u8; 4usize],
    #[doc = "0xef8 - ETM Integration Test ATB Control 0 Register"]
    pub _itatbctr0: _ITATBCTR0,
    _reserved21: [u8; 4usize],
    #[doc = "0xf00 - Integration Mode Control Register"]
    pub itctrl: ITCTRL,
    _reserved22: [u8; 156usize],
    #[doc = "0xfa0 - Claim Tag Set Register"]
    pub claimset: CLAIMSET,
    #[doc = "0xfa4 - Claim Tag Clear Register"]
    pub claimclr: CLAIMCLR,
    _reserved24: [u8; 8usize],
    #[doc = "0xfb0 - Lock Access Register"]
    pub lar: LAR,
    #[doc = "0xfb4 - Lock Status Register"]
    pub lsr: LSR,
    #[doc = "0xfb8 - Authentication Status Register"]
    pub authstatus: AUTHSTATUS,
    _reserved27: [u8; 16usize],
    #[doc = "0xfcc - CoreSight Device Type Register"]
    pub devtype: DEVTYPE,
    #[doc = "0xfd0 - Peripheral Identification Register 4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Peripheral Identification Register 5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Peripheral Identification Register 6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Peripheral Identification Register 7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Peripheral Identification Register 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral Identification Register 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral Identification Register 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral Identification Register 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component Identification Register 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component Identification Register 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component Identification Register 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component Identification Register 3"]
    pub cidr3: CIDR3,
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Main Control Register"]
pub mod cr;
#[doc = "Configuration Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "Configuration Code Register"]
pub mod ccr;
#[doc = "Trigger Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trigger](trigger) module"]
pub type TRIGGER = crate::Reg<u32, _TRIGGER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRIGGER;
#[doc = "`read()` method returns [trigger::R](trigger::R) reader structure"]
impl crate::Readable for TRIGGER {}
#[doc = "`write(|w| ..)` method takes [trigger::W](trigger::W) writer structure"]
impl crate::Writable for TRIGGER {}
#[doc = "Trigger Event Register"]
pub mod trigger;
#[doc = "ETM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "`write(|w| ..)` method takes [sr::W](sr::W) writer structure"]
impl crate::Writable for SR {}
#[doc = "ETM Status Register"]
pub mod sr;
#[doc = "System Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "System Configuration Register"]
pub mod scr;
#[doc = "Trace Enable Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eevr](eevr) module"]
pub type EEVR = crate::Reg<u32, _EEVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEVR;
#[doc = "`read()` method returns [eevr::R](eevr::R) reader structure"]
impl crate::Readable for EEVR {}
#[doc = "`write(|w| ..)` method takes [eevr::W](eevr::W) writer structure"]
impl crate::Writable for EEVR {}
#[doc = "Trace Enable Event Register"]
pub mod eevr;
#[doc = "Trace Enable Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tecr1](tecr1) module"]
pub type TECR1 = crate::Reg<u32, _TECR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TECR1;
#[doc = "`read()` method returns [tecr1::R](tecr1::R) reader structure"]
impl crate::Readable for TECR1 {}
#[doc = "`write(|w| ..)` method takes [tecr1::W](tecr1::W) writer structure"]
impl crate::Writable for TECR1 {}
#[doc = "Trace Enable Control 1 Register"]
pub mod tecr1;
#[doc = "FIFOFULL Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fflr](fflr) module"]
pub type FFLR = crate::Reg<u32, _FFLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FFLR;
#[doc = "`read()` method returns [fflr::R](fflr::R) reader structure"]
impl crate::Readable for FFLR {}
#[doc = "`write(|w| ..)` method takes [fflr::W](fflr::W) writer structure"]
impl crate::Writable for FFLR {}
#[doc = "FIFOFULL Level Register"]
pub mod fflr;
#[doc = "Free-running counter reload value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrldvr1](cntrldvr1) module"]
pub type CNTRLDVR1 = crate::Reg<u32, _CNTRLDVR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTRLDVR1;
#[doc = "`read()` method returns [cntrldvr1::R](cntrldvr1::R) reader structure"]
impl crate::Readable for CNTRLDVR1 {}
#[doc = "`write(|w| ..)` method takes [cntrldvr1::W](cntrldvr1::W) writer structure"]
impl crate::Writable for CNTRLDVR1 {}
#[doc = "Free-running counter reload value"]
pub mod cntrldvr1;
#[doc = "Synchronization Frequency Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncfr](syncfr) module"]
pub type SYNCFR = crate::Reg<u32, _SYNCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCFR;
#[doc = "`read()` method returns [syncfr::R](syncfr::R) reader structure"]
impl crate::Readable for SYNCFR {}
#[doc = "Synchronization Frequency Register"]
pub mod syncfr;
#[doc = "ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "ID Register"]
pub mod idr;
#[doc = "Configuration Code Extension Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccer](ccer) module"]
pub type CCER = crate::Reg<u32, _CCER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCER;
#[doc = "`read()` method returns [ccer::R](ccer::R) reader structure"]
impl crate::Readable for CCER {}
#[doc = "Configuration Code Extension Register"]
pub mod ccer;
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tesseicr](tesseicr) module"]
pub type TESSEICR = crate::Reg<u32, _TESSEICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESSEICR;
#[doc = "`read()` method returns [tesseicr::R](tesseicr::R) reader structure"]
impl crate::Readable for TESSEICR {}
#[doc = "`write(|w| ..)` method takes [tesseicr::W](tesseicr::W) writer structure"]
impl crate::Writable for TESSEICR {}
#[doc = "TraceEnable Start/Stop EmbeddedICE Control Register"]
pub mod tesseicr;
#[doc = "Timestamp Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsevr](tsevr) module"]
pub type TSEVR = crate::Reg<u32, _TSEVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSEVR;
#[doc = "`read()` method returns [tsevr::R](tsevr::R) reader structure"]
impl crate::Readable for TSEVR {}
#[doc = "`write(|w| ..)` method takes [tsevr::W](tsevr::W) writer structure"]
impl crate::Writable for TSEVR {}
#[doc = "Timestamp Event Register"]
pub mod tsevr;
#[doc = "CoreSight Trace ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [traceidr](traceidr) module"]
pub type TRACEIDR = crate::Reg<u32, _TRACEIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TRACEIDR;
#[doc = "`read()` method returns [traceidr::R](traceidr::R) reader structure"]
impl crate::Readable for TRACEIDR {}
#[doc = "`write(|w| ..)` method takes [traceidr::W](traceidr::W) writer structure"]
impl crate::Writable for TRACEIDR {}
#[doc = "CoreSight Trace ID Register"]
pub mod traceidr;
#[doc = "ETM ID Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](idr2) module"]
pub type IDR2 = crate::Reg<u32, _IDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR2;
#[doc = "`read()` method returns [idr2::R](idr2::R) reader structure"]
impl crate::Readable for IDR2 {}
#[doc = "ETM ID Register 2"]
pub mod idr2;
#[doc = "Device Power-Down Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsr](pdsr) module"]
pub type PDSR = crate::Reg<u32, _PDSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDSR;
#[doc = "`read()` method returns [pdsr::R](pdsr::R) reader structure"]
impl crate::Readable for PDSR {}
#[doc = "Device Power-Down Status Register"]
pub mod pdsr;
#[doc = "Integration Test Miscelaneous Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_itmiscin](_itmiscin) module"]
pub type _ITMISCIN = crate::Reg<u32, __ITMISCIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __ITMISCIN;
#[doc = "`read()` method returns [_itmiscin::R](_itmiscin::R) reader structure"]
impl crate::Readable for _ITMISCIN {}
#[doc = "Integration Test Miscelaneous Inputs Register"]
pub mod _itmiscin;
#[doc = "Integration Test Trigger Out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_ittrigout](_ittrigout) module"]
pub type _ITTRIGOUT = crate::Reg<u32, __ITTRIGOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __ITTRIGOUT;
#[doc = "`read()` method returns [_ittrigout::R](_ittrigout::R) reader structure"]
impl crate::Readable for _ITTRIGOUT {}
#[doc = "`write(|w| ..)` method takes [_ittrigout::W](_ittrigout::W) writer structure"]
impl crate::Writable for _ITTRIGOUT {}
#[doc = "Integration Test Trigger Out Register"]
pub mod _ittrigout;
#[doc = "ETM Integration Test ATB Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_itatbctr2](_itatbctr2) module"]
pub type _ITATBCTR2 = crate::Reg<u32, __ITATBCTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __ITATBCTR2;
#[doc = "`read()` method returns [_itatbctr2::R](_itatbctr2::R) reader structure"]
impl crate::Readable for _ITATBCTR2 {}
#[doc = "ETM Integration Test ATB Control 2 Register"]
pub mod _itatbctr2;
#[doc = "ETM Integration Test ATB Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_itatbctr0](_itatbctr0) module"]
pub type _ITATBCTR0 = crate::Reg<u32, __ITATBCTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __ITATBCTR0;
#[doc = "`read()` method returns [_itatbctr0::R](_itatbctr0::R) reader structure"]
impl crate::Readable for _ITATBCTR0 {}
#[doc = "`write(|w| ..)` method takes [_itatbctr0::W](_itatbctr0::W) writer structure"]
impl crate::Writable for _ITATBCTR0 {}
#[doc = "ETM Integration Test ATB Control 0 Register"]
pub mod _itatbctr0;
#[doc = "Integration Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itctrl](itctrl) module"]
pub type ITCTRL = crate::Reg<u32, _ITCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCTRL;
#[doc = "`read()` method returns [itctrl::R](itctrl::R) reader structure"]
impl crate::Readable for ITCTRL {}
#[doc = "`write(|w| ..)` method takes [itctrl::W](itctrl::W) writer structure"]
impl crate::Writable for ITCTRL {}
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "Claim Tag Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimset](claimset) module"]
pub type CLAIMSET = crate::Reg<u32, _CLAIMSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMSET;
#[doc = "`read()` method returns [claimset::R](claimset::R) reader structure"]
impl crate::Readable for CLAIMSET {}
#[doc = "`write(|w| ..)` method takes [claimset::W](claimset::W) writer structure"]
impl crate::Writable for CLAIMSET {}
#[doc = "Claim Tag Set Register"]
pub mod claimset;
#[doc = "Claim Tag Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [claimclr](claimclr) module"]
pub type CLAIMCLR = crate::Reg<u32, _CLAIMCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLAIMCLR;
#[doc = "`read()` method returns [claimclr::R](claimclr::R) reader structure"]
impl crate::Readable for CLAIMCLR {}
#[doc = "`write(|w| ..)` method takes [claimclr::W](claimclr::W) writer structure"]
impl crate::Writable for CLAIMCLR {}
#[doc = "Claim Tag Clear Register"]
pub mod claimclr;
#[doc = "Lock Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lar](lar) module"]
pub type LAR = crate::Reg<u32, _LAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LAR;
#[doc = "`read()` method returns [lar::R](lar::R) reader structure"]
impl crate::Readable for LAR {}
#[doc = "`write(|w| ..)` method takes [lar::W](lar::W) writer structure"]
impl crate::Writable for LAR {}
#[doc = "Lock Access Register"]
pub mod lar;
#[doc = "Lock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "Lock Status Register"]
pub mod lsr;
#[doc = "Authentication Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [authstatus](authstatus) module"]
pub type AUTHSTATUS = crate::Reg<u32, _AUTHSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTHSTATUS;
#[doc = "`read()` method returns [authstatus::R](authstatus::R) reader structure"]
impl crate::Readable for AUTHSTATUS {}
#[doc = "Authentication Status Register"]
pub mod authstatus;
#[doc = "CoreSight Device Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](devtype) module"]
pub type DEVTYPE = crate::Reg<u32, _DEVTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVTYPE;
#[doc = "`read()` method returns [devtype::R](devtype::R) reader structure"]
impl crate::Readable for DEVTYPE {}
#[doc = "CoreSight Device Type Register"]
pub mod devtype;
#[doc = "Peripheral Identification Register 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral Identification Register 4"]
pub mod pidr4;
#[doc = "Peripheral Identification Register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "Peripheral Identification Register 5"]
pub mod pidr5;
#[doc = "Peripheral Identification Register 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "Peripheral Identification Register 6"]
pub mod pidr6;
#[doc = "Peripheral Identification Register 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "Peripheral Identification Register 7"]
pub mod pidr7;
#[doc = "Peripheral Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Peripheral Identification Register 0"]
pub mod pidr0;
#[doc = "Peripheral Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Peripheral Identification Register 1"]
pub mod pidr1;
#[doc = "Peripheral Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Peripheral Identification Register 2"]
pub mod pidr2;
#[doc = "Peripheral Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Peripheral Identification Register 3"]
pub mod pidr3;
#[doc = "Component Identification Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component Identification Register 0"]
pub mod cidr0;
#[doc = "Component Identification Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component Identification Register 1"]
pub mod cidr1;
#[doc = "Component Identification Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component Identification Register 2"]
pub mod cidr2;
#[doc = "Component Identification Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component Identification Register 3"]
pub mod cidr3;
