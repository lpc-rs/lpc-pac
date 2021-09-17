#[doc = "Register `FCR` writer"]
pub struct W(crate::W<FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCR_SPEC>;
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
impl From<crate::W<FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FIFO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFOEN_AW {
    #[doc = "0: USART FIFOs are disabled. Must not be used in the application."]
    DISABLED = 0,
    #[doc = "1: Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
    ENABLED = 1,
}
impl From<FIFOEN_AW> for bool {
    #[inline(always)]
    fn from(variant: FIFOEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO enable"]
pub struct FIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFOEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USART FIFOs are disabled. Must not be used in the application."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FIFOEN_AW::DISABLED)
    }
    #[doc = "Active high enable for both USART Rx and TX FIFOs and FCR\\[7:1\\]
access. This bit must be set for proper USART operation. Any transition on this bit will automatically clear the USART FIFOs."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "RX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\]
will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<RXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset"]
pub struct RXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFORES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFIFORES_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(RXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\]
will clear all bytes in USART Rx FIFO, reset the pointer logic. This bit is self-clearing."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "TX FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFORES_AW {
    #[doc = "0: No impact on either of USART FIFOs."]
    NO_IMPACT = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\]
will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
    CLEAR = 1,
}
impl From<TXFIFORES_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFIFORES_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset"]
pub struct TXFIFORES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFORES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFIFORES_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No impact on either of USART FIFOs."]
    #[inline(always)]
    pub fn no_impact(self) -> &'a mut W {
        self.variant(TXFIFORES_AW::NO_IMPACT)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\]
will clear all bytes in USART TX FIFO, reset the pointer logic. This bit is self-clearing."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXTL_AW {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    LEVEL0 = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    LEVEL1 = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    LEVEL2 = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    LEVEL3 = 3,
}
impl From<RXTL_AW> for u8 {
    #[inline(always)]
    fn from(variant: RXTL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `RXTL` writer - RX Trigger Level. These two bits determine how many receiver USART FIFO characters must be written before an interrupt is activated."]
pub struct RXTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTL_AW) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register. Controls USART FIFO usage and modes.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcr](index.html) module"]
pub struct FCR_SPEC;
impl crate::RegisterSpec for FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fcr::W](W) writer structure"]
impl crate::Writable for FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
