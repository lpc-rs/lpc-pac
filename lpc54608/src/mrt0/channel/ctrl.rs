#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable the TIMERn interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: Disabled. TIMERn interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. TIMERn interrupt is enabled."]
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, INTEN_A>;
impl INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. TIMERn interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    #[doc = "Enabled. TIMERn interrupt is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
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
#[doc = "Selects timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Repeat interrupt mode."]
    REPEAT_INTERRUPT_MODE = 0,
    #[doc = "1: One-shot interrupt mode."]
    ONE_SHOT_INTERRUPT_MODE = 1,
    #[doc = "2: One-shot stall mode."]
    ONE_SHOT_STALL_MODE = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::REPEAT_INTERRUPT_MODE),
            1 => Val(MODE_A::ONE_SHOT_INTERRUPT_MODE),
            2 => Val(MODE_A::ONE_SHOT_STALL_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REPEAT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_repeat_interrupt_mode(&self) -> bool {
        *self == MODE_A::REPEAT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_INTERRUPT_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_interrupt_mode(&self) -> bool {
        *self == MODE_A::ONE_SHOT_INTERRUPT_MODE
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT_STALL_MODE`"]
    #[inline(always)]
    pub fn is_one_shot_stall_mode(&self) -> bool {
        *self == MODE_A::ONE_SHOT_STALL_MODE
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Repeat interrupt mode."]
    #[inline(always)]
    pub fn repeat_interrupt_mode(self) -> &'a mut W {
        self.variant(MODE_A::REPEAT_INTERRUPT_MODE)
    }
    #[doc = "One-shot interrupt mode."]
    #[inline(always)]
    pub fn one_shot_interrupt_mode(self) -> &'a mut W {
        self.variant(MODE_A::ONE_SHOT_INTERRUPT_MODE)
    }
    #[doc = "One-shot stall mode."]
    #[inline(always)]
    pub fn one_shot_stall_mode(self) -> &'a mut W {
        self.variant(MODE_A::ONE_SHOT_STALL_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bits 1:2 - Selects timer mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
