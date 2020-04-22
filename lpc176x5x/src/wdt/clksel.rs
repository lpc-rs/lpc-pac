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
#[repr(u8)]
pub enum CLKSEL_A {
    #[doc = "0: IRC"]
    IRC = 0,
    #[doc = "1: Peripheral clock"]
    PCLK = 1,
    #[doc = "2: RTC oscillator"]
    RTCOSC = 2,
}
impl From<CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, CLKSEL_A>;
impl CLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSEL_A {
        match self.bits {
            0 => CLKSEL_A::IRC,
            1 => CLKSEL_A::PCLK,
            2 => CLKSEL_A::RTCOSC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IRC`"]
    #[inline(always)]
    pub fn is_irc(&self) -> bool {
        *self == CLKSEL_A::IRC
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == CLKSEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `RTCOSC`"]
    #[inline(always)]
    pub fn is_rtcosc(&self) -> bool {
        *self == CLKSEL_A::RTCOSC
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IRC"]
    #[inline(always)]
    pub fn irc(self) -> &'a mut W {
        self.variant(CLKSEL_A::IRC)
    }
    #[doc = "Peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(CLKSEL_A::PCLK)
    }
    #[doc = "RTC oscillator"]
    #[inline(always)]
    pub fn rtcosc(self) -> &'a mut W {
        self.variant(CLKSEL_A::RTCOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCK_A {
    #[doc = "0: This bit is set to 0 on any reset. It cannot be cleared by software."]
    UNLOCKED = 0,
    #[doc = "1: Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register\n\t\t\t\t\t\t\t\t\t\tcannot be modified."]
    LOCKED = 1,
}
impl From<LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCK`"]
pub type LOCK_R = crate::R<bool, LOCK_A>;
impl LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_A {
        match self.bits {
            false => LOCK_A::UNLOCKED,
            true => LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `LOCK`"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This bit is set to 0 on any reset. It cannot be cleared by software."]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCK_A::UNLOCKED)
    }
    #[doc = "Software can set this bit to 1 at any time. Once WDLOCK is set, the bits of this register cannot be modified."]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCK_A::LOCKED)
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
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - If this bit is set to one writing to this register does not affect bit 0. The clock source can only be changed by first clearing this bit, then writing the new value of bit 0."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects source of WDT clock"]
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
