///Register `MATCHREL4` reader
pub struct R(crate::R<CAPCTRL_MATCHREL_MATCHREL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_MATCHREL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_MATCHREL_MATCHREL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_MATCHREL_MATCHREL4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MATCHREL4` writer
pub struct W(crate::W<CAPCTRL_MATCHREL_MATCHREL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_MATCHREL4_SPEC>;
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
impl From<crate::W<CAPCTRL_MATCHREL_MATCHREL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_MATCHREL_MATCHREL4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RELOADn_L` reader - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register.
pub struct RELOADN_L_R(crate::FieldReader<u16, u16>);
impl RELOADN_L_R {
    #[inline(always)]
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
///Field `RELOADn_L` writer - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register.
pub struct RELOADN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_L_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `RELOADn_H` reader - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register.
pub struct RELOADN_H_R(crate::FieldReader<u16, u16>);
impl RELOADN_H_R {
    #[inline(always)]
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
///Field `RELOADn_H` writer - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register.
pub struct RELOADN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> RELOADN_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register.
    #[inline(always)]
    pub fn reloadn_l(&self) -> RELOADN_L_R {
        RELOADN_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register.
    #[inline(always)]
    pub fn reloadn_h(&self) -> RELOADN_H_R {
        RELOADN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - When UNIFY = 0, specifies the 16-bit value to be loaded into the MATCHn_L register. When UNIFY = 1, specifies the lower 16 bits of the 32-bit value to be loaded into the MATCHn register.
    #[inline(always)]
    pub fn reloadn_l(&mut self) -> RELOADN_L_W {
        RELOADN_L_W { w: self }
    }
    ///Bits 16:31 - When UNIFY = 0, specifies the 16-bit to be loaded into the MATCHn_H register. When UNIFY = 1, specifies the upper 16 bits of the 32-bit value to be loaded into the MATCHn register.
    #[inline(always)]
    pub fn reloadn_h(&mut self) -> RELOADN_H_W {
        RELOADN_H_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SCT match reload value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [capctrl_matchrel_matchrel4](index.html) module
pub struct CAPCTRL_MATCHREL_MATCHREL4_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_MATCHREL4_SPEC {
    type Ux = u32;
}
///`read()` method returns [capctrl_matchrel_matchrel4::R](R) reader structure
impl crate::Readable for CAPCTRL_MATCHREL_MATCHREL4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [capctrl_matchrel_matchrel4::W](W) writer structure
impl crate::Writable for CAPCTRL_MATCHREL_MATCHREL4_SPEC {
    type Writer = W;
}
///`reset()` method sets MATCHREL4 to value 0
impl crate::Resettable for CAPCTRL_MATCHREL_MATCHREL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
