///Register `FBWST` reader
pub struct R(crate::R<FBWST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBWST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBWST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBWST_SPEC>) -> Self {
        R(reader)
    }
}
///Register `FBWST` writer
pub struct W(crate::W<FBWST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBWST_SPEC>;
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
impl From<crate::W<FBWST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBWST_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WAITSTATES` reader - Wait states for signature generation.
pub struct WAITSTATES_R(crate::FieldReader<u8, u8>);
impl WAITSTATES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WAITSTATES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITSTATES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `WAITSTATES` writer - Wait states for signature generation.
pub struct WAITSTATES_W<'a> {
    w: &'a mut W,
}
impl<'a> WAITSTATES_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Wait states for signature generation.
    #[inline(always)]
    pub fn waitstates(&self) -> WAITSTATES_R {
        WAITSTATES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Wait states for signature generation.
    #[inline(always)]
    pub fn waitstates(&mut self) -> WAITSTATES_W {
        WAITSTATES_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Wait state register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fbwst](index.html) module
pub struct FBWST_SPEC;
impl crate::RegisterSpec for FBWST_SPEC {
    type Ux = u32;
}
///`read()` method returns [fbwst::R](R) reader structure
impl crate::Readable for FBWST_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [fbwst::W](W) writer structure
impl crate::Writable for FBWST_SPEC {
    type Writer = W;
}
///`reset()` method sets FBWST to value 0xc005
impl crate::Resettable for FBWST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc005
    }
}
