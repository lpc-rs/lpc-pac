#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0x0c"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0c
    }
}
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXIDLE`"]
pub type TXIDLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DELTACTS`"]
pub type DELTACTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXDISINT`"]
pub type TXDISINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `OVERRUNINT`"]
pub type OVERRUNINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DELTARXBRK`"]
pub type DELTARXBRK_R = crate::R<bool, bool>;
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRAMERRINT`"]
pub type FRAMERRINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `PARITYERRINT`"]
pub type PARITYERRINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXNOISEINT`"]
pub type RXNOISEINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `ABERR`"]
pub type ABERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready flag."]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitter Ready flag."]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter idle status."]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a change in the state of the CTS input is detected."]
    #[inline(always)]
    pub fn deltacts(&self) -> DELTACTS_R {
        DELTACTS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmitter Disabled Interrupt flag."]
    #[inline(always)]
    pub fn txdisint(&self) -> TXDISINT_R {
        TXDISINT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Overrun Error interrupt flag."]
    #[inline(always)]
    pub fn overrunint(&self) -> OVERRUNINT_R {
        OVERRUNINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - This bit is set when a change in the state of receiver break detection occurs."]
    #[inline(always)]
    pub fn deltarxbrk(&self) -> DELTARXBRK_R {
        DELTARXBRK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - This bit is set when a start is detected on the receiver input."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Framing Error interrupt flag."]
    #[inline(always)]
    pub fn framerrint(&self) -> FRAMERRINT_R {
        FRAMERRINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Parity Error interrupt flag."]
    #[inline(always)]
    pub fn parityerrint(&self) -> PARITYERRINT_R {
        PARITYERRINT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Received Noise interrupt flag."]
    #[inline(always)]
    pub fn rxnoiseint(&self) -> RXNOISEINT_R {
        RXNOISEINT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Autobaud Error flag."]
    #[inline(always)]
    pub fn aberr(&self) -> ABERR_R {
        ABERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {}
