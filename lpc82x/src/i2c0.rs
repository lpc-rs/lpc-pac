#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Configuration for shared functions."]
    pub cfg: CFG,
    #[doc = "0x04 - Status register for Master, Slave, and Monitor functions."]
    pub stat: STAT,
    #[doc = "0x08 - Interrupt Enable Set and read register."]
    pub intenset: INTENSET,
    #[doc = "0x0c - Interrupt Enable Clear register."]
    pub intenclr: INTENCLR,
    #[doc = "0x10 - Time-out value register."]
    pub timeout: TIMEOUT,
    #[doc = "0x14 - Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
    pub clkdiv: CLKDIV,
    #[doc = "0x18 - Interrupt Status register for Master, Slave, and Monitor functions."]
    pub intstat: INTSTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Master control register."]
    pub mstctl: MSTCTL,
    #[doc = "0x24 - Master timing configuration."]
    pub msttime: MSTTIME,
    #[doc = "0x28 - Combined Master receiver and transmitter data register."]
    pub mstdat: MSTDAT,
    _reserved10: [u8; 20usize],
    #[doc = "0x40 - Slave control register."]
    pub slvctl: SLVCTL,
    #[doc = "0x44 - Combined Slave receiver and transmitter data register."]
    pub slvdat: SLVDAT,
    #[doc = "0x48 - Slave address register."]
    pub slvadr: [SLVADR; 4],
    #[doc = "0x58 - Slave Qualification for address 0."]
    pub slvqual0: SLVQUAL0,
    _reserved14: [u8; 36usize],
    #[doc = "0x80 - Monitor receiver data register."]
    pub monrxdat: MONRXDAT,
}
#[doc = "Configuration for shared functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Configuration for shared functions."]
pub mod cfg;
#[doc = "Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register for Master, Slave, and Monitor functions."]
pub mod stat;
#[doc = "Interrupt Enable Set and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt Enable Set and read register."]
pub mod intenset;
#[doc = "Interrupt Enable Clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt Enable Clear register."]
pub mod intenclr;
#[doc = "Time-out value register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeout](timeout) module"]
pub type TIMEOUT = crate::Reg<u32, _TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEOUT;
#[doc = "`read()` method returns [timeout::R](timeout::R) reader structure"]
impl crate::Readable for TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [timeout::W](timeout::W) writer structure"]
impl crate::Writable for TIMEOUT {}
#[doc = "Time-out value register."]
pub mod timeout;
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv](clkdiv) module"]
pub type CLKDIV = crate::Reg<u32, _CLKDIV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLKDIV;
#[doc = "`read()` method returns [clkdiv::R](clkdiv::R) reader structure"]
impl crate::Readable for CLKDIV {}
#[doc = "`write(|w| ..)` method takes [clkdiv::W](clkdiv::W) writer structure"]
impl crate::Writable for CLKDIV {}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
pub mod clkdiv;
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](intstat) module"]
pub type INTSTAT = crate::Reg<u32, _INTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTSTAT;
#[doc = "`read()` method returns [intstat::R](intstat::R) reader structure"]
impl crate::Readable for INTSTAT {}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
pub mod intstat;
#[doc = "Master control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstctl](mstctl) module"]
pub type MSTCTL = crate::Reg<u32, _MSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTCTL;
#[doc = "`read()` method returns [mstctl::R](mstctl::R) reader structure"]
impl crate::Readable for MSTCTL {}
#[doc = "`write(|w| ..)` method takes [mstctl::W](mstctl::W) writer structure"]
impl crate::Writable for MSTCTL {}
#[doc = "Master control register."]
pub mod mstctl;
#[doc = "Master timing configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msttime](msttime) module"]
pub type MSTTIME = crate::Reg<u32, _MSTTIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTTIME;
#[doc = "`read()` method returns [msttime::R](msttime::R) reader structure"]
impl crate::Readable for MSTTIME {}
#[doc = "`write(|w| ..)` method takes [msttime::W](msttime::W) writer structure"]
impl crate::Writable for MSTTIME {}
#[doc = "Master timing configuration."]
pub mod msttime;
#[doc = "Combined Master receiver and transmitter data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstdat](mstdat) module"]
pub type MSTDAT = crate::Reg<u32, _MSTDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSTDAT;
#[doc = "`read()` method returns [mstdat::R](mstdat::R) reader structure"]
impl crate::Readable for MSTDAT {}
#[doc = "`write(|w| ..)` method takes [mstdat::W](mstdat::W) writer structure"]
impl crate::Writable for MSTDAT {}
#[doc = "Combined Master receiver and transmitter data register."]
pub mod mstdat;
#[doc = "Slave control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvctl](slvctl) module"]
pub type SLVCTL = crate::Reg<u32, _SLVCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVCTL;
#[doc = "`read()` method returns [slvctl::R](slvctl::R) reader structure"]
impl crate::Readable for SLVCTL {}
#[doc = "`write(|w| ..)` method takes [slvctl::W](slvctl::W) writer structure"]
impl crate::Writable for SLVCTL {}
#[doc = "Slave control register."]
pub mod slvctl;
#[doc = "Combined Slave receiver and transmitter data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvdat](slvdat) module"]
pub type SLVDAT = crate::Reg<u32, _SLVDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVDAT;
#[doc = "`read()` method returns [slvdat::R](slvdat::R) reader structure"]
impl crate::Readable for SLVDAT {}
#[doc = "`write(|w| ..)` method takes [slvdat::W](slvdat::W) writer structure"]
impl crate::Writable for SLVDAT {}
#[doc = "Combined Slave receiver and transmitter data register."]
pub mod slvdat;
#[doc = "Slave address register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvadr](slvadr) module"]
pub type SLVADR = crate::Reg<u32, _SLVADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVADR;
#[doc = "`read()` method returns [slvadr::R](slvadr::R) reader structure"]
impl crate::Readable for SLVADR {}
#[doc = "`write(|w| ..)` method takes [slvadr::W](slvadr::W) writer structure"]
impl crate::Writable for SLVADR {}
#[doc = "Slave address register."]
pub mod slvadr;
#[doc = "Slave Qualification for address 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvqual0](slvqual0) module"]
pub type SLVQUAL0 = crate::Reg<u32, _SLVQUAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLVQUAL0;
#[doc = "`read()` method returns [slvqual0::R](slvqual0::R) reader structure"]
impl crate::Readable for SLVQUAL0 {}
#[doc = "`write(|w| ..)` method takes [slvqual0::W](slvqual0::W) writer structure"]
impl crate::Writable for SLVQUAL0 {}
#[doc = "Slave Qualification for address 0."]
pub mod slvqual0;
#[doc = "Monitor receiver data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [monrxdat](monrxdat) module"]
pub type MONRXDAT = crate::Reg<u32, _MONRXDAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MONRXDAT;
#[doc = "`read()` method returns [monrxdat::R](monrxdat::R) reader structure"]
impl crate::Readable for MONRXDAT {}
#[doc = "Monitor receiver data register."]
pub mod monrxdat;
