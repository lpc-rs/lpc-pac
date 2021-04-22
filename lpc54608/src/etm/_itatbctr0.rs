#[doc = "Register `_ITATBCTR0` reader"]
pub struct R(crate::R<_ITATBCTR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<_ITATBCTR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<_ITATBCTR0_SPEC>> for R {
    fn from(reader: crate::R<_ITATBCTR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `_ITATBCTR0` writer"]
pub struct W(crate::W<_ITATBCTR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<_ITATBCTR0_SPEC>;
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
impl core::convert::From<crate::W<_ITATBCTR0_SPEC>> for W {
    fn from(writer: crate::W<_ITATBCTR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATVALID` writer - A write to this bit sets the value of the ETM ATVALID output."]
pub struct ATVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> ATVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - A write to this bit sets the value of the ETM ATVALID output."]
    #[inline(always)]
    pub fn atvalid(&mut self) -> ATVALID_W {
        ATVALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETM Integration Test ATB Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_itatbctr0](index.html) module"]
pub struct _ITATBCTR0_SPEC;
impl crate::RegisterSpec for _ITATBCTR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [_itatbctr0::R](R) reader structure"]
impl crate::Readable for _ITATBCTR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [_itatbctr0::W](W) writer structure"]
impl crate::Writable for _ITATBCTR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets _ITATBCTR0 to value 0"]
impl crate::Resettable for _ITATBCTR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
