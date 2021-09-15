#[doc = "Register `MAC_TX_FLOW_CTRL_Q[%s]` reader"]
pub struct R(crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC_TX_FLOW_CTRL_Q_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC_TX_FLOW_CTRL_Q[%s]` writer"]
pub struct W(crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>;
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
impl From<crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC_TX_FLOW_CTRL_Q_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FCB` reader - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
pub struct FCB_R(crate::FieldReader<bool, bool>);
impl FCB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCB` writer - Flow Control Busy/Backpressure Activate This register field can be read by the application (Read), can be set to 1 by the application with a register write of 1 (Write Set), and is cleared to 0 by the core (Self Clear)."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TFE` reader - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` writer - Transmit Flow Control Enable In Full-Duplex mode, when this bit is set, the MAC enables the flow control operation to transmit Pause frames."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PLT` reader - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
pub struct PLT_R(crate::FieldReader<u8, u8>);
impl PLT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLT` writer - Pause Low Threshold This field configures the threshold of the PAUSE timer at which the input flow control signal is checked for automatic retransmission of PAUSE Frame."]
pub struct PLT_W<'a> {
    w: &'a mut W,
}
impl<'a> PLT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DZPQ` reader - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
pub struct DZPQ_R(crate::FieldReader<bool, bool>);
impl DZPQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DZPQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DZPQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DZPQ` writer - Disable Zero-Quanta Pause When set, this bit disables the automatic generation of Zero-Quanta Pause Control frames on the deassertion of the flow-control signal from the FIFO layer."]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PT` reader - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
pub struct PT_R(crate::FieldReader<u16, u16>);
impl PT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PT` writer - Pause time This field holds the value to be used in the Pause Time field in the transmit control frame."]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit flow control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac_tx_flow_ctrl_q](index.html) module"]
pub struct MAC_TX_FLOW_CTRL_Q_SPEC;
impl crate::RegisterSpec for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac_tx_flow_ctrl_q::R](R) reader structure"]
impl crate::Readable for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac_tx_flow_ctrl_q::W](W) writer structure"]
impl crate::Writable for MAC_TX_FLOW_CTRL_Q_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC_TX_FLOW_CTRL_Q[%s]
to value 0"]
impl crate::Resettable for MAC_TX_FLOW_CTRL_Q_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
