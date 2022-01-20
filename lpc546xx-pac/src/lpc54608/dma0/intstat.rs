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
#[doc = "Summarizes whether any enabled interrupts (other than error interrupts) are pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEINT_A {
    #[doc = "0: Not pending. No enabled interrupts are pending."]
    NOT_PENDING = 0,
    #[doc = "1: Pending. At least one enabled interrupt is pending."]
    PENDING = 1,
}
impl From<ACTIVEINT_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVEINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVEINT` reader - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
pub struct ACTIVEINT_R(crate::FieldReader<bool, ACTIVEINT_A>);
impl ACTIVEINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVEINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVEINT_A {
        match self.bits {
            false => ACTIVEINT_A::NOT_PENDING,
            true => ACTIVEINT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == ACTIVEINT_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ACTIVEINT_A::PENDING
    }
}
impl core::ops::Deref for ACTIVEINT_R {
    type Target = crate::FieldReader<bool, ACTIVEINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Summarizes whether any error interrupts are pending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTIVEERRINT_A {
    #[doc = "0: Not pending. No error interrupts are pending."]
    NOT_PENDING = 0,
    #[doc = "1: Pending. At least one error interrupt is pending."]
    PENDING = 1,
}
impl From<ACTIVEERRINT_A> for bool {
    #[inline(always)]
    fn from(variant: ACTIVEERRINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTIVEERRINT` reader - Summarizes whether any error interrupts are pending."]
pub struct ACTIVEERRINT_R(crate::FieldReader<bool, ACTIVEERRINT_A>);
impl ACTIVEERRINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACTIVEERRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVEERRINT_A {
        match self.bits {
            false => ACTIVEERRINT_A::NOT_PENDING,
            true => ACTIVEERRINT_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_PENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        **self == ACTIVEERRINT_A::NOT_PENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        **self == ACTIVEERRINT_A::PENDING
    }
}
impl core::ops::Deref for ACTIVEERRINT_R {
    type Target = crate::FieldReader<bool, ACTIVEERRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Summarizes whether any enabled interrupts (other than error interrupts) are pending."]
    #[inline(always)]
    pub fn activeint(&self) -> ACTIVEINT_R {
        ACTIVEINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Summarizes whether any error interrupts are pending."]
    #[inline(always)]
    pub fn activeerrint(&self) -> ACTIVEERRINT_R {
        ACTIVEERRINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intstat](index.html) module"]
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
