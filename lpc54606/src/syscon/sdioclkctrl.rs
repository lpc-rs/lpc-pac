#[doc = "Reader of register SDIOCLKCTRL"]
pub type R = crate::R<u32, super::SDIOCLKCTRL>;
#[doc = "Writer for register SDIOCLKCTRL"]
pub type W = crate::W<u32, super::SDIOCLKCTRL>;
#[doc = "Register SDIOCLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIOCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCLK_DRV_PHASE`"]
pub type CCLK_DRV_PHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_DRV_PHASE`"]
pub struct CCLK_DRV_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_PHASE`"]
pub type CCLK_SAMPLE_PHASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_SAMPLE_PHASE`"]
pub struct CCLK_SAMPLE_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_PHASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PHASE_ACTIVE`"]
pub type PHASE_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHASE_ACTIVE`"]
pub struct PHASE_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_ACTIVE_W<'a> {
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
#[doc = "Reader of field `CCLK_DRV_DELAY`"]
pub type CCLK_DRV_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_DRV_DELAY`"]
pub struct CCLK_DRV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CCLK_DRV_DELAY_ACTIVE`"]
pub type CCLK_DRV_DELAY_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK_DRV_DELAY_ACTIVE`"]
pub struct CCLK_DRV_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_DELAY`"]
pub type CCLK_SAMPLE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CCLK_SAMPLE_DELAY`"]
pub struct CCLK_SAMPLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
#[doc = "Reader of field `CCLK_SAMPLE_DELAY_ACTIVE`"]
pub type CCLK_SAMPLE_DELAY_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCLK_SAMPLE_DELAY_ACTIVE`"]
pub struct CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
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
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&self) -> CCLK_DRV_PHASE_R {
        CCLK_DRV_PHASE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&self) -> CCLK_SAMPLE_PHASE_R {
        CCLK_SAMPLE_PHASE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
    #[inline(always)]
    pub fn phase_active(&self) -> PHASE_ACTIVE_R {
        PHASE_ACTIVE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&self) -> CCLK_DRV_DELAY_R {
        CCLK_DRV_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&self) -> CCLK_DRV_DELAY_ACTIVE_R {
        CCLK_DRV_DELAY_ACTIVE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&self) -> CCLK_SAMPLE_DELAY_R {
        CCLK_SAMPLE_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&self) -> CCLK_SAMPLE_DELAY_ACTIVE_R {
        CCLK_SAMPLE_DELAY_ACTIVE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W {
        CCLK_DRV_PHASE_W { w: self }
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W {
        CCLK_SAMPLE_PHASE_W { w: self }
    }
    #[doc = "Bit 7 - sdio_clk by 2, before feeding into ccl_in, cclk_in_sample, and cclk_in_drv."]
    #[inline(always)]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W {
        PHASE_ACTIVE_W { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W {
        CCLK_DRV_DELAY_W { w: self }
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W {
        CCLK_DRV_DELAY_ACTIVE_W { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W {
        CCLK_SAMPLE_DELAY_W { w: self }
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W {
        CCLK_SAMPLE_DELAY_ACTIVE_W { w: self }
    }
}
