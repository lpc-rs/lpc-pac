#[doc = "Register `THR1_LOW` reader"]
pub struct R(crate::R<THR1_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THR1_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<THR1_LOW_SPEC>> for R {
    fn from(reader: crate::R<THR1_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THR1_LOW` writer"]
pub struct W(crate::W<THR1_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THR1_LOW_SPEC>;
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
impl core::convert::From<crate::W<THR1_LOW_SPEC>> for W {
    fn from(writer: crate::W<THR1_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRLOW` reader - Low threshold value against which ADC results will be compared"]
pub struct THRLOW_R(crate::FieldReader<u16, u16>);
impl THRLOW_R {
    pub(crate) fn new(bits: u16) -> Self {
        THRLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THRLOW_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRLOW` writer - Low threshold value against which ADC results will be compared"]
pub struct THRLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> THRLOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | ((value as u32 & 0x0fff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:15 - Low threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrlow(&self) -> THRLOW_R {
        THRLOW_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 4:15 - Low threshold value against which ADC results will be compared"]
    #[inline(always)]
    pub fn thrlow(&mut self) -> THRLOW_W {
        THRLOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Low Compare Threshold register 1: Contains the lower threshold level for automatic threshold comparison for any channels linked to threshold pair 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr1_low](index.html) module"]
pub struct THR1_LOW_SPEC;
impl crate::RegisterSpec for THR1_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thr1_low::R](R) reader structure"]
impl crate::Readable for THR1_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thr1_low::W](W) writer structure"]
impl crate::Writable for THR1_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets THR1_LOW to value 0"]
impl crate::Resettable for THR1_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
