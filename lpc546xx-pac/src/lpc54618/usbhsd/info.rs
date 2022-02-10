///Register `INFO` reader
pub struct R(crate::R<INFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INFO_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FRAME_NR` reader - Frame number.
pub struct FRAME_NR_R(crate::FieldReader<u16, u16>);
impl FRAME_NR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FRAME_NR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_NR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `ERR_CODE` reader - The error code which last occurred:.
pub struct ERR_CODE_R(crate::FieldReader<u8, u8>);
impl ERR_CODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERR_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Minrev` reader - Minor revision.
pub struct MINREV_R(crate::FieldReader<u8, u8>);
impl MINREV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MINREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `Majrev` reader - Major revision.
pub struct MAJREV_R(crate::FieldReader<u8, u8>);
impl MAJREV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAJREV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAJREV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:10 - Frame number.
    #[inline(always)]
    pub fn frame_nr(&self) -> FRAME_NR_R {
        FRAME_NR_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 11:14 - The error code which last occurred:.
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    ///Bits 16:23 - Minor revision.
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - Major revision.
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
///USB Info register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [info](index.html) module
pub struct INFO_SPEC;
impl crate::RegisterSpec for INFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [info::R](R) reader structure
impl crate::Readable for INFO_SPEC {
    type Reader = R;
}
///`reset()` method sets INFO to value 0x0200_0000
impl crate::Resettable for INFO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0000
    }
}
