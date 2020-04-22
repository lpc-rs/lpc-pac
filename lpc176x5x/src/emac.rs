#[doc = r"Register block"]
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
#[doc = "MAC configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac1](mac1) module"]
pub type MAC1 = crate::Reg<u32, _MAC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC1;
#[doc = "`read()` method returns [mac1::R](mac1::R) reader structure"]
impl crate::Readable for MAC1 {}
#[doc = "`write(|w| ..)` method takes [mac1::W](mac1::W) writer structure"]
impl crate::Writable for MAC1 {}
#[doc = "MAC configuration register 1."]
pub mod mac1;
#[doc = "MAC configuration register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac2](mac2) module"]
pub type MAC2 = crate::Reg<u32, _MAC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAC2;
#[doc = "`read()` method returns [mac2::R](mac2::R) reader structure"]
impl crate::Readable for MAC2 {}
#[doc = "`write(|w| ..)` method takes [mac2::W](mac2::W) writer structure"]
impl crate::Writable for MAC2 {}
#[doc = "MAC configuration register 2."]
pub mod mac2;
#[doc = "Back-to-Back Inter-Packet-Gap register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgt](ipgt) module"]
pub type IPGT = crate::Reg<u32, _IPGT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGT;
#[doc = "`read()` method returns [ipgt::R](ipgt::R) reader structure"]
impl crate::Readable for IPGT {}
#[doc = "`write(|w| ..)` method takes [ipgt::W](ipgt::W) writer structure"]
impl crate::Writable for IPGT {}
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub mod ipgt;
#[doc = "Non Back-to-Back Inter-Packet-Gap register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipgr](ipgr) module"]
pub type IPGR = crate::Reg<u32, _IPGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPGR;
#[doc = "`read()` method returns [ipgr::R](ipgr::R) reader structure"]
impl crate::Readable for IPGR {}
#[doc = "`write(|w| ..)` method takes [ipgr::W](ipgr::W) writer structure"]
impl crate::Writable for IPGR {}
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub mod ipgr;
#[doc = "Collision window / Retry register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrt](clrt) module"]
pub type CLRT = crate::Reg<u32, _CLRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLRT;
#[doc = "`read()` method returns [clrt::R](clrt::R) reader structure"]
impl crate::Readable for CLRT {}
#[doc = "`write(|w| ..)` method takes [clrt::W](clrt::W) writer structure"]
impl crate::Writable for CLRT {}
#[doc = "Collision window / Retry register."]
pub mod clrt;
#[doc = "Maximum Frame register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxf](maxf) module"]
pub type MAXF = crate::Reg<u32, _MAXF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXF;
#[doc = "`read()` method returns [maxf::R](maxf::R) reader structure"]
impl crate::Readable for MAXF {}
#[doc = "`write(|w| ..)` method takes [maxf::W](maxf::W) writer structure"]
impl crate::Writable for MAXF {}
#[doc = "Maximum Frame register."]
pub mod maxf;
#[doc = "PHY Support register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supp](supp) module"]
pub type SUPP = crate::Reg<u32, _SUPP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUPP;
#[doc = "`read()` method returns [supp::R](supp::R) reader structure"]
impl crate::Readable for SUPP {}
#[doc = "`write(|w| ..)` method takes [supp::W](supp::W) writer structure"]
impl crate::Writable for SUPP {}
#[doc = "PHY Support register."]
pub mod supp;
#[doc = "Test register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](test) module"]
pub type TEST = crate::Reg<u32, _TEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEST;
#[doc = "`read()` method returns [test::R](test::R) reader structure"]
impl crate::Readable for TEST {}
#[doc = "`write(|w| ..)` method takes [test::W](test::W) writer structure"]
impl crate::Writable for TEST {}
#[doc = "Test register."]
pub mod test;
#[doc = "MII Mgmt Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfg](mcfg) module"]
pub type MCFG = crate::Reg<u32, _MCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCFG;
#[doc = "`read()` method returns [mcfg::R](mcfg::R) reader structure"]
impl crate::Readable for MCFG {}
#[doc = "`write(|w| ..)` method takes [mcfg::W](mcfg::W) writer structure"]
impl crate::Writable for MCFG {}
#[doc = "MII Mgmt Configuration register."]
pub mod mcfg;
#[doc = "MII Mgmt Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcmd](mcmd) module"]
pub type MCMD = crate::Reg<u32, _MCMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCMD;
#[doc = "`read()` method returns [mcmd::R](mcmd::R) reader structure"]
impl crate::Readable for MCMD {}
#[doc = "`write(|w| ..)` method takes [mcmd::W](mcmd::W) writer structure"]
impl crate::Writable for MCMD {}
#[doc = "MII Mgmt Command register."]
pub mod mcmd;
#[doc = "MII Mgmt Address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [madr](madr) module"]
pub type MADR = crate::Reg<u32, _MADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MADR;
#[doc = "`read()` method returns [madr::R](madr::R) reader structure"]
impl crate::Readable for MADR {}
#[doc = "`write(|w| ..)` method takes [madr::W](madr::W) writer structure"]
impl crate::Writable for MADR {}
#[doc = "MII Mgmt Address register."]
pub mod madr;
#[doc = "MII Mgmt Write Data register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mwtd](mwtd) module"]
pub type MWTD = crate::Reg<u32, _MWTD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MWTD;
#[doc = "`write(|w| ..)` method takes [mwtd::W](mwtd::W) writer structure"]
impl crate::Writable for MWTD {}
#[doc = "MII Mgmt Write Data register."]
pub mod mwtd;
#[doc = "MII Mgmt Read Data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrdd](mrdd) module"]
pub type MRDD = crate::Reg<u32, _MRDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRDD;
#[doc = "`read()` method returns [mrdd::R](mrdd::R) reader structure"]
impl crate::Readable for MRDD {}
#[doc = "MII Mgmt Read Data register."]
pub mod mrdd;
#[doc = "MII Mgmt Indicators register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mind](mind) module"]
pub type MIND = crate::Reg<u32, _MIND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIND;
#[doc = "`read()` method returns [mind::R](mind::R) reader structure"]
impl crate::Readable for MIND {}
#[doc = "MII Mgmt Indicators register."]
pub mod mind;
#[doc = "Station Address 0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa0](sa0) module"]
pub type SA0 = crate::Reg<u32, _SA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA0;
#[doc = "`read()` method returns [sa0::R](sa0::R) reader structure"]
impl crate::Readable for SA0 {}
#[doc = "`write(|w| ..)` method takes [sa0::W](sa0::W) writer structure"]
impl crate::Writable for SA0 {}
#[doc = "Station Address 0 register."]
pub mod sa0;
#[doc = "Station Address 1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa1](sa1) module"]
pub type SA1 = crate::Reg<u32, _SA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA1;
#[doc = "`read()` method returns [sa1::R](sa1::R) reader structure"]
impl crate::Readable for SA1 {}
#[doc = "`write(|w| ..)` method takes [sa1::W](sa1::W) writer structure"]
impl crate::Writable for SA1 {}
#[doc = "Station Address 1 register."]
pub mod sa1;
#[doc = "Station Address 2 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa2](sa2) module"]
pub type SA2 = crate::Reg<u32, _SA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SA2;
#[doc = "`read()` method returns [sa2::R](sa2::R) reader structure"]
impl crate::Readable for SA2 {}
#[doc = "`write(|w| ..)` method takes [sa2::W](sa2::W) writer structure"]
impl crate::Writable for SA2 {}
#[doc = "Station Address 2 register."]
pub mod sa2;
#[doc = "Command register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](command) module"]
pub type COMMAND = crate::Reg<u32, _COMMAND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMMAND;
#[doc = "`read()` method returns [command::R](command::R) reader structure"]
impl crate::Readable for COMMAND {}
#[doc = "`write(|w| ..)` method takes [command::W](command::W) writer structure"]
impl crate::Writable for COMMAND {}
#[doc = "Command register."]
pub mod command;
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status register."]
pub mod status;
#[doc = "Receive descriptor base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdescriptor](rxdescriptor) module"]
pub type RXDESCRIPTOR = crate::Reg<u32, _RXDESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDESCRIPTOR;
#[doc = "`read()` method returns [rxdescriptor::R](rxdescriptor::R) reader structure"]
impl crate::Readable for RXDESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [rxdescriptor::W](rxdescriptor::W) writer structure"]
impl crate::Writable for RXDESCRIPTOR {}
#[doc = "Receive descriptor base address register."]
pub mod rxdescriptor;
#[doc = "Receive status base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxstatus](rxstatus) module"]
pub type RXSTATUS = crate::Reg<u32, _RXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSTATUS;
#[doc = "`read()` method returns [rxstatus::R](rxstatus::R) reader structure"]
impl crate::Readable for RXSTATUS {}
#[doc = "`write(|w| ..)` method takes [rxstatus::W](rxstatus::W) writer structure"]
impl crate::Writable for RXSTATUS {}
#[doc = "Receive status base address register."]
pub mod rxstatus;
#[doc = "Receive number of descriptors register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdescriptornumber](rxdescriptornumber) module"]
pub type RXDESCRIPTORNUMBER = crate::Reg<u32, _RXDESCRIPTORNUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDESCRIPTORNUMBER;
#[doc = "`read()` method returns [rxdescriptornumber::R](rxdescriptornumber::R) reader structure"]
impl crate::Readable for RXDESCRIPTORNUMBER {}
#[doc = "`write(|w| ..)` method takes [rxdescriptornumber::W](rxdescriptornumber::W) writer structure"]
impl crate::Writable for RXDESCRIPTORNUMBER {}
#[doc = "Receive number of descriptors register."]
pub mod rxdescriptornumber;
#[doc = "Receive produce index register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxproduceindex](rxproduceindex) module"]
pub type RXPRODUCEINDEX = crate::Reg<u32, _RXPRODUCEINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPRODUCEINDEX;
#[doc = "`read()` method returns [rxproduceindex::R](rxproduceindex::R) reader structure"]
impl crate::Readable for RXPRODUCEINDEX {}
#[doc = "Receive produce index register."]
pub mod rxproduceindex;
#[doc = "Receive consume index register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxconsumeindex](rxconsumeindex) module"]
pub type RXCONSUMEINDEX = crate::Reg<u32, _RXCONSUMEINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXCONSUMEINDEX;
#[doc = "`read()` method returns [rxconsumeindex::R](rxconsumeindex::R) reader structure"]
impl crate::Readable for RXCONSUMEINDEX {}
#[doc = "`write(|w| ..)` method takes [rxconsumeindex::W](rxconsumeindex::W) writer structure"]
impl crate::Writable for RXCONSUMEINDEX {}
#[doc = "Receive consume index register."]
pub mod rxconsumeindex;
#[doc = "Transmit descriptor base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdescriptor](txdescriptor) module"]
pub type TXDESCRIPTOR = crate::Reg<u32, _TXDESCRIPTOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDESCRIPTOR;
#[doc = "`read()` method returns [txdescriptor::R](txdescriptor::R) reader structure"]
impl crate::Readable for TXDESCRIPTOR {}
#[doc = "`write(|w| ..)` method takes [txdescriptor::W](txdescriptor::W) writer structure"]
impl crate::Writable for TXDESCRIPTOR {}
#[doc = "Transmit descriptor base address register."]
pub mod txdescriptor;
#[doc = "Transmit status base address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txstatus](txstatus) module"]
pub type TXSTATUS = crate::Reg<u32, _TXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXSTATUS;
#[doc = "`read()` method returns [txstatus::R](txstatus::R) reader structure"]
impl crate::Readable for TXSTATUS {}
#[doc = "`write(|w| ..)` method takes [txstatus::W](txstatus::W) writer structure"]
impl crate::Writable for TXSTATUS {}
#[doc = "Transmit status base address register."]
pub mod txstatus;
#[doc = "Transmit number of descriptors register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdescriptornumber](txdescriptornumber) module"]
pub type TXDESCRIPTORNUMBER = crate::Reg<u32, _TXDESCRIPTORNUMBER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDESCRIPTORNUMBER;
#[doc = "`read()` method returns [txdescriptornumber::R](txdescriptornumber::R) reader structure"]
impl crate::Readable for TXDESCRIPTORNUMBER {}
#[doc = "`write(|w| ..)` method takes [txdescriptornumber::W](txdescriptornumber::W) writer structure"]
impl crate::Writable for TXDESCRIPTORNUMBER {}
#[doc = "Transmit number of descriptors register."]
pub mod txdescriptornumber;
#[doc = "Transmit produce index register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txproduceindex](txproduceindex) module"]
pub type TXPRODUCEINDEX = crate::Reg<u32, _TXPRODUCEINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPRODUCEINDEX;
#[doc = "`read()` method returns [txproduceindex::R](txproduceindex::R) reader structure"]
impl crate::Readable for TXPRODUCEINDEX {}
#[doc = "`write(|w| ..)` method takes [txproduceindex::W](txproduceindex::W) writer structure"]
impl crate::Writable for TXPRODUCEINDEX {}
#[doc = "Transmit produce index register."]
pub mod txproduceindex;
#[doc = "Transmit consume index register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txconsumeindex](txconsumeindex) module"]
pub type TXCONSUMEINDEX = crate::Reg<u32, _TXCONSUMEINDEX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXCONSUMEINDEX;
#[doc = "`read()` method returns [txconsumeindex::R](txconsumeindex::R) reader structure"]
impl crate::Readable for TXCONSUMEINDEX {}
#[doc = "Transmit consume index register."]
pub mod txconsumeindex;
#[doc = "Transmit status vector 0 register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsv0](tsv0) module"]
pub type TSV0 = crate::Reg<u32, _TSV0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSV0;
#[doc = "`read()` method returns [tsv0::R](tsv0::R) reader structure"]
impl crate::Readable for TSV0 {}
#[doc = "Transmit status vector 0 register."]
pub mod tsv0;
#[doc = "Transmit status vector 1 register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsv1](tsv1) module"]
pub type TSV1 = crate::Reg<u32, _TSV1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSV1;
#[doc = "`read()` method returns [tsv1::R](tsv1::R) reader structure"]
impl crate::Readable for TSV1 {}
#[doc = "Transmit status vector 1 register."]
pub mod tsv1;
#[doc = "Receive status vector register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsv](rsv) module"]
pub type RSV = crate::Reg<u32, _RSV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSV;
#[doc = "`read()` method returns [rsv::R](rsv::R) reader structure"]
impl crate::Readable for RSV {}
#[doc = "Receive status vector register."]
pub mod rsv;
#[doc = "Flow control counter register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowcontrolcounter](flowcontrolcounter) module"]
pub type FLOWCONTROLCOUNTER = crate::Reg<u32, _FLOWCONTROLCOUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOWCONTROLCOUNTER;
#[doc = "`read()` method returns [flowcontrolcounter::R](flowcontrolcounter::R) reader structure"]
impl crate::Readable for FLOWCONTROLCOUNTER {}
#[doc = "`write(|w| ..)` method takes [flowcontrolcounter::W](flowcontrolcounter::W) writer structure"]
impl crate::Writable for FLOWCONTROLCOUNTER {}
#[doc = "Flow control counter register."]
pub mod flowcontrolcounter;
#[doc = "Flow control status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flowcontrolstatus](flowcontrolstatus) module"]
pub type FLOWCONTROLSTATUS = crate::Reg<u32, _FLOWCONTROLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLOWCONTROLSTATUS;
#[doc = "`read()` method returns [flowcontrolstatus::R](flowcontrolstatus::R) reader structure"]
impl crate::Readable for FLOWCONTROLSTATUS {}
#[doc = "Flow control status register."]
pub mod flowcontrolstatus;
#[doc = "Receive filter control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfilterctrl](rxfilterctrl) module"]
pub type RXFILTERCTRL = crate::Reg<u32, _RXFILTERCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFILTERCTRL;
#[doc = "`read()` method returns [rxfilterctrl::R](rxfilterctrl::R) reader structure"]
impl crate::Readable for RXFILTERCTRL {}
#[doc = "`write(|w| ..)` method takes [rxfilterctrl::W](rxfilterctrl::W) writer structure"]
impl crate::Writable for RXFILTERCTRL {}
#[doc = "Receive filter control register."]
pub mod rxfilterctrl;
#[doc = "Receive filter WoL status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfilterwolstatus](rxfilterwolstatus) module"]
pub type RXFILTERWOLSTATUS = crate::Reg<u32, _RXFILTERWOLSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFILTERWOLSTATUS;
#[doc = "`read()` method returns [rxfilterwolstatus::R](rxfilterwolstatus::R) reader structure"]
impl crate::Readable for RXFILTERWOLSTATUS {}
#[doc = "Receive filter WoL status register."]
pub mod rxfilterwolstatus;
#[doc = "Receive filter WoL clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfilterwolclear](rxfilterwolclear) module"]
pub type RXFILTERWOLCLEAR = crate::Reg<u32, _RXFILTERWOLCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXFILTERWOLCLEAR;
#[doc = "`write(|w| ..)` method takes [rxfilterwolclear::W](rxfilterwolclear::W) writer structure"]
impl crate::Writable for RXFILTERWOLCLEAR {}
#[doc = "Receive filter WoL clear register."]
pub mod rxfilterwolclear;
#[doc = "Hash filter table LSBs register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashfilterl](hashfilterl) module"]
pub type HASHFILTERL = crate::Reg<u32, _HASHFILTERL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHFILTERL;
#[doc = "`read()` method returns [hashfilterl::R](hashfilterl::R) reader structure"]
impl crate::Readable for HASHFILTERL {}
#[doc = "`write(|w| ..)` method takes [hashfilterl::W](hashfilterl::W) writer structure"]
impl crate::Writable for HASHFILTERL {}
#[doc = "Hash filter table LSBs register."]
pub mod hashfilterl;
#[doc = "Hash filter table MSBs register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hashfilterh](hashfilterh) module"]
pub type HASHFILTERH = crate::Reg<u32, _HASHFILTERH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHFILTERH;
#[doc = "`read()` method returns [hashfilterh::R](hashfilterh::R) reader structure"]
impl crate::Readable for HASHFILTERH {}
#[doc = "`write(|w| ..)` method takes [hashfilterh::W](hashfilterh::W) writer structure"]
impl crate::Writable for HASHFILTERH {}
#[doc = "Hash filter table MSBs register."]
pub mod hashfilterh;
#[doc = "Interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstatus](intstatus) module"]
pub type INTSTATUS = crate::Reg<u32, _INTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTATUS;
#[doc = "`read()` method returns [intstatus::R](intstatus::R) reader structure"]
impl crate::Readable for INTSTATUS {}
#[doc = "Interrupt status register."]
pub mod intstatus;
#[doc = "Interrupt enable register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenable](intenable) module"]
pub type INTENABLE = crate::Reg<u32, _INTENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENABLE;
#[doc = "`read()` method returns [intenable::R](intenable::R) reader structure"]
impl crate::Readable for INTENABLE {}
#[doc = "`write(|w| ..)` method takes [intenable::W](intenable::W) writer structure"]
impl crate::Writable for INTENABLE {}
#[doc = "Interrupt enable register."]
pub mod intenable;
#[doc = "Interrupt clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclear](intclear) module"]
pub type INTCLEAR = crate::Reg<u32, _INTCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCLEAR;
#[doc = "`write(|w| ..)` method takes [intclear::W](intclear::W) writer structure"]
impl crate::Writable for INTCLEAR {}
#[doc = "Interrupt clear register."]
pub mod intclear;
#[doc = "Interrupt set register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intset](intset) module"]
pub type INTSET = crate::Reg<u32, _INTSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSET;
#[doc = "`write(|w| ..)` method takes [intset::W](intset::W) writer structure"]
impl crate::Writable for INTSET {}
#[doc = "Interrupt set register."]
pub mod intset;
#[doc = "Power-down register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerdown](powerdown) module"]
pub type POWERDOWN = crate::Reg<u32, _POWERDOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWERDOWN;
#[doc = "`read()` method returns [powerdown::R](powerdown::R) reader structure"]
impl crate::Readable for POWERDOWN {}
#[doc = "`write(|w| ..)` method takes [powerdown::W](powerdown::W) writer structure"]
impl crate::Writable for POWERDOWN {}
#[doc = "Power-down register."]
pub mod powerdown;
