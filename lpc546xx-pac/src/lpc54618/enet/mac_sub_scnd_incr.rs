#[doc = "Register `MAC_SUB_SCND_INCR` reader"]
pub struct R(crate::R<MAC_SUB_SCND_INCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_SUB_SCND_INCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_SUB_SCND_INCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_SUB_SCND_INCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_SUB_SCND_INCR` writer"]
pub struct W(crate::W<MAC_SUB_SCND_INCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_SUB_SCND_INCR_SPEC>;
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
impl From<crate::W<MAC_SUB_SCND_INCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_SUB_SCND_INCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSINC` reader - Sub-second increment value."]
pub struct SSINC_R(crate::FieldReader<u8, u8>);
impl SSINC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SSINC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSINC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSINC` writer - Sub-second increment value."]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Sub-second increment value."]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sub-second increment value."]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sub-second increment register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_sub_scnd_incr](index.html) module"]
pub struct MAC_SUB_SCND_INCR_SPEC;
impl crate::RegisterSpec for MAC_SUB_SCND_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_sub_scnd_incr::R](R) reader structure"]
impl crate::Readable for MAC_SUB_SCND_INCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_sub_scnd_incr::W](W) writer structure"]
impl crate::Writable for MAC_SUB_SCND_INCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_SUB_SCND_INCR to value 0"]
impl crate::Resettable for MAC_SUB_SCND_INCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
