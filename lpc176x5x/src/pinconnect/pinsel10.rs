#[doc = "Reader of register PINSEL10"]
pub type R = crate::R<u32, super::PINSEL10>;
#[doc = "Writer for register PINSEL10"]
pub type W = crate::W<u32, super::PINSEL10>;
#[doc = "Register PINSEL10 `reset()`'s with value 0"]
impl crate::ResetValue for super::PINSEL10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TPIU interface pins control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPIUCTRL_A {
    #[doc = "0: Disabled. TPIU interface is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    ENABLED = 1,
}
impl From<TPIUCTRL_A> for bool {
    #[inline(always)]
    fn from(variant: TPIUCTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPIUCTRL`"]
pub type TPIUCTRL_R = crate::R<bool, TPIUCTRL_A>;
impl TPIUCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPIUCTRL_A {
        match self.bits {
            false => TPIUCTRL_A::DISABLED,
            true => TPIUCTRL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TPIUCTRL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TPIUCTRL_A::ENABLED
    }
}
#[doc = "Write proxy for field `TPIUCTRL`"]
pub struct TPIUCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TPIUCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPIUCTRL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. TPIU interface is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TPIUCTRL_A::DISABLED)
    }
    #[doc = "Enabled. TPIU interface is enabled. TPIU signals are available on the pins hosting them regardless of the PINSEL4 content."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TPIUCTRL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&self) -> TPIUCTRL_R {
        TPIUCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TPIU interface pins control."]
    #[inline(always)]
    pub fn tpiuctrl(&mut self) -> TPIUCTRL_W {
        TPIUCTRL_W { w: self }
    }
}
