#[doc = "Reader of register CALIB"]
pub type R = crate::R<u32, super::CALIB>;
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SKEW_A {
    #[doc = "0: 10ms calibration value is exact"]
    SKEW_0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    SKEW_1 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, SKEW_A>;
impl SKEW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::SKEW_0,
            true => SKEW_A::SKEW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SKEW_0`"]
    #[inline(always)]
    pub fn is_skew_0(&self) -> bool {
        *self == SKEW_A::SKEW_0
    }
    #[doc = "Checks if the value of the field is `SKEW_1`"]
    #[inline(always)]
    pub fn is_skew_1(&self) -> bool {
        *self == SKEW_A::SKEW_1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOREF_A {
    #[doc = "0: The reference clock is provided"]
    NOREF_0 = 0,
    #[doc = "1: The reference clock is not provided"]
    NOREF_1 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, NOREF_A>;
impl NOREF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::NOREF_0,
            true => NOREF_A::NOREF_1,
        }
    }
    #[doc = "Checks if the value of the field is `NOREF_0`"]
    #[inline(always)]
    pub fn is_noref_0(&self) -> bool {
        *self == NOREF_A::NOREF_0
    }
    #[doc = "Checks if the value of the field is `NOREF_1`"]
    #[inline(always)]
    pub fn is_noref_1(&self) -> bool {
        *self == NOREF_A::NOREF_1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
