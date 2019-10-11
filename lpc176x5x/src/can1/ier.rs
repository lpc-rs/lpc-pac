#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RIE`"]
pub type RIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RIE`"]
pub struct RIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIE_W<'a> {
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
#[doc = "Reader of field `TIE1`"]
pub type TIE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE1`"]
pub struct TIE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE1_W<'a> {
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
#[doc = "Reader of field `EIE`"]
pub type EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIE`"]
pub struct EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EIE_W<'a> {
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
#[doc = "Reader of field `DOIE`"]
pub type DOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DOIE`"]
pub struct DOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOIE_W<'a> {
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
#[doc = "Reader of field `WUIE`"]
pub type WUIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUIE`"]
pub struct WUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WUIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `EPIE`"]
pub type EPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EPIE`"]
pub struct EPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIE_W<'a> {
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
#[doc = "Reader of field `ALIE`"]
pub type ALIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALIE`"]
pub struct ALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALIE_W<'a> {
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
#[doc = "Reader of field `BEIE`"]
pub type BEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIE`"]
pub struct BEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IDIE`"]
pub type IDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDIE`"]
pub struct IDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIE2`"]
pub type TIE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE2`"]
pub struct TIE2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TIE3`"]
pub type TIE3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIE3`"]
pub struct TIE3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie1(&self) -> TIE1_R {
        TIE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn doie(&self) -> DOIE_R {
        DOIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    pub fn wuie(&self) -> WUIE_R {
        WUIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn beie(&self) -> BEIE_R {
        BEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie2(&self) -> TIE2_R {
        TIE2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie3(&self) -> TIE3_R {
        TIE3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W {
        RIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie1(&mut self) -> TIE1_W {
        TIE1_W { w: self }
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W { w: self }
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn doie(&mut self) -> DOIE_W {
        DOIE_W { w: self }
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    pub fn wuie(&mut self) -> WUIE_W {
        WUIE_W { w: self }
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    pub fn epie(&mut self) -> EPIE_W {
        EPIE_W { w: self }
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    pub fn alie(&mut self) -> ALIE_W {
        ALIE_W { w: self }
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn beie(&mut self) -> BEIE_W {
        BEIE_W { w: self }
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn idie(&mut self) -> IDIE_W {
        IDIE_W { w: self }
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie2(&mut self) -> TIE2_W {
        TIE2_W { w: self }
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie3(&mut self) -> TIE3_W {
        TIE3_W { w: self }
    }
}
