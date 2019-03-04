#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TXDATCTL {
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
pub struct TXDATR {
    bits: u16,
}
impl TXDATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `TXSSEL0_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL0_NR {
    #[doc = "SSEL0 asserted."]
    TXSSEL0_N_0,
    #[doc = "SSEL0 not asserted."]
    TXSSEL0_N_1,
}
impl TXSSEL0_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXSSEL0_NR::TXSSEL0_N_0 => false,
            TXSSEL0_NR::TXSSEL0_N_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSSEL0_NR {
        match value {
            false => TXSSEL0_NR::TXSSEL0_N_0,
            true => TXSSEL0_NR::TXSSEL0_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXSSEL0_N_0`"]
    #[inline]
    pub fn is_txssel0_n_0(&self) -> bool {
        *self == TXSSEL0_NR::TXSSEL0_N_0
    }
    #[doc = "Checks if the value of the field is `TXSSEL0_N_1`"]
    #[inline]
    pub fn is_txssel0_n_1(&self) -> bool {
        *self == TXSSEL0_NR::TXSSEL0_N_1
    }
}
#[doc = "Possible values of the field `TXSSEL1_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL1_NR {
    #[doc = "SSEL1 asserted."]
    TXSSEL1_N_0,
    #[doc = "SSEL1 not asserted."]
    TXSSEL1_N_1,
}
impl TXSSEL1_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXSSEL1_NR::TXSSEL1_N_0 => false,
            TXSSEL1_NR::TXSSEL1_N_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSSEL1_NR {
        match value {
            false => TXSSEL1_NR::TXSSEL1_N_0,
            true => TXSSEL1_NR::TXSSEL1_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXSSEL1_N_0`"]
    #[inline]
    pub fn is_txssel1_n_0(&self) -> bool {
        *self == TXSSEL1_NR::TXSSEL1_N_0
    }
    #[doc = "Checks if the value of the field is `TXSSEL1_N_1`"]
    #[inline]
    pub fn is_txssel1_n_1(&self) -> bool {
        *self == TXSSEL1_NR::TXSSEL1_N_1
    }
}
#[doc = "Possible values of the field `TXSSEL2_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL2_NR {
    #[doc = "SSEL2 asserted."]
    TXSSEL2_N_0,
    #[doc = "SSEL2 not asserted."]
    TXSSEL2_N_1,
}
impl TXSSEL2_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXSSEL2_NR::TXSSEL2_N_0 => false,
            TXSSEL2_NR::TXSSEL2_N_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSSEL2_NR {
        match value {
            false => TXSSEL2_NR::TXSSEL2_N_0,
            true => TXSSEL2_NR::TXSSEL2_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXSSEL2_N_0`"]
    #[inline]
    pub fn is_txssel2_n_0(&self) -> bool {
        *self == TXSSEL2_NR::TXSSEL2_N_0
    }
    #[doc = "Checks if the value of the field is `TXSSEL2_N_1`"]
    #[inline]
    pub fn is_txssel2_n_1(&self) -> bool {
        *self == TXSSEL2_NR::TXSSEL2_N_1
    }
}
#[doc = "Possible values of the field `TXSSEL3_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSSEL3_NR {
    #[doc = "SSEL3 asserted."]
    TXSSEL3_N_0,
    #[doc = "SSEL3 not asserted."]
    TXSSEL3_N_1,
}
impl TXSSEL3_NR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TXSSEL3_NR::TXSSEL3_N_0 => false,
            TXSSEL3_NR::TXSSEL3_N_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXSSEL3_NR {
        match value {
            false => TXSSEL3_NR::TXSSEL3_N_0,
            true => TXSSEL3_NR::TXSSEL3_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXSSEL3_N_0`"]
    #[inline]
    pub fn is_txssel3_n_0(&self) -> bool {
        *self == TXSSEL3_NR::TXSSEL3_N_0
    }
    #[doc = "Checks if the value of the field is `TXSSEL3_N_1`"]
    #[inline]
    pub fn is_txssel3_n_1(&self) -> bool {
        *self == TXSSEL3_NR::TXSSEL3_N_1
    }
}
#[doc = "Possible values of the field `EOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOTR {
    #[doc = "This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    SSEL_DEASSERTED,
    #[doc = "This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    SSEL_NOT_DEASSERTED,
}
impl EOTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EOTR::SSEL_DEASSERTED => false,
            EOTR::SSEL_NOT_DEASSERTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOTR {
        match value {
            false => EOTR::SSEL_DEASSERTED,
            true => EOTR::SSEL_NOT_DEASSERTED,
        }
    }
    #[doc = "Checks if the value of the field is `SSEL_DEASSERTED`"]
    #[inline]
    pub fn is_ssel_deasserted(&self) -> bool {
        *self == EOTR::SSEL_DEASSERTED
    }
    #[doc = "Checks if the value of the field is `SSEL_NOT_DEASSERTED`"]
    #[inline]
    pub fn is_ssel_not_deasserted(&self) -> bool {
        *self == EOTR::SSEL_NOT_DEASSERTED
    }
}
#[doc = "Possible values of the field `EOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EOFR {
    #[doc = "This piece of data transmitted is not treated as the end of a frame."]
    DATA_NOT_EOF,
    #[doc = "This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    DATA_EOF,
}
impl EOFR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EOFR::DATA_NOT_EOF => false,
            EOFR::DATA_EOF => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EOFR {
        match value {
            false => EOFR::DATA_NOT_EOF,
            true => EOFR::DATA_EOF,
        }
    }
    #[doc = "Checks if the value of the field is `DATA_NOT_EOF`"]
    #[inline]
    pub fn is_data_not_eof(&self) -> bool {
        *self == EOFR::DATA_NOT_EOF
    }
    #[doc = "Checks if the value of the field is `DATA_EOF`"]
    #[inline]
    pub fn is_data_eof(&self) -> bool {
        *self == EOFR::DATA_EOF
    }
}
#[doc = "Possible values of the field `RXIGNORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIGNORER {
    #[doc = "Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ_RECEIVED_DATA,
    #[doc = "Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE_RECEIVED_DATA,
}
impl RXIGNORER {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RXIGNORER::READ_RECEIVED_DATA => false,
            RXIGNORER::IGNORE_RECEIVED_DATA => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIGNORER {
        match value {
            false => RXIGNORER::READ_RECEIVED_DATA,
            true => RXIGNORER::IGNORE_RECEIVED_DATA,
        }
    }
    #[doc = "Checks if the value of the field is `READ_RECEIVED_DATA`"]
    #[inline]
    pub fn is_read_received_data(&self) -> bool {
        *self == RXIGNORER::READ_RECEIVED_DATA
    }
    #[doc = "Checks if the value of the field is `IGNORE_RECEIVED_DATA`"]
    #[inline]
    pub fn is_ignore_received_data(&self) -> bool {
        *self == RXIGNORER::IGNORE_RECEIVED_DATA
    }
}
#[doc = "Possible values of the field `LEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENR {
    #[doc = "no description available"]
    LEN_0,
    #[doc = "Data transfer is 1 bit in length."]
    LEN_1,
    #[doc = "Data transfer is 2 bit in length."]
    LEN_2,
    #[doc = "Data transfer is 3 bit in length."]
    LEN_3,
    #[doc = "Data transfer is 4 bit in length."]
    LEN_4,
    #[doc = "Data transfer is 5 bit in length."]
    LEN_5,
    #[doc = "Data transfer is 6 bit in length."]
    LEN_6,
    #[doc = "Data transfer is 7 bit in length."]
    LEN_7,
    #[doc = "Data transfer is 8 bit in length."]
    LEN_8,
    #[doc = "Data transfer is 9 bit in length."]
    LEN_9,
    #[doc = "Data transfer is 10 bit in length."]
    LEN_10,
    #[doc = "Data transfer is 11 bit in length."]
    LEN_11,
    #[doc = "Data transfer is 12 bit in length."]
    LEN_12,
    #[doc = "Data transfer is 13 bit in length."]
    LEN_13,
    #[doc = "Data transfer is 14 bit in length."]
    LEN_14,
    #[doc = "Data transfer is 15 bit in length."]
    LEN_15,
}
impl LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LENR::LEN_0 => 0,
            LENR::LEN_1 => 1,
            LENR::LEN_2 => 2,
            LENR::LEN_3 => 3,
            LENR::LEN_4 => 4,
            LENR::LEN_5 => 5,
            LENR::LEN_6 => 6,
            LENR::LEN_7 => 7,
            LENR::LEN_8 => 8,
            LENR::LEN_9 => 9,
            LENR::LEN_10 => 10,
            LENR::LEN_11 => 11,
            LENR::LEN_12 => 12,
            LENR::LEN_13 => 13,
            LENR::LEN_14 => 14,
            LENR::LEN_15 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LENR {
        match value {
            0 => LENR::LEN_0,
            1 => LENR::LEN_1,
            2 => LENR::LEN_2,
            3 => LENR::LEN_3,
            4 => LENR::LEN_4,
            5 => LENR::LEN_5,
            6 => LENR::LEN_6,
            7 => LENR::LEN_7,
            8 => LENR::LEN_8,
            9 => LENR::LEN_9,
            10 => LENR::LEN_10,
            11 => LENR::LEN_11,
            12 => LENR::LEN_12,
            13 => LENR::LEN_13,
            14 => LENR::LEN_14,
            15 => LENR::LEN_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEN_0`"]
    #[inline]
    pub fn is_len_0(&self) -> bool {
        *self == LENR::LEN_0
    }
    #[doc = "Checks if the value of the field is `LEN_1`"]
    #[inline]
    pub fn is_len_1(&self) -> bool {
        *self == LENR::LEN_1
    }
    #[doc = "Checks if the value of the field is `LEN_2`"]
    #[inline]
    pub fn is_len_2(&self) -> bool {
        *self == LENR::LEN_2
    }
    #[doc = "Checks if the value of the field is `LEN_3`"]
    #[inline]
    pub fn is_len_3(&self) -> bool {
        *self == LENR::LEN_3
    }
    #[doc = "Checks if the value of the field is `LEN_4`"]
    #[inline]
    pub fn is_len_4(&self) -> bool {
        *self == LENR::LEN_4
    }
    #[doc = "Checks if the value of the field is `LEN_5`"]
    #[inline]
    pub fn is_len_5(&self) -> bool {
        *self == LENR::LEN_5
    }
    #[doc = "Checks if the value of the field is `LEN_6`"]
    #[inline]
    pub fn is_len_6(&self) -> bool {
        *self == LENR::LEN_6
    }
    #[doc = "Checks if the value of the field is `LEN_7`"]
    #[inline]
    pub fn is_len_7(&self) -> bool {
        *self == LENR::LEN_7
    }
    #[doc = "Checks if the value of the field is `LEN_8`"]
    #[inline]
    pub fn is_len_8(&self) -> bool {
        *self == LENR::LEN_8
    }
    #[doc = "Checks if the value of the field is `LEN_9`"]
    #[inline]
    pub fn is_len_9(&self) -> bool {
        *self == LENR::LEN_9
    }
    #[doc = "Checks if the value of the field is `LEN_10`"]
    #[inline]
    pub fn is_len_10(&self) -> bool {
        *self == LENR::LEN_10
    }
    #[doc = "Checks if the value of the field is `LEN_11`"]
    #[inline]
    pub fn is_len_11(&self) -> bool {
        *self == LENR::LEN_11
    }
    #[doc = "Checks if the value of the field is `LEN_12`"]
    #[inline]
    pub fn is_len_12(&self) -> bool {
        *self == LENR::LEN_12
    }
    #[doc = "Checks if the value of the field is `LEN_13`"]
    #[inline]
    pub fn is_len_13(&self) -> bool {
        *self == LENR::LEN_13
    }
    #[doc = "Checks if the value of the field is `LEN_14`"]
    #[inline]
    pub fn is_len_14(&self) -> bool {
        *self == LENR::LEN_14
    }
    #[doc = "Checks if the value of the field is `LEN_15`"]
    #[inline]
    pub fn is_len_15(&self) -> bool {
        *self == LENR::LEN_15
    }
}
#[doc = r" Proxy"]
pub struct _TXDATW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDATW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL0_N`"]
pub enum TXSSEL0_NW {
    #[doc = "SSEL0 asserted."]
    TXSSEL0_N_0,
    #[doc = "SSEL0 not asserted."]
    TXSSEL0_N_1,
}
impl TXSSEL0_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL0_NW::TXSSEL0_N_0 => false,
            TXSSEL0_NW::TXSSEL0_N_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSSEL0_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL0_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSSEL0_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL0 asserted."]
    #[inline]
    pub fn txssel0_n_0(self) -> &'a mut W {
        self.variant(TXSSEL0_NW::TXSSEL0_N_0)
    }
    #[doc = "SSEL0 not asserted."]
    #[inline]
    pub fn txssel0_n_1(self) -> &'a mut W {
        self.variant(TXSSEL0_NW::TXSSEL0_N_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL1_N`"]
pub enum TXSSEL1_NW {
    #[doc = "SSEL1 asserted."]
    TXSSEL1_N_0,
    #[doc = "SSEL1 not asserted."]
    TXSSEL1_N_1,
}
impl TXSSEL1_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL1_NW::TXSSEL1_N_0 => false,
            TXSSEL1_NW::TXSSEL1_N_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSSEL1_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL1_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSSEL1_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL1 asserted."]
    #[inline]
    pub fn txssel1_n_0(self) -> &'a mut W {
        self.variant(TXSSEL1_NW::TXSSEL1_N_0)
    }
    #[doc = "SSEL1 not asserted."]
    #[inline]
    pub fn txssel1_n_1(self) -> &'a mut W {
        self.variant(TXSSEL1_NW::TXSSEL1_N_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL2_N`"]
pub enum TXSSEL2_NW {
    #[doc = "SSEL2 asserted."]
    TXSSEL2_N_0,
    #[doc = "SSEL2 not asserted."]
    TXSSEL2_N_1,
}
impl TXSSEL2_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL2_NW::TXSSEL2_N_0 => false,
            TXSSEL2_NW::TXSSEL2_N_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSSEL2_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL2_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSSEL2_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL2 asserted."]
    #[inline]
    pub fn txssel2_n_0(self) -> &'a mut W {
        self.variant(TXSSEL2_NW::TXSSEL2_N_0)
    }
    #[doc = "SSEL2 not asserted."]
    #[inline]
    pub fn txssel2_n_1(self) -> &'a mut W {
        self.variant(TXSSEL2_NW::TXSSEL2_N_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXSSEL3_N`"]
pub enum TXSSEL3_NW {
    #[doc = "SSEL3 asserted."]
    TXSSEL3_N_0,
    #[doc = "SSEL3 not asserted."]
    TXSSEL3_N_1,
}
impl TXSSEL3_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXSSEL3_NW::TXSSEL3_N_0 => false,
            TXSSEL3_NW::TXSSEL3_N_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXSSEL3_NW<'a> {
    w: &'a mut W,
}
impl<'a> _TXSSEL3_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXSSEL3_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SSEL3 asserted."]
    #[inline]
    pub fn txssel3_n_0(self) -> &'a mut W {
        self.variant(TXSSEL3_NW::TXSSEL3_N_0)
    }
    #[doc = "SSEL3 not asserted."]
    #[inline]
    pub fn txssel3_n_1(self) -> &'a mut W {
        self.variant(TXSSEL3_NW::TXSSEL3_N_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOT`"]
pub enum EOTW {
    #[doc = "This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    SSEL_DEASSERTED,
    #[doc = "This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    SSEL_NOT_DEASSERTED,
}
impl EOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOTW::SSEL_DEASSERTED => false,
            EOTW::SSEL_NOT_DEASSERTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOTW<'a> {
    w: &'a mut W,
}
impl<'a> _EOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This piece of data is not treated as the end of a transfer. SSEL will not be deasserted at the end of this data."]
    #[inline]
    pub fn ssel_deasserted(self) -> &'a mut W {
        self.variant(EOTW::SSEL_DEASSERTED)
    }
    #[doc = "This piece of data is treated as the end of a transfer. SSEL will be deasserted at the end of this piece of data."]
    #[inline]
    pub fn ssel_not_deasserted(self) -> &'a mut W {
        self.variant(EOTW::SSEL_NOT_DEASSERTED)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EOF`"]
pub enum EOFW {
    #[doc = "This piece of data transmitted is not treated as the end of a frame."]
    DATA_NOT_EOF,
    #[doc = "This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    DATA_EOF,
}
impl EOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EOFW::DATA_NOT_EOF => false,
            EOFW::DATA_EOF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EOFW<'a> {
    w: &'a mut W,
}
impl<'a> _EOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This piece of data transmitted is not treated as the end of a frame."]
    #[inline]
    pub fn data_not_eof(self) -> &'a mut W {
        self.variant(EOFW::DATA_NOT_EOF)
    }
    #[doc = "This piece of data is treated as the end of a frame, causing the FRAME_DELAY time to be inserted before subsequent data is transmitted."]
    #[inline]
    pub fn data_eof(self) -> &'a mut W {
        self.variant(EOFW::DATA_EOF)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXIGNORE`"]
pub enum RXIGNOREW {
    #[doc = "Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    READ_RECEIVED_DATA,
    #[doc = "Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    IGNORE_RECEIVED_DATA,
}
impl RXIGNOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIGNOREW::READ_RECEIVED_DATA => false,
            RXIGNOREW::IGNORE_RECEIVED_DATA => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIGNOREW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIGNOREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIGNOREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data must be read in order to allow transmission to progress. In slave mode, an overrun error will occur if received data is not read before new data is received."]
    #[inline]
    pub fn read_received_data(self) -> &'a mut W {
        self.variant(RXIGNOREW::READ_RECEIVED_DATA)
    }
    #[doc = "Received data is ignored, allowing transmission without reading unneeded received data. No receiver flags are generated."]
    #[inline]
    pub fn ignore_received_data(self) -> &'a mut W {
        self.variant(RXIGNOREW::IGNORE_RECEIVED_DATA)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LEN`"]
pub enum LENW {
    #[doc = "no description available"]
    LEN_0,
    #[doc = "Data transfer is 1 bit in length."]
    LEN_1,
    #[doc = "Data transfer is 2 bit in length."]
    LEN_2,
    #[doc = "Data transfer is 3 bit in length."]
    LEN_3,
    #[doc = "Data transfer is 4 bit in length."]
    LEN_4,
    #[doc = "Data transfer is 5 bit in length."]
    LEN_5,
    #[doc = "Data transfer is 6 bit in length."]
    LEN_6,
    #[doc = "Data transfer is 7 bit in length."]
    LEN_7,
    #[doc = "Data transfer is 8 bit in length."]
    LEN_8,
    #[doc = "Data transfer is 9 bit in length."]
    LEN_9,
    #[doc = "Data transfer is 10 bit in length."]
    LEN_10,
    #[doc = "Data transfer is 11 bit in length."]
    LEN_11,
    #[doc = "Data transfer is 12 bit in length."]
    LEN_12,
    #[doc = "Data transfer is 13 bit in length."]
    LEN_13,
    #[doc = "Data transfer is 14 bit in length."]
    LEN_14,
    #[doc = "Data transfer is 15 bit in length."]
    LEN_15,
}
impl LENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LENW::LEN_0 => 0,
            LENW::LEN_1 => 1,
            LENW::LEN_2 => 2,
            LENW::LEN_3 => 3,
            LENW::LEN_4 => 4,
            LENW::LEN_5 => 5,
            LENW::LEN_6 => 6,
            LENW::LEN_7 => 7,
            LENW::LEN_8 => 8,
            LENW::LEN_9 => 9,
            LENW::LEN_10 => 10,
            LENW::LEN_11 => 11,
            LENW::LEN_12 => 12,
            LENW::LEN_13 => 13,
            LENW::LEN_14 => 14,
            LENW::LEN_15 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "no description available"]
    #[inline]
    pub fn len_0(self) -> &'a mut W {
        self.variant(LENW::LEN_0)
    }
    #[doc = "Data transfer is 1 bit in length."]
    #[inline]
    pub fn len_1(self) -> &'a mut W {
        self.variant(LENW::LEN_1)
    }
    #[doc = "Data transfer is 2 bit in length."]
    #[inline]
    pub fn len_2(self) -> &'a mut W {
        self.variant(LENW::LEN_2)
    }
    #[doc = "Data transfer is 3 bit in length."]
    #[inline]
    pub fn len_3(self) -> &'a mut W {
        self.variant(LENW::LEN_3)
    }
    #[doc = "Data transfer is 4 bit in length."]
    #[inline]
    pub fn len_4(self) -> &'a mut W {
        self.variant(LENW::LEN_4)
    }
    #[doc = "Data transfer is 5 bit in length."]
    #[inline]
    pub fn len_5(self) -> &'a mut W {
        self.variant(LENW::LEN_5)
    }
    #[doc = "Data transfer is 6 bit in length."]
    #[inline]
    pub fn len_6(self) -> &'a mut W {
        self.variant(LENW::LEN_6)
    }
    #[doc = "Data transfer is 7 bit in length."]
    #[inline]
    pub fn len_7(self) -> &'a mut W {
        self.variant(LENW::LEN_7)
    }
    #[doc = "Data transfer is 8 bit in length."]
    #[inline]
    pub fn len_8(self) -> &'a mut W {
        self.variant(LENW::LEN_8)
    }
    #[doc = "Data transfer is 9 bit in length."]
    #[inline]
    pub fn len_9(self) -> &'a mut W {
        self.variant(LENW::LEN_9)
    }
    #[doc = "Data transfer is 10 bit in length."]
    #[inline]
    pub fn len_10(self) -> &'a mut W {
        self.variant(LENW::LEN_10)
    }
    #[doc = "Data transfer is 11 bit in length."]
    #[inline]
    pub fn len_11(self) -> &'a mut W {
        self.variant(LENW::LEN_11)
    }
    #[doc = "Data transfer is 12 bit in length."]
    #[inline]
    pub fn len_12(self) -> &'a mut W {
        self.variant(LENW::LEN_12)
    }
    #[doc = "Data transfer is 13 bit in length."]
    #[inline]
    pub fn len_13(self) -> &'a mut W {
        self.variant(LENW::LEN_13)
    }
    #[doc = "Data transfer is 14 bit in length."]
    #[inline]
    pub fn len_14(self) -> &'a mut W {
        self.variant(LENW::LEN_14)
    }
    #[doc = "Data transfer is 15 bit in length."]
    #[inline]
    pub fn len_15(self) -> &'a mut W {
        self.variant(LENW::LEN_15)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline]
    pub fn txdat(&self) -> TXDATR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXDATR { bits }
    }
    #[doc = "Bit 16 - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel0_n(&self) -> TXSSEL0_NR {
        TXSSEL0_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmit Slave Select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL1 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel1_n(&self) -> TXSSEL1_NR {
        TXSSEL1_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Transmit Slave Select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL2 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel2_n(&self) -> TXSSEL2_NR {
        TXSSEL2_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Transmit Slave Select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL3 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel3_n(&self) -> TXSSEL3_NR {
        TXSSEL3_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline]
    pub fn eot(&self) -> EOTR {
        EOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline]
    pub fn eof(&self) -> EOFR {
        EOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline]
    pub fn rxignore(&self) -> RXIGNORER {
        RXIGNORER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
    #[inline]
    pub fn len(&self) -> LENR {
        LENR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:15 - Transmit Data. This field provides from 1 to 16 bits of data to be transmitted."]
    #[inline]
    pub fn txdat(&mut self) -> _TXDATW {
        _TXDATW { w: self }
    }
    #[doc = "Bit 16 - Transmit Slave Select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL0 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel0_n(&mut self) -> _TXSSEL0_NW {
        _TXSSEL0_NW { w: self }
    }
    #[doc = "Bit 17 - Transmit Slave Select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL1 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel1_n(&mut self) -> _TXSSEL1_NW {
        _TXSSEL1_NW { w: self }
    }
    #[doc = "Bit 18 - Transmit Slave Select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL2 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel2_n(&mut self) -> _TXSSEL2_NW {
        _TXSSEL2_NW { w: self }
    }
    #[doc = "Bit 19 - Transmit Slave Select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default. Remark: The active state of the SSEL3 pin is configured by bits in the CFG register."]
    #[inline]
    pub fn txssel3_n(&mut self) -> _TXSSEL3_NW {
        _TXSSEL3_NW { w: self }
    }
    #[doc = "Bit 20 - End of Transfer. The asserted SSEL will be deasserted at the end of a transfer, and remain so for at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline]
    pub fn eot(&mut self) -> _EOTW {
        _EOTW { w: self }
    }
    #[doc = "Bit 21 - End of Frame. Between frames, a delay may be inserted, as defined by the FRAME_DELAY value in the DLY register. The end of a frame may not be particularly meaningful if the FRAME_DELAY value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline]
    pub fn eof(&mut self) -> _EOFW {
        _EOFW { w: self }
    }
    #[doc = "Bit 22 - Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver.Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline]
    pub fn rxignore(&mut self) -> _RXIGNOREW {
        _RXIGNOREW { w: self }
    }
    #[doc = "Bits 24:27 - Data Length. Specifies the data length from 1 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0 = Data transfer is 1 bit in length. 0x1 = Data transfer is 2 bits in length. 0x2 = Data transfer is 3 bits in length. ... 0xF = Data transfer is 16 bits in length."]
    #[inline]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
}
