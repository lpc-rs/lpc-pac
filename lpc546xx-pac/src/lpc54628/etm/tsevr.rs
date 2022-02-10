///Register `TSEVR` reader
pub struct R(crate::R<TSEVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSEVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSEVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSEVR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TSEVR` writer
pub struct W(crate::W<TSEVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSEVR_SPEC>;
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
impl From<crate::W<TSEVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSEVR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TimestampEvent` reader - Timestamp event.
pub struct TIMESTAMPEVENT_R(crate::FieldReader<u16, u16>);
impl TIMESTAMPEVENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TIMESTAMPEVENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMESTAMPEVENT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TimestampEvent` writer - Timestamp event.
pub struct TIMESTAMPEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMESTAMPEVENT_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    ///Bits 0:11 - Timestamp event.
    #[inline(always)]
    pub fn timestamp_event(&self) -> TIMESTAMPEVENT_R {
        TIMESTAMPEVENT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    ///Bits 0:11 - Timestamp event.
    #[inline(always)]
    pub fn timestamp_event(&mut self) -> TIMESTAMPEVENT_W {
        TIMESTAMPEVENT_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timestamp Event Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tsevr](index.html) module
pub struct TSEVR_SPEC;
impl crate::RegisterSpec for TSEVR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tsevr::R](R) reader structure
impl crate::Readable for TSEVR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tsevr::W](W) writer structure
impl crate::Writable for TSEVR_SPEC {
    type Writer = W;
}
///`reset()` method sets TSEVR to value 0
impl crate::Resettable for TSEVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
