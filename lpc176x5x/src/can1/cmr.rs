#[doc = "Writer for register CMR"]
pub type W = crate::W<u32, super::CMR>;
#[doc = "Register CMR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Transmission Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TR_AW {
    #[doc = "0: Absent.No transmission request."]
    ABSENT_NO_TRANSMISSI = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    PRESENT_THE_MESSAGE = 1,
}
impl From<TR_AW> for bool {
    #[inline(always)]
    fn from(variant: TR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TR`"]
pub struct TR_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Absent.No transmission request."]
    #[inline(always)]
    pub fn absent_no_transmissi(self) -> &'a mut W {
        self.variant(TR_AW::ABSENT_NO_TRANSMISSI)
    }
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    #[inline(always)]
    pub fn present_the_message(self) -> &'a mut W {
        self.variant(TR_AW::PRESENT_THE_MESSAGE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Abort Transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AT_AW {
    #[doc = "0: No action. Do not abort the transmission."]
    NO_ACTION_DO_NOT_AB = 0,
    #[doc = "1: Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    PRESENT_IF_NOT_ALRE = 1,
}
impl From<AT_AW> for bool {
    #[inline(always)]
    fn from(variant: AT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `AT`"]
pub struct AT_W<'a> {
    w: &'a mut W,
}
impl<'a> AT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action. Do not abort the transmission."]
    #[inline(always)]
    pub fn no_action_do_not_ab(self) -> &'a mut W {
        self.variant(AT_AW::NO_ACTION_DO_NOT_AB)
    }
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    #[inline(always)]
    pub fn present_if_not_alre(self) -> &'a mut W {
        self.variant(AT_AW::PRESENT_IF_NOT_ALRE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Release Receive Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RRB_AW {
    #[doc = "0: No action. Do not release the receive buffer."]
    NO_ACTION_DO_NOT_RE = 0,
    #[doc = "1: Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    RELEASED_THE_INFORM = 1,
}
impl From<RRB_AW> for bool {
    #[inline(always)]
    fn from(variant: RRB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RRB`"]
pub struct RRB_W<'a> {
    w: &'a mut W,
}
impl<'a> RRB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RRB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action. Do not release the receive buffer."]
    #[inline(always)]
    pub fn no_action_do_not_re(self) -> &'a mut W {
        self.variant(RRB_AW::NO_ACTION_DO_NOT_RE)
    }
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    #[inline(always)]
    pub fn released_the_inform(self) -> &'a mut W {
        self.variant(RRB_AW::RELEASED_THE_INFORM)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Clear Data Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CDO_AW {
    #[doc = "0: No action. Do not clear the data overrun bit."]
    NO_ACTION_DO_NOT_CL = 0,
    #[doc = "1: Clear. The Data Overrun bit in Status Register(s) is cleared."]
    CLEAR_THE_DATA_OVER = 1,
}
impl From<CDO_AW> for bool {
    #[inline(always)]
    fn from(variant: CDO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CDO`"]
pub struct CDO_W<'a> {
    w: &'a mut W,
}
impl<'a> CDO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CDO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No action. Do not clear the data overrun bit."]
    #[inline(always)]
    pub fn no_action_do_not_cl(self) -> &'a mut W {
        self.variant(CDO_AW::NO_ACTION_DO_NOT_CL)
    }
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    #[inline(always)]
    pub fn clear_the_data_over(self) -> &'a mut W {
        self.variant(CDO_AW::CLEAR_THE_DATA_OVER)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Self Reception Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRR_AW {
    #[doc = "0: Absent. No self reception request."]
    ABSENT_NO_SELF_RECE = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    PRESENT_THE_MESSAGE = 1,
}
impl From<SRR_AW> for bool {
    #[inline(always)]
    fn from(variant: SRR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SRR`"]
pub struct SRR_W<'a> {
    w: &'a mut W,
}
impl<'a> SRR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Absent. No self reception request."]
    #[inline(always)]
    pub fn absent_no_self_rece(self) -> &'a mut W {
        self.variant(SRR_AW::ABSENT_NO_SELF_RECE)
    }
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    #[inline(always)]
    pub fn present_the_message(self) -> &'a mut W {
        self.variant(SRR_AW::PRESENT_THE_MESSAGE)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Select Tx Buffer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STB1_AW {
    #[doc = "0: Not selected. Tx Buffer 1 is not selected for transmission."]
    NOT_SELECTED_TX_BUF = 0,
    #[doc = "1: Selected. Tx Buffer 1 is selected for transmission."]
    SELECTED_TX_BUFFER_ = 1,
}
impl From<STB1_AW> for bool {
    #[inline(always)]
    fn from(variant: STB1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STB1`"]
pub struct STB1_W<'a> {
    w: &'a mut W,
}
impl<'a> STB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STB1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB1_AW::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB1_AW::SELECTED_TX_BUFFER_)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Select Tx Buffer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STB2_AW {
    #[doc = "0: Not selected. Tx Buffer 2 is not selected for transmission."]
    NOT_SELECTED_TX_BUF = 0,
    #[doc = "1: Selected. Tx Buffer 2 is selected for transmission."]
    SELECTED_TX_BUFFER_ = 1,
}
impl From<STB2_AW> for bool {
    #[inline(always)]
    fn from(variant: STB2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STB2`"]
pub struct STB2_W<'a> {
    w: &'a mut W,
}
impl<'a> STB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STB2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB2_AW::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB2_AW::SELECTED_TX_BUFFER_)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Select Tx Buffer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STB3_AW {
    #[doc = "0: Not selected. Tx Buffer 3 is not selected for transmission."]
    NOT_SELECTED_TX_BUF = 0,
    #[doc = "1: Selected. Tx Buffer 3 is selected for transmission."]
    SELECTED_TX_BUFFER_ = 1,
}
impl From<STB3_AW> for bool {
    #[inline(always)]
    fn from(variant: STB3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STB3`"]
pub struct STB3_W<'a> {
    w: &'a mut W,
}
impl<'a> STB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STB3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB3_AW::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB3_AW::SELECTED_TX_BUFFER_)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Request."]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W { w: self }
    }
    #[doc = "Bit 1 - Abort Transmission."]
    #[inline(always)]
    pub fn at(&mut self) -> AT_W {
        AT_W { w: self }
    }
    #[doc = "Bit 2 - Release Receive Buffer."]
    #[inline(always)]
    pub fn rrb(&mut self) -> RRB_W {
        RRB_W { w: self }
    }
    #[doc = "Bit 3 - Clear Data Overrun."]
    #[inline(always)]
    pub fn cdo(&mut self) -> CDO_W {
        CDO_W { w: self }
    }
    #[doc = "Bit 4 - Self Reception Request."]
    #[inline(always)]
    pub fn srr(&mut self) -> SRR_W {
        SRR_W { w: self }
    }
    #[doc = "Bit 5 - Select Tx Buffer 1."]
    #[inline(always)]
    pub fn stb1(&mut self) -> STB1_W {
        STB1_W { w: self }
    }
    #[doc = "Bit 6 - Select Tx Buffer 2."]
    #[inline(always)]
    pub fn stb2(&mut self) -> STB2_W {
        STB2_W { w: self }
    }
    #[doc = "Bit 7 - Select Tx Buffer 3."]
    #[inline(always)]
    pub fn stb3(&mut self) -> STB3_W {
        STB3_W { w: self }
    }
}
