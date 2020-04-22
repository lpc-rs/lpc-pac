#[doc = "Reader of register TXMODE"]
pub type R = crate::R<u32, super::TXMODE>;
#[doc = "Writer for register TXMODE"]
pub type W = crate::W<u32, super::TXMODE>;
#[doc = "Register TXMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source selection for the transmit bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXCLKSEL_A {
    #[doc = "0: Select the TX fractional rate divider clock output as the source"]
    SELECT_THE_TX_FRACTI = 0,
    #[doc = "2: Select the RX_MCLK signal as the TX_MCLK clock source"]
    SELECT_THE_RX_MCLK_S = 2,
}
impl From<TXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TXCLKSEL`"]
pub type TXCLKSEL_R = crate::R<u8, TXCLKSEL_A>;
impl TXCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXCLKSEL_A::SELECT_THE_TX_FRACTI),
            2 => Val(TXCLKSEL_A::SELECT_THE_RX_MCLK_S),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_FRACTI`"]
    #[inline(always)]
    pub fn is_select_the_tx_fracti(&self) -> bool {
        *self == TXCLKSEL_A::SELECT_THE_TX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_MCLK_S`"]
    #[inline(always)]
    pub fn is_select_the_rx_mclk_s(&self) -> bool {
        *self == TXCLKSEL_A::SELECT_THE_RX_MCLK_S
    }
}
#[doc = "Write proxy for field `TXCLKSEL`"]
pub struct TXCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_tx_fracti(self) -> &'a mut W {
        self.variant(TXCLKSEL_A::SELECT_THE_TX_FRACTI)
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_rx_mclk_s(self) -> &'a mut W {
        self.variant(TXCLKSEL_A::SELECT_THE_RX_MCLK_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TX4PIN`"]
pub type TX4PIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX4PIN`"]
pub struct TX4PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX4PIN_W<'a> {
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
#[doc = "Reader of field `TXMCENA`"]
pub type TXMCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMCENA`"]
pub struct TXMCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCENA_W<'a> {
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
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&self) -> TXCLKSEL_R {
        TXCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&self) -> TX4PIN_R {
        TX4PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&self) -> TXMCENA_R {
        TXMCENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&mut self) -> TXCLKSEL_W {
        TXCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&mut self) -> TX4PIN_W {
        TX4PIN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&mut self) -> TXMCENA_W {
        TXMCENA_W { w: self }
    }
}
