#[doc = "Register `USBPLLSTAT` reader"]
pub struct R(crate::R<USBPLLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPLLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBPLLSTAT_SPEC>> for R {
    fn from(reader: crate::R<USBPLLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "PLL lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: PLL not locked"]
    PLL_NOT_LOCKED = 0,
    #[doc = "1: PLL locked"]
    PLL_LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - PLL lock status"]
pub struct LOCK_R(crate::FieldReader<bool, LOCK_A>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::PLL_NOT_LOCKED,
            true => LOCK_A::PLL_LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `PLL_NOT_LOCKED`"]
    #[inline(always)]
    pub fn is_pll_not_locked(&self) -> bool {
        **self == LOCK_A::PLL_NOT_LOCKED
    }
    #[doc = "Checks if the value of the field is `PLL_LOCKED`"]
    #[inline(always)]
    pub fn is_pll_locked(&self) -> bool {
        **self == LOCK_A::PLL_LOCKED
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, LOCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - PLL lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "USB PLL status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpllstat](index.html) module"]
pub struct USBPLLSTAT_SPEC;
impl crate::RegisterSpec for USBPLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbpllstat::R](R) reader structure"]
impl crate::Readable for USBPLLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBPLLSTAT to value 0"]
impl crate::Resettable for USBPLLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
