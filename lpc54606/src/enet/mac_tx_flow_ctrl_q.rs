#[doc = "Reader of register MAC_TX_FLOW_CTRL_Q[%s]"]
pub type R = crate::R<u32, super::MAC_TX_FLOW_CTRL_Q>;
#[doc = "Writer for register MAC_TX_FLOW_CTRL_Q[%s]"]
pub type W = crate::W<u32, super::MAC_TX_FLOW_CTRL_Q>;
#[doc = "Register MAC_TX_FLOW_CTRL_Q[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_TX_FLOW_CTRL_Q {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FCB`"]
pub type FCB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCB`"]
pub struct FCB_W<'a> {
    w: &'a mut W,
}
impl<'a> FCB_W<'a> {
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
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
#[doc = "Reader of field `PLT`"]
pub type PLT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PLT`"]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `DZPQ`"]
pub type DZPQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DZPQ`"]
pub struct DZPQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DZPQ_W<'a> {
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
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&self) -> FCB_R {
        FCB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&self) -> PLT_R {
        PLT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&self) -> DZPQ_R {
        DZPQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
    #[inline(always)]
    pub fn fcb(&mut self) -> FCB_W {
        FCB_W { w: self }
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
    #[doc = "Bits 4:6 - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
    #[inline(always)]
    pub fn plt(&mut self) -> PLT_W {
        PLT_W { w: self }
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
    #[inline(always)]
    pub fn dzpq(&mut self) -> DZPQ_W {
        DZPQ_W { w: self }
    }
    #[doc = "Bits 16:31 - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
}
