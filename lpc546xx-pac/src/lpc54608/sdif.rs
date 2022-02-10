///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub ctrl: crate::Reg<ctrl::CTRL_SPEC>,
    ///0x04 - Power Enable register
    pub pwren: crate::Reg<pwren::PWREN_SPEC>,
    ///0x08 - Clock Divider register
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    _reserved3: [u8; 0x04],
    ///0x10 - Clock Enable register
    pub clkena: crate::Reg<clkena::CLKENA_SPEC>,
    ///0x14 - Time-out register
    pub tmout: crate::Reg<tmout::TMOUT_SPEC>,
    ///0x18 - Card Type register
    pub ctype: crate::Reg<ctype::CTYPE_SPEC>,
    ///0x1c - Block Size register
    pub blksiz: crate::Reg<blksiz::BLKSIZ_SPEC>,
    ///0x20 - Byte Count register
    pub bytcnt: crate::Reg<bytcnt::BYTCNT_SPEC>,
    ///0x24 - Interrupt Mask register
    pub intmask: crate::Reg<intmask::INTMASK_SPEC>,
    ///0x28 - Command Argument register
    pub cmdarg: crate::Reg<cmdarg::CMDARG_SPEC>,
    ///0x2c - Command register
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    ///0x30..0x40 - Response register
    pub resp: [crate::Reg<resp::RESP_SPEC>; 4],
    ///0x40 - Masked Interrupt Status register
    pub mintsts: crate::Reg<mintsts::MINTSTS_SPEC>,
    ///0x44 - Raw Interrupt Status register
    pub rintsts: crate::Reg<rintsts::RINTSTS_SPEC>,
    ///0x48 - Status register
    pub status: crate::Reg<status::STATUS_SPEC>,
    ///0x4c - FIFO Threshold Watermark register
    pub fifoth: crate::Reg<fifoth::FIFOTH_SPEC>,
    ///0x50 - Card Detect register
    pub cdetect: crate::Reg<cdetect::CDETECT_SPEC>,
    ///0x54 - Write Protect register
    pub wrtprt: crate::Reg<wrtprt::WRTPRT_SPEC>,
    _reserved18: [u8; 0x04],
    ///0x5c - Transferred CIU Card Byte Count register
    pub tcbcnt: crate::Reg<tcbcnt::TCBCNT_SPEC>,
    ///0x60 - Transferred Host to BIU-FIFO Byte Count register
    pub tbbcnt: crate::Reg<tbbcnt::TBBCNT_SPEC>,
    ///0x64 - Debounce Count register
    pub debnce: crate::Reg<debnce::DEBNCE_SPEC>,
    _reserved21: [u8; 0x10],
    ///0x78 - Hardware Reset
    pub rst_n: crate::Reg<rst_n::RST_N_SPEC>,
    _reserved22: [u8; 0x04],
    ///0x80 - Bus Mode register
    pub bmod: crate::Reg<bmod::BMOD_SPEC>,
    ///0x84 - Poll Demand register
    pub pldmnd: crate::Reg<pldmnd::PLDMND_SPEC>,
    ///0x88 - Descriptor List Base Address register
    pub dbaddr: crate::Reg<dbaddr::DBADDR_SPEC>,
    ///0x8c - Internal DMAC Status register
    pub idsts: crate::Reg<idsts::IDSTS_SPEC>,
    ///0x90 - Internal DMAC Interrupt Enable register
    pub idinten: crate::Reg<idinten::IDINTEN_SPEC>,
    ///0x94 - Current Host Descriptor Address register
    pub dscaddr: crate::Reg<dscaddr::DSCADDR_SPEC>,
    ///0x98 - Current Buffer Descriptor Address register
    pub bufaddr: crate::Reg<bufaddr::BUFADDR_SPEC>,
    _reserved29: [u8; 0x64],
    ///0x100 - Card Threshold Control
    pub cardthrctl: crate::Reg<cardthrctl::CARDTHRCTL_SPEC>,
    ///0x104 - Power control
    pub backendpwr: crate::Reg<backendpwr::BACKENDPWR_SPEC>,
    _reserved31: [u8; 0xf8],
    ///0x200..0x300 - SDIF FIFO
    pub fifo: [crate::Reg<fifo::FIFO_SPEC>; 64],
}
///CTRL register accessor: an alias for `Reg<CTRL_SPEC>`
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
///Control register
pub mod ctrl;
///PWREN register accessor: an alias for `Reg<PWREN_SPEC>`
pub type PWREN = crate::Reg<pwren::PWREN_SPEC>;
///Power Enable register
pub mod pwren;
///CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
///Clock Divider register
pub mod clkdiv;
///CLKENA register accessor: an alias for `Reg<CLKENA_SPEC>`
pub type CLKENA = crate::Reg<clkena::CLKENA_SPEC>;
///Clock Enable register
pub mod clkena;
///TMOUT register accessor: an alias for `Reg<TMOUT_SPEC>`
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
///Time-out register
pub mod tmout;
///CTYPE register accessor: an alias for `Reg<CTYPE_SPEC>`
pub type CTYPE = crate::Reg<ctype::CTYPE_SPEC>;
///Card Type register
pub mod ctype;
///BLKSIZ register accessor: an alias for `Reg<BLKSIZ_SPEC>`
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
///Block Size register
pub mod blksiz;
///BYTCNT register accessor: an alias for `Reg<BYTCNT_SPEC>`
pub type BYTCNT = crate::Reg<bytcnt::BYTCNT_SPEC>;
///Byte Count register
pub mod bytcnt;
///INTMASK register accessor: an alias for `Reg<INTMASK_SPEC>`
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
///Interrupt Mask register
pub mod intmask;
///CMDARG register accessor: an alias for `Reg<CMDARG_SPEC>`
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
///Command Argument register
pub mod cmdarg;
///CMD register accessor: an alias for `Reg<CMD_SPEC>`
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
///Command register
pub mod cmd;
///RESP register accessor: an alias for `Reg<RESP_SPEC>`
pub type RESP = crate::Reg<resp::RESP_SPEC>;
///Response register
pub mod resp;
///MINTSTS register accessor: an alias for `Reg<MINTSTS_SPEC>`
pub type MINTSTS = crate::Reg<mintsts::MINTSTS_SPEC>;
///Masked Interrupt Status register
pub mod mintsts;
///RINTSTS register accessor: an alias for `Reg<RINTSTS_SPEC>`
pub type RINTSTS = crate::Reg<rintsts::RINTSTS_SPEC>;
///Raw Interrupt Status register
pub mod rintsts;
///STATUS register accessor: an alias for `Reg<STATUS_SPEC>`
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///Status register
pub mod status;
///FIFOTH register accessor: an alias for `Reg<FIFOTH_SPEC>`
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
///FIFO Threshold Watermark register
pub mod fifoth;
///CDETECT register accessor: an alias for `Reg<CDETECT_SPEC>`
pub type CDETECT = crate::Reg<cdetect::CDETECT_SPEC>;
///Card Detect register
pub mod cdetect;
///WRTPRT register accessor: an alias for `Reg<WRTPRT_SPEC>`
pub type WRTPRT = crate::Reg<wrtprt::WRTPRT_SPEC>;
///Write Protect register
pub mod wrtprt;
///TCBCNT register accessor: an alias for `Reg<TCBCNT_SPEC>`
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
///Transferred CIU Card Byte Count register
pub mod tcbcnt;
///TBBCNT register accessor: an alias for `Reg<TBBCNT_SPEC>`
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
///Transferred Host to BIU-FIFO Byte Count register
pub mod tbbcnt;
///DEBNCE register accessor: an alias for `Reg<DEBNCE_SPEC>`
pub type DEBNCE = crate::Reg<debnce::DEBNCE_SPEC>;
///Debounce Count register
pub mod debnce;
///RST_N register accessor: an alias for `Reg<RST_N_SPEC>`
pub type RST_N = crate::Reg<rst_n::RST_N_SPEC>;
///Hardware Reset
pub mod rst_n;
///BMOD register accessor: an alias for `Reg<BMOD_SPEC>`
pub type BMOD = crate::Reg<bmod::BMOD_SPEC>;
///Bus Mode register
pub mod bmod;
///PLDMND register accessor: an alias for `Reg<PLDMND_SPEC>`
pub type PLDMND = crate::Reg<pldmnd::PLDMND_SPEC>;
///Poll Demand register
pub mod pldmnd;
///DBADDR register accessor: an alias for `Reg<DBADDR_SPEC>`
pub type DBADDR = crate::Reg<dbaddr::DBADDR_SPEC>;
///Descriptor List Base Address register
pub mod dbaddr;
///IDSTS register accessor: an alias for `Reg<IDSTS_SPEC>`
pub type IDSTS = crate::Reg<idsts::IDSTS_SPEC>;
///Internal DMAC Status register
pub mod idsts;
///IDINTEN register accessor: an alias for `Reg<IDINTEN_SPEC>`
pub type IDINTEN = crate::Reg<idinten::IDINTEN_SPEC>;
///Internal DMAC Interrupt Enable register
pub mod idinten;
///DSCADDR register accessor: an alias for `Reg<DSCADDR_SPEC>`
pub type DSCADDR = crate::Reg<dscaddr::DSCADDR_SPEC>;
///Current Host Descriptor Address register
pub mod dscaddr;
///BUFADDR register accessor: an alias for `Reg<BUFADDR_SPEC>`
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
///Current Buffer Descriptor Address register
pub mod bufaddr;
///CARDTHRCTL register accessor: an alias for `Reg<CARDTHRCTL_SPEC>`
pub type CARDTHRCTL = crate::Reg<cardthrctl::CARDTHRCTL_SPEC>;
///Card Threshold Control
pub mod cardthrctl;
///BACKENDPWR register accessor: an alias for `Reg<BACKENDPWR_SPEC>`
pub type BACKENDPWR = crate::Reg<backendpwr::BACKENDPWR_SPEC>;
///Power control
pub mod backendpwr;
///FIFO register accessor: an alias for `Reg<FIFO_SPEC>`
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
///SDIF FIFO
pub mod fifo;
