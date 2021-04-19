#[doc = "Register `SYSTCKCAL` reader"]
pub struct R(crate::R<SYSTCKCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSTCKCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SYSTCKCAL_SPEC>> for R {
    fn from(reader: crate::R<SYSTCKCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSTCKCAL` writer"]
pub struct W(crate::W<SYSTCKCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTCKCAL_SPEC>;
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
impl core::convert::From<crate::W<SYSTCKCAL_SPEC>> for W {
    fn from(writer: crate::W<SYSTCKCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAL` reader - System tick timer calibration value."]
pub struct CAL_R(crate::FieldReader<u32, u32>);
impl CAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - System tick timer calibration value."]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `SKEW` reader - Initial value for the Systick timer."]
pub struct SKEW_R(crate::FieldReader<bool, bool>);
impl SKEW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SKEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SKEW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SKEW` writer - Initial value for the Systick timer."]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `NOREF` reader - Initial value for the Systick timer."]
pub struct NOREF_R(crate::FieldReader<bool, bool>);
impl NOREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NOREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NOREF` writer - Initial value for the Systick timer."]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - System tick timer calibration value."]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 24 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bit 25 - Initial value for the Systick timer."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System tick counter calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systckcal](index.html) module"]
pub struct SYSTCKCAL_SPEC;
impl crate::RegisterSpec for SYSTCKCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [systckcal::R](R) reader structure"]
impl crate::Readable for SYSTCKCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [systckcal::W](W) writer structure"]
impl crate::Writable for SYSTCKCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTCKCAL to value 0"]
impl crate::Resettable for SYSTCKCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
