#[doc = "Register `INTRAW` reader"]
pub struct R(crate::R<INTRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<INTRAW_SPEC>> for R {
    fn from(reader: crate::R<INTRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FUFRIS` reader - FIFO underflow raw interrupt status."]
pub struct FUFRIS_R(crate::FieldReader<bool, bool>);
impl FUFRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUFRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUFRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LNBURIS` reader - LCD next address base update raw interrupt status."]
pub struct LNBURIS_R(crate::FieldReader<bool, bool>);
impl LNBURIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNBURIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNBURIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOMPRIS` reader - Vertical compare raw interrupt status."]
pub struct VCOMPRIS_R(crate::FieldReader<bool, bool>);
impl VCOMPRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VCOMPRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCOMPRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BERRAW` reader - AHB master bus error raw interrupt status."]
pub struct BERRAW_R(crate::FieldReader<bool, bool>);
impl BERRAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        BERRAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BERRAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - FIFO underflow raw interrupt status."]
    #[inline(always)]
    pub fn fufris(&self) -> FUFRIS_R {
        FUFRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update raw interrupt status."]
    #[inline(always)]
    pub fn lnburis(&self) -> LNBURIS_R {
        LNBURIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical compare raw interrupt status."]
    #[inline(always)]
    pub fn vcompris(&self) -> VCOMPRIS_R {
        VCOMPRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error raw interrupt status."]
    #[inline(always)]
    pub fn berraw(&self) -> BERRAW_R {
        BERRAW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
#[doc = "Raw Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intraw](index.html) module"]
pub struct INTRAW_SPEC;
impl crate::RegisterSpec for INTRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intraw::R](R) reader structure"]
impl crate::Readable for INTRAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTRAW to value 0"]
impl crate::Resettable for INTRAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
