#[doc = "Oversample Rate register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osr](osr) module"]
pub type OSR = crate::Reg<u32, _OSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSR;
#[doc = "`read()` method returns [osr::R](osr::R) reader structure"]
impl crate::Readable for OSR {}
#[doc = "`write(|w| ..)` method takes [osr::W](osr::W) writer structure"]
impl crate::Writable for OSR {}
#[doc = "Oversample Rate register 0"]
pub mod osr;
#[doc = "DMIC Clock Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divhfclk](divhfclk) module"]
pub type DIVHFCLK = crate::Reg<u32, _DIVHFCLK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVHFCLK;
#[doc = "`read()` method returns [divhfclk::R](divhfclk::R) reader structure"]
impl crate::Readable for DIVHFCLK {}
#[doc = "`write(|w| ..)` method takes [divhfclk::W](divhfclk::W) writer structure"]
impl crate::Writable for DIVHFCLK {}
#[doc = "DMIC Clock Register 0"]
pub mod divhfclk;
#[doc = "Pre-Emphasis Filter Coefficient for 2 FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preac2fscoef](preac2fscoef) module"]
pub type PREAC2FSCOEF = crate::Reg<u32, _PREAC2FSCOEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREAC2FSCOEF;
#[doc = "`read()` method returns [preac2fscoef::R](preac2fscoef::R) reader structure"]
impl crate::Readable for PREAC2FSCOEF {}
#[doc = "`write(|w| ..)` method takes [preac2fscoef::W](preac2fscoef::W) writer structure"]
impl crate::Writable for PREAC2FSCOEF {}
#[doc = "Pre-Emphasis Filter Coefficient for 2 FS register"]
pub mod preac2fscoef;
#[doc = "Pre-Emphasis Filter Coefficient for 4 FS register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [preac4fscoef](preac4fscoef) module"]
pub type PREAC4FSCOEF = crate::Reg<u32, _PREAC4FSCOEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PREAC4FSCOEF;
#[doc = "`read()` method returns [preac4fscoef::R](preac4fscoef::R) reader structure"]
impl crate::Readable for PREAC4FSCOEF {}
#[doc = "`write(|w| ..)` method takes [preac4fscoef::W](preac4fscoef::W) writer structure"]
impl crate::Writable for PREAC4FSCOEF {}
#[doc = "Pre-Emphasis Filter Coefficient for 4 FS register"]
pub mod preac4fscoef;
#[doc = "Decimator Gain Shift register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gainshift](gainshift) module"]
pub type GAINSHIFT = crate::Reg<u32, _GAINSHIFT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAINSHIFT;
#[doc = "`read()` method returns [gainshift::R](gainshift::R) reader structure"]
impl crate::Readable for GAINSHIFT {}
#[doc = "`write(|w| ..)` method takes [gainshift::W](gainshift::W) writer structure"]
impl crate::Writable for GAINSHIFT {}
#[doc = "Decimator Gain Shift register"]
pub mod gainshift;
#[doc = "FIFO Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](fifo_ctrl) module"]
pub type FIFO_CTRL = crate::Reg<u32, _FIFO_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_CTRL;
#[doc = "`read()` method returns [fifo_ctrl::R](fifo_ctrl::R) reader structure"]
impl crate::Readable for FIFO_CTRL {}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](fifo_ctrl::W) writer structure"]
impl crate::Writable for FIFO_CTRL {}
#[doc = "FIFO Control register 0"]
pub mod fifo_ctrl;
#[doc = "FIFO Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_status](fifo_status) module"]
pub type FIFO_STATUS = crate::Reg<u32, _FIFO_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_STATUS;
#[doc = "`read()` method returns [fifo_status::R](fifo_status::R) reader structure"]
impl crate::Readable for FIFO_STATUS {}
#[doc = "`write(|w| ..)` method takes [fifo_status::W](fifo_status::W) writer structure"]
impl crate::Writable for FIFO_STATUS {}
#[doc = "FIFO Status register 0"]
pub mod fifo_status;
#[doc = "FIFO Data Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_data](fifo_data) module"]
pub type FIFO_DATA = crate::Reg<u32, _FIFO_DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO_DATA;
#[doc = "`read()` method returns [fifo_data::R](fifo_data::R) reader structure"]
impl crate::Readable for FIFO_DATA {}
#[doc = "`write(|w| ..)` method takes [fifo_data::W](fifo_data::W) writer structure"]
impl crate::Writable for FIFO_DATA {}
#[doc = "FIFO Data Register 0"]
pub mod fifo_data;
#[doc = "PDM Source Configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_ctrl](phy_ctrl) module"]
pub type PHY_CTRL = crate::Reg<u32, _PHY_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PHY_CTRL;
#[doc = "`read()` method returns [phy_ctrl::R](phy_ctrl::R) reader structure"]
impl crate::Readable for PHY_CTRL {}
#[doc = "`write(|w| ..)` method takes [phy_ctrl::W](phy_ctrl::W) writer structure"]
impl crate::Writable for PHY_CTRL {}
#[doc = "PDM Source Configuration register 0"]
pub mod phy_ctrl;
#[doc = "DC Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc_ctrl](dc_ctrl) module"]
pub type DC_CTRL = crate::Reg<u32, _DC_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC_CTRL;
#[doc = "`read()` method returns [dc_ctrl::R](dc_ctrl::R) reader structure"]
impl crate::Readable for DC_CTRL {}
#[doc = "`write(|w| ..)` method takes [dc_ctrl::W](dc_ctrl::W) writer structure"]
impl crate::Writable for DC_CTRL {}
#[doc = "DC Control register 0"]
pub mod dc_ctrl;
