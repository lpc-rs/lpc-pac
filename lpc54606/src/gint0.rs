#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO grouped interrupt control register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - GPIO grouped interrupt port 0 polarity register"]
    pub port_pol: [PORT_POL; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x40 - GPIO grouped interrupt port 0 enable register"]
    pub port_ena: [PORT_ENA; 2],
}
#[doc = "GPIO grouped interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "GPIO grouped interrupt control register"]
pub mod ctrl;
#[doc = "GPIO grouped interrupt port 0 polarity register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_pol](port_pol) module"]
pub type PORT_POL = crate::Reg<u32, _PORT_POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_POL;
#[doc = "`read()` method returns [port_pol::R](port_pol::R) reader structure"]
impl crate::Readable for PORT_POL {}
#[doc = "`write(|w| ..)` method takes [port_pol::W](port_pol::W) writer structure"]
impl crate::Writable for PORT_POL {}
#[doc = "GPIO grouped interrupt port 0 polarity register"]
pub mod port_pol;
#[doc = "GPIO grouped interrupt port 0 enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [port_ena](port_ena) module"]
pub type PORT_ENA = crate::Reg<u32, _PORT_ENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PORT_ENA;
#[doc = "`read()` method returns [port_ena::R](port_ena::R) reader structure"]
impl crate::Readable for PORT_ENA {}
#[doc = "`write(|w| ..)` method takes [port_ena::W](port_ena::W) writer structure"]
impl crate::Writable for PORT_ENA {}
#[doc = "GPIO grouped interrupt port 0 enable register"]
pub mod port_ena;
