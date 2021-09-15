#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    #[doc = "0x04 - Power Enable register"]
    pub pwren: crate::Reg<pwren::PWREN_SPEC>,
    #[doc = "0x08 - Clock Divider register"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Clock Enable register"]
    pub clkena: crate::Reg<clkena::CLKENA_SPEC>,
    #[doc = "0x14 - Time-out register"]
    pub tmout: crate::Reg<tmout::TMOUT_SPEC>,
    #[doc = "0x18 - Card Type register"]
    pub ctype: crate::Reg<ctype::CTYPE_SPEC>,
    #[doc = "0x1c - Block Size register"]
    pub blksiz: crate::Reg<blksiz::BLKSIZ_SPEC>,
    #[doc = "0x20 - Byte Count register"]
    pub bytcnt: crate::Reg<bytcnt::BYTCNT_SPEC>,
    #[doc = "0x24 - Interrupt Mask register"]
    pub intmask: crate::Reg<intmask::INTMASK_SPEC>,
    #[doc = "0x28 - Command Argument register"]
    pub cmdarg: crate::Reg<cmdarg::CMDARG_SPEC>,
    #[doc = "0x2c - Command register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x30..0x40 - Response register"]
    pub resp: [crate::Reg<resp::RESP_SPEC>; 4],
    #[doc = "0x40 - Masked Interrupt Status register"]
    pub mintsts: crate::Reg<mintsts::MINTSTS_SPEC>,
    #[doc = "0x44 - Raw Interrupt Status register"]
    pub rintsts: crate::Reg<rintsts::RINTSTS_SPEC>,
    #[doc = "0x48 - Status register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x4c - FIFO Threshold Watermark register"]
    pub fifoth: crate::Reg<fifoth::FIFOTH_SPEC>,
    #[doc = "0x50 - Card Detect register"]
    pub cdetect: crate::Reg<cdetect::CDETECT_SPEC>,
    #[doc = "0x54 - Write Protect register"]
    pub wrtprt: crate::Reg<wrtprt::WRTPRT_SPEC>,
    _reserved18: [u8; 0x04],
    #[doc = "0x5c - Transferred CIU Card Byte Count register"]
    pub tcbcnt: crate::Reg<tcbcnt::TCBCNT_SPEC>,
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count register"]
    pub tbbcnt: crate::Reg<tbbcnt::TBBCNT_SPEC>,
    #[doc = "0x64 - Debounce Count register"]
    pub debnce: crate::Reg<debnce::DEBNCE_SPEC>,
    _reserved21: [u8; 0x10],
    #[doc = "0x78 - Hardware Reset"]
    pub rst_n: crate::Reg<rst_n::RST_N_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x80 - Bus Mode register"]
    pub bmod: crate::Reg<bmod::BMOD_SPEC>,
    #[doc = "0x84 - Poll Demand register"]
    pub pldmnd: crate::Reg<pldmnd::PLDMND_SPEC>,
    #[doc = "0x88 - Descriptor List Base Address register"]
    pub dbaddr: crate::Reg<dbaddr::DBADDR_SPEC>,
    #[doc = "0x8c - Internal DMAC Status register"]
    pub idsts: crate::Reg<idsts::IDSTS_SPEC>,
    #[doc = "0x90 - Internal DMAC Interrupt Enable register"]
    pub idinten: crate::Reg<idinten::IDINTEN_SPEC>,
    #[doc = "0x94 - Current Host Descriptor Address register"]
    pub dscaddr: crate::Reg<dscaddr::DSCADDR_SPEC>,
    #[doc = "0x98 - Current Buffer Descriptor Address register"]
    pub bufaddr: crate::Reg<bufaddr::BUFADDR_SPEC>,
    _reserved29: [u8; 0x64],
    #[doc = "0x100 - Card Threshold Control"]
    pub cardthrctl: crate::Reg<cardthrctl::CARDTHRCTL_SPEC>,
    #[doc = "0x104 - Power control"]
    pub backendpwr: crate::Reg<backendpwr::BACKENDPWR_SPEC>,
    _reserved31: [u8; 0xf8],
    #[doc = "0x200..0x300 - SDIF FIFO"]
    pub fifo: [crate::Reg<fifo::FIFO_SPEC>; 64],
}
#[doc = "CTRL register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "PWREN register accessor: an alias for `Reg<PWREN_SPEC>`"]
pub type PWREN = crate::Reg<pwren::PWREN_SPEC>;
#[doc = "Power Enable register"]
pub mod pwren;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "CLKENA register accessor: an alias for `Reg<CLKENA_SPEC>`"]
pub type CLKENA = crate::Reg<clkena::CLKENA_SPEC>;
#[doc = "Clock Enable register"]
pub mod clkena;
#[doc = "TMOUT register accessor: an alias for `Reg<TMOUT_SPEC>`"]
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "CTYPE register accessor: an alias for `Reg<CTYPE_SPEC>`"]
pub type CTYPE = crate::Reg<ctype::CTYPE_SPEC>;
#[doc = "Card Type register"]
pub mod ctype;
#[doc = "BLKSIZ register accessor: an alias for `Reg<BLKSIZ_SPEC>`"]
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
#[doc = "Block Size register"]
pub mod blksiz;
#[doc = "BYTCNT register accessor: an alias for `Reg<BYTCNT_SPEC>`"]
pub type BYTCNT = crate::Reg<bytcnt::BYTCNT_SPEC>;
#[doc = "Byte Count register"]
pub mod bytcnt;
#[doc = "INTMASK register accessor: an alias for `Reg<INTMASK_SPEC>`"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "Interrupt Mask register"]
pub mod intmask;
#[doc = "CMDARG register accessor: an alias for `Reg<CMDARG_SPEC>`"]
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "RESP register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "Response register"]
pub mod resp;
#[doc = "MINTSTS register accessor: an alias for `Reg<MINTSTS_SPEC>`"]
pub type MINTSTS = crate::Reg<mintsts::MINTSTS_SPEC>;
#[doc = "Masked Interrupt Status register"]
pub mod mintsts;
#[doc = "RINTSTS register accessor: an alias for `Reg<RINTSTS_SPEC>`"]
pub type RINTSTS = crate::Reg<rintsts::RINTSTS_SPEC>;
#[doc = "Raw Interrupt Status register"]
pub mod rintsts;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "FIFOTH register accessor: an alias for `Reg<FIFOTH_SPEC>`"]
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
#[doc = "FIFO Threshold Watermark register"]
pub mod fifoth;
#[doc = "CDETECT register accessor: an alias for `Reg<CDETECT_SPEC>`"]
pub type CDETECT = crate::Reg<cdetect::CDETECT_SPEC>;
#[doc = "Card Detect register"]
pub mod cdetect;
#[doc = "WRTPRT register accessor: an alias for `Reg<WRTPRT_SPEC>`"]
pub type WRTPRT = crate::Reg<wrtprt::WRTPRT_SPEC>;
#[doc = "Write Protect register"]
pub mod wrtprt;
#[doc = "TCBCNT register accessor: an alias for `Reg<TCBCNT_SPEC>`"]
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
#[doc = "Transferred CIU Card Byte Count register"]
pub mod tcbcnt;
#[doc = "TBBCNT register accessor: an alias for `Reg<TBBCNT_SPEC>`"]
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub mod tbbcnt;
#[doc = "DEBNCE register accessor: an alias for `Reg<DEBNCE_SPEC>`"]
pub type DEBNCE = crate::Reg<debnce::DEBNCE_SPEC>;
#[doc = "Debounce Count register"]
pub mod debnce;
#[doc = "RST_N register accessor: an alias for `Reg<RST_N_SPEC>`"]
pub type RST_N = crate::Reg<rst_n::RST_N_SPEC>;
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "BMOD register accessor: an alias for `Reg<BMOD_SPEC>`"]
pub type BMOD = crate::Reg<bmod::BMOD_SPEC>;
#[doc = "Bus Mode register"]
pub mod bmod;
#[doc = "PLDMND register accessor: an alias for `Reg<PLDMND_SPEC>`"]
pub type PLDMND = crate::Reg<pldmnd::PLDMND_SPEC>;
#[doc = "Poll Demand register"]
pub mod pldmnd;
#[doc = "DBADDR register accessor: an alias for `Reg<DBADDR_SPEC>`"]
pub type DBADDR = crate::Reg<dbaddr::DBADDR_SPEC>;
#[doc = "Descriptor List Base Address register"]
pub mod dbaddr;
#[doc = "IDSTS register accessor: an alias for `Reg<IDSTS_SPEC>`"]
pub type IDSTS = crate::Reg<idsts::IDSTS_SPEC>;
#[doc = "Internal DMAC Status register"]
pub mod idsts;
#[doc = "IDINTEN register accessor: an alias for `Reg<IDINTEN_SPEC>`"]
pub type IDINTEN = crate::Reg<idinten::IDINTEN_SPEC>;
#[doc = "Internal DMAC Interrupt Enable register"]
pub mod idinten;
#[doc = "DSCADDR register accessor: an alias for `Reg<DSCADDR_SPEC>`"]
pub type DSCADDR = crate::Reg<dscaddr::DSCADDR_SPEC>;
#[doc = "Current Host Descriptor Address register"]
pub mod dscaddr;
#[doc = "BUFADDR register accessor: an alias for `Reg<BUFADDR_SPEC>`"]
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
#[doc = "Current Buffer Descriptor Address register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL register accessor: an alias for `Reg<CARDTHRCTL_SPEC>`"]
pub type CARDTHRCTL = crate::Reg<cardthrctl::CARDTHRCTL_SPEC>;
#[doc = "Card Threshold Control"]
pub mod cardthrctl;
#[doc = "BACKENDPWR register accessor: an alias for `Reg<BACKENDPWR_SPEC>`"]
pub type BACKENDPWR = crate::Reg<backendpwr::BACKENDPWR_SPEC>;
#[doc = "Power control"]
pub mod backendpwr;
#[doc = "FIFO register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "SDIF FIFO"]
pub mod fifo;
