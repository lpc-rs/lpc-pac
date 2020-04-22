#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status register."]
    pub stat: STAT,
    #[doc = "0x08 - Capture configuration register."]
    pub cfg: CFG,
    #[doc = "0x0c - Capture clear register."]
    pub capclr: CAPCLR,
    #[doc = "0x10 - Capture register ."]
    pub cap: [CAP; 4],
}
#[doc = "Control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control register."]
pub mod ctrl;
#[doc = "Status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](stat) module"]
pub type STAT = crate::Reg<u32, _STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STAT;
#[doc = "`read()` method returns [stat::R](stat::R) reader structure"]
impl crate::Readable for STAT {}
#[doc = "`write(|w| ..)` method takes [stat::W](stat::W) writer structure"]
impl crate::Writable for STAT {}
#[doc = "Status register."]
pub mod stat;
#[doc = "Capture configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<u32, _CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG;
#[doc = "`read()` method returns [cfg::R](cfg::R) reader structure"]
impl crate::Readable for CFG {}
#[doc = "`write(|w| ..)` method takes [cfg::W](cfg::W) writer structure"]
impl crate::Writable for CFG {}
#[doc = "Capture configuration register."]
pub mod cfg;
#[doc = "Capture clear register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capclr](capclr) module"]
pub type CAPCLR = crate::Reg<u32, _CAPCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPCLR;
#[doc = "`write(|w| ..)` method takes [capclr::W](capclr::W) writer structure"]
impl crate::Writable for CAPCLR {}
#[doc = "Capture clear register."]
pub mod capclr;
#[doc = "Capture register .\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap](cap) module"]
pub type CAP = crate::Reg<u32, _CAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAP;
#[doc = "`read()` method returns [cap::R](cap::R) reader structure"]
impl crate::Readable for CAP {}
#[doc = "Capture register ."]
pub mod cap;
