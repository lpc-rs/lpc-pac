#[doc = "Reader of register CLKENA"]
pub type R = crate::R<u32, super::CLKENA>;
#[doc = "Writer for register CLKENA"]
pub type W = crate::W<u32, super::CLKENA>;
#[doc = "Register CLKENA `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKENA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLK_ENABLE`"]
pub type CCLK_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK_ENABLE`"]
pub struct CCLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_ENABLE_W<'a> {
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
#[doc = "Reader of field `CCLK_LOW_POWER`"]
pub type CCLK_LOW_POWER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK_LOW_POWER`"]
pub struct CCLK_LOW_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_LOW_POWER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock-enable control for SD card clock."]
    #[inline(always)]
    pub fn cclk_enable(&self) -> CCLK_ENABLE_R {
        CCLK_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Low-power control for SD card clock."]
    #[inline(always)]
    pub fn cclk_low_power(&self) -> CCLK_LOW_POWER_R {
        CCLK_LOW_POWER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock-enable control for SD card clock."]
    #[inline(always)]
    pub fn cclk_enable(&mut self) -> CCLK_ENABLE_W {
        CCLK_ENABLE_W { w: self }
    }
    #[doc = "Bit 16 - Low-power control for SD card clock."]
    #[inline(always)]
    pub fn cclk_low_power(&mut self) -> CCLK_LOW_POWER_W {
        CCLK_LOW_POWER_W { w: self }
    }
}
