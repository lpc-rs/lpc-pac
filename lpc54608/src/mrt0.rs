#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - no description available"]
    pub channel: [CHANNEL; 4],
    _reserved1: [u8; 176usize],
    #[doc = "0xf0 - Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
    pub modcfg: MODCFG,
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    pub idle_ch: IDLE_CH,
    #[doc = "0xf8 - Global interrupt flag register"]
    pub irq_flag: IRQ_FLAG,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - MRT Time interval value register. This value is loaded into the TIMER register."]
    pub intval: self::channel::INTVAL,
    #[doc = "0x04 - MRT Timer register. This register reads the value of the down-counter."]
    pub timer: self::channel::TIMER,
    #[doc = "0x08 - MRT Control register. This register controls the MRT modes."]
    pub ctrl: self::channel::CTRL,
    #[doc = "0x0c - MRT Status register."]
    pub stat: self::channel::STAT,
}
#[doc = r"Register block"]
#[doc = "no description available"]
pub mod channel;
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [modcfg](modcfg) module"]
pub type MODCFG = crate::Reg<u32, _MODCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODCFG;
#[doc = "`read()` method returns [modcfg::R](modcfg::R) reader structure"]
impl crate::Readable for MODCFG {}
#[doc = "`write(|w| ..)` method takes [modcfg::W](modcfg::W) writer structure"]
impl crate::Writable for MODCFG {}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
pub mod modcfg;
#[doc = "Idle channel register. This register returns the number of the first idle channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_ch](idle_ch) module"]
pub type IDLE_CH = crate::Reg<u32, _IDLE_CH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDLE_CH;
#[doc = "`read()` method returns [idle_ch::R](idle_ch::R) reader structure"]
impl crate::Readable for IDLE_CH {}
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub mod idle_ch;
#[doc = "Global interrupt flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irq_flag](irq_flag) module"]
pub type IRQ_FLAG = crate::Reg<u32, _IRQ_FLAG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRQ_FLAG;
#[doc = "`read()` method returns [irq_flag::R](irq_flag::R) reader structure"]
impl crate::Readable for IRQ_FLAG {}
#[doc = "`write(|w| ..)` method takes [irq_flag::W](irq_flag::W) writer structure"]
impl crate::Writable for IRQ_FLAG {}
#[doc = "Global interrupt flag register"]
pub mod irq_flag;
