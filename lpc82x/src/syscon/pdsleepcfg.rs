#[doc = "Reader of register PDSLEEPCFG"]
pub type R = crate::R<u32, super::PDSLEEPCFG>;
#[doc = "Writer for register PDSLEEPCFG"]
pub type W = crate::W<u32, super::PDSLEEPCFG>;
#[doc = "Register PDSLEEPCFG `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::PDSLEEPCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "BOD power-down control for Deep-sleep and Power-down mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BOD_PD`"]
pub type BOD_PD_R = crate::R<bool, BOD_PD_A>;
impl BOD_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PD_A::POWERED_DOWN
    }
}
#[doc = "Write proxy for field `BOD_PD`"]
pub struct BOD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
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
#[doc = "Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDTOSC_PD`"]
pub type WDTOSC_PD_R = crate::R<bool, WDTOSC_PD_A>;
impl WDTOSC_PD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::DISABLED,
            true => WDTOSC_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDTOSC_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WDTOSC_PD_A::ENABLED
    }
}
#[doc = "Write proxy for field `WDTOSC_PD`"]
pub struct WDTOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - BOD power-down control for Deep-sleep and Power-down mode"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W {
        BOD_PD_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator power-down control for Deep-sleep and Power-down mode. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running."]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W {
        WDTOSC_PD_W { w: self }
    }
}
