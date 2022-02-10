///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_rbr_thr_dll: [u8; 0x04],
    _reserved_1_dlm_ier: [u8; 0x04],
    _reserved_2_iir_fcr: [u8; 0x04],
    ///0x0c - Line Control Register
    pub lcr: crate::Reg<lcr::LCR_SPEC>,
    _reserved4: [u8; 0x04],
    ///0x14 - Line Status Register
    pub lsr: crate::Reg<lsr::LSR_SPEC>,
    _reserved5: [u8; 0x04],
    ///0x1c - Scratch Pad Register
    pub scr: crate::Reg<scr::SCR_SPEC>,
    _reserved6: [u8; 0x0c],
    ///0x2c - Oversampling register
    pub osr: crate::Reg<osr::OSR_SPEC>,
    _reserved7: [u8; 0x18],
    ///0x48 - Smart Card Interface control register
    pub scictrl: crate::Reg<scictrl::SCICTRL_SPEC>,
}
impl RegisterBlock {
    ///0x00 - Transmit Holding Register
    #[inline(always)]
    pub fn rbr_thr_dll_thr(&self) -> &crate::Reg<rbr_thr_dll_thr::RBR_THR_DLL_THR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<rbr_thr_dll_thr::RBR_THR_DLL_THR_SPEC>)
        }
    }
    ///0x00 - Receiver Buffer Register
    #[inline(always)]
    pub fn rbr_thr_dll_rbr(&self) -> &crate::Reg<rbr_thr_dll_rbr::RBR_THR_DLL_RBR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<rbr_thr_dll_rbr::RBR_THR_DLL_RBR_SPEC>)
        }
    }
    ///0x00 - Divisor Latch LSB
    #[inline(always)]
    pub fn rbr_thr_dll_dll(&self) -> &crate::Reg<rbr_thr_dll_dll::RBR_THR_DLL_DLL_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(0usize)
                as *const crate::Reg<rbr_thr_dll_dll::RBR_THR_DLL_DLL_SPEC>)
        }
    }
    ///0x04 - Interrupt Enable Register
    #[inline(always)]
    pub fn dlm_ier_ier(&self) -> &crate::Reg<dlm_ier_ier::DLM_IER_IER_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<dlm_ier_ier::DLM_IER_IER_SPEC>)
        }
    }
    ///0x04 - Divisor Latch MSB
    #[inline(always)]
    pub fn dlm_ier_dlm(&self) -> &crate::Reg<dlm_ier_dlm::DLM_IER_DLM_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(4usize)
                as *const crate::Reg<dlm_ier_dlm::DLM_IER_DLM_SPEC>)
        }
    }
    ///0x08 - Interrupt ID Register
    #[inline(always)]
    pub fn iir_fcr_iir(&self) -> &crate::Reg<iir_fcr_iir::IIR_FCR_IIR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<iir_fcr_iir::IIR_FCR_IIR_SPEC>)
        }
    }
    ///0x08 - FIFO Control Register
    #[inline(always)]
    pub fn iir_fcr_fcr(&self) -> &crate::Reg<iir_fcr_fcr::IIR_FCR_FCR_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(8usize)
                as *const crate::Reg<iir_fcr_fcr::IIR_FCR_FCR_SPEC>)
        }
    }
}
///RBR_THR_DLL_DLL register accessor: an alias for `Reg<RBR_THR_DLL_DLL_SPEC>`
pub type RBR_THR_DLL_DLL = crate::Reg<rbr_thr_dll_dll::RBR_THR_DLL_DLL_SPEC>;
///Divisor Latch LSB
pub mod rbr_thr_dll_dll;
///RBR_THR_DLL_RBR register accessor: an alias for `Reg<RBR_THR_DLL_RBR_SPEC>`
pub type RBR_THR_DLL_RBR = crate::Reg<rbr_thr_dll_rbr::RBR_THR_DLL_RBR_SPEC>;
///Receiver Buffer Register
pub mod rbr_thr_dll_rbr;
///RBR_THR_DLL_THR register accessor: an alias for `Reg<RBR_THR_DLL_THR_SPEC>`
pub type RBR_THR_DLL_THR = crate::Reg<rbr_thr_dll_thr::RBR_THR_DLL_THR_SPEC>;
///Transmit Holding Register
pub mod rbr_thr_dll_thr;
///DLM_IER_DLM register accessor: an alias for `Reg<DLM_IER_DLM_SPEC>`
pub type DLM_IER_DLM = crate::Reg<dlm_ier_dlm::DLM_IER_DLM_SPEC>;
///Divisor Latch MSB
pub mod dlm_ier_dlm;
///DLM_IER_IER register accessor: an alias for `Reg<DLM_IER_IER_SPEC>`
pub type DLM_IER_IER = crate::Reg<dlm_ier_ier::DLM_IER_IER_SPEC>;
///Interrupt Enable Register
pub mod dlm_ier_ier;
///IIR_FCR_FCR register accessor: an alias for `Reg<IIR_FCR_FCR_SPEC>`
pub type IIR_FCR_FCR = crate::Reg<iir_fcr_fcr::IIR_FCR_FCR_SPEC>;
///FIFO Control Register
pub mod iir_fcr_fcr;
///IIR_FCR_IIR register accessor: an alias for `Reg<IIR_FCR_IIR_SPEC>`
pub type IIR_FCR_IIR = crate::Reg<iir_fcr_iir::IIR_FCR_IIR_SPEC>;
///Interrupt ID Register
pub mod iir_fcr_iir;
///LCR register accessor: an alias for `Reg<LCR_SPEC>`
pub type LCR = crate::Reg<lcr::LCR_SPEC>;
///Line Control Register
pub mod lcr;
///LSR register accessor: an alias for `Reg<LSR_SPEC>`
pub type LSR = crate::Reg<lsr::LSR_SPEC>;
///Line Status Register
pub mod lsr;
///SCR register accessor: an alias for `Reg<SCR_SPEC>`
pub type SCR = crate::Reg<scr::SCR_SPEC>;
///Scratch Pad Register
pub mod scr;
///OSR register accessor: an alias for `Reg<OSR_SPEC>`
pub type OSR = crate::Reg<osr::OSR_SPEC>;
///Oversampling register
pub mod osr;
///SCICTRL register accessor: an alias for `Reg<SCICTRL_SPEC>`
pub type SCICTRL = crate::Reg<scictrl::SCICTRL_SPEC>;
///Smart Card Interface control register
pub mod scictrl;
