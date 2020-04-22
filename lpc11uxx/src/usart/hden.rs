#[doc = "Reader of register HDEN"]
pub type R = crate::R<u32, super::HDEN>;
#[doc = "Writer for register HDEN"]
pub type W = crate::W<u32, super::HDEN>;
#[doc = "Register HDEN `reset()`'s with value 0"]
impl crate::ResetValue for super::HDEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Half-duplex mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDEN_A {
    #[doc = "0: Disable half-duplex mode."]
    DISABLED = 0,
    #[doc = "1: Enable half-duplex mode."]
    ENABLED = 1,
}
impl From<HDEN_A> for bool {
    #[inline(always)]
    fn from(variant: HDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HDEN`"]
pub type HDEN_R = crate::R<bool, HDEN_A>;
impl HDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDEN_A {
        match self.bits {
            false => HDEN_A::DISABLED,
            true => HDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HDEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `HDEN`"]
pub struct HDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable half-duplex mode."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HDEN_A::DISABLED)
    }
    #[doc = "Enable half-duplex mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Half-duplex mode enable"]
    #[inline(always)]
    pub fn hden(&mut self) -> HDEN_W {
        HDEN_W { w: self }
    }
}
