#[doc = "Reader of register AUDPLLCTRL"]
pub type R = crate::R<u32, super::AUDPLLCTRL>;
#[doc = "Writer for register AUDPLLCTRL"]
pub type W = crate::W<u32, super::AUDPLLCTRL>;
#[doc = "Register AUDPLLCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::AUDPLLCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SELR`"]
pub type SELR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELR`"]
pub struct SELR_W<'a> {
    w: &'a mut W,
}
impl<'a> SELR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SELI`"]
pub type SELI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELI`"]
pub struct SELI_W<'a> {
    w: &'a mut W,
}
impl<'a> SELI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `SELP`"]
pub type SELP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SELP`"]
pub struct SELP_W<'a> {
    w: &'a mut W,
}
impl<'a> SELP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | (((value as u32) & 0x1f) << 10);
        self.w
    }
}
#[doc = "PLL bypass control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    DISABLED = 0,
    #[doc = "1: Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
    ENABLED = 1,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bypass disabled. PLL CCO is sent to the PLL post-dividers."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Bypass enabled. PLL input clock is sent directly to the PLL output (default)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `UPLIMOFF`"]
pub type UPLIMOFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UPLIMOFF`"]
pub struct UPLIMOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> UPLIMOFF_W<'a> {
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
#[doc = "Reader of field `DIRECTI`"]
pub type DIRECTI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIRECTI`"]
pub struct DIRECTI_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "PLL direct output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTO_A {
    #[doc = "0: Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    DISABLED = 0,
    #[doc = "1: Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    ENABLED = 1,
}
impl From<DIRECTO_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRECTO`"]
pub type DIRECTO_R = crate::R<bool, DIRECTO_A>;
impl DIRECTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTO_A {
        match self.bits {
            false => DIRECTO_A::DISABLED,
            true => DIRECTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRECTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRECTO_A::ENABLED
    }
}
#[doc = "Write proxy for field `DIRECTO`"]
pub struct DIRECTO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIRECTO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. The PLL output divider (P divider) is used to create the PLL output."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::DISABLED)
    }
    #[doc = "Enabled. The PLL output divider (P divider) is bypassed, the PLL CCO output is used as the PLL output."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DIRECTO_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&self) -> SELR_R {
        SELR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&self) -> SELI_R {
        SELI_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:14 - ."]
    #[inline(always)]
    pub fn selp(&self) -> SELP_R {
        SELP_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&self) -> UPLIMOFF_R {
        UPLIMOFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - PLL direct input enable."]
    #[inline(always)]
    pub fn directi(&self) -> DIRECTI_R {
        DIRECTI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - PLL direct output enable"]
    #[inline(always)]
    pub fn directo(&self) -> DIRECTO_R {
        DIRECTO_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bandwidth select R value."]
    #[inline(always)]
    pub fn selr(&mut self) -> SELR_W {
        SELR_W { w: self }
    }
    #[doc = "Bits 4:9 - Bandwidth select I value."]
    #[inline(always)]
    pub fn seli(&mut self) -> SELI_W {
        SELI_W { w: self }
    }
    #[doc = "Bits 10:14 - ."]
    #[inline(always)]
    pub fn selp(&mut self) -> SELP_W {
        SELP_W { w: self }
    }
    #[doc = "Bit 15 - PLL bypass control."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 17 - Disable upper frequency limiter."]
    #[inline(always)]
    pub fn uplimoff(&mut self) -> UPLIMOFF_W {
        UPLIMOFF_W { w: self }
    }
    #[doc = "Bit 19 - PLL direct input enable."]
    #[inline(always)]
    pub fn directi(&mut self) -> DIRECTI_W {
        DIRECTI_W { w: self }
    }
    #[doc = "Bit 20 - PLL direct output enable"]
    #[inline(always)]
    pub fn directo(&mut self) -> DIRECTO_W {
        DIRECTO_W { w: self }
    }
}
