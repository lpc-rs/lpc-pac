#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INFO {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct FRAME_NRR {
    bits: u16,
}
impl FRAME_NRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `ERR_CODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERR_CODER {
    #[doc = "No error"]
    NO_ERROR,
    #[doc = "PID encoding error"]
    PID_ENCODING_ERROR,
    #[doc = "PID unknown"]
    PID_UNKNOWN,
    #[doc = "Packet unexpected"]
    PACKET_UNEXPECTED,
    #[doc = "Token CRC error"]
    TOKEN_CRC_ERROR,
    #[doc = "Data CRC error"]
    DATA_CRC_ERROR,
    #[doc = "Time out"]
    TIME_OUT,
    #[doc = "Babble"]
    BABBLE,
    #[doc = "Truncated EOP"]
    TRUNCATED_EOP,
    #[doc = "Sent/Received NAK"]
    SENT_RECEIVED_NAK,
    #[doc = "Sent Stall"]
    SENT_STALL,
    #[doc = "Overrun"]
    OVERRUN,
    #[doc = "Sent empty packet"]
    SENT_EMPTY_PACKET,
    #[doc = "Bitstuff error"]
    BITSTUFF_ERROR,
    #[doc = "Sync error"]
    SYNC_ERROR,
    #[doc = "Wrong data toggle"]
    WRONG_DATA_TOGGLE,
}
impl ERR_CODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ERR_CODER::NO_ERROR => 0,
            ERR_CODER::PID_ENCODING_ERROR => 1,
            ERR_CODER::PID_UNKNOWN => 2,
            ERR_CODER::PACKET_UNEXPECTED => 3,
            ERR_CODER::TOKEN_CRC_ERROR => 4,
            ERR_CODER::DATA_CRC_ERROR => 5,
            ERR_CODER::TIME_OUT => 6,
            ERR_CODER::BABBLE => 7,
            ERR_CODER::TRUNCATED_EOP => 8,
            ERR_CODER::SENT_RECEIVED_NAK => 9,
            ERR_CODER::SENT_STALL => 10,
            ERR_CODER::OVERRUN => 11,
            ERR_CODER::SENT_EMPTY_PACKET => 12,
            ERR_CODER::BITSTUFF_ERROR => 13,
            ERR_CODER::SYNC_ERROR => 14,
            ERR_CODER::WRONG_DATA_TOGGLE => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ERR_CODER {
        match value {
            0 => ERR_CODER::NO_ERROR,
            1 => ERR_CODER::PID_ENCODING_ERROR,
            2 => ERR_CODER::PID_UNKNOWN,
            3 => ERR_CODER::PACKET_UNEXPECTED,
            4 => ERR_CODER::TOKEN_CRC_ERROR,
            5 => ERR_CODER::DATA_CRC_ERROR,
            6 => ERR_CODER::TIME_OUT,
            7 => ERR_CODER::BABBLE,
            8 => ERR_CODER::TRUNCATED_EOP,
            9 => ERR_CODER::SENT_RECEIVED_NAK,
            10 => ERR_CODER::SENT_STALL,
            11 => ERR_CODER::OVERRUN,
            12 => ERR_CODER::SENT_EMPTY_PACKET,
            13 => ERR_CODER::BITSTUFF_ERROR,
            14 => ERR_CODER::SYNC_ERROR,
            15 => ERR_CODER::WRONG_DATA_TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_CODER::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `PID_ENCODING_ERROR`"]
    #[inline]
    pub fn is_pid_encoding_error(&self) -> bool {
        *self == ERR_CODER::PID_ENCODING_ERROR
    }
    #[doc = "Checks if the value of the field is `PID_UNKNOWN`"]
    #[inline]
    pub fn is_pid_unknown(&self) -> bool {
        *self == ERR_CODER::PID_UNKNOWN
    }
    #[doc = "Checks if the value of the field is `PACKET_UNEXPECTED`"]
    #[inline]
    pub fn is_packet_unexpected(&self) -> bool {
        *self == ERR_CODER::PACKET_UNEXPECTED
    }
    #[doc = "Checks if the value of the field is `TOKEN_CRC_ERROR`"]
    #[inline]
    pub fn is_token_crc_error(&self) -> bool {
        *self == ERR_CODER::TOKEN_CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `DATA_CRC_ERROR`"]
    #[inline]
    pub fn is_data_crc_error(&self) -> bool {
        *self == ERR_CODER::DATA_CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `TIME_OUT`"]
    #[inline]
    pub fn is_time_out(&self) -> bool {
        *self == ERR_CODER::TIME_OUT
    }
    #[doc = "Checks if the value of the field is `BABBLE`"]
    #[inline]
    pub fn is_babble(&self) -> bool {
        *self == ERR_CODER::BABBLE
    }
    #[doc = "Checks if the value of the field is `TRUNCATED_EOP`"]
    #[inline]
    pub fn is_truncated_eop(&self) -> bool {
        *self == ERR_CODER::TRUNCATED_EOP
    }
    #[doc = "Checks if the value of the field is `SENT_RECEIVED_NAK`"]
    #[inline]
    pub fn is_sent_received_nak(&self) -> bool {
        *self == ERR_CODER::SENT_RECEIVED_NAK
    }
    #[doc = "Checks if the value of the field is `SENT_STALL`"]
    #[inline]
    pub fn is_sent_stall(&self) -> bool {
        *self == ERR_CODER::SENT_STALL
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline]
    pub fn is_overrun(&self) -> bool {
        *self == ERR_CODER::OVERRUN
    }
    #[doc = "Checks if the value of the field is `SENT_EMPTY_PACKET`"]
    #[inline]
    pub fn is_sent_empty_packet(&self) -> bool {
        *self == ERR_CODER::SENT_EMPTY_PACKET
    }
    #[doc = "Checks if the value of the field is `BITSTUFF_ERROR`"]
    #[inline]
    pub fn is_bitstuff_error(&self) -> bool {
        *self == ERR_CODER::BITSTUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `SYNC_ERROR`"]
    #[inline]
    pub fn is_sync_error(&self) -> bool {
        *self == ERR_CODER::SYNC_ERROR
    }
    #[doc = "Checks if the value of the field is `WRONG_DATA_TOGGLE`"]
    #[inline]
    pub fn is_wrong_data_toggle(&self) -> bool {
        *self == ERR_CODER::WRONG_DATA_TOGGLE
    }
}
#[doc = r" Proxy"]
pub struct _FRAME_NRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAME_NRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 2047;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ERR_CODE`"]
pub enum ERR_CODEW {
    #[doc = "No error"]
    NO_ERROR,
    #[doc = "PID encoding error"]
    PID_ENCODING_ERROR,
    #[doc = "PID unknown"]
    PID_UNKNOWN,
    #[doc = "Packet unexpected"]
    PACKET_UNEXPECTED,
    #[doc = "Token CRC error"]
    TOKEN_CRC_ERROR,
    #[doc = "Data CRC error"]
    DATA_CRC_ERROR,
    #[doc = "Time out"]
    TIME_OUT,
    #[doc = "Babble"]
    BABBLE,
    #[doc = "Truncated EOP"]
    TRUNCATED_EOP,
    #[doc = "Sent/Received NAK"]
    SENT_RECEIVED_NAK,
    #[doc = "Sent Stall"]
    SENT_STALL,
    #[doc = "Overrun"]
    OVERRUN,
    #[doc = "Sent empty packet"]
    SENT_EMPTY_PACKET,
    #[doc = "Bitstuff error"]
    BITSTUFF_ERROR,
    #[doc = "Sync error"]
    SYNC_ERROR,
    #[doc = "Wrong data toggle"]
    WRONG_DATA_TOGGLE,
}
impl ERR_CODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ERR_CODEW::NO_ERROR => 0,
            ERR_CODEW::PID_ENCODING_ERROR => 1,
            ERR_CODEW::PID_UNKNOWN => 2,
            ERR_CODEW::PACKET_UNEXPECTED => 3,
            ERR_CODEW::TOKEN_CRC_ERROR => 4,
            ERR_CODEW::DATA_CRC_ERROR => 5,
            ERR_CODEW::TIME_OUT => 6,
            ERR_CODEW::BABBLE => 7,
            ERR_CODEW::TRUNCATED_EOP => 8,
            ERR_CODEW::SENT_RECEIVED_NAK => 9,
            ERR_CODEW::SENT_STALL => 10,
            ERR_CODEW::OVERRUN => 11,
            ERR_CODEW::SENT_EMPTY_PACKET => 12,
            ERR_CODEW::BITSTUFF_ERROR => 13,
            ERR_CODEW::SYNC_ERROR => 14,
            ERR_CODEW::WRONG_DATA_TOGGLE => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERR_CODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ERR_CODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERR_CODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No error"]
    #[inline]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::NO_ERROR)
    }
    #[doc = "PID encoding error"]
    #[inline]
    pub fn pid_encoding_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::PID_ENCODING_ERROR)
    }
    #[doc = "PID unknown"]
    #[inline]
    pub fn pid_unknown(self) -> &'a mut W {
        self.variant(ERR_CODEW::PID_UNKNOWN)
    }
    #[doc = "Packet unexpected"]
    #[inline]
    pub fn packet_unexpected(self) -> &'a mut W {
        self.variant(ERR_CODEW::PACKET_UNEXPECTED)
    }
    #[doc = "Token CRC error"]
    #[inline]
    pub fn token_crc_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::TOKEN_CRC_ERROR)
    }
    #[doc = "Data CRC error"]
    #[inline]
    pub fn data_crc_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::DATA_CRC_ERROR)
    }
    #[doc = "Time out"]
    #[inline]
    pub fn time_out(self) -> &'a mut W {
        self.variant(ERR_CODEW::TIME_OUT)
    }
    #[doc = "Babble"]
    #[inline]
    pub fn babble(self) -> &'a mut W {
        self.variant(ERR_CODEW::BABBLE)
    }
    #[doc = "Truncated EOP"]
    #[inline]
    pub fn truncated_eop(self) -> &'a mut W {
        self.variant(ERR_CODEW::TRUNCATED_EOP)
    }
    #[doc = "Sent/Received NAK"]
    #[inline]
    pub fn sent_received_nak(self) -> &'a mut W {
        self.variant(ERR_CODEW::SENT_RECEIVED_NAK)
    }
    #[doc = "Sent Stall"]
    #[inline]
    pub fn sent_stall(self) -> &'a mut W {
        self.variant(ERR_CODEW::SENT_STALL)
    }
    #[doc = "Overrun"]
    #[inline]
    pub fn overrun(self) -> &'a mut W {
        self.variant(ERR_CODEW::OVERRUN)
    }
    #[doc = "Sent empty packet"]
    #[inline]
    pub fn sent_empty_packet(self) -> &'a mut W {
        self.variant(ERR_CODEW::SENT_EMPTY_PACKET)
    }
    #[doc = "Bitstuff error"]
    #[inline]
    pub fn bitstuff_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::BITSTUFF_ERROR)
    }
    #[doc = "Sync error"]
    #[inline]
    pub fn sync_error(self) -> &'a mut W {
        self.variant(ERR_CODEW::SYNC_ERROR)
    }
    #[doc = "Wrong data toggle"]
    #[inline]
    pub fn wrong_data_toggle(self) -> &'a mut W {
        self.variant(ERR_CODEW::WRONG_DATA_TOGGLE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:10 - Frame number. This contains the frame number of the last successfully received SOF. In case no SOF was received by the device at the beginning of a frame, the frame number returned is that of the last successfully received SOF. In case the SOF frame number contained a CRC error, the frame number returned will be the corrupted frame number as received by the device."]
    #[inline]
    pub fn frame_nr(&self) -> FRAME_NRR {
        let bits = {
            const MASK: u16 = 2047;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRAME_NRR { bits }
    }
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline]
    pub fn err_code(&self) -> ERR_CODER {
        ERR_CODER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:10 - Frame number. This contains the frame number of the last successfully received SOF. In case no SOF was received by the device at the beginning of a frame, the frame number returned is that of the last successfully received SOF. In case the SOF frame number contained a CRC error, the frame number returned will be the corrupted frame number as received by the device."]
    #[inline]
    pub fn frame_nr(&mut self) -> _FRAME_NRW {
        _FRAME_NRW { w: self }
    }
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline]
    pub fn err_code(&mut self) -> _ERR_CODEW {
        _ERR_CODEW { w: self }
    }
}
