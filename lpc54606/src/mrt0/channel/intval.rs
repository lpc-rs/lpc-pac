#[doc = "Reader of register INTVAL"]
pub type R = crate::R<u32, super::INTVAL>;
#[doc = "Writer for register INTVAL"]
pub type W = crate::W<u32, super::INTVAL>;
#[doc = "Register INTVAL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTVAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVALUE`"]
pub type IVALUE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IVALUE`"]
pub struct IVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> IVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOAD_A {
    #[doc = "0: No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    NO_FORCE_LOAD = 0,
    #[doc = "1: Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    FORCE_LOAD = 1,
}
impl From<LOAD_A> for bool {
    #[inline(always)]
    fn from(variant: LOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOAD`"]
pub type LOAD_R = crate::R<bool, LOAD_A>;
impl LOAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOAD_A {
        match self.bits {
            false => LOAD_A::NO_FORCE_LOAD,
            true => LOAD_A::FORCE_LOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_no_force_load(&self) -> bool {
        *self == LOAD_A::NO_FORCE_LOAD
    }
    #[doc = "Checks if the value of the field is `FORCE_LOAD`"]
    #[inline(always)]
    pub fn is_force_load(&self) -> bool {
        *self == LOAD_A::FORCE_LOAD
    }
}
#[doc = "Write proxy for field `LOAD`"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No force load. The load from the INTVALn register to the TIMERn register is processed at the end of the time interval if the repeat mode is selected."]
    #[inline(always)]
    pub fn no_force_load(self) -> &'a mut W {
        self.variant(LOAD_A::NO_FORCE_LOAD)
    }
    #[doc = "Force load. The INTVALn interval value IVALUE -1 is immediately loaded into the TIMERn register while TIMERn is running."]
    #[inline(always)]
    pub fn force_load(self) -> &'a mut W {
        self.variant(LOAD_A::FORCE_LOAD)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&self) -> IVALUE_R {
        IVALUE_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn ivalue(&mut self) -> IVALUE_W {
        IVALUE_W { w: self }
    }
    #[doc = "Bit 31 - Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
}
