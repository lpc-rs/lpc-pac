#[doc = "Reader of register ASYNCAPBCTRL"]
pub type R = crate::R<u32, super::ASYNCAPBCTRL>;
#[doc = "Writer for register ASYNCAPBCTRL"]
pub type W = crate::W<u32, super::ASYNCAPBCTRL>;
#[doc = "Register ASYNCAPBCTRL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::ASYNCAPBCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enables the asynchronous APB bridge and subsystem.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled. Asynchronous APB bridge is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Asynchronous APB bridge is enabled."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Asynchronous APB bridge is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled. Asynchronous APB bridge is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
    #[doc = "Bit 0 - Enables the asynchronous APB bridge and subsystem."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the asynchronous APB bridge and subsystem."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
