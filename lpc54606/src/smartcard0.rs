#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 4usize],
    _reserved_1_dlm: [u8; 4usize],
    _reserved_2_fcr: [u8; 4usize],
    #[doc = "0x0c - Line Control Register"]
    pub lcr: LCR,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Line Status Register"]
    pub lsr: LSR,
    _reserved5: [u8; 4usize],
    #[doc = "0x1c - Scratch Pad Register"]
    pub scr: SCR,
    _reserved6: [u8; 12usize],
    #[doc = "0x2c - Oversampling register"]
    pub osr: OSR,
    _reserved7: [u8; 24usize],
    #[doc = "0x48 - Smart Card Interface control register"]
    pub scictrl: SCICTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn thr(&self) -> &THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const THR) }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub fn thr_mut(&self) -> &mut THR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut THR) }
    }
    #[doc = "0x00 - Receiver Buffer Register"]
    #[inline(always)]
    pub fn rbr(&self) -> &RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR) }
    }
    #[doc = "0x00 - Receiver Buffer Register"]
    #[inline(always)]
    pub fn rbr_mut(&self) -> &mut RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut RBR) }
    }
    #[doc = "0x00 - Divisor Latch LSB"]
    #[inline(always)]
    pub fn dll(&self) -> &DLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DLL) }
    }
    #[doc = "0x00 - Divisor Latch LSB"]
    #[inline(always)]
    pub fn dll_mut(&self) -> &mut DLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DLL) }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const IER) }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut IER) }
    }
    #[doc = "0x04 - Divisor Latch MSB"]
    #[inline(always)]
    pub fn dlm(&self) -> &DLM {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM) }
    }
    #[doc = "0x04 - Divisor Latch MSB"]
    #[inline(always)]
    pub fn dlm_mut(&self) -> &mut DLM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut DLM) }
    }
    #[doc = "0x08 - Interrupt ID Register"]
    #[inline(always)]
    pub fn iir(&self) -> &IIR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR) }
    }
    #[doc = "0x08 - Interrupt ID Register"]
    #[inline(always)]
    pub fn iir_mut(&self) -> &mut IIR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IIR) }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn fcr(&self) -> &FCR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const FCR) }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub fn fcr_mut(&self) -> &mut FCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut FCR) }
    }
}
#[doc = "Divisor Latch LSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](dll) module"]
pub type DLL = crate::Reg<u32, _DLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLL;
#[doc = "`read()` method returns [dll::R](dll::R) reader structure"]
impl crate::Readable for DLL {}
#[doc = "`write(|w| ..)` method takes [dll::W](dll::W) writer structure"]
impl crate::Writable for DLL {}
#[doc = "Divisor Latch LSB"]
pub mod dll;
#[doc = "Receiver Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr](rbr) module"]
pub type RBR = crate::Reg<u32, _RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBR;
#[doc = "`read()` method returns [rbr::R](rbr::R) reader structure"]
impl crate::Readable for RBR {}
#[doc = "Receiver Buffer Register"]
pub mod rbr;
#[doc = "Transmit Holding Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Divisor Latch MSB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm](dlm) module"]
pub type DLM = crate::Reg<u32, _DLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLM;
#[doc = "`read()` method returns [dlm::R](dlm::R) reader structure"]
impl crate::Readable for DLM {}
#[doc = "`write(|w| ..)` method takes [dlm::W](dlm::W) writer structure"]
impl crate::Writable for DLM {}
#[doc = "Divisor Latch MSB"]
pub mod dlm;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "FIFO Control Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "Interrupt ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](iir) module"]
pub type IIR = crate::Reg<u32, _IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIR;
#[doc = "`read()` method returns [iir::R](iir::R) reader structure"]
impl crate::Readable for IIR {}
#[doc = "Interrupt ID Register"]
pub mod iir;
#[doc = "Line Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](lcr) module"]
pub type LCR = crate::Reg<u32, _LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCR;
#[doc = "`read()` method returns [lcr::R](lcr::R) reader structure"]
impl crate::Readable for LCR {}
#[doc = "`write(|w| ..)` method takes [lcr::W](lcr::W) writer structure"]
impl crate::Writable for LCR {}
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "Line Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "Scratch Pad Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Scratch Pad Register"]
pub mod scr;
#[doc = "Oversampling register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](osr) module"]
pub type OSR = crate::Reg<u32, _OSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSR;
#[doc = "`read()` method returns [osr::R](osr::R) reader structure"]
impl crate::Readable for OSR {}
#[doc = "`write(|w| ..)` method takes [osr::W](osr::W) writer structure"]
impl crate::Writable for OSR {}
#[doc = "Oversampling register"]
pub mod osr;
#[doc = "Smart Card Interface control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](scictrl) module"]
pub type SCICTRL = crate::Reg<u32, _SCICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCICTRL;
#[doc = "`read()` method returns [scictrl::R](scictrl::R) reader structure"]
impl crate::Readable for SCICTRL {}
#[doc = "`write(|w| ..)` method takes [scictrl::W](scictrl::W) writer structure"]
impl crate::Writable for SCICTRL {}
#[doc = "Smart Card Interface control register"]
pub mod scictrl;
