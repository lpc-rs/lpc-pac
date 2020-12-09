#[doc = "Reader of register IOCFG"]
pub type R = crate::R<u32, super::IOCFG>;
#[doc = "Writer for register IOCFG"]
pub type W = crate::W<u32, super::IOCFG>;
#[doc = "Register IOCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::IOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_BYPASS0`"]
pub type CLK_BYPASS0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_BYPASS0`"]
pub struct CLK_BYPASS0_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_BYPASS0_W<'a> {
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
#[doc = "Reader of field `CLK_BYPASS1`"]
pub type CLK_BYPASS1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_BYPASS1`"]
pub struct CLK_BYPASS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_BYPASS1_W<'a> {
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
#[doc = "Reader of field `STEREO_DATA0`"]
pub type STEREO_DATA0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STEREO_DATA0`"]
pub struct STEREO_DATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> STEREO_DATA0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&self) -> CLK_BYPASS0_R {
        CLK_BYPASS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&self) -> CLK_BYPASS1_R {
        CLK_BYPASS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&self) -> STEREO_DATA0_R {
        STEREO_DATA0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass CLK0. When 1, PDM_DATA1 becomes the clock for PDM channel 0. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass0(&mut self) -> CLK_BYPASS0_W {
        CLK_BYPASS0_W { w: self }
    }
    #[doc = "Bit 1 - Bypass CLK1. When 1, PDM_DATA1 becomes the clock for PDM channel 1. This provides for the possibility of an external codec taking over the PDM bus."]
    #[inline(always)]
    pub fn clk_bypass1(&mut self) -> CLK_BYPASS1_W {
        CLK_BYPASS1_W { w: self }
    }
    #[doc = "Bit 2 - Stereo PDM select. When 1, PDM_DATA0 is routed to both PDM channels in a configuration that supports a single stereo digital microphone."]
    #[inline(always)]
    pub fn stereo_data0(&mut self) -> STEREO_DATA0_W {
        STEREO_DATA0_W { w: self }
    }
}
