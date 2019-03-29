#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub con: CON,
    #[doc = "0x04 - Status register"]
    pub stat: STAT,
    #[doc = "0x08 - Configuration register"]
    pub conf: CONF,
    #[doc = "0x0c - Position register"]
    pub pos: POS,
    #[doc = "0x10 - Maximum position register"]
    pub maxpos: MAXPOS,
    #[doc = "0x14 - Position compare register 0"]
    pub cmpos0: CMPOS0,
    #[doc = "0x18 - Position compare register 1"]
    pub cmpos1: CMPOS1,
    #[doc = "0x1c - Position compare register 2"]
    pub cmpos2: CMPOS2,
    #[doc = "0x20 - Index count register 0"]
    pub inxcnt: INXCNT,
    #[doc = "0x24 - Index compare register 0"]
    pub inxcmp0: INXCMP0,
    #[doc = "0x28 - Velocity timer reload register"]
    pub load: LOAD,
    #[doc = "0x2c - Velocity timer register"]
    pub time: TIME,
    #[doc = "0x30 - Velocity counter register"]
    pub vel: VEL,
    #[doc = "0x34 - Velocity capture register"]
    pub cap: CAP,
    #[doc = "0x38 - Velocity compare register"]
    pub velcomp: VELCOMP,
    #[doc = "0x3c - Digital filter register"]
    pub filter: FILTER,
    _reserved16: [u8; 3992usize],
    #[doc = "0xfd8 - Interrupt enable clear register"]
    pub iec: IEC,
    #[doc = "0xfdc - Interrupt enable set register"]
    pub ies: IES,
    #[doc = "0xfe0 - Interrupt status register"]
    pub intstat: INTSTAT,
    #[doc = "0xfe4 - Interrupt enable register"]
    pub ie: IE,
    #[doc = "0xfe8 - Interrupt status clear register"]
    pub clr: CLR,
    #[doc = "0xfec - Interrupt status set register"]
    pub set: SET,
}
#[doc = "Control register"]
pub struct CON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control register"]
pub mod con;
#[doc = "Configuration register"]
pub struct CONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod conf;
#[doc = "Status register"]
pub struct STAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status register"]
pub mod stat;
#[doc = "Position register"]
pub struct POS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Position register"]
pub mod pos;
#[doc = "Maximum position register"]
pub struct MAXPOS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum position register"]
pub mod maxpos;
#[doc = "Position compare register 0"]
pub struct CMPOS0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Position compare register 0"]
pub mod cmpos0;
#[doc = "Position compare register 1"]
pub struct CMPOS1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Position compare register 1"]
pub mod cmpos1;
#[doc = "Position compare register 2"]
pub struct CMPOS2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Position compare register 2"]
pub mod cmpos2;
#[doc = "Index count register 0"]
pub struct INXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index count register 0"]
pub mod inxcnt;
#[doc = "Index compare register 0"]
pub struct INXCMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Index compare register 0"]
pub mod inxcmp0;
#[doc = "Velocity timer reload register"]
pub struct LOAD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity timer reload register"]
pub mod load;
#[doc = "Velocity timer register"]
pub struct TIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity timer register"]
pub mod time;
#[doc = "Velocity counter register"]
pub struct VEL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity counter register"]
pub mod vel;
#[doc = "Velocity capture register"]
pub struct CAP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity capture register"]
pub mod cap;
#[doc = "Velocity compare register"]
pub struct VELCOMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Velocity compare register"]
pub mod velcomp;
#[doc = "Digital filter register"]
pub struct FILTER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Digital filter register"]
pub mod filter;
#[doc = "Interrupt status register"]
pub struct INTSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "Interrupt status set register"]
pub struct SET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status set register"]
pub mod set;
#[doc = "Interrupt status clear register"]
pub struct CLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status clear register"]
pub mod clr;
#[doc = "Interrupt enable register"]
pub struct IE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable register"]
pub mod ie;
#[doc = "Interrupt enable set register"]
pub struct IES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register"]
pub mod ies;
#[doc = "Interrupt enable clear register"]
pub struct IEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register"]
pub mod iec;
