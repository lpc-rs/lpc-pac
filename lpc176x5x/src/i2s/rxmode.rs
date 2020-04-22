#[doc = "Reader of register RXMODE"]
pub type R = crate::R<u32, super::RXMODE>;
#[doc = "Writer for register RXMODE"]
pub type W = crate::W<u32, super::RXMODE>;
#[doc = "Register RXMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::RXMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source selection for the receive bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXCLKSEL_A {
    #[doc = "0: Select the RX fractional rate divider clock output as the source"]
    SELECT_THE_RX_FRACTI = 0,
    #[doc = "2: Select the TX_MCLK signal as the RX_MCLK clock source"]
    SELECT_THE_TX_MCLK_S = 2,
}
impl From<RXCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXCLKSEL`"]
pub type RXCLKSEL_R = crate::R<u8, RXCLKSEL_A>;
impl RXCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXCLKSEL_A::SELECT_THE_RX_FRACTI),
            2 => Val(RXCLKSEL_A::SELECT_THE_TX_MCLK_S),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_RX_FRACTI`"]
    #[inline(always)]
    pub fn is_select_the_rx_fracti(&self) -> bool {
        *self == RXCLKSEL_A::SELECT_THE_RX_FRACTI
    }
    #[doc = "Checks if the value of the field is `SELECT_THE_TX_MCLK_S`"]
    #[inline(always)]
    pub fn is_select_the_tx_mclk_s(&self) -> bool {
        *self == RXCLKSEL_A::SELECT_THE_TX_MCLK_S
    }
}
#[doc = "Write proxy for field `RXCLKSEL`"]
pub struct RXCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_rx_fracti(self) -> &'a mut W {
        self.variant(RXCLKSEL_A::SELECT_THE_RX_FRACTI)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_tx_mclk_s(self) -> &'a mut W {
        self.variant(RXCLKSEL_A::SELECT_THE_TX_MCLK_S)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RX4PIN`"]
pub type RX4PIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX4PIN`"]
pub struct RX4PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX4PIN_W<'a> {
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
#[doc = "Reader of field `RXMCENA`"]
pub type RXMCENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMCENA`"]
pub struct RXMCENA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMCENA_W<'a> {
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
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&self) -> RXCLKSEL_R {
        RXCLKSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&self) -> RX4PIN_R {
        RX4PIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&self) -> RXMCENA_R {
        RXMCENA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&mut self) -> RXCLKSEL_W {
        RXCLKSEL_W { w: self }
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&mut self) -> RX4PIN_W {
        RX4PIN_W { w: self }
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&mut self) -> RXMCENA_W {
        RXMCENA_W { w: self }
    }
}
