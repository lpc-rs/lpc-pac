#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The SPI controller sends and receives 8 bits of data per transfer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITENABLE_A {
    #[doc = "1: The SPI controller sends and receives the number of bits selected by bits 11:8."]
    THE_SPI_CONTROLLER_S = 1,
}
impl From<BITENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: BITENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BITENABLE`"]
pub type BITENABLE_R = crate::R<bool, BITENABLE_A>;
impl BITENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, BITENABLE_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(BITENABLE_A::THE_SPI_CONTROLLER_S),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `THE_SPI_CONTROLLER_S`"]
    #[inline(always)]
    pub fn is_the_spi_controller_s(&self) -> bool {
        *self == BITENABLE_A::THE_SPI_CONTROLLER_S
    }
}
#[doc = "Write proxy for field `BITENABLE`"]
pub struct BITENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> BITENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline(always)]
    pub fn the_spi_controller_s(self) -> &'a mut W {
        self.variant(BITENABLE_A::THE_SPI_CONTROLLER_S)
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
#[doc = "Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FIRST_EDGE = 0,
    #[doc = "1: Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SECOND_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRST_EDGE,
            true => CPHA_A::SECOND_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST_EDGE`"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRST_EDGE
    }
    #[doc = "Checks if the value of the field is `SECOND_EDGE`"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECOND_EDGE
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FIRST_EDGE)
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SECOND_EDGE)
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
#[doc = "Clock polarity control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SCK is active high."]
    SCK_IS_ACTIVE_HIGH_ = 0,
    #[doc = "1: SCK is active low."]
    SCK_IS_ACTIVE_LOW_ = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::SCK_IS_ACTIVE_HIGH_,
            true => CPOL_A::SCK_IS_ACTIVE_LOW_,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_IS_ACTIVE_HIGH_`"]
    #[inline(always)]
    pub fn is_sck_is_active_high_(&self) -> bool {
        *self == CPOL_A::SCK_IS_ACTIVE_HIGH_
    }
    #[doc = "Checks if the value of the field is `SCK_IS_ACTIVE_LOW_`"]
    #[inline(always)]
    pub fn is_sck_is_active_low_(&self) -> bool {
        *self == CPOL_A::SCK_IS_ACTIVE_LOW_
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SCK is active high."]
    #[inline(always)]
    pub fn sck_is_active_high_(self) -> &'a mut W {
        self.variant(CPOL_A::SCK_IS_ACTIVE_HIGH_)
    }
    #[doc = "SCK is active low."]
    #[inline(always)]
    pub fn sck_is_active_low_(self) -> &'a mut W {
        self.variant(CPOL_A::SCK_IS_ACTIVE_LOW_)
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
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: The SPI operates in Slave mode."]
    SLAVE = 0,
    #[doc = "1: The SPI operates in Master mode."]
    MASTER = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSTR`"]
pub type MSTR_R = crate::R<bool, MSTR_A>;
impl MSTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::SLAVE,
            true => MSTR_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::MASTER
    }
}
#[doc = "Write proxy for field `MSTR`"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The SPI operates in Slave mode."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::SLAVE)
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::MASTER)
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
#[doc = "LSB First controls which direction each byte is shifted when transferred.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBF_A {
    #[doc = "0: SPI data is transferred MSB (bit 7) first."]
    MSB = 0,
    #[doc = "1: SPI data is transferred LSB (bit 0) first."]
    LSB = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSBF`"]
pub type LSBF_R = crate::R<bool, LSBF_A>;
impl LSBF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::MSB,
            true => LSBF_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == LSBF_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == LSBF_A::LSB
    }
}
#[doc = "Write proxy for field `LSBF`"]
pub struct LSBF_W<'a> {
    w: &'a mut W,
}
impl<'a> LSBF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSBF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(LSBF_A::MSB)
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(LSBF_A::LSB)
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
#[doc = "Serial peripheral interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIE_A {
    #[doc = "0: SPI interrupts are inhibited."]
    INTBLOCK = 0,
    #[doc = "1: A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    HWINT = 1,
}
impl From<SPIE_A> for bool {
    #[inline(always)]
    fn from(variant: SPIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIE`"]
pub type SPIE_R = crate::R<bool, SPIE_A>;
impl SPIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIE_A {
        match self.bits {
            false => SPIE_A::INTBLOCK,
            true => SPIE_A::HWINT,
        }
    }
    #[doc = "Checks if the value of the field is `INTBLOCK`"]
    #[inline(always)]
    pub fn is_intblock(&self) -> bool {
        *self == SPIE_A::INTBLOCK
    }
    #[doc = "Checks if the value of the field is `HWINT`"]
    #[inline(always)]
    pub fn is_hwint(&self) -> bool {
        *self == SPIE_A::HWINT
    }
}
#[doc = "Write proxy for field `SPIE`"]
pub struct SPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SPI interrupts are inhibited."]
    #[inline(always)]
    pub fn intblock(self) -> &'a mut W {
        self.variant(SPIE_A::INTBLOCK)
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline(always)]
    pub fn hwint(self) -> &'a mut W {
        self.variant(SPIE_A::HWINT)
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
#[doc = "When bit 2 of this register is 1, this field controls the number of bits per transfer:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITS_A {
    #[doc = "8: 8 bits per transfer"]
    _8_BITS_PER_TRANSFER = 8,
    #[doc = "9: 9 bits per transfer"]
    _9_BITS_PER_TRANSFER = 9,
    #[doc = "10: 10 bits per transfer"]
    _10_BITS_PER_TRANSFER = 10,
    #[doc = "11: 11 bits per transfer"]
    _11_BITS_PER_TRANSFER = 11,
    #[doc = "12: 12 bits per transfer"]
    _12_BITS_PER_TRANSFER = 12,
    #[doc = "13: 13 bits per transfer"]
    _13_BITS_PER_TRANSFER = 13,
    #[doc = "14: 14 bits per transfer"]
    _14_BITS_PER_TRANSFER = 14,
    #[doc = "15: 15 bits per transfer"]
    _15_BITS_PER_TRANSFER = 15,
    #[doc = "0: 16 bits per transfer"]
    _16_BITS_PER_TRANSFER = 0,
}
impl From<BITS_A> for u8 {
    #[inline(always)]
    fn from(variant: BITS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BITS`"]
pub type BITS_R = crate::R<u8, BITS_A>;
impl BITS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BITS_A> {
        use crate::Variant::*;
        match self.bits {
            8 => Val(BITS_A::_8_BITS_PER_TRANSFER),
            9 => Val(BITS_A::_9_BITS_PER_TRANSFER),
            10 => Val(BITS_A::_10_BITS_PER_TRANSFER),
            11 => Val(BITS_A::_11_BITS_PER_TRANSFER),
            12 => Val(BITS_A::_12_BITS_PER_TRANSFER),
            13 => Val(BITS_A::_13_BITS_PER_TRANSFER),
            14 => Val(BITS_A::_14_BITS_PER_TRANSFER),
            15 => Val(BITS_A::_15_BITS_PER_TRANSFER),
            0 => Val(BITS_A::_16_BITS_PER_TRANSFER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_8_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_8_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_9_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_9_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_10_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_10_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_11_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_11_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_12_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_12_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_13_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_13_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_14_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_14_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_15_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_15_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BITS_PER_TRANSFER`"]
    #[inline(always)]
    pub fn is_16_bits_per_transfer(&self) -> bool {
        *self == BITS_A::_16_BITS_PER_TRANSFER
    }
}
#[doc = "Write proxy for field `BITS`"]
pub struct BITS_W<'a> {
    w: &'a mut W,
}
impl<'a> BITS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bits per transfer"]
    #[inline(always)]
    pub fn _8_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_8_BITS_PER_TRANSFER)
    }
    #[doc = "9 bits per transfer"]
    #[inline(always)]
    pub fn _9_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_9_BITS_PER_TRANSFER)
    }
    #[doc = "10 bits per transfer"]
    #[inline(always)]
    pub fn _10_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_10_BITS_PER_TRANSFER)
    }
    #[doc = "11 bits per transfer"]
    #[inline(always)]
    pub fn _11_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_11_BITS_PER_TRANSFER)
    }
    #[doc = "12 bits per transfer"]
    #[inline(always)]
    pub fn _12_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_12_BITS_PER_TRANSFER)
    }
    #[doc = "13 bits per transfer"]
    #[inline(always)]
    pub fn _13_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_13_BITS_PER_TRANSFER)
    }
    #[doc = "14 bits per transfer"]
    #[inline(always)]
    pub fn _14_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_14_BITS_PER_TRANSFER)
    }
    #[doc = "15 bits per transfer"]
    #[inline(always)]
    pub fn _15_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_15_BITS_PER_TRANSFER)
    }
    #[doc = "16 bits per transfer"]
    #[inline(always)]
    pub fn _16_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITS_A::_16_BITS_PER_TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn bitenable(&self) -> BITENABLE_R {
        BITENABLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&self) -> SPIE_R {
        SPIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline(always)]
    pub fn bitenable(&mut self) -> BITENABLE_W {
        BITENABLE_W { w: self }
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LSBF_W {
        LSBF_W { w: self }
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline(always)]
    pub fn spie(&mut self) -> SPIE_W {
        SPIE_W { w: self }
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline(always)]
    pub fn bits_(&mut self) -> BITS_W {
        BITS_W { w: self }
    }
}
