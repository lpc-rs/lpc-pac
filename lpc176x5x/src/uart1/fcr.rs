#[doc = "Writer for register FCR"]
pub type W = crate::W<u32, super::FCR>;
#[doc = "Register FCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FIFO enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_AW {
    #[doc = "0: Must not be used in the application."]
    MUST_NOT_BE_USED_IN_ = 0,
    #[doc = "1: Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    ACTIVE_HIGH_ENABLE_F = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FIFOEN`"]
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Must not be used in the application."]
    #[inline(always)]
    pub fn must_not_be_used_in_(self) -> &'a mut W {
        self.variant(FIFOEN_AW::MUST_NOT_BE_USED_IN_)
    }
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    #[inline(always)]
    pub fn active_high_enable_f(self) -> &'a mut W {
        self.variant(FIFOEN_AW::ACTIVE_HIGH_ENABLE_F)
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
#[doc = "RX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFORES_AW {
    #[doc = "0: No impact on either of UART1 FIFOs."]
    NO_IMPACT_ON_EITHER_ = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    WRITING_A_LOGIC_1_TO = 1,
}
impl From<RXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXFIFORES`"]
pub struct RXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFORES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFORES_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline(always)]
    pub fn no_impact_on_either_(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::NO_IMPACT_ON_EITHER_)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\]
will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn writing_a_logic_1_to(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::WRITING_A_LOGIC_1_TO)
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
#[doc = "TX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFORES_AW {
    #[doc = "0: No impact on either of UART1 FIFOs."]
    NO_IMPACT_ON_EITHER_ = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    WRITING_A_LOGIC_1_TO = 1,
}
impl From<TXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXFIFORES`"]
pub struct TXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFORES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFORES_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline(always)]
    pub fn no_impact_on_either_(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::NO_IMPACT_ON_EITHER_)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\]
will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn writing_a_logic_1_to(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::WRITING_A_LOGIC_1_TO)
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
#[doc = "Write proxy for field `DMAMODE`"]
pub struct DMAMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMODE_W<'a> {
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
#[doc = "RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXTRIGLVL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    TRIGGER_LEVEL_0_1_C = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    TRIGGER_LEVEL_1_4_C = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    TRIGGER_LEVEL_2_8_C = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    TRIGGER_LEVEL_3_14_ = 3,
}
impl From<RXTRIGLVL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTRIGLVL_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `RXTRIGLVL`"]
pub struct RXTRIGLVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTRIGLVL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTRIGLVL_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn trigger_level_0_1_c(self) -> &'a mut W {
        self.variant(RXTRIGLVL_AW::TRIGGER_LEVEL_0_1_C)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn trigger_level_1_4_c(self) -> &'a mut W {
        self.variant(RXTRIGLVL_AW::TRIGGER_LEVEL_1_4_C)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn trigger_level_2_8_c(self) -> &'a mut W {
        self.variant(RXTRIGLVL_AW::TRIGGER_LEVEL_2_8_C)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn trigger_level_3_14_(self) -> &'a mut W {
        self.variant(RXTRIGLVL_AW::TRIGGER_LEVEL_3_14_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline(always)]
    pub fn rxfifores(&mut self) -> RXFIFORES_W {
        RXFIFORES_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline(always)]
    pub fn txfifores(&mut self) -> TXFIFORES_W {
        TXFIFORES_W { w: self }
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1."]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DMAMODE_W {
        DMAMODE_W { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtriglvl(&mut self) -> RXTRIGLVL_W {
        RXTRIGLVL_W { w: self }
    }
}
