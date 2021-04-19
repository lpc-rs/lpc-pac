#[doc = "Register `USER0` reader"]
pub struct R(crate::R<USER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USER0_SPEC>> for R {
    fn from(reader: crate::R<USER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USER0` reader - User application specific option."]
pub struct USER0_R(crate::FieldReader<u32, u32>);
impl USER0_R {
    pub(crate) fn new(bits: u32) -> Self {
        USER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USER0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - User application specific option."]
    #[inline(always)]
    pub fn user0(&self) -> USER0_R {
        USER0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "User application specific options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user0](index.html) module"]
pub struct USER0_SPEC;
impl crate::RegisterSpec for USER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user0::R](R) reader structure"]
impl crate::Readable for USER0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USER0 to value 0"]
impl crate::Resettable for USER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
