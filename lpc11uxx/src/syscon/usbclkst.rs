#[doc = "Register `USBCLKST` reader"]
pub struct R(crate::R<USBCLKST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCLKST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCLKST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCLKST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "USB need_clock signal status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEED_CLKST_A {
    #[doc = "0: LOW"]
    LOW = 0,
    #[doc = "1: HIGH"]
    HIGH = 1,
}
impl From<NEED_CLKST_A> for bool {
    #[inline(always)]
    fn from(variant: NEED_CLKST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEED_CLKST` reader - USB need_clock signal status"]
pub struct NEED_CLKST_R(crate::FieldReader<bool, NEED_CLKST_A>);
impl NEED_CLKST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NEED_CLKST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEED_CLKST_A {
        match self.bits {
            false => NEED_CLKST_A::LOW,
            true => NEED_CLKST_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == NEED_CLKST_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == NEED_CLKST_A::HIGH
    }
}
impl core::ops::Deref for NEED_CLKST_R {
    type Target = crate::FieldReader<bool, NEED_CLKST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - USB need_clock signal status"]
    #[inline(always)]
    pub fn need_clkst(&self) -> NEED_CLKST_R {
        NEED_CLKST_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "USB clock status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbclkst](index.html) module"]
pub struct USBCLKST_SPEC;
impl crate::RegisterSpec for USBCLKST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbclkst::R](R) reader structure"]
impl crate::Readable for USBCLKST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBCLKST to value 0x01"]
impl crate::Resettable for USBCLKST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
