///Register `MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND` reader
pub struct R(crate::R<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>) -> Self {
        R(reader)
    }
}
///Register `MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND` writer
pub struct W(crate::W<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>;
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
impl From<crate::W<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TSEC` reader - Transmit egress correction.
pub struct TSEC_R(crate::FieldReader<u32, u32>);
impl TSEC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        TSEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSEC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `TSEC` writer - Transmit egress correction.
pub struct TSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEC_W<'a> {
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    ///Bits 0:31 - Transmit egress correction.
    #[inline(always)]
    pub fn tsec(&self) -> TSEC_R {
        TSEC_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Transmit egress correction.
    #[inline(always)]
    pub fn tsec(&mut self) -> TSEC_W {
        TSEC_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Timestamp egress correction
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [mac_timestamp_egress_corr_nanosecond](index.html) module
pub struct MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC;
impl crate::RegisterSpec for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC {
    type Ux = u32;
}
///`read()` method returns [mac_timestamp_egress_corr_nanosecond::R](R) reader structure
impl crate::Readable for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [mac_timestamp_egress_corr_nanosecond::W](W) writer structure
impl crate::Writable for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC {
    type Writer = W;
}
///`reset()` method sets MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND to value 0
impl crate::Resettable for MAC_TIMESTAMP_EGRESS_CORR_NANOSECOND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
