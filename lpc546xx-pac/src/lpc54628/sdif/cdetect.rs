///Register `CDETECT` reader
pub struct R(crate::R<CDETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDETECT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CDETECT` writer
pub struct W(crate::W<CDETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDETECT_SPEC>;
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
impl From<crate::W<CDETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDETECT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CARD_DETECT` reader - Card detect.
pub struct CARD_DETECT_R(crate::FieldReader<bool, bool>);
impl CARD_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CARD_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CARD_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `CARD_DETECT` writer - Card detect.
pub struct CARD_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CARD_DETECT_W<'a> {
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    ///Bit 0 - Card detect.
    #[inline(always)]
    pub fn card_detect(&self) -> CARD_DETECT_R {
        CARD_DETECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    ///Bit 0 - Card detect.
    #[inline(always)]
    pub fn card_detect(&mut self) -> CARD_DETECT_W {
        CARD_DETECT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Card Detect register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cdetect](index.html) module
pub struct CDETECT_SPEC;
impl crate::RegisterSpec for CDETECT_SPEC {
    type Ux = u32;
}
///`read()` method returns [cdetect::R](R) reader structure
impl crate::Readable for CDETECT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cdetect::W](W) writer structure
impl crate::Writable for CDETECT_SPEC {
    type Writer = W;
}
///`reset()` method sets CDETECT to value 0
impl crate::Resettable for CDETECT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
