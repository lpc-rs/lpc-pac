#[doc = "Register `INTSTAT` reader"]
pub struct R(crate::R<INTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FUFMIS` reader - FIFO underflow masked interrupt status."]
pub struct FUFMIS_R(crate::FieldReader<bool, bool>);
impl FUFMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FUFMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUFMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNBUMIS` reader - LCD next address base update masked interrupt status."]
pub struct LNBUMIS_R(crate::FieldReader<bool, bool>);
impl LNBUMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LNBUMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNBUMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOMPMIS` reader - Vertical compare masked interrupt status."]
pub struct VCOMPMIS_R(crate::FieldReader<bool, bool>);
impl VCOMPMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCOMPMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCOMPMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERMIS` reader - AHB master bus error masked interrupt status."]
pub struct BERMIS_R(crate::FieldReader<bool, bool>);
impl BERMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BERMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - FIFO underflow masked interrupt status."]
    #[inline(always)]
    pub fn fufmis(&self) -> FUFMIS_R {
        FUFMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update masked interrupt status."]
    #[inline(always)]
    pub fn lnbumis(&self) -> LNBUMIS_R {
        LNBUMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare masked interrupt status."]
    #[inline(always)]
    pub fn vcompmis(&self) -> VCOMPMIS_R {
        VCOMPMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error masked interrupt status."]
    #[inline(always)]
    pub fn bermis(&self) -> BERMIS_R {
        BERMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Masked Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
pub struct INTSTAT_SPEC;
impl crate::RegisterSpec for INTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intstat::R](R) reader structure"]
impl crate::Readable for INTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for INTSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
