#[doc = "Reader of register CLKSEL"]
pub type R = crate::R<u32, super::CLKSEL>;
#[doc = "Writer for register CLKSEL"]
pub type W = crate::W<u32, super::CLKSEL>;
#[doc = "Register CLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Selects source of WDT clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Watchdog oscillator (WDOSC)"]
    WATCHDOG_OSCILLATOR = 1,
}
impl From<CLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<bool, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            false => CLKSEL_A::IRC,
            true => CLKSEL_A::WATCHDOG_OSCILLATOR,
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == CLKSEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `WATCHDOG_OSCILLATOR`"]
    #[inline(always)]
    pub fn is_watchdog_oscillator(&self) -> bool {
        *self == CLKSEL_A::WATCHDOG_OSCILLATOR
    }
}
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(CLKSEL_A::IRC)
    }
    #[doc = "Watchdog oscillator (WDOSC)"]
    #[inline(always)]
    pub fn watchdog_oscillator(self) -> &'a mut W {
        self.variant(CLKSEL_A::WATCHDOG_OSCILLATOR)
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
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
    #[doc = "Bit 0 - Selects source of WDT clock"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects source of WDT clock"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
}
