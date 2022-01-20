#[doc = "Register `WDTOSCCTRL` reader"]
pub struct R(crate::R<WDTOSCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTOSCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTOSCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTOSCCTRL` writer"]
pub struct W(crate::W<WDTOSCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTOSCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDTOSCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTOSCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVSEL` reader - Divider select."]
pub struct DIVSEL_R(crate::FieldReader<u8, u8>);
impl DIVSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DIVSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVSEL` writer - Divider select."]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `FREQSEL` reader - Frequency select."]
pub struct FREQSEL_R(crate::FieldReader<u8, u8>);
impl FREQSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FREQSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQSEL` writer - Frequency select."]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider select."]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Frequency select."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider select."]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bits 5:9 - Frequency select."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtoscctrl](index.html) module"]
pub struct WDTOSCCTRL_SPEC;
impl crate::RegisterSpec for WDTOSCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtoscctrl::R](R) reader structure"]
impl crate::Readable for WDTOSCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtoscctrl::W](W) writer structure"]
impl crate::Writable for WDTOSCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTOSCCTRL to value 0xa0"]
impl crate::Resettable for WDTOSCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa0
    }
}
