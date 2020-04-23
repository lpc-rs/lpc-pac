#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Power Enable register"]
    pub pwren: PWREN,
    #[doc = "0x08 - Clock Divider register"]
    pub clkdiv: CLKDIV,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Clock Enable register"]
    pub clkena: CLKENA,
    #[doc = "0x14 - Time-out register"]
    pub tmout: TMOUT,
    #[doc = "0x18 - Card Type register"]
    pub ctype: CTYPE,
    #[doc = "0x1c - Block Size register"]
    pub blksiz: BLKSIZ,
    #[doc = "0x20 - Byte Count register"]
    pub bytcnt: BYTCNT,
    #[doc = "0x24 - Interrupt Mask register"]
    pub intmask: INTMASK,
    #[doc = "0x28 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x2c - Command register"]
    pub cmd: CMD,
    #[doc = "0x30 - Response register"]
    pub resp: [RESP; 4],
    #[doc = "0x40 - Masked Interrupt Status register"]
    pub mintsts: MINTSTS,
    #[doc = "0x44 - Raw Interrupt Status register"]
    pub rintsts: RINTSTS,
    #[doc = "0x48 - Status register"]
    pub status: STATUS,
    #[doc = "0x4c - FIFO Threshold Watermark register"]
    pub fifoth: FIFOTH,
    #[doc = "0x50 - Card Detect register"]
    pub cdetect: CDETECT,
    #[doc = "0x54 - Write Protect register"]
    pub wrtprt: WRTPRT,
    _reserved18: [u8; 4usize],
    #[doc = "0x5c - Transferred CIU Card Byte Count register"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count register"]
    pub tbbcnt: TBBCNT,
    #[doc = "0x64 - Debounce Count register"]
    pub debnce: DEBNCE,
    _reserved21: [u8; 16usize],
    #[doc = "0x78 - Hardware Reset"]
    pub rst_n: RST_N,
    _reserved22: [u8; 4usize],
    #[doc = "0x80 - Bus Mode register"]
    pub bmod: BMOD,
    #[doc = "0x84 - Poll Demand register"]
    pub pldmnd: PLDMND,
    #[doc = "0x88 - Descriptor List Base Address register"]
    pub dbaddr: DBADDR,
    #[doc = "0x8c - Internal DMAC Status register"]
    pub idsts: IDSTS,
    #[doc = "0x90 - Internal DMAC Interrupt Enable register"]
    pub idinten: IDINTEN,
    #[doc = "0x94 - Current Host Descriptor Address register"]
    pub dscaddr: DSCADDR,
    #[doc = "0x98 - Current Buffer Descriptor Address register"]
    pub bufaddr: BUFADDR,
    _reserved29: [u8; 100usize],
    #[doc = "0x100 - Card Threshold Control"]
    pub cardthrctl: CARDTHRCTL,
    #[doc = "0x104 - Power control"]
    pub backendpwr: BACKENDPWR,
    _reserved31: [u8; 248usize],
    #[doc = "0x200 - SDIF FIFO"]
    pub fifo: [FIFO; 64],
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control register"]
pub mod ctrl;
#[doc = "Power Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwren](pwren) module"]
pub type PWREN = crate::Reg<u32, _PWREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PWREN;
#[doc = "`read()` method returns [pwren::R](pwren::R) reader structure"]
impl crate::Readable for PWREN {}
#[doc = "`write(|w| ..)` method takes [pwren::W](pwren::W) writer structure"]
impl crate::Writable for PWREN {}
#[doc = "Power Enable register"]
pub mod pwren;
#[doc = "Clock Divider register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "Clock Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkena](clkena) module"]
pub type CLKENA = crate::Reg<u32, _CLKENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKENA;
#[doc = "`read()` method returns [clkena::R](clkena::R) reader structure"]
impl crate::Readable for CLKENA {}
#[doc = "`write(|w| ..)` method takes [clkena::W](clkena::W) writer structure"]
impl crate::Writable for CLKENA {}
#[doc = "Clock Enable register"]
pub mod clkena;
#[doc = "Time-out register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmout](tmout) module"]
pub type TMOUT = crate::Reg<u32, _TMOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMOUT;
#[doc = "`read()` method returns [tmout::R](tmout::R) reader structure"]
impl crate::Readable for TMOUT {}
#[doc = "`write(|w| ..)` method takes [tmout::W](tmout::W) writer structure"]
impl crate::Writable for TMOUT {}
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "Card Type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctype](ctype) module"]
pub type CTYPE = crate::Reg<u32, _CTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTYPE;
#[doc = "`read()` method returns [ctype::R](ctype::R) reader structure"]
impl crate::Readable for CTYPE {}
#[doc = "`write(|w| ..)` method takes [ctype::W](ctype::W) writer structure"]
impl crate::Writable for CTYPE {}
#[doc = "Card Type register"]
pub mod ctype;
#[doc = "Block Size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blksiz](blksiz) module"]
pub type BLKSIZ = crate::Reg<u32, _BLKSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLKSIZ;
#[doc = "`read()` method returns [blksiz::R](blksiz::R) reader structure"]
impl crate::Readable for BLKSIZ {}
#[doc = "`write(|w| ..)` method takes [blksiz::W](blksiz::W) writer structure"]
impl crate::Writable for BLKSIZ {}
#[doc = "Block Size register"]
pub mod blksiz;
#[doc = "Byte Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytcnt](bytcnt) module"]
pub type BYTCNT = crate::Reg<u32, _BYTCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BYTCNT;
#[doc = "`read()` method returns [bytcnt::R](bytcnt::R) reader structure"]
impl crate::Readable for BYTCNT {}
#[doc = "`write(|w| ..)` method takes [bytcnt::W](bytcnt::W) writer structure"]
impl crate::Writable for BYTCNT {}
#[doc = "Byte Count register"]
pub mod bytcnt;
#[doc = "Interrupt Mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intmask](intmask) module"]
pub type INTMASK = crate::Reg<u32, _INTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTMASK;
#[doc = "`read()` method returns [intmask::R](intmask::R) reader structure"]
impl crate::Readable for INTMASK {}
#[doc = "`write(|w| ..)` method takes [intmask::W](intmask::W) writer structure"]
impl crate::Writable for INTMASK {}
#[doc = "Interrupt Mask register"]
pub mod intmask;
#[doc = "Command Argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdarg](cmdarg) module"]
pub type CMDARG = crate::Reg<u32, _CMDARG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDARG;
#[doc = "`read()` method returns [cmdarg::R](cmdarg::R) reader structure"]
impl crate::Readable for CMDARG {}
#[doc = "`write(|w| ..)` method takes [cmdarg::W](cmdarg::W) writer structure"]
impl crate::Writable for CMDARG {}
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command register"]
pub mod cmd;
#[doc = "Response register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resp](resp) module"]
pub type RESP = crate::Reg<u32, _RESP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP;
#[doc = "`read()` method returns [resp::R](resp::R) reader structure"]
impl crate::Readable for RESP {}
#[doc = "`write(|w| ..)` method takes [resp::W](resp::W) writer structure"]
impl crate::Writable for RESP {}
#[doc = "Response register"]
pub mod resp;
#[doc = "Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mintsts](mintsts) module"]
pub type MINTSTS = crate::Reg<u32, _MINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MINTSTS;
#[doc = "`read()` method returns [mintsts::R](mintsts::R) reader structure"]
impl crate::Readable for MINTSTS {}
#[doc = "`write(|w| ..)` method takes [mintsts::W](mintsts::W) writer structure"]
impl crate::Writable for MINTSTS {}
#[doc = "Masked Interrupt Status register"]
pub mod mintsts;
#[doc = "Raw Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rintsts](rintsts) module"]
pub type RINTSTS = crate::Reg<u32, _RINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RINTSTS;
#[doc = "`read()` method returns [rintsts::R](rintsts::R) reader structure"]
impl crate::Readable for RINTSTS {}
#[doc = "`write(|w| ..)` method takes [rintsts::W](rintsts::W) writer structure"]
impl crate::Writable for RINTSTS {}
#[doc = "Raw Interrupt Status register"]
pub mod rintsts;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status register"]
pub mod status;
#[doc = "FIFO Threshold Watermark register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoth](fifoth) module"]
pub type FIFOTH = crate::Reg<u32, _FIFOTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOTH;
#[doc = "`read()` method returns [fifoth::R](fifoth::R) reader structure"]
impl crate::Readable for FIFOTH {}
#[doc = "`write(|w| ..)` method takes [fifoth::W](fifoth::W) writer structure"]
impl crate::Writable for FIFOTH {}
#[doc = "FIFO Threshold Watermark register"]
pub mod fifoth;
#[doc = "Card Detect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdetect](cdetect) module"]
pub type CDETECT = crate::Reg<u32, _CDETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDETECT;
#[doc = "`read()` method returns [cdetect::R](cdetect::R) reader structure"]
impl crate::Readable for CDETECT {}
#[doc = "`write(|w| ..)` method takes [cdetect::W](cdetect::W) writer structure"]
impl crate::Writable for CDETECT {}
#[doc = "Card Detect register"]
pub mod cdetect;
#[doc = "Write Protect register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtprt](wrtprt) module"]
pub type WRTPRT = crate::Reg<u32, _WRTPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRTPRT;
#[doc = "`read()` method returns [wrtprt::R](wrtprt::R) reader structure"]
impl crate::Readable for WRTPRT {}
#[doc = "`write(|w| ..)` method takes [wrtprt::W](wrtprt::W) writer structure"]
impl crate::Writable for WRTPRT {}
#[doc = "Write Protect register"]
pub mod wrtprt;
#[doc = "Transferred CIU Card Byte Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcbcnt](tcbcnt) module"]
pub type TCBCNT = crate::Reg<u32, _TCBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCBCNT;
#[doc = "`read()` method returns [tcbcnt::R](tcbcnt::R) reader structure"]
impl crate::Readable for TCBCNT {}
#[doc = "`write(|w| ..)` method takes [tcbcnt::W](tcbcnt::W) writer structure"]
impl crate::Writable for TCBCNT {}
#[doc = "Transferred CIU Card Byte Count register"]
pub mod tcbcnt;
#[doc = "Transferred Host to BIU-FIFO Byte Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbbcnt](tbbcnt) module"]
pub type TBBCNT = crate::Reg<u32, _TBBCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TBBCNT;
#[doc = "`read()` method returns [tbbcnt::R](tbbcnt::R) reader structure"]
impl crate::Readable for TBBCNT {}
#[doc = "`write(|w| ..)` method takes [tbbcnt::W](tbbcnt::W) writer structure"]
impl crate::Writable for TBBCNT {}
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub mod tbbcnt;
#[doc = "Debounce Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debnce](debnce) module"]
pub type DEBNCE = crate::Reg<u32, _DEBNCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEBNCE;
#[doc = "`read()` method returns [debnce::R](debnce::R) reader structure"]
impl crate::Readable for DEBNCE {}
#[doc = "`write(|w| ..)` method takes [debnce::W](debnce::W) writer structure"]
impl crate::Writable for DEBNCE {}
#[doc = "Debounce Count register"]
pub mod debnce;
#[doc = "Hardware Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_n](rst_n) module"]
pub type RST_N = crate::Reg<u32, _RST_N>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RST_N;
#[doc = "`read()` method returns [rst_n::R](rst_n::R) reader structure"]
impl crate::Readable for RST_N {}
#[doc = "`write(|w| ..)` method takes [rst_n::W](rst_n::W) writer structure"]
impl crate::Writable for RST_N {}
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "Bus Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmod](bmod) module"]
pub type BMOD = crate::Reg<u32, _BMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMOD;
#[doc = "`read()` method returns [bmod::R](bmod::R) reader structure"]
impl crate::Readable for BMOD {}
#[doc = "`write(|w| ..)` method takes [bmod::W](bmod::W) writer structure"]
impl crate::Writable for BMOD {}
#[doc = "Bus Mode register"]
pub mod bmod;
#[doc = "Poll Demand register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pldmnd](pldmnd) module"]
pub type PLDMND = crate::Reg<u32, _PLDMND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLDMND;
#[doc = "`read()` method returns [pldmnd::R](pldmnd::R) reader structure"]
impl crate::Readable for PLDMND {}
#[doc = "`write(|w| ..)` method takes [pldmnd::W](pldmnd::W) writer structure"]
impl crate::Writable for PLDMND {}
#[doc = "Poll Demand register"]
pub mod pldmnd;
#[doc = "Descriptor List Base Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbaddr](dbaddr) module"]
pub type DBADDR = crate::Reg<u32, _DBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBADDR;
#[doc = "`read()` method returns [dbaddr::R](dbaddr::R) reader structure"]
impl crate::Readable for DBADDR {}
#[doc = "`write(|w| ..)` method takes [dbaddr::W](dbaddr::W) writer structure"]
impl crate::Writable for DBADDR {}
#[doc = "Descriptor List Base Address register"]
pub mod dbaddr;
#[doc = "Internal DMAC Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idsts](idsts) module"]
pub type IDSTS = crate::Reg<u32, _IDSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDSTS;
#[doc = "`read()` method returns [idsts::R](idsts::R) reader structure"]
impl crate::Readable for IDSTS {}
#[doc = "`write(|w| ..)` method takes [idsts::W](idsts::W) writer structure"]
impl crate::Writable for IDSTS {}
#[doc = "Internal DMAC Status register"]
pub mod idsts;
#[doc = "Internal DMAC Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idinten](idinten) module"]
pub type IDINTEN = crate::Reg<u32, _IDINTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDINTEN;
#[doc = "`read()` method returns [idinten::R](idinten::R) reader structure"]
impl crate::Readable for IDINTEN {}
#[doc = "`write(|w| ..)` method takes [idinten::W](idinten::W) writer structure"]
impl crate::Writable for IDINTEN {}
#[doc = "Internal DMAC Interrupt Enable register"]
pub mod idinten;
#[doc = "Current Host Descriptor Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscaddr](dscaddr) module"]
pub type DSCADDR = crate::Reg<u32, _DSCADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSCADDR;
#[doc = "`read()` method returns [dscaddr::R](dscaddr::R) reader structure"]
impl crate::Readable for DSCADDR {}
#[doc = "`write(|w| ..)` method takes [dscaddr::W](dscaddr::W) writer structure"]
impl crate::Writable for DSCADDR {}
#[doc = "Current Host Descriptor Address register"]
pub mod dscaddr;
#[doc = "Current Buffer Descriptor Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bufaddr](bufaddr) module"]
pub type BUFADDR = crate::Reg<u32, _BUFADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFADDR;
#[doc = "`read()` method returns [bufaddr::R](bufaddr::R) reader structure"]
impl crate::Readable for BUFADDR {}
#[doc = "`write(|w| ..)` method takes [bufaddr::W](bufaddr::W) writer structure"]
impl crate::Writable for BUFADDR {}
#[doc = "Current Buffer Descriptor Address register"]
pub mod bufaddr;
#[doc = "Card Threshold Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cardthrctl](cardthrctl) module"]
pub type CARDTHRCTL = crate::Reg<u32, _CARDTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CARDTHRCTL;
#[doc = "`read()` method returns [cardthrctl::R](cardthrctl::R) reader structure"]
impl crate::Readable for CARDTHRCTL {}
#[doc = "`write(|w| ..)` method takes [cardthrctl::W](cardthrctl::W) writer structure"]
impl crate::Writable for CARDTHRCTL {}
#[doc = "Card Threshold Control"]
pub mod cardthrctl;
#[doc = "Power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backendpwr](backendpwr) module"]
pub type BACKENDPWR = crate::Reg<u32, _BACKENDPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACKENDPWR;
#[doc = "`read()` method returns [backendpwr::R](backendpwr::R) reader structure"]
impl crate::Readable for BACKENDPWR {}
#[doc = "`write(|w| ..)` method takes [backendpwr::W](backendpwr::W) writer structure"]
impl crate::Writable for BACKENDPWR {}
#[doc = "Power control"]
pub mod backendpwr;
#[doc = "SDIF FIFO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "`write(|w| ..)` method takes [fifo::W](fifo::W) writer structure"]
impl crate::Writable for FIFO {}
#[doc = "SDIF FIFO"]
pub mod fifo;
