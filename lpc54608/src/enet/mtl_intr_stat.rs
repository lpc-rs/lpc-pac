#[doc = "Register `MTL_INTR_STAT` reader"]
pub struct R(crate::R<MTL_INTR_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTL_INTR_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MTL_INTR_STAT_SPEC>> for R {
    fn from(reader: crate::R<MTL_INTR_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `Q0IS` reader - Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0."]
pub struct Q0IS_R(crate::FieldReader<bool, bool>);
impl Q0IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q0IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q0IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Q1IS` reader - Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1."]
pub struct Q1IS_R(crate::FieldReader<bool, bool>);
impl Q1IS_R {
    pub(crate) fn new(bits: bool) -> Self {
        Q1IS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for Q1IS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Queue 0 Interrupt status This bit indicates that there is an interrupt from Queue 0."]
    #[inline(always)]
    pub fn q0is(&self) -> Q0IS_R {
        Q0IS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Queue 1 Interrupt status This bit indicates that there is an interrupt from Queue 1."]
    #[inline(always)]
    pub fn q1is(&self) -> Q1IS_R {
        Q1IS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "MTL Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtl_intr_stat](index.html) module"]
pub struct MTL_INTR_STAT_SPEC;
impl crate::RegisterSpec for MTL_INTR_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtl_intr_stat::R](R) reader structure"]
impl crate::Readable for MTL_INTR_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTL_INTR_STAT to value 0"]
impl crate::Resettable for MTL_INTR_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
