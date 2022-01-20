#[doc = "Register `PDRUNCFGCLR1` reader"]
pub struct R(crate::R<PDRUNCFGCLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRUNCFGCLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRUNCFGCLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRUNCFGCLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRUNCFGCLR1` writer"]
pub struct W(crate::W<PDRUNCFGCLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRUNCFGCLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDRUNCFGCLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRUNCFGCLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDEN_USB1_PHY` reader - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
pub struct PDEN_USB1_PHY_R(crate::FieldReader<bool, bool>);
impl PDEN_USB1_PHY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_USB1_PHY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_USB1_PHY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_USB1_PHY` writer - USB1 high speed PHY (also, enable/disable bit 28 in PDRUNCFG0 register)."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `PDEN_USB1_PLL` reader - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub struct PDEN_USB1_PLL_R(crate::FieldReader<bool, bool>);
impl PDEN_USB1_PLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_USB1_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_USB1_PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_USB1_PLL` writer - USB PLL (PLL1) power (also, enable/disable bit 26 in PDRUNCFG0 register)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PDEN_AUD_PLL` reader - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
pub struct PDEN_AUD_PLL_R(crate::FieldReader<bool, bool>);
impl PDEN_AUD_PLL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_AUD_PLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_AUD_PLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_AUD_PLL` writer - Audio PLL (PLL2) power and fractional divider (also, enable/disable bit 26 in PDRUNCFG0 register)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PDEN_SYSOSC` reader - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
pub struct PDEN_SYSOSC_R(crate::FieldReader<bool, bool>);
impl PDEN_SYSOSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_SYSOSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_SYSOSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_SYSOSC` writer - System Oscillator Power (also, enable/disable bit 9 in PDRUNCFG0 register)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PDEN_EEPROM` reader - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
pub struct PDEN_EEPROM_R(crate::FieldReader<bool, bool>);
impl PDEN_EEPROM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_EEPROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_EEPROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_EEPROM` writer - EEPROM power (also, enable/disable bit 29 in PDRUNCFG0 register)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PDEN_RNG` reader - Random Number Generator Power."]
pub struct PDEN_RNG_R(crate::FieldReader<bool, bool>);
impl PDEN_RNG_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_RNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_RNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN_RNG` writer - Random Number Generator Power."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power configuration clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdruncfgclr1](index.html) module"]
pub struct PDRUNCFGCLR1_SPEC;
impl crate::RegisterSpec for PDRUNCFGCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdruncfgclr1::R](R) reader structure"]
impl crate::Readable for PDRUNCFGCLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdruncfgclr1::W](W) writer structure"]
impl crate::Writable for PDRUNCFGCLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDRUNCFGCLR1 to value 0"]
impl crate::Resettable for PDRUNCFGCLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
