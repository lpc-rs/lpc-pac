///Register `OSR` reader
pub struct R(crate::R<OSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OSR` writer
pub struct W(crate::W<OSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSR_SPEC>;
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
impl From<crate::W<OSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OSR` reader - Selects the oversample rate for the related input channel.
pub struct OSR_R(crate::FieldReader<u8, u8>);
impl OSR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        OSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OSR` writer - Selects the oversample rate for the related input channel.
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Selects the oversample rate for the related input channel.
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Selects the oversample rate for the related input channel.
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Oversample Rate register 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [osr](index.html) module
pub struct OSR_SPEC;
impl crate::RegisterSpec for OSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [osr::R](R) reader structure
impl crate::Readable for OSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [osr::W](W) writer structure
impl crate::Writable for OSR_SPEC {
    type Writer = W;
}
///`reset()` method sets OSR to value 0
impl crate::Resettable for OSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
