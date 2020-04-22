#[doc = "Reader of register MAC_LPI_CTRL_STAT"]
pub type R = crate::R<u32, super::MAC_LPI_CTRL_STAT>;
#[doc = "Writer for register MAC_LPI_CTRL_STAT"]
pub type W = crate::W<u32, super::MAC_LPI_CTRL_STAT>;
#[doc = "Register MAC_LPI_CTRL_STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_LPI_CTRL_STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLPIEN`"]
pub type TLPIEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `TLPIEX`"]
pub type TLPIEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIEN`"]
pub type RLPIEN_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIEX`"]
pub type RLPIEX_R = crate::R<bool, bool>;
#[doc = "Reader of field `TLPIST`"]
pub type TLPIST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RLPIST`"]
pub type RLPIST_R = crate::R<bool, bool>;
#[doc = "Reader of field `LPIEN`"]
pub type LPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPIEN`"]
pub struct LPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PLS`"]
pub type PLS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PLS`"]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `LPITXA`"]
pub type LPITXA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPITXA`"]
pub struct LPITXA_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITXA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `LPIATE`"]
pub type LPIATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPIATE`"]
pub struct LPIATE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `LPITCSE`"]
pub type LPITCSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPITCSE`"]
pub struct LPITCSE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPITCSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry When this bit is set, it indicates that the MAC Transmitter has entered the LPI state because of the setting of the LPIEN bit."]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit When this bit is set, it indicates that the MAC transmitter exited the LPI state after the application cleared the LPIEN bit and the LPI TW Timer has expired."]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry When this bit is set, it indicates that the MAC Receiver has received an LPI pattern and entered the LPI state."]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit When this bit is set, it indicates that the MAC Receiver has stopped receiving the LPI pattern on the MII interface, exited the LPI state, and resumed the normal reception."]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State When this bit is set, it indicates that the MAC is transmitting the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State When this bit is set, it indicates that the MAC is receiving the LPI pattern on the MII interface."]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&self) -> LPIATE_R {
        LPIATE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&self) -> LPITCSE_R {
        LPITCSE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPI Enable When this bit is set, it instructs the MAC Transmitter to enter the LPI state."]
    #[inline(always)]
    pub fn lpien(&mut self) -> LPIEN_W {
        LPIEN_W { w: self }
    }
    #[doc = "Bit 17 - PHY Link Status This bit indicates the link status of the PHY."]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 19 - LPI Tx Automate This bit controls the behavior of the MAC when it is entering or coming out of the LPI mode on the Transmit side."]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LPITXA_W {
        LPITXA_W { w: self }
    }
    #[doc = "Bit 20 - LPI Timer Enable This bit controls the automatic entry of the MAC Transmitter into and exit out of the LPI state."]
    #[inline(always)]
    pub fn lpiate(&mut self) -> LPIATE_W {
        LPIATE_W { w: self }
    }
    #[doc = "Bit 21 - LPI Tx Clock Stop Enable When this bit is set, the MAC asserts LPI Tx Clock Gating Control signal high after it enters Tx LPI mode to indicate that the Tx clock to MAC can be stopped."]
    #[inline(always)]
    pub fn lpitcse(&mut self) -> LPITCSE_W {
        LPITCSE_W { w: self }
    }
}
