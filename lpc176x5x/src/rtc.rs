#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Location Register"]
    pub ilr: ILR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Clock Control Register"]
    pub ccr: CCR,
    #[doc = "0x0c - Counter Increment Interrupt Register"]
    pub ciir: CIIR,
    #[doc = "0x10 - Alarm Mask Register"]
    pub amr: AMR,
    #[doc = "0x14 - Consolidated Time Register 0"]
    pub ctime0: CTIME0,
    #[doc = "0x18 - Consolidated Time Register 1"]
    pub ctime1: CTIME1,
    #[doc = "0x1c - Consolidated Time Register 2"]
    pub ctime2: CTIME2,
    #[doc = "0x20 - Seconds Counter"]
    pub sec: SEC,
    #[doc = "0x24 - Minutes Register"]
    pub min: MIN,
    #[doc = "0x28 - Hours Register"]
    pub hrs: HRS,
    #[doc = "0x2c - Day of Month Register"]
    pub dom: DOM,
    #[doc = "0x30 - Day of Week Register"]
    pub dow: DOW,
    #[doc = "0x34 - Day of Year Register"]
    pub doy: DOY,
    #[doc = "0x38 - Months Register"]
    pub month: MONTH,
    #[doc = "0x3c - Years Register"]
    pub year: YEAR,
    #[doc = "0x40 - Calibration Value Register"]
    pub calibration: CALIBRATION,
    #[doc = "0x44 - General Purpose Register 0"]
    pub gpreg: [GPREG; 5],
    #[doc = "0x58 - RTC Auxiliary Enable register"]
    pub rtc_auxen: RTC_AUXEN,
    #[doc = "0x5c - RTC Auxiliary control register"]
    pub rtc_aux: RTC_AUX,
    #[doc = "0x60 - Alarm value for Seconds"]
    pub asec: ASEC,
    #[doc = "0x64 - Alarm value for Minutes"]
    pub amin: AMIN,
    #[doc = "0x68 - Alarm value for Hours"]
    pub ahrs: AHRS,
    #[doc = "0x6c - Alarm value for Day of Month"]
    pub adom: ADOM,
    #[doc = "0x70 - Alarm value for Day of Week"]
    pub adow: ADOW,
    #[doc = "0x74 - Alarm value for Day of Year"]
    pub adoy: ADOY,
    #[doc = "0x78 - Alarm value for Months"]
    pub amon: AMON,
    #[doc = "0x7c - Alarm value for Year"]
    pub ayrs: AYRS,
}
#[doc = "Interrupt Location Register"]
pub struct ILR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Location Register"]
pub mod ilr;
#[doc = "Clock Control Register"]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Control Register"]
pub mod ccr;
#[doc = "Counter Increment Interrupt Register"]
pub struct CIIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Counter Increment Interrupt Register"]
pub mod ciir;
#[doc = "Alarm Mask Register"]
pub struct AMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Mask Register"]
pub mod amr;
#[doc = "Consolidated Time Register 0"]
pub struct CTIME0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Consolidated Time Register 0"]
pub mod ctime0;
#[doc = "Consolidated Time Register 1"]
pub struct CTIME1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Consolidated Time Register 1"]
pub mod ctime1;
#[doc = "Consolidated Time Register 2"]
pub struct CTIME2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Consolidated Time Register 2"]
pub mod ctime2;
#[doc = "Seconds Counter"]
pub struct SEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Seconds Counter"]
pub mod sec;
#[doc = "Minutes Register"]
pub struct MIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Minutes Register"]
pub mod min;
#[doc = "Hours Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hours Register"]
pub mod hrs;
#[doc = "Day of Month Register"]
pub struct DOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Day of Month Register"]
pub mod dom;
#[doc = "Day of Week Register"]
pub struct DOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Day of Week Register"]
pub mod dow;
#[doc = "Day of Year Register"]
pub struct DOY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Day of Year Register"]
pub mod doy;
#[doc = "Months Register"]
pub struct MONTH {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Months Register"]
pub mod month;
#[doc = "Years Register"]
pub struct YEAR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Years Register"]
pub mod year;
#[doc = "Calibration Value Register"]
pub struct CALIBRATION {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calibration Value Register"]
pub mod calibration;
#[doc = "General Purpose Register 0"]
pub struct GPREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Register 0"]
pub mod gpreg;
#[doc = "RTC Auxiliary control register"]
pub struct RTC_AUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Auxiliary control register"]
pub mod rtc_aux;
#[doc = "RTC Auxiliary Enable register"]
pub struct RTC_AUXEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RTC Auxiliary Enable register"]
pub mod rtc_auxen;
#[doc = "Alarm value for Seconds"]
pub struct ASEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Seconds"]
pub mod asec;
#[doc = "Alarm value for Minutes"]
pub struct AMIN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Minutes"]
pub mod amin;
#[doc = "Alarm value for Hours"]
pub struct AHRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Hours"]
pub mod ahrs;
#[doc = "Alarm value for Day of Month"]
pub struct ADOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Day of Month"]
pub mod adom;
#[doc = "Alarm value for Day of Week"]
pub struct ADOW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Day of Week"]
pub mod adow;
#[doc = "Alarm value for Day of Year"]
pub struct ADOY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Day of Year"]
pub mod adoy;
#[doc = "Alarm value for Months"]
pub struct AMON {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Months"]
pub mod amon;
#[doc = "Alarm value for Year"]
pub struct AYRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm value for Year"]
pub mod ayrs;
