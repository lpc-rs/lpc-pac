#[doc = "Register `IDLE_CH` reader"]
pub struct R(crate::R<IDLE_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLE_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLE_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLE_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CHAN` reader - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
pub struct CHAN_R(crate::FieldReader<u8, u8>);
impl CHAN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CHAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 4:7 - Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub fn chan(&self) -> CHAN_R {
        CHAN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "Idle channel register. This register returns the number of the first idle channel.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idle_ch](index.html) module"]
pub struct IDLE_CH_SPEC;
impl crate::RegisterSpec for IDLE_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idle_ch::R](R) reader structure"]
impl crate::Readable for IDLE_CH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDLE_CH to value 0"]
impl crate::Resettable for IDLE_CH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
