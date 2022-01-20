#[doc = "Register `JTAGIDCODE` reader"]
pub struct R(crate::R<JTAGIDCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JTAGIDCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JTAGIDCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JTAGIDCODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JTAGID` reader - JTAG ID code."]
pub struct JTAGID_R(crate::FieldReader<u32, u32>);
impl JTAGID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        JTAGID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAGID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - JTAG ID code."]
    #[inline(always)]
    pub fn jtagid(&self) -> JTAGID_R {
        JTAGID_R::new(self.bits)
    }
}
#[doc = "JTAG ID code register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jtagidcode](index.html) module"]
pub struct JTAGIDCODE_SPEC;
impl crate::RegisterSpec for JTAGIDCODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jtagidcode::R](R) reader structure"]
impl crate::Readable for JTAGIDCODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets JTAGIDCODE to value 0"]
impl crate::Resettable for JTAGIDCODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
