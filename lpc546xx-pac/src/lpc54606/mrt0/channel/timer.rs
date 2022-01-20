#[doc = "Register `TIMER` reader"]
pub struct R(crate::R<TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VALUE` reader - Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
pub struct VALUE_R(crate::FieldReader<u32, u32>);
impl VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "MRT Timer register. This register reads the value of the down-counter.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer](index.html) module"]
pub struct TIMER_SPEC;
impl crate::RegisterSpec for TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer::R](R) reader structure"]
impl crate::Readable for TIMER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER to value 0x00ff_ffff"]
impl crate::Resettable for TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ffff
    }
}
