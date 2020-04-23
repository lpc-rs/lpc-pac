#[doc = "Reader of register PDRUNCFG1"]
pub type R = crate::R<u32, super::PDRUNCFG1>;
#[doc = "Writer for register PDRUNCFG1"]
pub type W = crate::W<u32, super::PDRUNCFG1>;
#[doc = "Register PDRUNCFG1 `reset()`'s with value 0x14f8_1f40"]
impl crate::ResetValue for super::PDRUNCFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14f8_1f40
    }
}
#[doc = "Reader of field `PDEN_USB1_PHY`"]
pub type PDEN_USB1_PHY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_USB1_PHY`"]
pub struct PDEN_USB1_PHY_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB1_PHY_W<'a> {
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
#[doc = "Reader of field `PDEN_USB1_PLL`"]
pub type PDEN_USB1_PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_USB1_PLL`"]
pub struct PDEN_USB1_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_USB1_PLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `PDEN_AUD_PLL`"]
pub type PDEN_AUD_PLL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_AUD_PLL`"]
pub struct PDEN_AUD_PLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_AUD_PLL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PDEN_SYSOSC`"]
pub type PDEN_SYSOSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_SYSOSC`"]
pub struct PDEN_SYSOSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_SYSOSC_W<'a> {
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
#[doc = "Reader of field `PDEN_EEPROM`"]
pub type PDEN_EEPROM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_EEPROM`"]
pub struct PDEN_EEPROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_EEPROM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PDEN_RNG`"]
pub type PDEN_RNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN_RNG`"]
pub struct PDEN_RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_RNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&self) -> PDEN_USB1_PHY_R {
        PDEN_USB1_PHY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&self) -> PDEN_USB1_PLL_R {
        PDEN_USB1_PLL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&self) -> PDEN_AUD_PLL_R {
        PDEN_AUD_PLL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&self) -> PDEN_SYSOSC_R {
        PDEN_SYSOSC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&self) -> PDEN_EEPROM_R {
        PDEN_EEPROM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&self) -> PDEN_RNG_R {
        PDEN_RNG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_phy(&mut self) -> PDEN_USB1_PHY_W {
        PDEN_USB1_PHY_W { w: self }
    }
    #[doc = "Bit 1 - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_usb1_pll(&mut self) -> PDEN_USB1_PLL_W {
        PDEN_USB1_PLL_W { w: self }
    }
    #[doc = "Bit 2 - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_aud_pll(&mut self) -> PDEN_AUD_PLL_W {
        PDEN_AUD_PLL_W { w: self }
    }
    #[doc = "Bit 3 - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_sysosc(&mut self) -> PDEN_SYSOSC_W {
        PDEN_SYSOSC_W { w: self }
    }
    #[doc = "Bit 5 - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
    #[inline(always)]
    pub fn pden_eeprom(&mut self) -> PDEN_EEPROM_W {
        PDEN_EEPROM_W { w: self }
    }
    #[doc = "Bit 7 - Random Number Generator Power."]
    #[inline(always)]
    pub fn pden_rng(&mut self) -> PDEN_RNG_W {
        PDEN_RNG_W { w: self }
    }
}
