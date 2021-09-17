#[doc = "Register `SCLH` reader"]
pub struct R(crate::R<SCLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCLH` writer"]
pub struct W(crate::W<SCLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCLH_SPEC>;
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
impl From<crate::W<SCLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCLH` reader - Count for SCL HIGH time period selection."]
pub struct SCLH_R(crate::FieldReader<u16, u16>);
impl SCLH_R {
    pub(crate) fn new(bits: u16) -> Self {
        SCLH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLH` writer - Count for SCL HIGH time period selection."]
pub struct SCLH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SCLH_R {
        SCLH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&mut self) -> SCLH_W {
        SCLH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sclh](index.html) module"]
pub struct SCLH_SPEC;
impl crate::RegisterSpec for SCLH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sclh::R](R) reader structure"]
impl crate::Readable for SCLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sclh::W](W) writer structure"]
impl crate::Writable for SCLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCLH to value 0x04"]
impl crate::Resettable for SCLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
