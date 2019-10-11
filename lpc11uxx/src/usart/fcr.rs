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
#[doc = "FIFO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_AW {
    #[doc = "0: USART FIFOs are disabled. Must not be used in the application."]
    DISABLED,
    #[doc = "1: Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    ENABLED,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        match variant {
            FIFOEN_AW::DISABLED => false,
            FIFOEN_AW::ENABLED => true,
        }
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
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::DISABLED)
    }
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::ENABLED)
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
#[doc = "RX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl From<RXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFORES_AW) -> Self {
        match variant {
            RXFIFORES_AW::NO_IMPACT => false,
            RXFIFORES_AW::CLEAR => true,
        }
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
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::CLEAR)
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
#[doc = "TX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR,
}
impl From<TXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFORES_AW) -> Self {
        match variant {
            TXFIFORES_AW::NO_IMPACT => false,
            TXFIFORES_AW::CLEAR => true,
        }
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
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::CLEAR)
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
#[doc = "RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    LEVEL0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    LEVEL1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    LEVEL2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    LEVEL3,
}
impl From<RXTL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTL_AW) -> Self {
        match variant {
            RXTL_AW::LEVEL0 => 0,
            RXTL_AW::LEVEL1 => 1,
            RXTL_AW::LEVEL2 => 2,
            RXTL_AW::LEVEL3 => 3,
        }
    }
}
#[doc = "Write proxy for field `RXTL`"]
pub struct RXTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTL_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL0)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL1)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL2)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(RXTL_AW::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FIFOEN_W {
        FIFOEN_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Reset"]
    #[inline(always)]
    pub fn rxfifores(&mut self) -> RXFIFORES_W {
        RXFIFORES_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Reset"]
    #[inline(always)]
    pub fn txfifores(&mut self) -> TXFIFORES_W {
        TXFIFORES_W { w: self }
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtl(&mut self) -> RXTL_W {
        RXTL_W { w: self }
    }
}
