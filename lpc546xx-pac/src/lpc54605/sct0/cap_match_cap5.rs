#[doc = "Register `CAP5` reader"]
pub struct R(crate::R<CAP_MATCH_CAP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_MATCH_CAP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAP_MATCH_CAP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAP_MATCH_CAP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAP5` writer"]
pub struct W(crate::W<CAP_MATCH_CAP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_MATCH_CAP5_SPEC>;
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
impl From<crate::W<CAP_MATCH_CAP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAP_MATCH_CAP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPn_L` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub struct CAPN_L_R(crate::FieldReader<u16, u16>);
impl CAPN_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAPN_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPN_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPn_L` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
pub struct CAPN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `CAPn_H` reader - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub struct CAPN_H_R(crate::FieldReader<u16, u16>);
impl CAPN_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAPN_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPN_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPn_H` writer - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
pub struct CAPN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&self) -> CAPN_L_R {
        CAPN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&self) -> CAPN_H_R {
        CAPN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the lower 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_l(&mut self) -> CAPN_L_W {
        CAPN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read the 16-bit counter value at which this register was last captured. When UNIFY = 1, read the upper 16 bits of the 32-bit value at which this register was last captured."]
    #[inline(always)]
    pub fn capn_h(&mut self) -> CAPN_H_W {
        CAPN_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT capture register of capture channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_match_cap5](index.html) module"]
pub struct CAP_MATCH_CAP5_SPEC;
impl crate::RegisterSpec for CAP_MATCH_CAP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_match_cap5::R](R) reader structure"]
impl crate::Readable for CAP_MATCH_CAP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_match_cap5::W](W) writer structure"]
impl crate::Writable for CAP_MATCH_CAP5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAP5 to value 0"]
impl crate::Resettable for CAP_MATCH_CAP5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
