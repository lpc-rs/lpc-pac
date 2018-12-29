#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSSR {
    #[doc = "4-bit transfer"]
    _4_BIT_TRANSFER,
    #[doc = "5-bit transfer"]
    _5_BIT_TRANSFER,
    #[doc = "6-bit transfer"]
    _6_BIT_TRANSFER,
    #[doc = "7-bit transfer"]
    _7_BIT_TRANSFER,
    #[doc = "8-bit transfer"]
    _8_BIT_TRANSFER,
    #[doc = "9-bit transfer"]
    _9_BIT_TRANSFER,
    #[doc = "10-bit transfer"]
    _10_BIT_TRANSFER,
    #[doc = "11-bit transfer"]
    _11_BIT_TRANSFER,
    #[doc = "12-bit transfer"]
    _12_BIT_TRANSFER,
    #[doc = "13-bit transfer"]
    _13_BIT_TRANSFER,
    #[doc = "14-bit transfer"]
    _14_BIT_TRANSFER,
    #[doc = "15-bit transfer"]
    _15_BIT_TRANSFER,
    #[doc = "16-bit transfer"]
    _16_BIT_TRANSFER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DSSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSSR::_4_BIT_TRANSFER => 3,
            DSSR::_5_BIT_TRANSFER => 4,
            DSSR::_6_BIT_TRANSFER => 5,
            DSSR::_7_BIT_TRANSFER => 6,
            DSSR::_8_BIT_TRANSFER => 7,
            DSSR::_9_BIT_TRANSFER => 8,
            DSSR::_10_BIT_TRANSFER => 9,
            DSSR::_11_BIT_TRANSFER => 10,
            DSSR::_12_BIT_TRANSFER => 11,
            DSSR::_13_BIT_TRANSFER => 12,
            DSSR::_14_BIT_TRANSFER => 13,
            DSSR::_15_BIT_TRANSFER => 14,
            DSSR::_16_BIT_TRANSFER => 15,
            DSSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSSR {
        match value {
            3 => DSSR::_4_BIT_TRANSFER,
            4 => DSSR::_5_BIT_TRANSFER,
            5 => DSSR::_6_BIT_TRANSFER,
            6 => DSSR::_7_BIT_TRANSFER,
            7 => DSSR::_8_BIT_TRANSFER,
            8 => DSSR::_9_BIT_TRANSFER,
            9 => DSSR::_10_BIT_TRANSFER,
            10 => DSSR::_11_BIT_TRANSFER,
            11 => DSSR::_12_BIT_TRANSFER,
            12 => DSSR::_13_BIT_TRANSFER,
            13 => DSSR::_14_BIT_TRANSFER,
            14 => DSSR::_15_BIT_TRANSFER,
            15 => DSSR::_16_BIT_TRANSFER,
            i => DSSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BIT_TRANSFER`"]
    #[inline]
    pub fn is_4_bit_transfer(&self) -> bool {
        *self == DSSR::_4_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_5_BIT_TRANSFER`"]
    #[inline]
    pub fn is_5_bit_transfer(&self) -> bool {
        *self == DSSR::_5_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_6_BIT_TRANSFER`"]
    #[inline]
    pub fn is_6_bit_transfer(&self) -> bool {
        *self == DSSR::_6_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_7_BIT_TRANSFER`"]
    #[inline]
    pub fn is_7_bit_transfer(&self) -> bool {
        *self == DSSR::_7_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_8_BIT_TRANSFER`"]
    #[inline]
    pub fn is_8_bit_transfer(&self) -> bool {
        *self == DSSR::_8_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BIT_TRANSFER`"]
    #[inline]
    pub fn is_9_bit_transfer(&self) -> bool {
        *self == DSSR::_9_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BIT_TRANSFER`"]
    #[inline]
    pub fn is_10_bit_transfer(&self) -> bool {
        *self == DSSR::_10_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BIT_TRANSFER`"]
    #[inline]
    pub fn is_11_bit_transfer(&self) -> bool {
        *self == DSSR::_11_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BIT_TRANSFER`"]
    #[inline]
    pub fn is_12_bit_transfer(&self) -> bool {
        *self == DSSR::_12_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BIT_TRANSFER`"]
    #[inline]
    pub fn is_13_bit_transfer(&self) -> bool {
        *self == DSSR::_13_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BIT_TRANSFER`"]
    #[inline]
    pub fn is_14_bit_transfer(&self) -> bool {
        *self == DSSR::_14_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BIT_TRANSFER`"]
    #[inline]
    pub fn is_15_bit_transfer(&self) -> bool {
        *self == DSSR::_15_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BIT_TRANSFER`"]
    #[inline]
    pub fn is_16_bit_transfer(&self) -> bool {
        *self == DSSR::_16_BIT_TRANSFER
    }
}
#[doc = "Possible values of the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFR {
    #[doc = "SPI"]
    SPI,
    #[doc = "TI"]
    TI,
    #[doc = "Microwire"]
    MICROWIRE,
    #[doc = "This combination is not supported and should not be used."]
    THIS_COMBINATION_IS_,
}
impl FRFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            FRFR::SPI => 0,
            FRFR::TI => 1,
            FRFR::MICROWIRE => 2,
            FRFR::THIS_COMBINATION_IS_ => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> FRFR {
        match value {
            0 => FRFR::SPI,
            1 => FRFR::TI,
            2 => FRFR::MICROWIRE,
            3 => FRFR::THIS_COMBINATION_IS_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline]
    pub fn is_spi(&self) -> bool {
        *self == FRFR::SPI
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline]
    pub fn is_ti(&self) -> bool {
        *self == FRFR::TI
    }
    #[doc = "Checks if the value of the field is `MICROWIRE`"]
    #[inline]
    pub fn is_microwire(&self) -> bool {
        *self == FRFR::MICROWIRE
    }
    #[doc = "Checks if the value of the field is `THIS_COMBINATION_IS_`"]
    #[inline]
    pub fn is_this_combination_is_(&self) -> bool {
        *self == FRFR::THIS_COMBINATION_IS_
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "SSP controller maintains the bus clock low between frames."]
    BUS_LOW,
    #[doc = "SSP controller maintains the bus clock high between frames."]
    BUS_HIGH,
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
            CPOLR::BUS_LOW => false,
            CPOLR::BUS_HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::BUS_LOW,
            true => CPOLR::BUS_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_LOW`"]
    #[inline]
    pub fn is_bus_low(&self) -> bool {
        *self == CPOLR::BUS_LOW
    }
    #[doc = "Checks if the value of the field is `BUS_HIGH`"]
    #[inline]
    pub fn is_bus_high(&self) -> bool {
        *self == CPOLR::BUS_HIGH
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRST_CLOCK,
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECOND_CLOCK,
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
            CPHAR::FIRST_CLOCK => false,
            CPHAR::SECOND_CLOCK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::FIRST_CLOCK,
            true => CPHAR::SECOND_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST_CLOCK`"]
    #[inline]
    pub fn is_first_clock(&self) -> bool {
        *self == CPHAR::FIRST_CLOCK
    }
    #[doc = "Checks if the value of the field is `SECOND_CLOCK`"]
    #[inline]
    pub fn is_second_clock(&self) -> bool {
        *self == CPHAR::SECOND_CLOCK
    }
}
#[doc = r" Value of the field"]
pub struct SCRR {
    bits: u8,
}
impl SCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `DSS`"]
pub enum DSSW {
    #[doc = "4-bit transfer"]
    _4_BIT_TRANSFER,
    #[doc = "5-bit transfer"]
    _5_BIT_TRANSFER,
    #[doc = "6-bit transfer"]
    _6_BIT_TRANSFER,
    #[doc = "7-bit transfer"]
    _7_BIT_TRANSFER,
    #[doc = "8-bit transfer"]
    _8_BIT_TRANSFER,
    #[doc = "9-bit transfer"]
    _9_BIT_TRANSFER,
    #[doc = "10-bit transfer"]
    _10_BIT_TRANSFER,
    #[doc = "11-bit transfer"]
    _11_BIT_TRANSFER,
    #[doc = "12-bit transfer"]
    _12_BIT_TRANSFER,
    #[doc = "13-bit transfer"]
    _13_BIT_TRANSFER,
    #[doc = "14-bit transfer"]
    _14_BIT_TRANSFER,
    #[doc = "15-bit transfer"]
    _15_BIT_TRANSFER,
    #[doc = "16-bit transfer"]
    _16_BIT_TRANSFER,
}
impl DSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DSSW::_4_BIT_TRANSFER => 3,
            DSSW::_5_BIT_TRANSFER => 4,
            DSSW::_6_BIT_TRANSFER => 5,
            DSSW::_7_BIT_TRANSFER => 6,
            DSSW::_8_BIT_TRANSFER => 7,
            DSSW::_9_BIT_TRANSFER => 8,
            DSSW::_10_BIT_TRANSFER => 9,
            DSSW::_11_BIT_TRANSFER => 10,
            DSSW::_12_BIT_TRANSFER => 11,
            DSSW::_13_BIT_TRANSFER => 12,
            DSSW::_14_BIT_TRANSFER => 13,
            DSSW::_15_BIT_TRANSFER => 14,
            DSSW::_16_BIT_TRANSFER => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit transfer"]
    #[inline]
    pub fn _4_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_4_BIT_TRANSFER)
    }
    #[doc = "5-bit transfer"]
    #[inline]
    pub fn _5_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_5_BIT_TRANSFER)
    }
    #[doc = "6-bit transfer"]
    #[inline]
    pub fn _6_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_6_BIT_TRANSFER)
    }
    #[doc = "7-bit transfer"]
    #[inline]
    pub fn _7_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_7_BIT_TRANSFER)
    }
    #[doc = "8-bit transfer"]
    #[inline]
    pub fn _8_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_8_BIT_TRANSFER)
    }
    #[doc = "9-bit transfer"]
    #[inline]
    pub fn _9_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_9_BIT_TRANSFER)
    }
    #[doc = "10-bit transfer"]
    #[inline]
    pub fn _10_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_10_BIT_TRANSFER)
    }
    #[doc = "11-bit transfer"]
    #[inline]
    pub fn _11_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_11_BIT_TRANSFER)
    }
    #[doc = "12-bit transfer"]
    #[inline]
    pub fn _12_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_12_BIT_TRANSFER)
    }
    #[doc = "13-bit transfer"]
    #[inline]
    pub fn _13_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_13_BIT_TRANSFER)
    }
    #[doc = "14-bit transfer"]
    #[inline]
    pub fn _14_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_14_BIT_TRANSFER)
    }
    #[doc = "15-bit transfer"]
    #[inline]
    pub fn _15_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_15_BIT_TRANSFER)
    }
    #[doc = "16-bit transfer"]
    #[inline]
    pub fn _16_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_16_BIT_TRANSFER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FRF`"]
pub enum FRFW {
    #[doc = "SPI"]
    SPI,
    #[doc = "TI"]
    TI,
    #[doc = "Microwire"]
    MICROWIRE,
    #[doc = "This combination is not supported and should not be used."]
    THIS_COMBINATION_IS_,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRFW::SPI => 0,
            FRFW::TI => 1,
            FRFW::MICROWIRE => 2,
            FRFW::THIS_COMBINATION_IS_ => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SPI"]
    #[inline]
    pub fn spi(self) -> &'a mut W {
        self.variant(FRFW::SPI)
    }
    #[doc = "TI"]
    #[inline]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRFW::TI)
    }
    #[doc = "Microwire"]
    #[inline]
    pub fn microwire(self) -> &'a mut W {
        self.variant(FRFW::MICROWIRE)
    }
    #[doc = "This combination is not supported and should not be used."]
    #[inline]
    pub fn this_combination_is_(self) -> &'a mut W {
        self.variant(FRFW::THIS_COMBINATION_IS_)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "SSP controller maintains the bus clock low between frames."]
    BUS_LOW,
    #[doc = "SSP controller maintains the bus clock high between frames."]
    BUS_HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::BUS_LOW => false,
            CPOLW::BUS_HIGH => true,
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
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline]
    pub fn bus_low(self) -> &'a mut W {
        self.variant(CPOLW::BUS_LOW)
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline]
    pub fn bus_high(self) -> &'a mut W {
        self.variant(CPOLW::BUS_HIGH)
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
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRST_CLOCK,
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECOND_CLOCK,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRST_CLOCK => false,
            CPHAW::SECOND_CLOCK => true,
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
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline]
    pub fn first_clock(self) -> &'a mut W {
        self.variant(CPHAW::FIRST_CLOCK)
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline]
    pub fn second_clock(self) -> &'a mut W {
        self.variant(CPHAW::SECOND_CLOCK)
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
#[doc = r" Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline]
    pub fn dss(&self) -> DSSR {
        DSSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline]
    pub fn frf(&self) -> FRFR {
        FRFR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline]
    pub fn scr(&self) -> SCRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCRR { bits }
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
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline]
    pub fn dss(&mut self) -> _DSSW {
        _DSSW { w: self }
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
}
