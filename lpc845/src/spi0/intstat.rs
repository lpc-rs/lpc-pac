#[doc = "Reader of register INTSTAT"]
pub type R = crate::R<u32, super::INTSTAT>;
#[doc = "Writer for register INTSTAT"]
pub type W = crate::W<u32, super::INTSTAT>;
#[doc = "Register INTSTAT `reset()`'s with value 0x02"]
impl crate::ResetValue for super::INTSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOV`"]
pub type RXOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXUR`"]
pub type TXUR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSA`"]
pub type SSA_R = crate::R<bool, bool>;
#[doc = "Reader of field `SSD`"]
pub type SSD_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSTIDLE`"]
pub type MSTIDLE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready flag."]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready flag."]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Overrun interrupt flag."]
    #[inline(always)]
    pub fn rxov(&self) -> RXOV_R {
        RXOV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmitter Underrun interrupt flag."]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Slave Select Assert."]
    #[inline(always)]
    pub fn ssa(&self) -> SSA_R {
        SSA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Deassert."]
    #[inline(always)]
    pub fn ssd(&self) -> SSD_R {
        SSD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Master Idle status flag."]
    #[inline(always)]
    pub fn mstidle(&self) -> MSTIDLE_R {
        MSTIDLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {}
