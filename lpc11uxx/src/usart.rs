#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 4usize],
    _reserved_1_dlm: [u8; 4usize],
    _reserved_2_fcr: [u8; 4usize],
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: LCR,
    #[doc = "0x10 - Modem Control Register."]
    pub mcr: MCR,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: LSR,
    #[doc = "0x18 - Modem Status Register."]
    pub msr: MSR,
    #[doc = "0x1c - Scratch Pad Register. Eight-bit temporary storage for software."]
    pub scr: SCR,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: ACR,
    #[doc = "0x24 - IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
    pub icr: ICR,
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: FDR,
    #[doc = "0x2c - Oversampling Register. Controls the degree of oversampling during each bit time."]
    pub osr: OSR,
    #[doc = "0x30 - Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
    pub ter: TER,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - Half duplex enable register."]
    pub hden: HDEN,
    _reserved14: [u8; 4usize],
    #[doc = "0x48 - Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
    pub scictrl: SCICTRL,
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: RS485CTRL,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: RS485ADRMATCH,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: RS485DLY,
    #[doc = "0x58 - Synchronous mode control register."]
    pub syncctrl: SYNCCTRL,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dll(&self) -> &DLL {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DLL) }
    }
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dll_mut(&self) -> &mut DLL {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DLL) }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub fn thr(&self) -> &THR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const THR) }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub fn thr_mut(&self) -> &mut THR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut THR) }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub fn rbr(&self) -> &RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RBR) }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub fn rbr_mut(&self) -> &mut RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut RBR) }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub fn ier(&self) -> &IER {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const IER) }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub fn ier_mut(&self) -> &mut IER {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut IER) }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dlm(&self) -> &DLM {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const DLM) }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dlm_mut(&self) -> &mut DLM {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut DLM) }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub fn fcr(&self) -> &FCR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const FCR) }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub fn fcr_mut(&self) -> &mut FCR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut FCR) }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub fn iir(&self) -> &IIR {
        unsafe { &*(((self as *const Self) as *const u8).add(8usize) as *const IIR) }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub fn iir_mut(&self) -> &mut IIR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(8usize) as *mut IIR) }
    }
}
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbr](rbr) module"]
pub type RBR = crate::Reg<u32, _RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBR;
#[doc = "`read()` method returns [rbr::R](rbr::R) reader structure"]
impl crate::Readable for RBR {}
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
pub mod rbr;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
pub mod thr;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](dll) module"]
pub type DLL = crate::Reg<u32, _DLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLL;
#[doc = "`read()` method returns [dll::R](dll::R) reader structure"]
impl crate::Readable for DLL {}
#[doc = "`write(|w| ..)` method takes [dll::W](dll::W) writer structure"]
impl crate::Writable for DLL {}
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dll;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlm](dlm) module"]
pub type DLM = crate::Reg<u32, _DLM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLM;
#[doc = "`read()` method returns [dlm::R](dlm::R) reader structure"]
impl crate::Readable for DLM {}
#[doc = "`write(|w| ..)` method takes [dlm::W](dlm::W) writer structure"]
impl crate::Writable for DLM {}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dlm;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
pub mod ier;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iir](iir) module"]
pub type IIR = crate::Reg<u32, _IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IIR;
#[doc = "`read()` method returns [iir::R](iir::R) reader structure"]
impl crate::Readable for IIR {}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](fcr) module"]
pub type FCR = crate::Reg<u32, _FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCR;
#[doc = "`write(|w| ..)` method takes [fcr::W](fcr::W) writer structure"]
impl crate::Writable for FCR {}
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub mod fcr;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](lcr) module"]
pub type LCR = crate::Reg<u32, _LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCR;
#[doc = "`read()` method returns [lcr::R](lcr::R) reader structure"]
impl crate::Readable for LCR {}
#[doc = "`write(|w| ..)` method takes [lcr::W](lcr::W) writer structure"]
impl crate::Writable for LCR {}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "Modem Control Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "Modem Control Register."]
pub mod mcr;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsr](lsr) module"]
pub type LSR = crate::Reg<u32, _LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LSR;
#[doc = "`read()` method returns [lsr::R](lsr::R) reader structure"]
impl crate::Readable for LSR {}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "Modem Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msr](msr) module"]
pub type MSR = crate::Reg<u32, _MSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSR;
#[doc = "`read()` method returns [msr::R](msr::R) reader structure"]
impl crate::Readable for MSR {}
#[doc = "Modem Status Register."]
pub mod msr;
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](scr) module"]
pub type SCR = crate::Reg<u32, _SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCR;
#[doc = "`read()` method returns [scr::R](scr::R) reader structure"]
impl crate::Readable for SCR {}
#[doc = "`write(|w| ..)` method takes [scr::W](scr::W) writer structure"]
impl crate::Writable for SCR {}
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub mod scr;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](acr) module"]
pub type ACR = crate::Reg<u32, _ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR;
#[doc = "`read()` method returns [acr::R](acr::R) reader structure"]
impl crate::Readable for ACR {}
#[doc = "`write(|w| ..)` method takes [acr::W](acr::W) writer structure"]
impl crate::Writable for ACR {}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`read()` method returns [icr::R](icr::R) reader structure"]
impl crate::Readable for ICR {}
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
pub mod icr;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](fdr) module"]
pub type FDR = crate::Reg<u32, _FDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FDR;
#[doc = "`read()` method returns [fdr::R](fdr::R) reader structure"]
impl crate::Readable for FDR {}
#[doc = "`write(|w| ..)` method takes [fdr::W](fdr::W) writer structure"]
impl crate::Writable for FDR {}
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](osr) module"]
pub type OSR = crate::Reg<u32, _OSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSR;
#[doc = "`read()` method returns [osr::R](osr::R) reader structure"]
impl crate::Readable for OSR {}
#[doc = "`write(|w| ..)` method takes [osr::W](osr::W) writer structure"]
impl crate::Writable for OSR {}
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter](ter) module"]
pub type TER = crate::Reg<u32, _TER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TER;
#[doc = "`read()` method returns [ter::R](ter::R) reader structure"]
impl crate::Readable for TER {}
#[doc = "`write(|w| ..)` method takes [ter::W](ter::W) writer structure"]
impl crate::Writable for TER {}
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub mod ter;
#[doc = "Half duplex enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hden](hden) module"]
pub type HDEN = crate::Reg<u32, _HDEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HDEN;
#[doc = "`read()` method returns [hden::R](hden::R) reader structure"]
impl crate::Readable for HDEN {}
#[doc = "`write(|w| ..)` method takes [hden::W](hden::W) writer structure"]
impl crate::Writable for HDEN {}
#[doc = "Half duplex enable register."]
pub mod hden;
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scictrl](scictrl) module"]
pub type SCICTRL = crate::Reg<u32, _SCICTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCICTRL;
#[doc = "`read()` method returns [scictrl::R](scictrl::R) reader structure"]
impl crate::Readable for SCICTRL {}
#[doc = "`write(|w| ..)` method takes [scictrl::W](scictrl::W) writer structure"]
impl crate::Writable for SCICTRL {}
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
pub mod scictrl;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485ctrl](rs485ctrl) module"]
pub type RS485CTRL = crate::Reg<u32, _RS485CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RS485CTRL;
#[doc = "`read()` method returns [rs485ctrl::R](rs485ctrl::R) reader structure"]
impl crate::Readable for RS485CTRL {}
#[doc = "`write(|w| ..)` method takes [rs485ctrl::W](rs485ctrl::W) writer structure"]
impl crate::Writable for RS485CTRL {}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485adrmatch](rs485adrmatch) module"]
pub type RS485ADRMATCH = crate::Reg<u32, _RS485ADRMATCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RS485ADRMATCH;
#[doc = "`read()` method returns [rs485adrmatch::R](rs485adrmatch::R) reader structure"]
impl crate::Readable for RS485ADRMATCH {}
#[doc = "`write(|w| ..)` method takes [rs485adrmatch::W](rs485adrmatch::W) writer structure"]
impl crate::Writable for RS485ADRMATCH {}
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS-485/EIA-485 direction control delay.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rs485dly](rs485dly) module"]
pub type RS485DLY = crate::Reg<u32, _RS485DLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RS485DLY;
#[doc = "`read()` method returns [rs485dly::R](rs485dly::R) reader structure"]
impl crate::Readable for RS485DLY {}
#[doc = "`write(|w| ..)` method takes [rs485dly::W](rs485dly::W) writer structure"]
impl crate::Writable for RS485DLY {}
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "Synchronous mode control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncctrl](syncctrl) module"]
pub type SYNCCTRL = crate::Reg<u32, _SYNCCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCCTRL;
#[doc = "`read()` method returns [syncctrl::R](syncctrl::R) reader structure"]
impl crate::Readable for SYNCCTRL {}
#[doc = "`write(|w| ..)` method takes [syncctrl::W](syncctrl::W) writer structure"]
impl crate::Writable for SYNCCTRL {}
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
