#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `TR`"]
pub enum TRW {
    #[doc = "Absent.No transmission request."]
    ABSENT_NO_TRANSMISSI,
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    PRESENT_THE_MESSAGE,
}
impl TRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRW::ABSENT_NO_TRANSMISSI => false,
            TRW::PRESENT_THE_MESSAGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRW<'a> {
    w: &'a mut W,
}
impl<'a> _TRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Absent.No transmission request."]
    #[inline]
    pub fn absent_no_transmissi(self) -> &'a mut W {
        self.variant(TRW::ABSENT_NO_TRANSMISSI)
    }
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    #[inline]
    pub fn present_the_message(self) -> &'a mut W {
        self.variant(TRW::PRESENT_THE_MESSAGE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `AT`"]
pub enum ATW {
    #[doc = "No action. Do not abort the transmission."]
    NO_ACTION_DO_NOT_AB,
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    PRESENT_IF_NOT_ALRE,
}
impl ATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATW::NO_ACTION_DO_NOT_AB => false,
            ATW::PRESENT_IF_NOT_ALRE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATW<'a> {
    w: &'a mut W,
}
impl<'a> _ATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not abort the transmission."]
    #[inline]
    pub fn no_action_do_not_ab(self) -> &'a mut W {
        self.variant(ATW::NO_ACTION_DO_NOT_AB)
    }
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    #[inline]
    pub fn present_if_not_alre(self) -> &'a mut W {
        self.variant(ATW::PRESENT_IF_NOT_ALRE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RRB`"]
pub enum RRBW {
    #[doc = "No action. Do not release the receive buffer."]
    NO_ACTION_DO_NOT_RE,
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    RELEASED_THE_INFORM,
}
impl RRBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRBW::NO_ACTION_DO_NOT_RE => false,
            RRBW::RELEASED_THE_INFORM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRBW<'a> {
    w: &'a mut W,
}
impl<'a> _RRBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not release the receive buffer."]
    #[inline]
    pub fn no_action_do_not_re(self) -> &'a mut W {
        self.variant(RRBW::NO_ACTION_DO_NOT_RE)
    }
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    #[inline]
    pub fn released_the_inform(self) -> &'a mut W {
        self.variant(RRBW::RELEASED_THE_INFORM)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CDO`"]
pub enum CDOW {
    #[doc = "No action. Do not clear the data overrun bit."]
    NO_ACTION_DO_NOT_CL,
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    CLEAR_THE_DATA_OVER,
}
impl CDOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CDOW::NO_ACTION_DO_NOT_CL => false,
            CDOW::CLEAR_THE_DATA_OVER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CDOW<'a> {
    w: &'a mut W,
}
impl<'a> _CDOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CDOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action. Do not clear the data overrun bit."]
    #[inline]
    pub fn no_action_do_not_cl(self) -> &'a mut W {
        self.variant(CDOW::NO_ACTION_DO_NOT_CL)
    }
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    #[inline]
    pub fn clear_the_data_over(self) -> &'a mut W {
        self.variant(CDOW::CLEAR_THE_DATA_OVER)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SRR`"]
pub enum SRRW {
    #[doc = "Absent. No self reception request."]
    ABSENT_NO_SELF_RECE,
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    PRESENT_THE_MESSAGE,
}
impl SRRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SRRW::ABSENT_NO_SELF_RECE => false,
            SRRW::PRESENT_THE_MESSAGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRRW<'a> {
    w: &'a mut W,
}
impl<'a> _SRRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Absent. No self reception request."]
    #[inline]
    pub fn absent_no_self_rece(self) -> &'a mut W {
        self.variant(SRRW::ABSENT_NO_SELF_RECE)
    }
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    #[inline]
    pub fn present_the_message(self) -> &'a mut W {
        self.variant(SRRW::PRESENT_THE_MESSAGE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB1`"]
pub enum STB1W {
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    NOT_SELECTED_TX_BUF,
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    SELECTED_TX_BUFFER_,
}
impl STB1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB1W::NOT_SELECTED_TX_BUF => false,
            STB1W::SELECTED_TX_BUFFER_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB1W<'a> {
    w: &'a mut W,
}
impl<'a> _STB1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    #[inline]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB1W::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    #[inline]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB1W::SELECTED_TX_BUFFER_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB2`"]
pub enum STB2W {
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    NOT_SELECTED_TX_BUF,
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    SELECTED_TX_BUFFER_,
}
impl STB2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB2W::NOT_SELECTED_TX_BUF => false,
            STB2W::SELECTED_TX_BUFFER_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB2W<'a> {
    w: &'a mut W,
}
impl<'a> _STB2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    #[inline]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB2W::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    #[inline]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB2W::SELECTED_TX_BUFFER_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STB3`"]
pub enum STB3W {
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    NOT_SELECTED_TX_BUF,
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    SELECTED_TX_BUFFER_,
}
impl STB3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STB3W::NOT_SELECTED_TX_BUF => false,
            STB3W::SELECTED_TX_BUFFER_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STB3W<'a> {
    w: &'a mut W,
}
impl<'a> _STB3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STB3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    #[inline]
    pub fn not_selected_tx_buf(self) -> &'a mut W {
        self.variant(STB3W::NOT_SELECTED_TX_BUF)
    }
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    #[inline]
    pub fn selected_tx_buffer_(self) -> &'a mut W {
        self.variant(STB3W::SELECTED_TX_BUFFER_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmission Request."]
    #[inline]
    pub fn tr(&mut self) -> _TRW {
        _TRW { w: self }
    }
    #[doc = "Bit 1 - Abort Transmission."]
    #[inline]
    pub fn at(&mut self) -> _ATW {
        _ATW { w: self }
    }
    #[doc = "Bit 2 - Release Receive Buffer."]
    #[inline]
    pub fn rrb(&mut self) -> _RRBW {
        _RRBW { w: self }
    }
    #[doc = "Bit 3 - Clear Data Overrun."]
    #[inline]
    pub fn cdo(&mut self) -> _CDOW {
        _CDOW { w: self }
    }
    #[doc = "Bit 4 - Self Reception Request."]
    #[inline]
    pub fn srr(&mut self) -> _SRRW {
        _SRRW { w: self }
    }
    #[doc = "Bit 5 - Select Tx Buffer 1."]
    #[inline]
    pub fn stb1(&mut self) -> _STB1W {
        _STB1W { w: self }
    }
    #[doc = "Bit 6 - Select Tx Buffer 2."]
    #[inline]
    pub fn stb2(&mut self) -> _STB2W {
        _STB2W { w: self }
    }
    #[doc = "Bit 7 - Select Tx Buffer 3."]
    #[inline]
    pub fn stb3(&mut self) -> _STB3W {
        _STB3W { w: self }
    }
}
