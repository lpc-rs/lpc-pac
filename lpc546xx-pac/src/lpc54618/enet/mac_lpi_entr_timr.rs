///Register `MAC_LPI_ENTR_TIMR` reader
pub struct R(crate::R<MAC_LPI_ENTR_TIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_LPI_ENTR_TIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_LPI_ENTR_TIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_LPI_ENTR_TIMR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_LPI_ENTR_TIMR` writer
pub struct W(crate::W<MAC_LPI_ENTR_TIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_LPI_ENTR_TIMR_SPEC>;
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
impl From<crate::W<MAC_LPI_ENTR_TIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_LPI_ENTR_TIMR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LPIET` reader - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames.
pub struct LPIET_R(crate::FieldReader<u32, u32>);
impl LPIET_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LPIET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `LPIET` writer - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames.
pub struct LPIET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIET_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 3)) | ((value as u32 & 0x0001_ffff) << 3);
        self.w
    }
}
impl R {
    ///Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames.
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(((self.bits >> 3) & 0x0001_ffff) as u32)
    }
}
impl W {
    ///Bits 3:19 - LPI Entry Timer This field specifies the time in microseconds the MAC will wait to enter LPI mode, after it has transmitted all the frames.
    #[inline(always)]
    pub fn lpiet(&mut self) -> LPIET_W {
        LPIET_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///LPI entry Timer register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_lpi_entr_timr](index.html) module
pub struct MAC_LPI_ENTR_TIMR_SPEC;
impl crate::RegisterSpec for MAC_LPI_ENTR_TIMR_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_lpi_entr_timr::R](R) reader structure
impl crate::Readable for MAC_LPI_ENTR_TIMR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_lpi_entr_timr::W](W) writer structure
impl crate::Writable for MAC_LPI_ENTR_TIMR_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_LPI_ENTR_TIMR to value 0
impl crate::Resettable for MAC_LPI_ENTR_TIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
