#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 4usize],
    _reserved_1_dlm: [u8; 4usize],
    _reserved_2_fcr: [u8; 4usize],
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    #[doc = "0x10 - Modem Control Register."]
    pub mcr: crate::Reg<mcr::MCR_SPEC>,
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    #[doc = "0x18 - Modem Status Register."]
    pub msr: crate::Reg<msr::MSR_SPEC>,
    #[doc = "0x1c - Scratch Pad Register. Eight-bit temporary storage for software."]
    pub scr: crate::Reg<scr::SCR_SPEC>,
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    pub acr: crate::Reg<acr::ACR_SPEC>,
    #[doc = "0x24 - IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    pub fdr: crate::Reg<fdr::FDR_SPEC>,
    #[doc = "0x2c - Oversampling Register. Controls the degree of oversampling during each bit time."]
    pub osr: crate::Reg<osr::OSR_SPEC>,
    #[doc = "0x30 - Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
    pub ter: crate::Reg<ter::TER_SPEC>,
    _reserved13: [u8; 12usize],
    #[doc = "0x40 - Half duplex enable register."]
    pub hden: crate::Reg<hden::HDEN_SPEC>,
    _reserved14: [u8; 4usize],
    #[doc = "0x48 - Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
    pub scictrl: crate::Reg<scictrl::SCICTRL_SPEC>,
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    pub rs485ctrl: crate::Reg<rs485ctrl::RS485CTRL_SPEC>,
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    pub rs485adrmatch: crate::Reg<rs485adrmatch::RS485ADRMATCH_SPEC>,
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    pub rs485dly: crate::Reg<rs485dly::RS485DLY_SPEC>,
    #[doc = "0x58 - Synchronous mode control register."]
    pub syncctrl: crate::Reg<syncctrl::SYNCCTRL_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dll(&self) -> &crate::Reg<dll::DLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<dll::DLL_SPEC>)
        }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
    #[inline(always)]
    pub fn thr(&self) -> &crate::Reg<thr::THR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<thr::THR_SPEC>)
        }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
    #[inline(always)]
    pub fn rbr(&self) -> &crate::Reg<rbr::RBR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize) as *const crate::Reg<rbr::RBR_SPEC>)
        }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
    #[inline(always)]
    pub fn ier(&self) -> &crate::Reg<ier::IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<ier::IER_SPEC>)
        }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
    #[inline(always)]
    pub fn dlm(&self) -> &crate::Reg<dlm::DLM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize) as *const crate::Reg<dlm::DLM_SPEC>)
        }
    }
    #[doc = "0x08 - FIFO Control Register. Controls USART FIFO usage and modes."]
    #[inline(always)]
    pub fn fcr(&self) -> &crate::Reg<fcr::FCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<fcr::FCR_SPEC>)
        }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub fn iir(&self) -> &crate::Reg<iir::IIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize) as *const crate::Reg<iir::IIR_SPEC>)
        }
    }
}
#[doc = "RBR register accessor: an alias for `Reg<RBR_SPEC>`"]
pub type RBR = crate::Reg<rbr::RBR_SPEC>;
#[doc = "Receiver Buffer Register. Contains the next received character to be read. (DLAB=0)"]
pub mod rbr;
#[doc = "THR register accessor: an alias for `Reg<THR_SPEC>`"]
pub type THR = crate::Reg<thr::THR_SPEC>;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here. (DLAB=0)"]
pub mod thr;
#[doc = "DLL register accessor: an alias for `Reg<DLL_SPEC>`"]
pub type DLL = crate::Reg<dll::DLL_SPEC>;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dll;
#[doc = "DLM register accessor: an alias for `Reg<DLM_SPEC>`"]
pub type DLM = crate::Reg<dlm::DLM_SPEC>;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider. (DLAB=1)"]
pub mod dlm;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential USART interrupts. (DLAB=0)"]
pub mod ier;
#[doc = "IIR register accessor: an alias for `Reg<IIR_SPEC>`"]
pub type IIR = crate::Reg<iir::IIR_SPEC>;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FCR register accessor: an alias for `Reg<FCR_SPEC>`"]
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes."]
pub mod fcr;
#[doc = "LCR register accessor: an alias for `Reg<LCR_SPEC>`"]
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "MCR register accessor: an alias for `Reg<MCR_SPEC>`"]
pub type MCR = crate::Reg<mcr::MCR_SPEC>;
#[doc = "Modem Control Register."]
pub mod mcr;
#[doc = "LSR register accessor: an alias for `Reg<LSR_SPEC>`"]
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "MSR register accessor: an alias for `Reg<MSR_SPEC>`"]
pub type MSR = crate::Reg<msr::MSR_SPEC>;
#[doc = "Modem Status Register."]
pub mod msr;
#[doc = "SCR register accessor: an alias for `Reg<SCR_SPEC>`"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Scratch Pad Register. Eight-bit temporary storage for software."]
pub mod scr;
#[doc = "ACR register accessor: an alias for `Reg<ACR_SPEC>`"]
pub type ACR = crate::Reg<acr::ACR_SPEC>;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "IrDA Control Register. Enables and configures the IrDA (remote control) mode."]
pub mod icr;
#[doc = "FDR register accessor: an alias for `Reg<FDR_SPEC>`"]
pub type FDR = crate::Reg<fdr::FDR_SPEC>;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "OSR register accessor: an alias for `Reg<OSR_SPEC>`"]
pub type OSR = crate::Reg<osr::OSR_SPEC>;
#[doc = "Oversampling Register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "TER register accessor: an alias for `Reg<TER_SPEC>`"]
pub type TER = crate::Reg<ter::TER_SPEC>;
#[doc = "Transmit Enable Register. Turns off USART transmitter for use with software flow control."]
pub mod ter;
#[doc = "HDEN register accessor: an alias for `Reg<HDEN_SPEC>`"]
pub type HDEN = crate::Reg<hden::HDEN_SPEC>;
#[doc = "Half duplex enable register."]
pub mod hden;
#[doc = "SCICTRL register accessor: an alias for `Reg<SCICTRL_SPEC>`"]
pub type SCICTRL = crate::Reg<scictrl::SCICTRL_SPEC>;
#[doc = "Smart Card Interface Control register. Enables and configures the Smart Card Interface feature."]
pub mod scictrl;
#[doc = "RS485CTRL register accessor: an alias for `Reg<RS485CTRL_SPEC>`"]
pub type RS485CTRL = crate::Reg<rs485ctrl::RS485CTRL_SPEC>;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS485ADRMATCH register accessor: an alias for `Reg<RS485ADRMATCH_SPEC>`"]
pub type RS485ADRMATCH = crate::Reg<rs485adrmatch::RS485ADRMATCH_SPEC>;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS485DLY register accessor: an alias for `Reg<RS485DLY_SPEC>`"]
pub type RS485DLY = crate::Reg<rs485dly::RS485DLY_SPEC>;
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "SYNCCTRL register accessor: an alias for `Reg<SYNCCTRL_SPEC>`"]
pub type SYNCCTRL = crate::Reg<syncctrl::SYNCCTRL_SPEC>;
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
