#[doc = "Reader of register INFO"]
pub type R = crate::R<u32, super::INFO>;
#[doc = "Writer for register INFO"]
pub type W = crate::W<u32, super::INFO>;
#[doc = "Register INFO `reset()`'s with value 0"]
impl crate::ResetValue for super::INFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAME_NR`"]
pub type FRAME_NR_R = crate::R<u16, u16>;
#[doc = "The error code which last occurred:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERR_CODE_A {
    #[doc = "0: No error"]
    NO_ERROR = 0,
    #[doc = "1: PID encoding error"]
    PID_ENCODING_ERROR = 1,
    #[doc = "2: PID unknown"]
    PID_UNKNOWN = 2,
    #[doc = "3: Packet unexpected"]
    PACKET_UNEXPECTED = 3,
    #[doc = "4: Token CRC error"]
    TOKEN_CRC_ERROR = 4,
    #[doc = "5: Data CRC error"]
    DATA_CRC_ERROR = 5,
    #[doc = "6: Time out"]
    TIMEOUT = 6,
    #[doc = "7: Babble"]
    BABBLE = 7,
    #[doc = "8: Truncated EOP"]
    TRUNCATED_EOP = 8,
    #[doc = "9: Sent/Received NAK"]
    SENT_RECEIVED_NAK = 9,
    #[doc = "10: Sent Stall"]
    SENT_STALL = 10,
    #[doc = "11: Overrun"]
    OVERRUN = 11,
    #[doc = "12: Sent empty packet"]
    SENT_EMPTY_PACKET = 12,
    #[doc = "13: Bitstuff error"]
    BITSTUFF_ERROR = 13,
    #[doc = "14: Sync error"]
    SYNC_ERROR = 14,
    #[doc = "15: Wrong data toggle"]
    WRONG_DATA_TOGGLE = 15,
}
impl From<ERR_CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_CODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERR_CODE`"]
pub type ERR_CODE_R = crate::R<u8, ERR_CODE_A>;
impl ERR_CODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERR_CODE_A {
        match self.bits {
            0 => ERR_CODE_A::NO_ERROR,
            1 => ERR_CODE_A::PID_ENCODING_ERROR,
            2 => ERR_CODE_A::PID_UNKNOWN,
            3 => ERR_CODE_A::PACKET_UNEXPECTED,
            4 => ERR_CODE_A::TOKEN_CRC_ERROR,
            5 => ERR_CODE_A::DATA_CRC_ERROR,
            6 => ERR_CODE_A::TIMEOUT,
            7 => ERR_CODE_A::BABBLE,
            8 => ERR_CODE_A::TRUNCATED_EOP,
            9 => ERR_CODE_A::SENT_RECEIVED_NAK,
            10 => ERR_CODE_A::SENT_STALL,
            11 => ERR_CODE_A::OVERRUN,
            12 => ERR_CODE_A::SENT_EMPTY_PACKET,
            13 => ERR_CODE_A::BITSTUFF_ERROR,
            14 => ERR_CODE_A::SYNC_ERROR,
            15 => ERR_CODE_A::WRONG_DATA_TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERR_CODE_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `PID_ENCODING_ERROR`"]
    #[inline(always)]
    pub fn is_pid_encoding_error(&self) -> bool {
        *self == ERR_CODE_A::PID_ENCODING_ERROR
    }
    #[doc = "Checks if the value of the field is `PID_UNKNOWN`"]
    #[inline(always)]
    pub fn is_pid_unknown(&self) -> bool {
        *self == ERR_CODE_A::PID_UNKNOWN
    }
    #[doc = "Checks if the value of the field is `PACKET_UNEXPECTED`"]
    #[inline(always)]
    pub fn is_packet_unexpected(&self) -> bool {
        *self == ERR_CODE_A::PACKET_UNEXPECTED
    }
    #[doc = "Checks if the value of the field is `TOKEN_CRC_ERROR`"]
    #[inline(always)]
    pub fn is_token_crc_error(&self) -> bool {
        *self == ERR_CODE_A::TOKEN_CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `DATA_CRC_ERROR`"]
    #[inline(always)]
    pub fn is_data_crc_error(&self) -> bool {
        *self == ERR_CODE_A::DATA_CRC_ERROR
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == ERR_CODE_A::TIMEOUT
    }
    #[doc = "Checks if the value of the field is `BABBLE`"]
    #[inline(always)]
    pub fn is_babble(&self) -> bool {
        *self == ERR_CODE_A::BABBLE
    }
    #[doc = "Checks if the value of the field is `TRUNCATED_EOP`"]
    #[inline(always)]
    pub fn is_truncated_eop(&self) -> bool {
        *self == ERR_CODE_A::TRUNCATED_EOP
    }
    #[doc = "Checks if the value of the field is `SENT_RECEIVED_NAK`"]
    #[inline(always)]
    pub fn is_sent_received_nak(&self) -> bool {
        *self == ERR_CODE_A::SENT_RECEIVED_NAK
    }
    #[doc = "Checks if the value of the field is `SENT_STALL`"]
    #[inline(always)]
    pub fn is_sent_stall(&self) -> bool {
        *self == ERR_CODE_A::SENT_STALL
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == ERR_CODE_A::OVERRUN
    }
    #[doc = "Checks if the value of the field is `SENT_EMPTY_PACKET`"]
    #[inline(always)]
    pub fn is_sent_empty_packet(&self) -> bool {
        *self == ERR_CODE_A::SENT_EMPTY_PACKET
    }
    #[doc = "Checks if the value of the field is `BITSTUFF_ERROR`"]
    #[inline(always)]
    pub fn is_bitstuff_error(&self) -> bool {
        *self == ERR_CODE_A::BITSTUFF_ERROR
    }
    #[doc = "Checks if the value of the field is `SYNC_ERROR`"]
    #[inline(always)]
    pub fn is_sync_error(&self) -> bool {
        *self == ERR_CODE_A::SYNC_ERROR
    }
    #[doc = "Checks if the value of the field is `WRONG_DATA_TOGGLE`"]
    #[inline(always)]
    pub fn is_wrong_data_toggle(&self) -> bool {
        *self == ERR_CODE_A::WRONG_DATA_TOGGLE
    }
}
#[doc = "Write proxy for field `ERR_CODE`"]
pub struct ERR_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_CODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_CODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::NO_ERROR)
    }
    #[doc = "PID encoding error"]
    #[inline(always)]
    pub fn pid_encoding_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::PID_ENCODING_ERROR)
    }
    #[doc = "PID unknown"]
    #[inline(always)]
    pub fn pid_unknown(self) -> &'a mut W {
        self.variant(ERR_CODE_A::PID_UNKNOWN)
    }
    #[doc = "Packet unexpected"]
    #[inline(always)]
    pub fn packet_unexpected(self) -> &'a mut W {
        self.variant(ERR_CODE_A::PACKET_UNEXPECTED)
    }
    #[doc = "Token CRC error"]
    #[inline(always)]
    pub fn token_crc_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::TOKEN_CRC_ERROR)
    }
    #[doc = "Data CRC error"]
    #[inline(always)]
    pub fn data_crc_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::DATA_CRC_ERROR)
    }
    #[doc = "Time out"]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(ERR_CODE_A::TIMEOUT)
    }
    #[doc = "Babble"]
    #[inline(always)]
    pub fn babble(self) -> &'a mut W {
        self.variant(ERR_CODE_A::BABBLE)
    }
    #[doc = "Truncated EOP"]
    #[inline(always)]
    pub fn truncated_eop(self) -> &'a mut W {
        self.variant(ERR_CODE_A::TRUNCATED_EOP)
    }
    #[doc = "Sent/Received NAK"]
    #[inline(always)]
    pub fn sent_received_nak(self) -> &'a mut W {
        self.variant(ERR_CODE_A::SENT_RECEIVED_NAK)
    }
    #[doc = "Sent Stall"]
    #[inline(always)]
    pub fn sent_stall(self) -> &'a mut W {
        self.variant(ERR_CODE_A::SENT_STALL)
    }
    #[doc = "Overrun"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(ERR_CODE_A::OVERRUN)
    }
    #[doc = "Sent empty packet"]
    #[inline(always)]
    pub fn sent_empty_packet(self) -> &'a mut W {
        self.variant(ERR_CODE_A::SENT_EMPTY_PACKET)
    }
    #[doc = "Bitstuff error"]
    #[inline(always)]
    pub fn bitstuff_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::BITSTUFF_ERROR)
    }
    #[doc = "Sync error"]
    #[inline(always)]
    pub fn sync_error(self) -> &'a mut W {
        self.variant(ERR_CODE_A::SYNC_ERROR)
    }
    #[doc = "Wrong data toggle"]
    #[inline(always)]
    pub fn wrong_data_toggle(self) -> &'a mut W {
        self.variant(ERR_CODE_A::WRONG_DATA_TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 11)) | (((value as u32) & 0x0f) << 11);
        self.w
    }
}
#[doc = "Reader of field `MINREV`"]
pub type MINREV_R = crate::R<u8, u8>;
#[doc = "Reader of field `MAJREV`"]
pub type MAJREV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - Frame number. This contains the frame number of the last successfully received SOF. In case no SOF was received by the device at the beginning of a frame, the frame number returned is that of the last successfully received SOF. In case the SOF frame number contained a CRC error, the frame number returned will be the corrupted frame number as received by the device."]
    #[inline(always)]
    pub fn frame_nr(&self) -> FRAME_NR_R {
        FRAME_NR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline(always)]
    pub fn err_code(&self) -> ERR_CODE_R {
        ERR_CODE_R::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Minor Revision."]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Revision."]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 11:14 - The error code which last occurred:"]
    #[inline(always)]
    pub fn err_code(&mut self) -> ERR_CODE_W {
        ERR_CODE_W { w: self }
    }
}
