///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    ///0x0c - Data Bit Timing Prescaler Register
    pub dbtp: crate::Reg<dbtp::DBTP_SPEC>,
    ///0x10 - Test Register
    pub test: crate::Reg<test::TEST_SPEC>,
    _reserved2: [u8; 0x04],
    ///0x18 - CC Control Register
    pub cccr: crate::Reg<cccr::CCCR_SPEC>,
    ///0x1c - Nominal Bit Timing and Prescaler Register
    pub nbtp: crate::Reg<nbtp::NBTP_SPEC>,
    ///0x20 - Timestamp Counter Configuration
    pub tscc: crate::Reg<tscc::TSCC_SPEC>,
    ///0x24 - Timestamp Counter Value
    pub tscv: crate::Reg<tscv::TSCV_SPEC>,
    ///0x28 - Timeout Counter Configuration
    pub tocc: crate::Reg<tocc::TOCC_SPEC>,
    ///0x2c - Timeout Counter Value
    pub tocv: crate::Reg<tocv::TOCV_SPEC>,
    _reserved8: [u8; 0x10],
    ///0x40 - Error Counter Register
    pub ecr: crate::Reg<ecr::ECR_SPEC>,
    ///0x44 - Protocol Status Register
    pub psr: crate::Reg<psr::PSR_SPEC>,
    ///0x48 - Transmitter Delay Compensator Register
    pub tdcr: crate::Reg<tdcr::TDCR_SPEC>,
    _reserved11: [u8; 0x04],
    ///0x50 - Interrupt Register
    pub ir: crate::Reg<ir::IR_SPEC>,
    ///0x54 - Interrupt Enable
    pub ie: crate::Reg<ie::IE_SPEC>,
    ///0x58 - Interrupt Line Select
    pub ils: crate::Reg<ils::ILS_SPEC>,
    ///0x5c - Interrupt Line Enable
    pub ile: crate::Reg<ile::ILE_SPEC>,
    _reserved15: [u8; 0x20],
    ///0x80 - Global Filter Configuration
    pub gfc: crate::Reg<gfc::GFC_SPEC>,
    ///0x84 - Standard ID Filter Configuration
    pub sidfc: crate::Reg<sidfc::SIDFC_SPEC>,
    ///0x88 - Extended ID Filter Configuration
    pub xidfc: crate::Reg<xidfc::XIDFC_SPEC>,
    _reserved18: [u8; 0x04],
    ///0x90 - Extended ID AND Mask
    pub xidam: crate::Reg<xidam::XIDAM_SPEC>,
    ///0x94 - High Priority Message Status
    pub hpms: crate::Reg<hpms::HPMS_SPEC>,
    ///0x98 - New Data 1
    pub ndat1: crate::Reg<ndat1::NDAT1_SPEC>,
    ///0x9c - New Data 2
    pub ndat2: crate::Reg<ndat2::NDAT2_SPEC>,
    ///0xa0 - Rx FIFO 0 Configuration
    pub rxf0c: crate::Reg<rxf0c::RXF0C_SPEC>,
    ///0xa4 - Rx FIFO 0 Status
    pub rxf0s: crate::Reg<rxf0s::RXF0S_SPEC>,
    ///0xa8 - Rx FIFO 0 Acknowledge
    pub rxf0a: crate::Reg<rxf0a::RXF0A_SPEC>,
    ///0xac - Rx Buffer Configuration
    pub rxbc: crate::Reg<rxbc::RXBC_SPEC>,
    ///0xb0 - Rx FIFO 1 Configuration
    pub rxf1c: crate::Reg<rxf1c::RXF1C_SPEC>,
    ///0xb4 - Rx FIFO 1 Status
    pub rxf1s: crate::Reg<rxf1s::RXF1S_SPEC>,
    ///0xb8 - Rx FIFO 1 Acknowledge
    pub rxf1a: crate::Reg<rxf1a::RXF1A_SPEC>,
    ///0xbc - Rx Buffer and FIFO Element Size Configuration
    pub rxesc: crate::Reg<rxesc::RXESC_SPEC>,
    ///0xc0 - Tx Buffer Configuration
    pub txbc: crate::Reg<txbc::TXBC_SPEC>,
    ///0xc4 - Tx FIFO/Queue Status
    pub txfqs: crate::Reg<txfqs::TXFQS_SPEC>,
    ///0xc8 - Tx Buffer Element Size Configuration
    pub txesc: crate::Reg<txesc::TXESC_SPEC>,
    ///0xcc - Tx Buffer Request Pending
    pub txbrp: crate::Reg<txbrp::TXBRP_SPEC>,
    ///0xd0 - Tx Buffer Add Request
    pub txbar: crate::Reg<txbar::TXBAR_SPEC>,
    ///0xd4 - Tx Buffer Cancellation Request
    pub txbcr: crate::Reg<txbcr::TXBCR_SPEC>,
    ///0xd8 - Tx Buffer Transmission Occurred
    pub txbto: crate::Reg<txbto::TXBTO_SPEC>,
    ///0xdc - Tx Buffer Cancellation Finished
    pub txbcf: crate::Reg<txbcf::TXBCF_SPEC>,
    ///0xe0 - Tx Buffer Transmission Interrupt Enable
    pub txbtie: crate::Reg<txbtie::TXBTIE_SPEC>,
    ///0xe4 - Tx Buffer Cancellation Finished Interrupt Enable
    pub txbcie: crate::Reg<txbcie::TXBCIE_SPEC>,
    _reserved40: [u8; 0x08],
    ///0xf0 - Tx Event FIFO Configuration
    pub txefc: crate::Reg<txefc::TXEFC_SPEC>,
    ///0xf4 - Tx Event FIFO Status
    pub txefs: crate::Reg<txefs::TXEFS_SPEC>,
    ///0xf8 - Tx Event FIFO Acknowledge
    pub txefa: crate::Reg<txefa::TXEFA_SPEC>,
    _reserved43: [u8; 0x0104],
    ///0x200 - CAN Message RAM Base Address
    pub mrba: crate::Reg<mrba::MRBA_SPEC>,
    _reserved44: [u8; 0x01fc],
    ///0x400 - External Timestamp Counter Configuration
    pub etscc: crate::Reg<etscc::ETSCC_SPEC>,
    _reserved45: [u8; 0x01fc],
    ///0x600 - External Timestamp Counter Value
    pub etscv: crate::Reg<etscv::ETSCV_SPEC>,
}
///DBTP register accessor: an alias for `Reg<DBTP_SPEC>`
pub type DBTP = crate::Reg<dbtp::DBTP_SPEC>;
///Data Bit Timing Prescaler Register
pub mod dbtp;
///TEST register accessor: an alias for `Reg<TEST_SPEC>`
pub type TEST = crate::Reg<test::TEST_SPEC>;
///Test Register
pub mod test;
///CCCR register accessor: an alias for `Reg<CCCR_SPEC>`
pub type CCCR = crate::Reg<cccr::CCCR_SPEC>;
///CC Control Register
pub mod cccr;
///NBTP register accessor: an alias for `Reg<NBTP_SPEC>`
pub type NBTP = crate::Reg<nbtp::NBTP_SPEC>;
///Nominal Bit Timing and Prescaler Register
pub mod nbtp;
///TSCC register accessor: an alias for `Reg<TSCC_SPEC>`
pub type TSCC = crate::Reg<tscc::TSCC_SPEC>;
///Timestamp Counter Configuration
pub mod tscc;
///TSCV register accessor: an alias for `Reg<TSCV_SPEC>`
pub type TSCV = crate::Reg<tscv::TSCV_SPEC>;
///Timestamp Counter Value
pub mod tscv;
///TOCC register accessor: an alias for `Reg<TOCC_SPEC>`
pub type TOCC = crate::Reg<tocc::TOCC_SPEC>;
///Timeout Counter Configuration
pub mod tocc;
///TOCV register accessor: an alias for `Reg<TOCV_SPEC>`
pub type TOCV = crate::Reg<tocv::TOCV_SPEC>;
///Timeout Counter Value
pub mod tocv;
///ECR register accessor: an alias for `Reg<ECR_SPEC>`
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
///Error Counter Register
pub mod ecr;
///PSR register accessor: an alias for `Reg<PSR_SPEC>`
pub type PSR = crate::Reg<psr::PSR_SPEC>;
///Protocol Status Register
pub mod psr;
///TDCR register accessor: an alias for `Reg<TDCR_SPEC>`
pub type TDCR = crate::Reg<tdcr::TDCR_SPEC>;
///Transmitter Delay Compensator Register
pub mod tdcr;
///IR register accessor: an alias for `Reg<IR_SPEC>`
pub type IR = crate::Reg<ir::IR_SPEC>;
///Interrupt Register
pub mod ir;
///IE register accessor: an alias for `Reg<IE_SPEC>`
pub type IE = crate::Reg<ie::IE_SPEC>;
///Interrupt Enable
pub mod ie;
///ILS register accessor: an alias for `Reg<ILS_SPEC>`
pub type ILS = crate::Reg<ils::ILS_SPEC>;
///Interrupt Line Select
pub mod ils;
///ILE register accessor: an alias for `Reg<ILE_SPEC>`
pub type ILE = crate::Reg<ile::ILE_SPEC>;
///Interrupt Line Enable
pub mod ile;
///GFC register accessor: an alias for `Reg<GFC_SPEC>`
pub type GFC = crate::Reg<gfc::GFC_SPEC>;
///Global Filter Configuration
pub mod gfc;
///SIDFC register accessor: an alias for `Reg<SIDFC_SPEC>`
pub type SIDFC = crate::Reg<sidfc::SIDFC_SPEC>;
///Standard ID Filter Configuration
pub mod sidfc;
///XIDFC register accessor: an alias for `Reg<XIDFC_SPEC>`
pub type XIDFC = crate::Reg<xidfc::XIDFC_SPEC>;
///Extended ID Filter Configuration
pub mod xidfc;
///XIDAM register accessor: an alias for `Reg<XIDAM_SPEC>`
pub type XIDAM = crate::Reg<xidam::XIDAM_SPEC>;
///Extended ID AND Mask
pub mod xidam;
///HPMS register accessor: an alias for `Reg<HPMS_SPEC>`
pub type HPMS = crate::Reg<hpms::HPMS_SPEC>;
///High Priority Message Status
pub mod hpms;
///NDAT1 register accessor: an alias for `Reg<NDAT1_SPEC>`
pub type NDAT1 = crate::Reg<ndat1::NDAT1_SPEC>;
///New Data 1
pub mod ndat1;
///NDAT2 register accessor: an alias for `Reg<NDAT2_SPEC>`
pub type NDAT2 = crate::Reg<ndat2::NDAT2_SPEC>;
///New Data 2
pub mod ndat2;
///RXF0C register accessor: an alias for `Reg<RXF0C_SPEC>`
pub type RXF0C = crate::Reg<rxf0c::RXF0C_SPEC>;
///Rx FIFO 0 Configuration
pub mod rxf0c;
///RXF0S register accessor: an alias for `Reg<RXF0S_SPEC>`
pub type RXF0S = crate::Reg<rxf0s::RXF0S_SPEC>;
///Rx FIFO 0 Status
pub mod rxf0s;
///RXF0A register accessor: an alias for `Reg<RXF0A_SPEC>`
pub type RXF0A = crate::Reg<rxf0a::RXF0A_SPEC>;
///Rx FIFO 0 Acknowledge
pub mod rxf0a;
///RXBC register accessor: an alias for `Reg<RXBC_SPEC>`
pub type RXBC = crate::Reg<rxbc::RXBC_SPEC>;
///Rx Buffer Configuration
pub mod rxbc;
///RXF1C register accessor: an alias for `Reg<RXF1C_SPEC>`
pub type RXF1C = crate::Reg<rxf1c::RXF1C_SPEC>;
///Rx FIFO 1 Configuration
pub mod rxf1c;
///RXF1S register accessor: an alias for `Reg<RXF1S_SPEC>`
pub type RXF1S = crate::Reg<rxf1s::RXF1S_SPEC>;
///Rx FIFO 1 Status
pub mod rxf1s;
///RXF1A register accessor: an alias for `Reg<RXF1A_SPEC>`
pub type RXF1A = crate::Reg<rxf1a::RXF1A_SPEC>;
///Rx FIFO 1 Acknowledge
pub mod rxf1a;
///RXESC register accessor: an alias for `Reg<RXESC_SPEC>`
pub type RXESC = crate::Reg<rxesc::RXESC_SPEC>;
///Rx Buffer and FIFO Element Size Configuration
pub mod rxesc;
///TXBC register accessor: an alias for `Reg<TXBC_SPEC>`
pub type TXBC = crate::Reg<txbc::TXBC_SPEC>;
///Tx Buffer Configuration
pub mod txbc;
///TXFQS register accessor: an alias for `Reg<TXFQS_SPEC>`
pub type TXFQS = crate::Reg<txfqs::TXFQS_SPEC>;
///Tx FIFO/Queue Status
pub mod txfqs;
///TXESC register accessor: an alias for `Reg<TXESC_SPEC>`
pub type TXESC = crate::Reg<txesc::TXESC_SPEC>;
///Tx Buffer Element Size Configuration
pub mod txesc;
///TXBRP register accessor: an alias for `Reg<TXBRP_SPEC>`
pub type TXBRP = crate::Reg<txbrp::TXBRP_SPEC>;
///Tx Buffer Request Pending
pub mod txbrp;
///TXBAR register accessor: an alias for `Reg<TXBAR_SPEC>`
pub type TXBAR = crate::Reg<txbar::TXBAR_SPEC>;
///Tx Buffer Add Request
pub mod txbar;
///TXBCR register accessor: an alias for `Reg<TXBCR_SPEC>`
pub type TXBCR = crate::Reg<txbcr::TXBCR_SPEC>;
///Tx Buffer Cancellation Request
pub mod txbcr;
///TXBTO register accessor: an alias for `Reg<TXBTO_SPEC>`
pub type TXBTO = crate::Reg<txbto::TXBTO_SPEC>;
///Tx Buffer Transmission Occurred
pub mod txbto;
///TXBCF register accessor: an alias for `Reg<TXBCF_SPEC>`
pub type TXBCF = crate::Reg<txbcf::TXBCF_SPEC>;
///Tx Buffer Cancellation Finished
pub mod txbcf;
///TXBTIE register accessor: an alias for `Reg<TXBTIE_SPEC>`
pub type TXBTIE = crate::Reg<txbtie::TXBTIE_SPEC>;
///Tx Buffer Transmission Interrupt Enable
pub mod txbtie;
///TXBCIE register accessor: an alias for `Reg<TXBCIE_SPEC>`
pub type TXBCIE = crate::Reg<txbcie::TXBCIE_SPEC>;
///Tx Buffer Cancellation Finished Interrupt Enable
pub mod txbcie;
///TXEFC register accessor: an alias for `Reg<TXEFC_SPEC>`
pub type TXEFC = crate::Reg<txefc::TXEFC_SPEC>;
///Tx Event FIFO Configuration
pub mod txefc;
///TXEFS register accessor: an alias for `Reg<TXEFS_SPEC>`
pub type TXEFS = crate::Reg<txefs::TXEFS_SPEC>;
///Tx Event FIFO Status
pub mod txefs;
///TXEFA register accessor: an alias for `Reg<TXEFA_SPEC>`
pub type TXEFA = crate::Reg<txefa::TXEFA_SPEC>;
///Tx Event FIFO Acknowledge
pub mod txefa;
///MRBA register accessor: an alias for `Reg<MRBA_SPEC>`
pub type MRBA = crate::Reg<mrba::MRBA_SPEC>;
///CAN Message RAM Base Address
pub mod mrba;
///ETSCC register accessor: an alias for `Reg<ETSCC_SPEC>`
pub type ETSCC = crate::Reg<etscc::ETSCC_SPEC>;
///External Timestamp Counter Configuration
pub mod etscc;
///ETSCV register accessor: an alias for `Reg<ETSCV_SPEC>`
pub type ETSCV = crate::Reg<etscv::ETSCV_SPEC>;
///External Timestamp Counter Value
pub mod etscv;
