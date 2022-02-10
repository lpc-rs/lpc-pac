///Register `CAPCTRL6` reader
pub struct R(crate::R<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CAPCTRL6` writer
pub struct W(crate::W<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>;
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
impl From<crate::W<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPCTRL_MATCHREL_CAPCTRL6_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CAPCONn_L` reader - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT.
pub struct CAPCONN_L_R(crate::FieldReader<u16, u16>);
impl CAPCONN_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAPCONN_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPCONN_L_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CAPCONn_L` writer - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT.
pub struct CAPCONN_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_L_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
///Field `CAPCONn_H` reader - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT.
pub struct CAPCONN_H_R(crate::FieldReader<u16, u16>);
impl CAPCONN_H_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CAPCONN_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAPCONN_H_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CAPCONn_H` writer - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT.
pub struct CAPCONN_H_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPCONN_H_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    ///Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT.
    #[inline(always)]
    pub fn capconn_l(&self) -> CAPCONN_L_R {
        CAPCONN_L_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT.
    #[inline(always)]
    pub fn capconn_h(&self) -> CAPCONN_H_R {
        CAPCONN_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - If bit m is one, event m causes the CAPn_L (UNIFY = 0) or the CAPn (UNIFY = 1) register to be loaded (event 0 = bit 0, event 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT.
    #[inline(always)]
    pub fn capconn_l(&mut self) -> CAPCONN_L_W {
        CAPCONN_L_W { w: self }
    }
    ///Bits 16:31 - If bit m is one, event m causes the CAPn_H (UNIFY = 0) register to be loaded (event 0 = bit 16, event 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT.
    #[inline(always)]
    pub fn capconn_h(&mut self) -> CAPCONN_H_W {
        CAPCONN_H_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SCT capture control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [capctrl_matchrel_capctrl6](index.html) module
pub struct CAPCTRL_MATCHREL_CAPCTRL6_SPEC;
impl crate::RegisterSpec for CAPCTRL_MATCHREL_CAPCTRL6_SPEC {
    type Ux = u32;
}
///`read()` method returns [capctrl_matchrel_capctrl6::R](R) reader structure
impl crate::Readable for CAPCTRL_MATCHREL_CAPCTRL6_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [capctrl_matchrel_capctrl6::W](W) writer structure
impl crate::Writable for CAPCTRL_MATCHREL_CAPCTRL6_SPEC {
    type Writer = W;
}
///`reset()` method sets CAPCTRL6 to value 0
impl crate::Resettable for CAPCTRL_MATCHREL_CAPCTRL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
