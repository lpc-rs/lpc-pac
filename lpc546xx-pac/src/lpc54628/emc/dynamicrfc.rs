///Register `DYNAMICRFC` reader
pub struct R(crate::R<DYNAMICRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRFC_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DYNAMICRFC` writer
pub struct W(crate::W<DYNAMICRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRFC_SPEC>;
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
impl From<crate::W<DYNAMICRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRFC_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRFC` reader - Auto-refresh period and auto-refresh to active command period.
pub struct TRFC_R(crate::FieldReader<u8, u8>);
impl TRFC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRFC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRFC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRFC` writer - Auto-refresh period and auto-refresh to active command period.
pub struct TRFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRFC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    ///Bits 0:4 - Auto-refresh period and auto-refresh to active command period.
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Auto-refresh period and auto-refresh to active command period.
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W {
        TRFC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Selects the auto-refresh period
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dynamicrfc](index.html) module
pub struct DYNAMICRFC_SPEC;
impl crate::RegisterSpec for DYNAMICRFC_SPEC {
    type Ux = u32;
}
///`read()` method returns [dynamicrfc::R](R) reader structure
impl crate::Readable for DYNAMICRFC_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dynamicrfc::W](W) writer structure
impl crate::Writable for DYNAMICRFC_SPEC {
    type Writer = W;
}
///`reset()` method sets DYNAMICRFC to value 0x1f
impl crate::Resettable for DYNAMICRFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
