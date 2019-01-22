#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MAC configuration register 1."]
    pub mac1: MAC1,
    #[doc = "0x04 - MAC configuration register 2."]
    pub mac2: MAC2,
    #[doc = "0x08 - Back-to-Back Inter-Packet-Gap register."]
    pub ipgt: IPGT,
    #[doc = "0x0c - Non Back-to-Back Inter-Packet-Gap register."]
    pub ipgr: IPGR,
    #[doc = "0x10 - Collision window / Retry register."]
    pub clrt: CLRT,
    #[doc = "0x14 - Maximum Frame register."]
    pub maxf: MAXF,
    #[doc = "0x18 - PHY Support register."]
    pub supp: SUPP,
    #[doc = "0x1c - Test register."]
    pub test: TEST,
    #[doc = "0x20 - MII Mgmt Configuration register."]
    pub mcfg: MCFG,
    #[doc = "0x24 - MII Mgmt Command register."]
    pub mcmd: MCMD,
    #[doc = "0x28 - MII Mgmt Address register."]
    pub madr: MADR,
    #[doc = "0x2c - MII Mgmt Write Data register."]
    pub mwtd: MWTD,
    #[doc = "0x30 - MII Mgmt Read Data register."]
    pub mrdd: MRDD,
    #[doc = "0x34 - MII Mgmt Indicators register."]
    pub mind: MIND,
    _reserved14: [u8; 8usize],
    #[doc = "0x40 - Station Address 0 register."]
    pub sa0: SA0,
    #[doc = "0x44 - Station Address 1 register."]
    pub sa1: SA1,
    #[doc = "0x48 - Station Address 2 register."]
    pub sa2: SA2,
    _reserved17: [u8; 180usize],
    #[doc = "0x100 - Command register."]
    pub command: COMMAND,
    #[doc = "0x104 - Status register."]
    pub status: STATUS,
    #[doc = "0x108 - Receive descriptor base address register."]
    pub rxdescriptor: RXDESCRIPTOR,
    #[doc = "0x10c - Receive status base address register."]
    pub rxstatus: RXSTATUS,
    #[doc = "0x110 - Receive number of descriptors register."]
    pub rxdescriptornumber: RXDESCRIPTORNUMBER,
    #[doc = "0x114 - Receive produce index register."]
    pub rxproduceindex: RXPRODUCEINDEX,
    #[doc = "0x118 - Receive consume index register."]
    pub rxconsumeindex: RXCONSUMEINDEX,
    #[doc = "0x11c - Transmit descriptor base address register."]
    pub txdescriptor: TXDESCRIPTOR,
    #[doc = "0x120 - Transmit status base address register."]
    pub txstatus: TXSTATUS,
    #[doc = "0x124 - Transmit number of descriptors register."]
    pub txdescriptornumber: TXDESCRIPTORNUMBER,
    #[doc = "0x128 - Transmit produce index register."]
    pub txproduceindex: TXPRODUCEINDEX,
    #[doc = "0x12c - Transmit consume index register."]
    pub txconsumeindex: TXCONSUMEINDEX,
    _reserved29: [u8; 40usize],
    #[doc = "0x158 - Transmit status vector 0 register."]
    pub tsv0: TSV0,
    #[doc = "0x15c - Transmit status vector 1 register."]
    pub tsv1: TSV1,
    #[doc = "0x160 - Receive status vector register."]
    pub rsv: RSV,
    _reserved32: [u8; 12usize],
    #[doc = "0x170 - Flow control counter register."]
    pub flowcontrolcounter: FLOWCONTROLCOUNTER,
    #[doc = "0x174 - Flow control status register."]
    pub flowcontrolstatus: FLOWCONTROLSTATUS,
    _reserved34: [u8; 136usize],
    #[doc = "0x200 - Receive filter control register."]
    pub rxfilterctrl: RXFILTERCTRL,
    #[doc = "0x204 - Receive filter WoL status register."]
    pub rxfilterwolstatus: RXFILTERWOLSTATUS,
    #[doc = "0x208 - Receive filter WoL clear register."]
    pub rxfilterwolclear: RXFILTERWOLCLEAR,
    _reserved37: [u8; 4usize],
    #[doc = "0x210 - Hash filter table LSBs register."]
    pub hashfilterl: HASHFILTERL,
    #[doc = "0x214 - Hash filter table MSBs register."]
    pub hashfilterh: HASHFILTERH,
    _reserved39: [u8; 3528usize],
    #[doc = "0xfe0 - Interrupt status register."]
    pub intstatus: INTSTATUS,
    #[doc = "0xfe4 - Interrupt enable register."]
    pub intenable: INTENABLE,
    #[doc = "0xfe8 - Interrupt clear register."]
    pub intclear: INTCLEAR,
    #[doc = "0xfec - Interrupt set register."]
    pub intset: INTSET,
    _reserved43: [u8; 4usize],
    #[doc = "0xff4 - Power-down register."]
    pub powerdown: POWERDOWN,
}
#[doc = "MAC configuration register 1."]
pub struct MAC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC configuration register 1."]
pub mod mac1;
#[doc = "MAC configuration register 2."]
pub struct MAC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MAC configuration register 2."]
pub mod mac2;
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub struct IPGT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub mod ipgt;
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub struct IPGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub mod ipgr;
#[doc = "Collision window / Retry register."]
pub struct CLRT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Collision window / Retry register."]
pub mod clrt;
#[doc = "Maximum Frame register."]
pub struct MAXF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Frame register."]
pub mod maxf;
#[doc = "PHY Support register."]
pub struct SUPP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY Support register."]
pub mod supp;
#[doc = "Test register."]
pub struct TEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Test register."]
pub mod test;
#[doc = "MII Mgmt Configuration register."]
pub struct MCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Configuration register."]
pub mod mcfg;
#[doc = "MII Mgmt Command register."]
pub struct MCMD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Command register."]
pub mod mcmd;
#[doc = "MII Mgmt Address register."]
pub struct MADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Address register."]
pub mod madr;
#[doc = "MII Mgmt Write Data register."]
pub struct MWTD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Write Data register."]
pub mod mwtd;
#[doc = "MII Mgmt Read Data register."]
pub struct MRDD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Read Data register."]
pub mod mrdd;
#[doc = "MII Mgmt Indicators register."]
pub struct MIND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "MII Mgmt Indicators register."]
pub mod mind;
#[doc = "Station Address 0 register."]
pub struct SA0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Station Address 0 register."]
pub mod sa0;
#[doc = "Station Address 1 register."]
pub struct SA1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Station Address 1 register."]
pub mod sa1;
#[doc = "Station Address 2 register."]
pub struct SA2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Station Address 2 register."]
pub mod sa2;
#[doc = "Command register."]
pub struct COMMAND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command register."]
pub mod command;
#[doc = "Status register."]
pub struct STATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register."]
pub mod status;
#[doc = "Receive descriptor base address register."]
pub struct RXDESCRIPTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive descriptor base address register."]
pub mod rxdescriptor;
#[doc = "Receive status base address register."]
pub struct RXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive status base address register."]
pub mod rxstatus;
#[doc = "Receive number of descriptors register."]
pub struct RXDESCRIPTORNUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive number of descriptors register."]
pub mod rxdescriptornumber;
#[doc = "Receive produce index register."]
pub struct RXPRODUCEINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive produce index register."]
pub mod rxproduceindex;
#[doc = "Receive consume index register."]
pub struct RXCONSUMEINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive consume index register."]
pub mod rxconsumeindex;
#[doc = "Transmit descriptor base address register."]
pub struct TXDESCRIPTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit descriptor base address register."]
pub mod txdescriptor;
#[doc = "Transmit status base address register."]
pub struct TXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit status base address register."]
pub mod txstatus;
#[doc = "Transmit number of descriptors register."]
pub struct TXDESCRIPTORNUMBER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit number of descriptors register."]
pub mod txdescriptornumber;
#[doc = "Transmit produce index register."]
pub struct TXPRODUCEINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit produce index register."]
pub mod txproduceindex;
#[doc = "Transmit consume index register."]
pub struct TXCONSUMEINDEX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit consume index register."]
pub mod txconsumeindex;
#[doc = "Transmit status vector 0 register."]
pub struct TSV0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit status vector 0 register."]
pub mod tsv0;
#[doc = "Transmit status vector 1 register."]
pub struct TSV1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit status vector 1 register."]
pub mod tsv1;
#[doc = "Receive status vector register."]
pub struct RSV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive status vector register."]
pub mod rsv;
#[doc = "Flow control counter register."]
pub struct FLOWCONTROLCOUNTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow control counter register."]
pub mod flowcontrolcounter;
#[doc = "Flow control status register."]
pub struct FLOWCONTROLSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Flow control status register."]
pub mod flowcontrolstatus;
#[doc = "Receive filter control register."]
pub struct RXFILTERCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive filter control register."]
pub mod rxfilterctrl;
#[doc = "Receive filter WoL status register."]
pub struct RXFILTERWOLSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive filter WoL status register."]
pub mod rxfilterwolstatus;
#[doc = "Receive filter WoL clear register."]
pub struct RXFILTERWOLCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive filter WoL clear register."]
pub mod rxfilterwolclear;
#[doc = "Hash filter table LSBs register."]
pub struct HASHFILTERL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash filter table LSBs register."]
pub mod hashfilterl;
#[doc = "Hash filter table MSBs register."]
pub struct HASHFILTERH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash filter table MSBs register."]
pub mod hashfilterh;
#[doc = "Interrupt status register."]
pub struct INTSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register."]
pub mod intstatus;
#[doc = "Interrupt enable register."]
pub struct INTENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register."]
pub mod intenable;
#[doc = "Interrupt clear register."]
pub struct INTCLEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt clear register."]
pub mod intclear;
#[doc = "Interrupt set register."]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt set register."]
pub mod intset;
#[doc = "Power-down register."]
pub struct POWERDOWN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power-down register."]
pub mod powerdown;
