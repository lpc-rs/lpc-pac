#[doc = "Register `MATCHREL3` reader"]
pub struct R(crate::R<CAPCTRL_MATCHREL_MATCHREL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_MATCHREL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CAPCTRL_MATCHREL_MATCHREL3_SPEC>> for R {
    fn from(reader: crate::R<CAPCTRL_MATCHREL_MATCHREL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATCHREL3` writer"]
pub struct W(crate::W<CAPCTRL_MATCHREL_MATCHREL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_MATCHREL3_SPEC>;
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
impl core::convert::From<crate::W<CAPCTRL_MATCHREL_MATCHREL3_SPEC>> for W {
    fn from(writer: crate::W<CAPCTRL_MATCHREL_MATCHREL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOADn_L` reader - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub struct RELOADN_L_R(crate::FieldReader<u16, u16>);
impl RELOADN_L_R {
    pub(crate) fn new(bits: u16) -> Self {
        RELOADN_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOADN_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOADn_L` writer - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub struct RELOADN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `RELOADn_H` reader - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub struct RELOADN_H_R(crate::FieldReader<u16, u16>);
impl RELOADN_H_R {
    pub(crate) fn new(bits: u16) -> Self {
        RELOADN_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RELOADN_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RELOADn_H` writer - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
pub struct RELOADN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&self) -> RELOADN_L_R {
        RELOADN_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&self) -> RELOADN_H_R {
        RELOADN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_l(&mut self) -> RELOADN_L_W {
        RELOADN_L_W { w: self }
    }
    #[doc = "Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register."]
    #[inline(always)]
    pub fn reloadn_h(&mut self) -> RELOADN_H_W {
        RELOADN_H_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT match reload value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capctrl_matchrel_matchrel3](index.html) module"]
pub struct CAPCTRL_MATCHREL_MATCHREL3_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_MATCHREL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capctrl_matchrel_matchrel3::R](R) reader structure"]
impl crate::Readable for CAPCTRL_MATCHREL_MATCHREL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capctrl_matchrel_matchrel3::W](W) writer structure"]
impl crate::Writable for CAPCTRL_MATCHREL_MATCHREL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATCHREL3 to value 0"]
impl crate::Resettable for CAPCTRL_MATCHREL_MATCHREL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
