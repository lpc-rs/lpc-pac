#[doc = r"Register block"]
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
#[doc = "Interrupt Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ilr](ilr) module"]
pub type ILR = crate::Reg<u32, _ILR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ILR;
#[doc = "`read()` method returns [ilr::R](ilr::R) reader structure"]
impl crate::Readable for ILR {}
#[doc = "`write(|w| ..)` method takes [ilr::W](ilr::W) writer structure"]
impl crate::Writable for ILR {}
#[doc = "Interrupt Location Register"]
pub mod ilr;
#[doc = "Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](ccr) module"]
pub type CCR = crate::Reg<u32, _CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCR;
#[doc = "`read()` method returns [ccr::R](ccr::R) reader structure"]
impl crate::Readable for CCR {}
#[doc = "`write(|w| ..)` method takes [ccr::W](ccr::W) writer structure"]
impl crate::Writable for CCR {}
#[doc = "Clock Control Register"]
pub mod ccr;
#[doc = "Counter Increment Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ciir](ciir) module"]
pub type CIIR = crate::Reg<u32, _CIIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIIR;
#[doc = "`read()` method returns [ciir::R](ciir::R) reader structure"]
impl crate::Readable for CIIR {}
#[doc = "`write(|w| ..)` method takes [ciir::W](ciir::W) writer structure"]
impl crate::Writable for CIIR {}
#[doc = "Counter Increment Interrupt Register"]
pub mod ciir;
#[doc = "Alarm Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amr](amr) module"]
pub type AMR = crate::Reg<u32, _AMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMR;
#[doc = "`read()` method returns [amr::R](amr::R) reader structure"]
impl crate::Readable for AMR {}
#[doc = "`write(|w| ..)` method takes [amr::W](amr::W) writer structure"]
impl crate::Writable for AMR {}
#[doc = "Alarm Mask Register"]
pub mod amr;
#[doc = "Consolidated Time Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime0](ctime0) module"]
pub type CTIME0 = crate::Reg<u32, _CTIME0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIME0;
#[doc = "`read()` method returns [ctime0::R](ctime0::R) reader structure"]
impl crate::Readable for CTIME0 {}
#[doc = "Consolidated Time Register 0"]
pub mod ctime0;
#[doc = "Consolidated Time Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime1](ctime1) module"]
pub type CTIME1 = crate::Reg<u32, _CTIME1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIME1;
#[doc = "`read()` method returns [ctime1::R](ctime1::R) reader structure"]
impl crate::Readable for CTIME1 {}
#[doc = "Consolidated Time Register 1"]
pub mod ctime1;
#[doc = "Consolidated Time Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctime2](ctime2) module"]
pub type CTIME2 = crate::Reg<u32, _CTIME2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTIME2;
#[doc = "`read()` method returns [ctime2::R](ctime2::R) reader structure"]
impl crate::Readable for CTIME2 {}
#[doc = "Consolidated Time Register 2"]
pub mod ctime2;
#[doc = "Seconds Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec](sec) module"]
pub type SEC = crate::Reg<u32, _SEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEC;
#[doc = "`read()` method returns [sec::R](sec::R) reader structure"]
impl crate::Readable for SEC {}
#[doc = "`write(|w| ..)` method takes [sec::W](sec::W) writer structure"]
impl crate::Writable for SEC {}
#[doc = "Seconds Counter"]
pub mod sec;
#[doc = "Minutes Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [min](min) module"]
pub type MIN = crate::Reg<u32, _MIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIN;
#[doc = "`read()` method returns [min::R](min::R) reader structure"]
impl crate::Readable for MIN {}
#[doc = "`write(|w| ..)` method takes [min::W](min::W) writer structure"]
impl crate::Writable for MIN {}
#[doc = "Minutes Register"]
pub mod min;
#[doc = "Hours Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](hrs) module"]
pub type HRS = crate::Reg<u32, _HRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRS;
#[doc = "`read()` method returns [hrs::R](hrs::R) reader structure"]
impl crate::Readable for HRS {}
#[doc = "`write(|w| ..)` method takes [hrs::W](hrs::W) writer structure"]
impl crate::Writable for HRS {}
#[doc = "Hours Register"]
pub mod hrs;
#[doc = "Day of Month Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dom](dom) module"]
pub type DOM = crate::Reg<u32, _DOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOM;
#[doc = "`read()` method returns [dom::R](dom::R) reader structure"]
impl crate::Readable for DOM {}
#[doc = "`write(|w| ..)` method takes [dom::W](dom::W) writer structure"]
impl crate::Writable for DOM {}
#[doc = "Day of Month Register"]
pub mod dom;
#[doc = "Day of Week Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dow](dow) module"]
pub type DOW = crate::Reg<u32, _DOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOW;
#[doc = "`read()` method returns [dow::R](dow::R) reader structure"]
impl crate::Readable for DOW {}
#[doc = "`write(|w| ..)` method takes [dow::W](dow::W) writer structure"]
impl crate::Writable for DOW {}
#[doc = "Day of Week Register"]
pub mod dow;
#[doc = "Day of Year Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doy](doy) module"]
pub type DOY = crate::Reg<u32, _DOY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOY;
#[doc = "`read()` method returns [doy::R](doy::R) reader structure"]
impl crate::Readable for DOY {}
#[doc = "`write(|w| ..)` method takes [doy::W](doy::W) writer structure"]
impl crate::Writable for DOY {}
#[doc = "Day of Year Register"]
pub mod doy;
#[doc = "Months Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [month](month) module"]
pub type MONTH = crate::Reg<u32, _MONTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MONTH;
#[doc = "`read()` method returns [month::R](month::R) reader structure"]
impl crate::Readable for MONTH {}
#[doc = "`write(|w| ..)` method takes [month::W](month::W) writer structure"]
impl crate::Writable for MONTH {}
#[doc = "Months Register"]
pub mod month;
#[doc = "Years Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [year](year) module"]
pub type YEAR = crate::Reg<u32, _YEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _YEAR;
#[doc = "`read()` method returns [year::R](year::R) reader structure"]
impl crate::Readable for YEAR {}
#[doc = "`write(|w| ..)` method takes [year::W](year::W) writer structure"]
impl crate::Writable for YEAR {}
#[doc = "Years Register"]
pub mod year;
#[doc = "Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calibration](calibration) module"]
pub type CALIBRATION = crate::Reg<u32, _CALIBRATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CALIBRATION;
#[doc = "`read()` method returns [calibration::R](calibration::R) reader structure"]
impl crate::Readable for CALIBRATION {}
#[doc = "`write(|w| ..)` method takes [calibration::W](calibration::W) writer structure"]
impl crate::Writable for CALIBRATION {}
#[doc = "Calibration Value Register"]
pub mod calibration;
#[doc = "General Purpose Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpreg](gpreg) module"]
pub type GPREG = crate::Reg<u32, _GPREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREG;
#[doc = "`read()` method returns [gpreg::R](gpreg::R) reader structure"]
impl crate::Readable for GPREG {}
#[doc = "`write(|w| ..)` method takes [gpreg::W](gpreg::W) writer structure"]
impl crate::Writable for GPREG {}
#[doc = "General Purpose Register 0"]
pub mod gpreg;
#[doc = "RTC Auxiliary control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_aux](rtc_aux) module"]
pub type RTC_AUX = crate::Reg<u32, _RTC_AUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_AUX;
#[doc = "`read()` method returns [rtc_aux::R](rtc_aux::R) reader structure"]
impl crate::Readable for RTC_AUX {}
#[doc = "`write(|w| ..)` method takes [rtc_aux::W](rtc_aux::W) writer structure"]
impl crate::Writable for RTC_AUX {}
#[doc = "RTC Auxiliary control register"]
pub mod rtc_aux;
#[doc = "RTC Auxiliary Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_auxen](rtc_auxen) module"]
pub type RTC_AUXEN = crate::Reg<u32, _RTC_AUXEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_AUXEN;
#[doc = "`read()` method returns [rtc_auxen::R](rtc_auxen::R) reader structure"]
impl crate::Readable for RTC_AUXEN {}
#[doc = "`write(|w| ..)` method takes [rtc_auxen::W](rtc_auxen::W) writer structure"]
impl crate::Writable for RTC_AUXEN {}
#[doc = "RTC Auxiliary Enable register"]
pub mod rtc_auxen;
#[doc = "Alarm value for Seconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asec](asec) module"]
pub type ASEC = crate::Reg<u32, _ASEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASEC;
#[doc = "`read()` method returns [asec::R](asec::R) reader structure"]
impl crate::Readable for ASEC {}
#[doc = "`write(|w| ..)` method takes [asec::W](asec::W) writer structure"]
impl crate::Writable for ASEC {}
#[doc = "Alarm value for Seconds"]
pub mod asec;
#[doc = "Alarm value for Minutes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amin](amin) module"]
pub type AMIN = crate::Reg<u32, _AMIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMIN;
#[doc = "`read()` method returns [amin::R](amin::R) reader structure"]
impl crate::Readable for AMIN {}
#[doc = "`write(|w| ..)` method takes [amin::W](amin::W) writer structure"]
impl crate::Writable for AMIN {}
#[doc = "Alarm value for Minutes"]
pub mod amin;
#[doc = "Alarm value for Hours\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahrs](ahrs) module"]
pub type AHRS = crate::Reg<u32, _AHRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHRS;
#[doc = "`read()` method returns [ahrs::R](ahrs::R) reader structure"]
impl crate::Readable for AHRS {}
#[doc = "`write(|w| ..)` method takes [ahrs::W](ahrs::W) writer structure"]
impl crate::Writable for AHRS {}
#[doc = "Alarm value for Hours"]
pub mod ahrs;
#[doc = "Alarm value for Day of Month\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adom](adom) module"]
pub type ADOM = crate::Reg<u32, _ADOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADOM;
#[doc = "`read()` method returns [adom::R](adom::R) reader structure"]
impl crate::Readable for ADOM {}
#[doc = "`write(|w| ..)` method takes [adom::W](adom::W) writer structure"]
impl crate::Writable for ADOM {}
#[doc = "Alarm value for Day of Month"]
pub mod adom;
#[doc = "Alarm value for Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adow](adow) module"]
pub type ADOW = crate::Reg<u32, _ADOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADOW;
#[doc = "`read()` method returns [adow::R](adow::R) reader structure"]
impl crate::Readable for ADOW {}
#[doc = "`write(|w| ..)` method takes [adow::W](adow::W) writer structure"]
impl crate::Writable for ADOW {}
#[doc = "Alarm value for Day of Week"]
pub mod adow;
#[doc = "Alarm value for Day of Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adoy](adoy) module"]
pub type ADOY = crate::Reg<u32, _ADOY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADOY;
#[doc = "`read()` method returns [adoy::R](adoy::R) reader structure"]
impl crate::Readable for ADOY {}
#[doc = "`write(|w| ..)` method takes [adoy::W](adoy::W) writer structure"]
impl crate::Writable for ADOY {}
#[doc = "Alarm value for Day of Year"]
pub mod adoy;
#[doc = "Alarm value for Months\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amon](amon) module"]
pub type AMON = crate::Reg<u32, _AMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMON;
#[doc = "`read()` method returns [amon::R](amon::R) reader structure"]
impl crate::Readable for AMON {}
#[doc = "`write(|w| ..)` method takes [amon::W](amon::W) writer structure"]
impl crate::Writable for AMON {}
#[doc = "Alarm value for Months"]
pub mod amon;
#[doc = "Alarm value for Year\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ayrs](ayrs) module"]
pub type AYRS = crate::Reg<u32, _AYRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AYRS;
#[doc = "`read()` method returns [ayrs::R](ayrs::R) reader structure"]
impl crate::Readable for AYRS {}
#[doc = "`write(|w| ..)` method takes [ayrs::W](ayrs::W) writer structure"]
impl crate::Writable for AYRS {}
#[doc = "Alarm value for Year"]
pub mod ayrs;
