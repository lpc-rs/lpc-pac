///Register `PINTSEL[%s]` reader
pub struct R(crate::R<PINTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINTSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PINTSEL[%s]` writer
pub struct W(crate::W<PINTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINTSEL_SPEC>;
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
impl From<crate::W<PINTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINTSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `INTPIN` reader - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO1_31 correspond to numbers 0 to 63).
pub struct INTPIN_R(crate::FieldReader<u8, u8>);
impl INTPIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INTPIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTPIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INTPIN` writer - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO1_31 correspond to numbers 0 to 63).
pub struct INTPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTPIN_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    ///Bits 0:7 - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO1_31 correspond to numbers 0 to 63).
    #[inline(always)]
    pub fn intpin(&self) -> INTPIN_R {
        INTPIN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - Pin number select for pin interrupt or pattern match engine input. (PIO0_0 to PIO1_31 correspond to numbers 0 to 63).
    #[inline(always)]
    pub fn intpin(&mut self) -> INTPIN_W {
        INTPIN_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Pin interrupt select register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pintsel](index.html) module
pub struct PINTSEL_SPEC;
impl crate::RegisterSpec for PINTSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [pintsel::R](R) reader structure
impl crate::Readable for PINTSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pintsel::W](W) writer structure
impl crate::Writable for PINTSEL_SPEC {
    type Writer = W;
}
///`reset()` method sets PINTSEL[%s]
///to value 0
impl crate::Resettable for PINTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
