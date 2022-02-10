///Register `DYNAMICRAS` reader
pub struct R(crate::R<DYNAMICRAS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICRAS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICRAS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICRAS_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DYNAMICRAS` writer
pub struct W(crate::W<DYNAMICRAS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICRAS_SPEC>;
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
impl From<crate::W<DYNAMICRAS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICRAS_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TRAS` reader - Active to precharge command period.
pub struct TRAS_R(crate::FieldReader<u8, u8>);
impl TRAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TRAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TRAS` writer - Active to precharge command period.
pub struct TRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAS_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Active to precharge command period.
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Active to precharge command period.
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W {
        TRAS_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Active to precharge command period
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dynamicras](index.html) module
pub struct DYNAMICRAS_SPEC;
impl crate::RegisterSpec for DYNAMICRAS_SPEC {
    type Ux = u32;
}
///`read()` method returns [dynamicras::R](R) reader structure
impl crate::Readable for DYNAMICRAS_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dynamicras::W](W) writer structure
impl crate::Writable for DYNAMICRAS_SPEC {
    type Writer = W;
}
///`reset()` method sets DYNAMICRAS to value 0x0f
impl crate::Resettable for DYNAMICRAS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
