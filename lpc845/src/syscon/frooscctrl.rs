#[doc = "Reader of register FROOSCCTRL"]
pub type R = crate::R<u32, super::FROOSCCTRL>;
#[doc = "Writer for register FROOSCCTRL"]
pub type W = crate::W<u32, super::FROOSCCTRL>;
#[doc = "Register FROOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FROOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "fro direct clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_DIRECT_A {
    #[doc = "0: fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    DISABLED = 0,
    #[doc = "1: fro clock is direct from FRO oscillator"]
    ENABLED = 1,
}
impl From<FRO_DIRECT_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_DIRECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRO_DIRECT`"]
pub type FRO_DIRECT_R = crate::R<bool, FRO_DIRECT_A>;
impl FRO_DIRECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_DIRECT_A {
        match self.bits {
            false => FRO_DIRECT_A::DISABLED,
            true => FRO_DIRECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FRO_DIRECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FRO_DIRECT_A::ENABLED
    }
}
#[doc = "Write proxy for field `FRO_DIRECT`"]
pub struct FRO_DIRECT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_DIRECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO_DIRECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "fro clock is divider by 2 or 16,depend on FAIM slow boot value"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FRO_DIRECT_A::DISABLED)
    }
    #[doc = "fro clock is direct from FRO oscillator"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FRO_DIRECT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline(always)]
    pub fn fro_direct(&self) -> FRO_DIRECT_R {
        FRO_DIRECT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - fro direct clock select"]
    #[inline(always)]
    pub fn fro_direct(&mut self) -> FRO_DIRECT_W {
        FRO_DIRECT_W { w: self }
    }
}
