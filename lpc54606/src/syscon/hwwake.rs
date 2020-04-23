#[doc = "Reader of register HWWAKE"]
pub type R = crate::R<u32, super::HWWAKE>;
#[doc = "Writer for register HWWAKE"]
pub type W = crate::W<u32, super::HWWAKE>;
#[doc = "Register HWWAKE `reset()`'s with value 0"]
impl crate::ResetValue for super::HWWAKE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FORCEWAKE`"]
pub type FORCEWAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEWAKE`"]
pub struct FORCEWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEWAKE_W<'a> {
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
#[doc = "Reader of field `FCWAKE`"]
pub type FCWAKE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCWAKE`"]
pub struct FCWAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCWAKE_W<'a> {
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
#[doc = "Reader of field `WAKEDMIC`"]
pub type WAKEDMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEDMIC`"]
pub struct WAKEDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDMIC_W<'a> {
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
#[doc = "Reader of field `WAKEDMA`"]
pub type WAKEDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAKEDMA`"]
pub struct WAKEDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEDMA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&self) -> FORCEWAKE_R {
        FORCEWAKE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&self) -> FCWAKE_R {
        FCWAKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&self) -> WAKEDMIC_R {
        WAKEDMIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&self) -> WAKEDMA_R {
        WAKEDMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force peripheral clocking to stay on during Deep Sleep and Power-down modes. When 1, clocking to peripherals is prevented from being shut down when the CPU enters Deep Sleep and Power-down modes. This is intended to allow a coprocessor to continue operating while the main CPU(s) are shut down."]
    #[inline(always)]
    pub fn forcewake(&mut self) -> FORCEWAKE_W {
        FORCEWAKE_W { w: self }
    }
    #[doc = "Bit 1 - Wake for Flexcomms. When 1, any Flexcomm FIFO reaching the level specified by its own TXLVL will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn fcwake(&mut self) -> FCWAKE_W {
        FCWAKE_W { w: self }
    }
    #[doc = "Bit 2 - Wake for Digital Microphone. When 1, the digital microphone input FIFO reaching the level specified by TRIGLVL of either channel will cause peripheral clocking to wake up temporarily while the related status is asserted."]
    #[inline(always)]
    pub fn wakedmic(&mut self) -> WAKEDMIC_W {
        WAKEDMIC_W { w: self }
    }
    #[doc = "Bit 3 - Wake for DMA. When 1, DMA being busy will cause peripheral clocking to remain running until DMA completes. This is generally used in conjunction with bit 1 and/or 2 in order to prevent peripheral clocking from being shut down as soon as the cause of wake-up is cleared, but before DMA has completed its related activity."]
    #[inline(always)]
    pub fn wakedma(&mut self) -> WAKEDMA_W {
        WAKEDMA_W { w: self }
    }
}
