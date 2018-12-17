#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `BITENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITENABLER {
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    THE_SPI_CONTROLLER_S,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl BITENABLER {
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
            BITENABLER::THE_SPI_CONTROLLER_S => true,
            BITENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BITENABLER {
        match value {
            true => BITENABLER::THE_SPI_CONTROLLER_S,
            i => BITENABLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `THE_SPI_CONTROLLER_S`"]
    #[inline]
    pub fn is_the_spi_controller_s(&self) -> bool {
        *self == BITENABLER::THE_SPI_CONTROLLER_S
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FIRST_EDGE,
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SECOND_EDGE,
}
impl CPHAR {
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
            CPHAR::FIRST_EDGE => false,
            CPHAR::SECOND_EDGE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::FIRST_EDGE,
            true => CPHAR::SECOND_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST_EDGE`"]
    #[inline]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHAR::FIRST_EDGE
    }
    #[doc = "Checks if the value of the field is `SECOND_EDGE`"]
    #[inline]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHAR::SECOND_EDGE
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "SCK is active high."]
    SCK_IS_ACTIVE_HIGH_,
    #[doc = "SCK is active low."]
    SCK_IS_ACTIVE_LOW_,
}
impl CPOLR {
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
            CPOLR::SCK_IS_ACTIVE_HIGH_ => false,
            CPOLR::SCK_IS_ACTIVE_LOW_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::SCK_IS_ACTIVE_HIGH_,
            true => CPOLR::SCK_IS_ACTIVE_LOW_,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_IS_ACTIVE_HIGH_`"]
    #[inline]
    pub fn is_sck_is_active_high_(&self) -> bool {
        *self == CPOLR::SCK_IS_ACTIVE_HIGH_
    }
    #[doc = "Checks if the value of the field is `SCK_IS_ACTIVE_LOW_`"]
    #[inline]
    pub fn is_sck_is_active_low_(&self) -> bool {
        *self == CPOLR::SCK_IS_ACTIVE_LOW_
    }
}
#[doc = "Possible values of the field `MSTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTRR {
    #[doc = "The SPI operates in Slave mode."]
    SLAVE,
    #[doc = "The SPI operates in Master mode."]
    MASTER,
}
impl MSTRR {
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
            MSTRR::SLAVE => false,
            MSTRR::MASTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSTRR {
        match value {
            false => MSTRR::SLAVE,
            true => MSTRR::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline]
    pub fn is_slave(&self) -> bool {
        *self == MSTRR::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline]
    pub fn is_master(&self) -> bool {
        *self == MSTRR::MASTER
    }
}
#[doc = "Possible values of the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFR {
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    MSB,
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    LSB,
}
impl LSBFR {
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
            LSBFR::MSB => false,
            LSBFR::LSB => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFR {
        match value {
            false => LSBFR::MSB,
            true => LSBFR::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline]
    pub fn is_msb(&self) -> bool {
        *self == LSBFR::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline]
    pub fn is_lsb(&self) -> bool {
        *self == LSBFR::LSB
    }
}
#[doc = "Possible values of the field `SPIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIER {
    #[doc = "SPI interrupts are inhibited."]
    INTBLOCK,
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    HWINT,
}
impl SPIER {
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
            SPIER::INTBLOCK => false,
            SPIER::HWINT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPIER {
        match value {
            false => SPIER::INTBLOCK,
            true => SPIER::HWINT,
        }
    }
    #[doc = "Checks if the value of the field is `INTBLOCK`"]
    #[inline]
    pub fn is_intblock(&self) -> bool {
        *self == SPIER::INTBLOCK
    }
    #[doc = "Checks if the value of the field is `HWINT`"]
    #[inline]
    pub fn is_hwint(&self) -> bool {
        *self == SPIER::HWINT
    }
}
#[doc = "Possible values of the field `BITS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BITSR {
    #[doc = "8 bits per transfer"]
    _8_BITS_PER_TRANSFER,
    #[doc = "9 bits per transfer"]
    _9_BITS_PER_TRANSFER,
    #[doc = "10 bits per transfer"]
    _10_BITS_PER_TRANSFER,
    #[doc = "11 bits per transfer"]
    _11_BITS_PER_TRANSFER,
    #[doc = "12 bits per transfer"]
    _12_BITS_PER_TRANSFER,
    #[doc = "13 bits per transfer"]
    _13_BITS_PER_TRANSFER,
    #[doc = "14 bits per transfer"]
    _14_BITS_PER_TRANSFER,
    #[doc = "15 bits per transfer"]
    _15_BITS_PER_TRANSFER,
    #[doc = "16 bits per transfer"]
    _16_BITS_PER_TRANSFER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BITSR::_8_BITS_PER_TRANSFER => 8,
            BITSR::_9_BITS_PER_TRANSFER => 9,
            BITSR::_10_BITS_PER_TRANSFER => 10,
            BITSR::_11_BITS_PER_TRANSFER => 11,
            BITSR::_12_BITS_PER_TRANSFER => 12,
            BITSR::_13_BITS_PER_TRANSFER => 13,
            BITSR::_14_BITS_PER_TRANSFER => 14,
            BITSR::_15_BITS_PER_TRANSFER => 15,
            BITSR::_16_BITS_PER_TRANSFER => 0,
            BITSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BITSR {
        match value {
            8 => BITSR::_8_BITS_PER_TRANSFER,
            9 => BITSR::_9_BITS_PER_TRANSFER,
            10 => BITSR::_10_BITS_PER_TRANSFER,
            11 => BITSR::_11_BITS_PER_TRANSFER,
            12 => BITSR::_12_BITS_PER_TRANSFER,
            13 => BITSR::_13_BITS_PER_TRANSFER,
            14 => BITSR::_14_BITS_PER_TRANSFER,
            15 => BITSR::_15_BITS_PER_TRANSFER,
            0 => BITSR::_16_BITS_PER_TRANSFER,
            i => BITSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_8_bits_per_transfer(&self) -> bool {
        *self == BITSR::_8_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_9_bits_per_transfer(&self) -> bool {
        *self == BITSR::_9_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_10_bits_per_transfer(&self) -> bool {
        *self == BITSR::_10_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_11_bits_per_transfer(&self) -> bool {
        *self == BITSR::_11_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_12_bits_per_transfer(&self) -> bool {
        *self == BITSR::_12_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_13_bits_per_transfer(&self) -> bool {
        *self == BITSR::_13_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_14_bits_per_transfer(&self) -> bool {
        *self == BITSR::_14_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_15_bits_per_transfer(&self) -> bool {
        *self == BITSR::_15_BITS_PER_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BITS_PER_TRANSFER`"]
    #[inline]
    pub fn is_16_bits_per_transfer(&self) -> bool {
        *self == BITSR::_16_BITS_PER_TRANSFER
    }
}
#[doc = "Values that can be written to the field `BITENABLE`"]
pub enum BITENABLEW {
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    THE_SPI_CONTROLLER_S,
}
impl BITENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BITENABLEW::THE_SPI_CONTROLLER_S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _BITENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI controller sends and receives the number of bits selected by bits 11:8."]
    #[inline]
    pub fn the_spi_controller_s(self) -> &'a mut W {
        self.variant(BITENABLEW::THE_SPI_CONTROLLER_S)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    FIRST_EDGE,
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    SECOND_EDGE,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRST_EDGE => false,
            CPHAW::SECOND_EDGE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is sampled on the first clock edge of SCK. A transfer starts and ends with activation and deactivation of the SSEL signal."]
    #[inline]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHAW::FIRST_EDGE)
    }
    #[doc = "Data is sampled on the second clock edge of the SCK. A transfer starts with the first clock edge, and ends with the last sampling edge when the SSEL signal is active."]
    #[inline]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHAW::SECOND_EDGE)
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
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "SCK is active high."]
    SCK_IS_ACTIVE_HIGH_,
    #[doc = "SCK is active low."]
    SCK_IS_ACTIVE_LOW_,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::SCK_IS_ACTIVE_HIGH_ => false,
            CPOLW::SCK_IS_ACTIVE_LOW_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SCK is active high."]
    #[inline]
    pub fn sck_is_active_high_(self) -> &'a mut W {
        self.variant(CPOLW::SCK_IS_ACTIVE_HIGH_)
    }
    #[doc = "SCK is active low."]
    #[inline]
    pub fn sck_is_active_low_(self) -> &'a mut W {
        self.variant(CPOLW::SCK_IS_ACTIVE_LOW_)
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
#[doc = "Values that can be written to the field `MSTR`"]
pub enum MSTRW {
    #[doc = "The SPI operates in Slave mode."]
    SLAVE,
    #[doc = "The SPI operates in Master mode."]
    MASTER,
}
impl MSTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSTRW::SLAVE => false,
            MSTRW::MASTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSTRW<'a> {
    w: &'a mut W,
}
impl<'a> _MSTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The SPI operates in Slave mode."]
    #[inline]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTRW::SLAVE)
    }
    #[doc = "The SPI operates in Master mode."]
    #[inline]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTRW::MASTER)
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
#[doc = "Values that can be written to the field `LSBF`"]
pub enum LSBFW {
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    MSB,
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    LSB,
}
impl LSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFW::MSB => false,
            LSBFW::LSB => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI data is transferred MSB (bit 7) first."]
    #[inline]
    pub fn msb(self) -> &'a mut W {
        self.variant(LSBFW::MSB)
    }
    #[doc = "SPI data is transferred LSB (bit 0) first."]
    #[inline]
    pub fn lsb(self) -> &'a mut W {
        self.variant(LSBFW::LSB)
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
#[doc = "Values that can be written to the field `SPIE`"]
pub enum SPIEW {
    #[doc = "SPI interrupts are inhibited."]
    INTBLOCK,
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    HWINT,
}
impl SPIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIEW::INTBLOCK => false,
            SPIEW::HWINT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI interrupts are inhibited."]
    #[inline]
    pub fn intblock(self) -> &'a mut W {
        self.variant(SPIEW::INTBLOCK)
    }
    #[doc = "A hardware interrupt is generated each time the SPIF or MODF bits are activated."]
    #[inline]
    pub fn hwint(self) -> &'a mut W {
        self.variant(SPIEW::HWINT)
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
#[doc = "Values that can be written to the field `BITS`"]
pub enum BITSW {
    #[doc = "8 bits per transfer"]
    _8_BITS_PER_TRANSFER,
    #[doc = "9 bits per transfer"]
    _9_BITS_PER_TRANSFER,
    #[doc = "10 bits per transfer"]
    _10_BITS_PER_TRANSFER,
    #[doc = "11 bits per transfer"]
    _11_BITS_PER_TRANSFER,
    #[doc = "12 bits per transfer"]
    _12_BITS_PER_TRANSFER,
    #[doc = "13 bits per transfer"]
    _13_BITS_PER_TRANSFER,
    #[doc = "14 bits per transfer"]
    _14_BITS_PER_TRANSFER,
    #[doc = "15 bits per transfer"]
    _15_BITS_PER_TRANSFER,
    #[doc = "16 bits per transfer"]
    _16_BITS_PER_TRANSFER,
}
impl BITSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BITSW::_8_BITS_PER_TRANSFER => 8,
            BITSW::_9_BITS_PER_TRANSFER => 9,
            BITSW::_10_BITS_PER_TRANSFER => 10,
            BITSW::_11_BITS_PER_TRANSFER => 11,
            BITSW::_12_BITS_PER_TRANSFER => 12,
            BITSW::_13_BITS_PER_TRANSFER => 13,
            BITSW::_14_BITS_PER_TRANSFER => 14,
            BITSW::_15_BITS_PER_TRANSFER => 15,
            BITSW::_16_BITS_PER_TRANSFER => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BITSW<'a> {
    w: &'a mut W,
}
impl<'a> _BITSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BITSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8 bits per transfer"]
    #[inline]
    pub fn _8_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_8_BITS_PER_TRANSFER)
    }
    #[doc = "9 bits per transfer"]
    #[inline]
    pub fn _9_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_9_BITS_PER_TRANSFER)
    }
    #[doc = "10 bits per transfer"]
    #[inline]
    pub fn _10_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_10_BITS_PER_TRANSFER)
    }
    #[doc = "11 bits per transfer"]
    #[inline]
    pub fn _11_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_11_BITS_PER_TRANSFER)
    }
    #[doc = "12 bits per transfer"]
    #[inline]
    pub fn _12_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_12_BITS_PER_TRANSFER)
    }
    #[doc = "13 bits per transfer"]
    #[inline]
    pub fn _13_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_13_BITS_PER_TRANSFER)
    }
    #[doc = "14 bits per transfer"]
    #[inline]
    pub fn _14_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_14_BITS_PER_TRANSFER)
    }
    #[doc = "15 bits per transfer"]
    #[inline]
    pub fn _15_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_15_BITS_PER_TRANSFER)
    }
    #[doc = "16 bits per transfer"]
    #[inline]
    pub fn _16_bits_per_transfer(self) -> &'a mut W {
        self.variant(BITSW::_16_BITS_PER_TRANSFER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline]
    pub fn bitenable(&self) -> BITENABLER {
        BITENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline]
    pub fn mstr(&self) -> MSTRR {
        MSTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline]
    pub fn lsbf(&self) -> LSBFR {
        LSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline]
    pub fn spie(&self) -> SPIER {
        SPIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline]
    pub fn bits_(&self) -> BITSR {
        BITSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 2 - The SPI controller sends and receives 8 bits of data per transfer."]
    #[inline]
    pub fn bitenable(&mut self) -> _BITENABLEW {
        _BITENABLEW { w: self }
    }
    #[doc = "Bit 3 - Clock phase control determines the relationship between the data and the clock on SPI transfers, and controls when a slave transfer is defined as starting and ending."]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 4 - Clock polarity control."]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 5 - Master mode select."]
    #[inline]
    pub fn mstr(&mut self) -> _MSTRW {
        _MSTRW { w: self }
    }
    #[doc = "Bit 6 - LSB First controls which direction each byte is shifted when transferred."]
    #[inline]
    pub fn lsbf(&mut self) -> _LSBFW {
        _LSBFW { w: self }
    }
    #[doc = "Bit 7 - Serial peripheral interrupt enable."]
    #[inline]
    pub fn spie(&mut self) -> _SPIEW {
        _SPIEW { w: self }
    }
    #[doc = "Bits 8:11 - When bit 2 of this register is 1, this field controls the number of bits per transfer:"]
    #[inline]
    pub fn bits_(&mut self) -> _BITSW {
        _BITSW { w: self }
    }
}
