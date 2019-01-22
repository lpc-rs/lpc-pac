#[doc = r" Register block"]
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
    #[doc = "I2C Transmit I2C Receive"]
    pub i2c: I2C_UNION,
    #[doc = "0x304 - I2C Status"]
    pub i2c_sts: I2C_STS,
    #[doc = "0x308 - I2C Control"]
    pub i2c_ctl: I2C_CTL,
    #[doc = "0x30c - I2C Clock High"]
    pub i2c_clkhi: I2C_CLKHI,
    #[doc = "0x310 - I2C Clock Low"]
    pub i2c_clklo: I2C_CLKLO,
    _reserved49: [u8; 3296usize],
    #[doc = "OTG clock controller USB Clock Control"]
    pub otgclkctrl: OTGCLKCTRL_UNION,
    #[doc = "OTG clock status USB Clock Status"]
    pub otgclkst: OTGCLKST_UNION,
}
#[doc = "I2C Transmit I2C Receive"]
#[repr(C)]
pub union I2C_UNION {
    #[doc = "0x300 - I2C Transmit"]
    pub i2c_wo: I2C_WO,
    #[doc = "0x300 - I2C Receive"]
    pub i2c_rx: I2C_RX,
}
#[doc = "OTG clock controller USB Clock Control"]
#[repr(C)]
pub union OTGCLKCTRL_UNION {
    #[doc = "0xff4 - OTG clock controller"]
    pub otgclkctrl: OTGCLKCTRL,
    #[doc = "0xff4 - USB Clock Control"]
    pub usbclkctrl: USBCLKCTRL,
}
#[doc = "OTG clock status USB Clock Status"]
#[repr(C)]
pub union OTGCLKST_UNION {
    #[doc = "0xff8 - OTG clock status"]
    pub otgclkst: OTGCLKST,
    #[doc = "0xff8 - USB Clock Status"]
    pub usbclkst: USBCLKST,
}
#[doc = "OTG Interrupt Status"]
pub struct INTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Interrupt Status"]
pub mod intst;
#[doc = "OTG Interrupt Enable"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Interrupt Enable"]
pub mod inten;
#[doc = "OTG Interrupt Set"]
pub struct INTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Interrupt Set"]
pub mod intset;
#[doc = "OTG Interrupt Clear"]
pub struct INTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Interrupt Clear"]
pub mod intclr;
#[doc = "OTG Status and Control and USB port select"]
pub struct STCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Status and Control and USB port select"]
pub mod stctrl;
#[doc = "OTG Timer"]
pub struct TMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG Timer"]
pub mod tmr;
#[doc = "USB Device Interrupt Status"]
pub struct DEVINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Status"]
pub mod devintst;
#[doc = "USB Device Interrupt Enable"]
pub struct DEVINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Enable"]
pub mod devinten;
#[doc = "USB Device Interrupt Clear"]
pub struct DEVINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Clear"]
pub mod devintclr;
#[doc = "USB Device Interrupt Set"]
pub struct DEVINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Set"]
pub mod devintset;
#[doc = "USB Command Code"]
pub struct CMDCODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Command Code"]
pub mod cmdcode;
#[doc = "USB Command Data"]
pub struct CMDDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Command Data"]
pub mod cmddata;
#[doc = "USB Receive Data"]
pub struct RXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Receive Data"]
pub mod rxdata;
#[doc = "USB Transmit Data"]
pub struct TXDATA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Transmit Data"]
pub mod txdata;
#[doc = "USB Receive Packet Length"]
pub struct RXPLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Receive Packet Length"]
pub mod rxplen;
#[doc = "USB Transmit Packet Length"]
pub struct TXPLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Transmit Packet Length"]
pub mod txplen;
#[doc = "USB Control"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Control"]
pub mod ctrl;
#[doc = "USB Device Interrupt Priority"]
pub struct DEVINTPRI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Device Interrupt Priority"]
pub mod devintpri;
#[doc = "USB Endpoint Interrupt Status"]
pub struct EPINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Interrupt Status"]
pub mod epintst;
#[doc = "USB Endpoint Interrupt Enable"]
pub struct EPINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Interrupt Enable"]
pub mod epinten;
#[doc = "USB Endpoint Interrupt Clear"]
pub struct EPINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Interrupt Clear"]
pub mod epintclr;
#[doc = "USB Endpoint Interrupt Set"]
pub struct EPINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Interrupt Set"]
pub mod epintset;
#[doc = "USB Endpoint Priority"]
pub struct EPINTPRI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Priority"]
pub mod epintpri;
#[doc = "USB Realize Endpoint"]
pub struct REEP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Realize Endpoint"]
pub mod reep;
#[doc = "USB Endpoint Index"]
pub struct EPIND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint Index"]
pub mod epind;
#[doc = "USB MaxPacketSize"]
pub struct MAXPSIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB MaxPacketSize"]
pub mod maxpsize;
#[doc = "USB DMA Request Status"]
pub struct DMARST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Request Status"]
pub mod dmarst;
#[doc = "USB DMA Request Clear"]
pub struct DMARCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Request Clear"]
pub mod dmarclr;
#[doc = "USB DMA Request Set"]
pub struct DMARSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Request Set"]
pub mod dmarset;
#[doc = "USB UDCA Head"]
pub struct UDCAH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB UDCA Head"]
pub mod udcah;
#[doc = "USB Endpoint DMA Status"]
pub struct EPDMAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint DMA Status"]
pub mod epdmast;
#[doc = "USB Endpoint DMA Enable"]
pub struct EPDMAEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint DMA Enable"]
pub mod epdmaen;
#[doc = "USB Endpoint DMA Disable"]
pub struct EPDMADIS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Endpoint DMA Disable"]
pub mod epdmadis;
#[doc = "USB DMA Interrupt Status"]
pub struct DMAINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Interrupt Status"]
pub mod dmaintst;
#[doc = "USB DMA Interrupt Enable"]
pub struct DMAINTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB DMA Interrupt Enable"]
pub mod dmainten;
#[doc = "USB End of Transfer Interrupt Status"]
pub struct EOTINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB End of Transfer Interrupt Status"]
pub mod eotintst;
#[doc = "USB End of Transfer Interrupt Clear"]
pub struct EOTINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB End of Transfer Interrupt Clear"]
pub mod eotintclr;
#[doc = "USB End of Transfer Interrupt Set"]
pub struct EOTINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB End of Transfer Interrupt Set"]
pub mod eotintset;
#[doc = "USB New DD Request Interrupt Status"]
pub struct NDDRINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB New DD Request Interrupt Status"]
pub mod nddrintst;
#[doc = "USB New DD Request Interrupt Clear"]
pub struct NDDRINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB New DD Request Interrupt Clear"]
pub mod nddrintclr;
#[doc = "USB New DD Request Interrupt Set"]
pub struct NDDRINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB New DD Request Interrupt Set"]
pub mod nddrintset;
#[doc = "USB System Error Interrupt Status"]
pub struct SYSERRINTST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB System Error Interrupt Status"]
pub mod syserrintst;
#[doc = "USB System Error Interrupt Clear"]
pub struct SYSERRINTCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB System Error Interrupt Clear"]
pub mod syserrintclr;
#[doc = "USB System Error Interrupt Set"]
pub struct SYSERRINTSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB System Error Interrupt Set"]
pub mod syserrintset;
#[doc = "I2C Receive"]
pub struct I2C_RX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Receive"]
pub mod i2c_rx;
#[doc = "I2C Transmit"]
pub struct I2C_WO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Transmit"]
pub mod i2c_wo;
#[doc = "I2C Status"]
pub struct I2C_STS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Status"]
pub mod i2c_sts;
#[doc = "I2C Control"]
pub struct I2C_CTL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Control"]
pub mod i2c_ctl;
#[doc = "I2C Clock High"]
pub struct I2C_CLKHI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock High"]
pub mod i2c_clkhi;
#[doc = "I2C Clock Low"]
pub struct I2C_CLKLO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I2C Clock Low"]
pub mod i2c_clklo;
#[doc = "USB Clock Control"]
pub struct USBCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Control"]
pub mod usbclkctrl;
#[doc = "OTG clock controller"]
pub struct OTGCLKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG clock controller"]
pub mod otgclkctrl;
#[doc = "USB Clock Status"]
pub struct USBCLKST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "USB Clock Status"]
pub mod usbclkst;
#[doc = "OTG clock status"]
pub struct OTGCLKST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "OTG clock status"]
pub mod otgclkst;
