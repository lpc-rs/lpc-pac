#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXIDLEEN`"]
pub type TXIDLEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIDLEEN`"]
pub struct TXIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIDLEEN_W<'a> {
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
#[doc = "Reader of field `DELTACTSEN`"]
pub type DELTACTSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DELTACTSEN`"]
pub struct DELTACTSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTACTSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `TXDISEN`"]
pub type TXDISEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDISEN`"]
pub struct TXDISEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDISEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DELTARXBRKEN`"]
pub type DELTARXBRKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DELTARXBRKEN`"]
pub struct DELTARXBRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELTARXBRKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `STARTEN`"]
pub type STARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STARTEN`"]
pub struct STARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FRAMERREN`"]
pub type FRAMERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMERREN`"]
pub struct FRAMERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `PARITYERREN`"]
pub type PARITYERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PARITYERREN`"]
pub struct PARITYERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYERREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RXNOISEEN`"]
pub type RXNOISEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXNOISEEN`"]
pub struct RXNOISEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXNOISEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `ABERREN`"]
pub type ABERREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ABERREN`"]
pub struct ABERREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABERREN_W<'a> {
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
impl R {
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&self) -> TXIDLEEN_R {
        TXIDLEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&self) -> DELTACTSEN_R {
        DELTACTSEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&self) -> TXDISEN_R {
        TXDISEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DELTARXBRKEN_R {
        DELTARXBRKEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&self) -> STARTEN_R {
        STARTEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&self) -> FRAMERREN_R {
        FRAMERREN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&self) -> PARITYERREN_R {
        PARITYERREN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RXNOISEEN_R {
        RXNOISEEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub fn aberren(&self) -> ABERREN_R {
        ABERREN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&mut self) -> TXIDLEEN_W {
        TXIDLEEN_W { w: self }
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&mut self) -> DELTACTSEN_W {
        DELTACTSEN_W { w: self }
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&mut self) -> TXDISEN_W {
        TXDISEN_W { w: self }
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&mut self) -> DELTARXBRKEN_W {
        DELTARXBRKEN_W { w: self }
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&mut self) -> STARTEN_W {
        STARTEN_W { w: self }
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&mut self) -> FRAMERREN_W {
        FRAMERREN_W { w: self }
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&mut self) -> PARITYERREN_W {
        PARITYERREN_W { w: self }
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected. See description of the RXNOISEINT bit in Table 354."]
    #[inline(always)]
    pub fn rxnoiseen(&mut self) -> RXNOISEEN_W {
        RXNOISEEN_W { w: self }
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto baud error occurs."]
    #[inline(always)]
    pub fn aberren(&mut self) -> ABERREN_W {
        ABERREN_W { w: self }
    }
}
