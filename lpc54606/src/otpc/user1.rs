#[doc = "Register `USER1` reader"]
pub struct R(crate::R<USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USER1` reader - User application specific option."]
pub struct USER1_R(crate::FieldReader<u32, u32>);
impl USER1_R {
    pub(crate) fn new(bits: u32) -> Self {
        USER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USER1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - User application specific option."]
    #[inline(always)]
    pub fn user1(&self) -> USER1_R {
        USER1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "User application specific options.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user1](index.html) module"]
pub struct USER1_SPEC;
impl crate::RegisterSpec for USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user1::R](R) reader structure"]
impl crate::Readable for USER1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USER1 to value 0"]
impl crate::Resettable for USER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
