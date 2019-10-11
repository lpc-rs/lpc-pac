#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
    pub mod_: MOD,
    #[doc = "0x04 - Watchdog timer constant register. This 24-bit register determines the time-out value."]
    pub tc: TC,
    #[doc = "0x08 - Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
    pub feed: FEED,
    #[doc = "0x0c - Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
    pub tv: TV,
    _reserved4: [u8; 4usize],
    #[doc = "0x14 - Watchdog Warning Interrupt compare value."]
    pub warnint: WARNINT,
    #[doc = "0x18 - Watchdog Window compare value."]
    pub window: WINDOW,
}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mod_](mod_) module"]
pub type MOD = crate::Reg<u32, _MOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MOD;
#[doc = "`read()` method returns [mod_::R](mod_::R) reader structure"]
impl crate::Readable for MOD {}
#[doc = "`write(|w| ..)` method takes [mod_::W](mod_::W) writer structure"]
impl crate::Writable for MOD {}
#[doc = "Watchdog mode register. This register contains the basic mode and status of the Watchdog Timer."]
pub mod mod_;
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tc](tc) module"]
pub type TC = crate::Reg<u32, _TC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TC;
#[doc = "`read()` method returns [tc::R](tc::R) reader structure"]
impl crate::Readable for TC {}
#[doc = "`write(|w| ..)` method takes [tc::W](tc::W) writer structure"]
impl crate::Writable for TC {}
#[doc = "Watchdog timer constant register. This 24-bit register determines the time-out value."]
pub mod tc;
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [feed](feed) module"]
pub type FEED = crate::Reg<u32, _FEED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEED;
#[doc = "`write(|w| ..)` method takes [feed::W](feed::W) writer structure"]
impl crate::Writable for FEED {}
#[doc = "Watchdog feed sequence register. Writing 0xAA followed by 0x55 to this register reloads the Watchdog timer with the value contained in TC."]
pub mod feed;
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tv](tv) module"]
pub type TV = crate::Reg<u32, _TV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TV;
#[doc = "`read()` method returns [tv::R](tv::R) reader structure"]
impl crate::Readable for TV {}
#[doc = "Watchdog timer value register. This 24-bit register reads out the current value of the Watchdog timer."]
pub mod tv;
#[doc = "Watchdog Warning Interrupt compare value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [warnint](warnint) module"]
pub type WARNINT = crate::Reg<u32, _WARNINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WARNINT;
#[doc = "`read()` method returns [warnint::R](warnint::R) reader structure"]
impl crate::Readable for WARNINT {}
#[doc = "`write(|w| ..)` method takes [warnint::W](warnint::W) writer structure"]
impl crate::Writable for WARNINT {}
#[doc = "Watchdog Warning Interrupt compare value."]
pub mod warnint;
#[doc = "Watchdog Window compare value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [window](window) module"]
pub type WINDOW = crate::Reg<u32, _WINDOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINDOW;
#[doc = "`read()` method returns [window::R](window::R) reader structure"]
impl crate::Readable for WINDOW {}
#[doc = "`write(|w| ..)` method takes [window::W](window::W) writer structure"]
impl crate::Writable for WINDOW {}
#[doc = "Watchdog Window compare value."]
pub mod window;
