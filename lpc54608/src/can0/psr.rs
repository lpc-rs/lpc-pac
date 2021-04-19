#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PSR_SPEC>> for R {
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LEC` reader - Last error code."]
pub struct LEC_R(crate::FieldReader<u8, u8>);
impl LEC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT` reader - Activity."]
pub struct ACT_R(crate::FieldReader<u8, u8>);
impl ACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP` reader - Error Passive."]
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EW` reader - Warning status."]
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BO` reader - Bus Off Status."]
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PXE` reader - Protocol exception event."]
pub struct PXE_R(crate::FieldReader<bool, bool>);
impl PXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDCV` reader - Transmitter delay compensation value."]
pub struct TDCV_R(crate::FieldReader<u8, u8>);
impl TDCV_R {
    pub(crate) fn new(bits: u8) -> Self {
        TDCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDCV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Last error code."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - Activity."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Error Passive."]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Warning status."]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Off Status."]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protocol exception event."]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter delay compensation value."]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
#[doc = "Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0707
    }
}
