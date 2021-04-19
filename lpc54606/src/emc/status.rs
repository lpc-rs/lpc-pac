#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STATUS_SPEC>> for R {
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `B` reader - Busy."]
pub struct B_R(crate::FieldReader<bool, bool>);
impl B_R {
    pub(crate) fn new(bits: bool) -> Self {
        B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S` reader - Write buffer status."]
pub struct S_R(crate::FieldReader<bool, bool>);
impl S_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SA` reader - Self-refresh acknowledge."]
pub struct SA_R(crate::FieldReader<bool, bool>);
impl SA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Busy."]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write buffer status."]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh acknowledge."]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Provides EMC status information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x05"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
