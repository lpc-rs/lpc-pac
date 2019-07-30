#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
}
impl crate::ToBits<u8> for DSSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
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
        }
    }
}
#[doc = r"Reader of the field"]
pub type DSS_R = crate::FR<u8, DSSR>;
impl DSS_R {
    #[doc = "Checks if the value of the field is `_4_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_4_bit_transfer(&self) -> bool {
        *self == DSSR::_4_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_5_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_5_bit_transfer(&self) -> bool {
        *self == DSSR::_5_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_6_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_6_bit_transfer(&self) -> bool {
        *self == DSSR::_6_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_7_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_7_bit_transfer(&self) -> bool {
        *self == DSSR::_7_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_8_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_8_bit_transfer(&self) -> bool {
        *self == DSSR::_8_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_9_bit_transfer(&self) -> bool {
        *self == DSSR::_9_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_10_bit_transfer(&self) -> bool {
        *self == DSSR::_10_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_11_bit_transfer(&self) -> bool {
        *self == DSSR::_11_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_12_bit_transfer(&self) -> bool {
        *self == DSSR::_12_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_13_bit_transfer(&self) -> bool {
        *self == DSSR::_13_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_14_bit_transfer(&self) -> bool {
        *self == DSSR::_14_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_15_bit_transfer(&self) -> bool {
        *self == DSSR::_15_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_16_bit_transfer(&self) -> bool {
        *self == DSSR::_16_BIT_TRANSFER
    }
}
#[doc = "Values that can be written to the field `DSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
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
#[doc = r"Proxy"]
pub struct _DSSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn _4_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_4_BIT_TRANSFER)
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn _5_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_5_BIT_TRANSFER)
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn _6_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_6_BIT_TRANSFER)
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn _7_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_7_BIT_TRANSFER)
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn _8_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_8_BIT_TRANSFER)
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn _9_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_9_BIT_TRANSFER)
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn _10_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_10_BIT_TRANSFER)
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn _11_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_11_BIT_TRANSFER)
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn _12_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_12_BIT_TRANSFER)
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn _13_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_13_BIT_TRANSFER)
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn _14_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_14_BIT_TRANSFER)
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn _15_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_15_BIT_TRANSFER)
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn _16_bit_transfer(self) -> &'a mut W {
        self.variant(DSSW::_16_BIT_TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
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
}
impl crate::ToBits<u8> for FRFR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            FRFR::SPI => 0,
            FRFR::TI => 1,
            FRFR::MICROWIRE => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRF_R = crate::FR<u8, FRFR>;
impl FRF_R {
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FRFR::SPI
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRFR::TI
    }
    #[doc = "Checks if the value of the field is `MICROWIRE`"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == FRFR::MICROWIRE
    }
}
#[doc = "Values that can be written to the field `FRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRFW {
    #[doc = "SPI"]
    SPI,
    #[doc = "TI"]
    TI,
    #[doc = "Microwire"]
    MICROWIRE,
}
impl FRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FRFW::SPI => 0,
            FRFW::TI => 1,
            FRFW::MICROWIRE => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FRFW<'a> {
    w: &'a mut W,
}
impl<'a> _FRFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRFW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(FRFW::SPI)
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRFW::TI)
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut W {
        self.variant(FRFW::MICROWIRE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "SPI controller maintains the bus clock low between frames."]
    LOW,
    #[doc = "SPI controller maintains the bus clock high between frames."]
    HIGH,
}
impl crate::ToBits<bool> for CPOLR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CPOLR::LOW => false,
            CPOLR::HIGH => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CPOL_R = crate::FR<bool, CPOLR>;
impl CPOL_R {
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOLR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOLR::HIGH
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLW {
    #[doc = "SPI controller maintains the bus clock low between frames."]
    LOW,
    #[doc = "SPI controller maintains the bus clock high between frames."]
    HIGH,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::LOW => false,
            CPOLW::HIGH => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOLW::LOW)
    }
    #[doc = "SPI controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOLW::HIGH)
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
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "SPI controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRSTCLOCK,
    #[doc = "SPI controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECONDCLOCK,
}
impl crate::ToBits<bool> for CPHAR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            CPHAR::FIRSTCLOCK => false,
            CPHAR::SECONDCLOCK => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CPHA_R = crate::FR<bool, CPHAR>;
impl CPHA_R {
    #[doc = "Checks if the value of the field is `FIRSTCLOCK`"]
    #[inline(always)]
    pub fn is_firstclock(&self) -> bool {
        *self == CPHAR::FIRSTCLOCK
    }
    #[doc = "Checks if the value of the field is `SECONDCLOCK`"]
    #[inline(always)]
    pub fn is_secondclock(&self) -> bool {
        *self == CPHAR::SECONDCLOCK
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAW {
    #[doc = "SPI controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRSTCLOCK,
    #[doc = "SPI controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECONDCLOCK,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::FIRSTCLOCK => false,
            CPHAW::SECONDCLOCK => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SPI controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn firstclock(self) -> &'a mut W {
        self.variant(CPHAW::FIRSTCLOCK)
    }
    #[doc = "SPI controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn secondclock(self) -> &'a mut W {
        self.variant(CPHAW::SECONDCLOCK)
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
#[doc = r"Reader of the field"]
pub type SCR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _SCRW<'a> {
    w: &'a mut W,
}
impl<'a> _SCRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits() & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&mut self) -> _DSSW {
        _DSSW { w: self }
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&mut self) -> _FRFW {
        _FRFW { w: self }
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&mut self) -> _SCRW {
        _SCRW { w: self }
    }
}
