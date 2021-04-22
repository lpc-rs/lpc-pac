#[doc = "Register `MATCH7` reader"]
pub struct R(crate::R<CAP_MATCH_MATCH7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAP_MATCH_MATCH7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAP_MATCH_MATCH7_SPEC>> for R {
    fn from(reader: crate::R<CAP_MATCH_MATCH7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCH7` writer"]
pub struct W(crate::W<CAP_MATCH_MATCH7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAP_MATCH_MATCH7_SPEC>;
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
impl core::convert::From<crate::W<CAP_MATCH_MATCH7_SPEC>> for W {
    fn from(writer: crate::W<CAP_MATCH_MATCH7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCHn_L` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub struct MATCHN_L_R(crate::FieldReader<u16, u16>);
impl MATCHN_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        MATCHN_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCHN_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCHn_L` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
pub struct MATCHN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `MATCHn_H` reader - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub struct MATCHN_H_R(crate::FieldReader<u16, u16>);
impl MATCHN_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        MATCHN_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MATCHN_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MATCHn_H` writer - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
pub struct MATCHN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCHN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&self) -> MATCHN_L_R {
        MATCHN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&self) -> MATCHN_H_R {
        MATCHN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, read or write the 16-bit value to be compared to the L counter. When UNIFY = 1, read or write the lower 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_l(&mut self) -> MATCHN_L_W {
        MATCHN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, read or write the 16-bit value to be compared to the H counter. When UNIFY = 1, read or write the upper 16 bits of the 32-bit value to be compared to the unified counter."]
    #[inline(always)]
    pub fn matchn_h(&mut self) -> MATCHN_H_W {
        MATCHN_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match value register of match channels\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cap_match_match7](index.html) module"]
pub struct CAP_MATCH_MATCH7_SPEC;
impl crate::RegisterSpec for CAP_MATCH_MATCH7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cap_match_match7::R](R) reader structure"]
impl crate::Readable for CAP_MATCH_MATCH7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cap_match_match7::W](W) writer structure"]
impl crate::Writable for CAP_MATCH_MATCH7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATCH7 to value 0"]
impl crate::Resettable for CAP_MATCH_MATCH7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
