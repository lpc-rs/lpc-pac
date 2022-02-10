///Register `DYNAMICRP` reader
pub struct R(crate::R<DYNAMICRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRP_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DYNAMICRP` writer
pub struct W(crate::W<DYNAMICRP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRP_SPEC>;
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
impl From<crate::W<DYNAMICRP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRP_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRP` reader - Precharge command period.
pub struct TRP_R(crate::FieldReader<u8, u8>);
impl TRP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRP` writer - Precharge command period.
pub struct TRP_W<'a> {
    w: &'a mut W,
}
impl<'a> TRP_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Precharge command period.
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Precharge command period.
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W {
        TRP_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Precharge command period
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dynamicrp](index.html) module
pub struct DYNAMICRP_SPEC;
impl crate::RegisterSpec for DYNAMICRP_SPEC {
    type Ux = u32;
}
///`read()` method returns [dynamicrp::R](R) reader structure
impl crate::Readable for DYNAMICRP_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dynamicrp::W](W) writer structure
impl crate::Writable for DYNAMICRP_SPEC {
    type Writer = W;
}
///`reset()` method sets DYNAMICRP to value 0x0f
impl crate::Resettable for DYNAMICRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
