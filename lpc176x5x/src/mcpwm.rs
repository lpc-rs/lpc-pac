#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Control read address"]
    pub con: CON,
    #[doc = "0x04 - PWM Control set address"]
    pub con_set: CON_SET,
    #[doc = "0x08 - PWM Control clear address"]
    pub con_clr: CON_CLR,
    #[doc = "0x0c - Capture Control read address"]
    pub capcon: CAPCON,
    #[doc = "0x10 - Capture Control set address"]
    pub capcon_set: CAPCON_SET,
    #[doc = "0x14 - Event Control clear address"]
    pub capcon_clr: CAPCON_CLR,
    #[doc = "0x18 - Timer Counter register"]
    pub tc: [TC; 3],
    #[doc = "0x24 - Limit register"]
    pub lim: [LIM; 3],
    #[doc = "0x30 - Match register"]
    pub mat: [MAT; 3],
    #[doc = "0x3c - Dead time register"]
    pub dt: DT,
    #[doc = "0x40 - Communication Pattern register"]
    pub cp: CP,
    #[doc = "0x44 - Capture register"]
    pub cap: [CAP; 3],
    #[doc = "0x50 - Interrupt Enable read address"]
    pub inten: INTEN,
    #[doc = "0x54 - Interrupt Enable set address"]
    pub inten_set: INTEN_SET,
    #[doc = "0x58 - Interrupt Enable clear address"]
    pub inten_clr: INTEN_CLR,
    #[doc = "0x5c - Count Control read address"]
    pub cntcon: CNTCON,
    #[doc = "0x60 - Count Control set address"]
    pub cntcon_set: CNTCON_SET,
    #[doc = "0x64 - Count Control clear address"]
    pub cntcon_clr: CNTCON_CLR,
    #[doc = "0x68 - Interrupt flags read address"]
    pub intf: INTF,
    #[doc = "0x6c - Interrupt flags set address"]
    pub intf_set: INTF_SET,
    #[doc = "0x70 - Interrupt flags clear address"]
    pub intf_clr: INTF_CLR,
    #[doc = "0x74 - Capture clear address"]
    pub cap_clr: CAP_CLR,
}
#[doc = "PWM Control read address"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control read address"]
pub mod con;
#[doc = "PWM Control set address"]
pub struct CON_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control set address"]
pub mod con_set;
#[doc = "PWM Control clear address"]
pub struct CON_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control clear address"]
pub mod con_clr;
#[doc = "Capture Control read address"]
pub struct CAPCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control read address"]
pub mod capcon;
#[doc = "Capture Control set address"]
pub struct CAPCON_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control set address"]
pub mod capcon_set;
#[doc = "Event Control clear address"]
pub struct CAPCON_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Event Control clear address"]
pub mod capcon_clr;
#[doc = "Timer Counter register"]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Counter register"]
pub mod tc;
#[doc = "Limit register"]
pub struct LIM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Limit register"]
pub mod lim;
#[doc = "Match register"]
pub struct MAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match register"]
pub mod mat;
#[doc = "Dead time register"]
pub struct DT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Dead time register"]
pub mod dt;
#[doc = "Communication Pattern register"]
pub struct CP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Communication Pattern register"]
pub mod cp;
#[doc = "Capture register"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture register"]
pub mod cap;
#[doc = "Interrupt Enable read address"]
pub struct INTEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable read address"]
pub mod inten;
#[doc = "Interrupt Enable set address"]
pub struct INTEN_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable set address"]
pub mod inten_set;
#[doc = "Interrupt Enable clear address"]
pub struct INTEN_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable clear address"]
pub mod inten_clr;
#[doc = "Interrupt flags read address"]
pub struct INTF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt flags read address"]
pub mod intf;
#[doc = "Interrupt flags set address"]
pub struct INTF_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt flags set address"]
pub mod intf_set;
#[doc = "Interrupt flags clear address"]
pub struct INTF_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt flags clear address"]
pub mod intf_clr;
#[doc = "Count Control read address"]
pub struct CNTCON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control read address"]
pub mod cntcon;
#[doc = "Count Control set address"]
pub struct CNTCON_SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control set address"]
pub mod cntcon_set;
#[doc = "Count Control clear address"]
pub struct CNTCON_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control clear address"]
pub mod cntcon_clr;
#[doc = "Capture clear address"]
pub struct CAP_CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture clear address"]
pub mod cap_clr;
