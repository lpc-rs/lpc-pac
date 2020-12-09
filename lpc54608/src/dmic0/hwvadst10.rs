#[doc = "Reader of register HWVADST10"]
pub type R = crate::R<u32, super::HWVADST10>;
#[doc = "Writer for register HWVADST10"]
pub type W = crate::W<u32, super::HWVADST10>;
#[doc = "Register HWVADST10 `reset()`'s with value 0"]
impl crate::ResetValue for super::HWVADST10 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stage 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ST10_A {
    #[doc = "0: Normal operation, waiting for HWVAD trigger event (stage 0)."]
    NORMAL = 0,
    #[doc = "1: Reset internal interrupt flag by writing a '1' pulse."]
    RESET = 1,
}
impl From<ST10_A> for bool {
    #[inline(always)]
    fn from(variant: ST10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ST10`"]
pub type ST10_R = crate::R<bool, ST10_A>;
impl ST10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ST10_A {
        match self.bits {
            false => ST10_A::NORMAL,
            true => ST10_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ST10_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == ST10_A::RESET
    }
}
#[doc = "Write proxy for field `ST10`"]
pub struct ST10_W<'a> {
    w: &'a mut W,
}
impl<'a> ST10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ST10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation, waiting for HWVAD trigger event (stage 0)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ST10_A::NORMAL)
    }
    #[doc = "Reset internal interrupt flag by writing a '1' pulse."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(ST10_A::RESET)
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
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&self) -> ST10_R {
        ST10_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stage 0"]
    #[inline(always)]
    pub fn st10(&mut self) -> ST10_W {
        ST10_W { w: self }
    }
}
