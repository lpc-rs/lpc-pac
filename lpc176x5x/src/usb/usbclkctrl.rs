#[doc = "Reader of register USBCLKCTRL"]
pub type R = crate::R<u32, super::USBCLKCTRL>;
#[doc = "Writer for register USBCLKCTRL"]
pub type W = crate::W<u32, super::USBCLKCTRL>;
#[doc = "Register USBCLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEV_CLK_EN`"]
pub type DEV_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEV_CLK_EN`"]
pub struct DEV_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_CLK_EN_W<'a> {
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
#[doc = "Reader of field `PORTSEL_CLK_EN`"]
pub type PORTSEL_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORTSEL_CLK_EN`"]
pub struct PORTSEL_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSEL_CLK_EN_W<'a> {
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
#[doc = "Reader of field `AHB_CLK_EN`"]
pub type AHB_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHB_CLK_EN`"]
pub struct AHB_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DEV_CLK_EN_R {
        DEV_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    pub fn portsel_clk_en(&self) -> PORTSEL_CLK_EN_R {
        PORTSEL_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AHB_CLK_EN_R {
        AHB_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Device clock enable. Enables the usbclk input to the device controller"]
    #[inline(always)]
    pub fn dev_clk_en(&mut self) -> DEV_CLK_EN_W {
        DEV_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - Port select register clock enable."]
    #[inline(always)]
    pub fn portsel_clk_en(&mut self) -> PORTSEL_CLK_EN_W {
        PORTSEL_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4 - AHB clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&mut self) -> AHB_CLK_EN_W {
        AHB_CLK_EN_W { w: self }
    }
}
