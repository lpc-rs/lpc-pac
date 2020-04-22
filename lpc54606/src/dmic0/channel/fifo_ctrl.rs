#[doc = "Reader of register FIFO_CTRL"]
pub type R = crate::R<u32, super::FIFO_CTRL>;
#[doc = "Writer for register FIFO_CTRL"]
pub type W = crate::W<u32, super::FIFO_CTRL>;
#[doc = "Register FIFO_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    DISABLED = 0,
    #[doc = "1: FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO is not enabled. Enabling a DMIC channel with the FIFO disabled could be useful while data is being streamed to the I2S, or in order to avoid a filter settling delay when a channel is re-enabled after a period when the data was not needed."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "FIFO is enabled. The FIFO must be enabled in order for the CPU or DMA to read data from the DMIC via the FIFODATA register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "FIFO reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETN_A {
    #[doc = "0: Reset the FIFO."]
    RESET = 0,
    #[doc = "1: Normal operation"]
    NORMAL = 1,
}
impl From<RESETN_A> for bool {
    #[inline(always)]
    fn from(variant: RESETN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESETN`"]
pub type RESETN_R = crate::R<bool, RESETN_A>;
impl RESETN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESETN_A {
        match self.bits {
            false => RESETN_A::RESET,
            true => RESETN_A::NORMAL,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETN_A::RESET
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RESETN_A::NORMAL
    }
}
#[doc = "Write proxy for field `RESETN`"]
pub struct RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESETN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the FIFO."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETN_A::RESET)
    }
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RESETN_A::NORMAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTEN_A {
    #[doc = "0: FIFO level interrupts are not enabled."]
    DISABLED = 0,
    #[doc = "1: FIFO level interrupts are enabled."]
    ENABLED = 1,
}
impl From<INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTEN`"]
pub type INTEN_R = crate::R<bool, INTEN_A>;
impl INTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTEN_A {
        match self.bits {
            false => INTEN_A::DISABLED,
            true => INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `INTEN`"]
pub struct INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FIFO level interrupts are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTEN_A::DISABLED)
    }
    #[doc = "FIFO level interrupts are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAEN_A {
    #[doc = "0: DMA requests are not enabled."]
    DISABLED = 0,
    #[doc = "1: DMA requests based on FIFO level are enabled."]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, DMAEN_A>;
impl DMAEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA requests are not enabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "DMA requests based on FIFO level are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
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
#[doc = "Reader of field `TRIGLVL`"]
pub type TRIGLVL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRIGLVL`"]
pub struct TRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGLVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&self) -> RESETN_R {
        RESETN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&self) -> TRIGLVL_R {
        TRIGLVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - FIFO reset."]
    #[inline(always)]
    pub fn resetn(&mut self) -> RESETN_W {
        RESETN_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable."]
    #[inline(always)]
    pub fn inten(&mut self) -> INTEN_W {
        INTEN_W { w: self }
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bits 16:20 - FIFO trigger level. Selects the data trigger level for interrupt or DMA operation. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode See Section 4.5.66 'Hardware Wake-up control register'. 0 = trigger when the FIFO has received one entry (is no longer empty). 1 = trigger when the FIFO has received two entries. 15 = trigger when the FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn triglvl(&mut self) -> TRIGLVL_W {
        TRIGLVL_W { w: self }
    }
}
