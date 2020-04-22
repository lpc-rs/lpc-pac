#[doc = "Reader of register CR0"]
pub type R = crate::R<u32, super::CR0>;
#[doc = "Writer for register CR0"]
pub type W = crate::W<u32, super::CR0>;
#[doc = "Register CR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSS_A {
    #[doc = "3: 4-bit transfer"]
    _4_BIT_TRANSFER = 3,
    #[doc = "4: 5-bit transfer"]
    _5_BIT_TRANSFER = 4,
    #[doc = "5: 6-bit transfer"]
    _6_BIT_TRANSFER = 5,
    #[doc = "6: 7-bit transfer"]
    _7_BIT_TRANSFER = 6,
    #[doc = "7: 8-bit transfer"]
    _8_BIT_TRANSFER = 7,
    #[doc = "8: 9-bit transfer"]
    _9_BIT_TRANSFER = 8,
    #[doc = "9: 10-bit transfer"]
    _10_BIT_TRANSFER = 9,
    #[doc = "10: 11-bit transfer"]
    _11_BIT_TRANSFER = 10,
    #[doc = "11: 12-bit transfer"]
    _12_BIT_TRANSFER = 11,
    #[doc = "12: 13-bit transfer"]
    _13_BIT_TRANSFER = 12,
    #[doc = "13: 14-bit transfer"]
    _14_BIT_TRANSFER = 13,
    #[doc = "14: 15-bit transfer"]
    _15_BIT_TRANSFER = 14,
    #[doc = "15: 16-bit transfer"]
    _16_BIT_TRANSFER = 15,
}
impl From<DSS_A> for u8 {
    #[inline(always)]
    fn from(variant: DSS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSS`"]
pub type DSS_R = crate::R<u8, DSS_A>;
impl DSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DSS_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DSS_A::_4_BIT_TRANSFER),
            4 => Val(DSS_A::_5_BIT_TRANSFER),
            5 => Val(DSS_A::_6_BIT_TRANSFER),
            6 => Val(DSS_A::_7_BIT_TRANSFER),
            7 => Val(DSS_A::_8_BIT_TRANSFER),
            8 => Val(DSS_A::_9_BIT_TRANSFER),
            9 => Val(DSS_A::_10_BIT_TRANSFER),
            10 => Val(DSS_A::_11_BIT_TRANSFER),
            11 => Val(DSS_A::_12_BIT_TRANSFER),
            12 => Val(DSS_A::_13_BIT_TRANSFER),
            13 => Val(DSS_A::_14_BIT_TRANSFER),
            14 => Val(DSS_A::_15_BIT_TRANSFER),
            15 => Val(DSS_A::_16_BIT_TRANSFER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_4_bit_transfer(&self) -> bool {
        *self == DSS_A::_4_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_5_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_5_bit_transfer(&self) -> bool {
        *self == DSS_A::_5_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_6_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_6_bit_transfer(&self) -> bool {
        *self == DSS_A::_6_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_7_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_7_bit_transfer(&self) -> bool {
        *self == DSS_A::_7_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_8_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_8_bit_transfer(&self) -> bool {
        *self == DSS_A::_8_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_9_bit_transfer(&self) -> bool {
        *self == DSS_A::_9_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_10_bit_transfer(&self) -> bool {
        *self == DSS_A::_10_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_11_bit_transfer(&self) -> bool {
        *self == DSS_A::_11_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_12_bit_transfer(&self) -> bool {
        *self == DSS_A::_12_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_13_bit_transfer(&self) -> bool {
        *self == DSS_A::_13_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_14_bit_transfer(&self) -> bool {
        *self == DSS_A::_14_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_15_bit_transfer(&self) -> bool {
        *self == DSS_A::_15_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_16_bit_transfer(&self) -> bool {
        *self == DSS_A::_16_BIT_TRANSFER
    }
}
#[doc = "Write proxy for field `DSS`"]
pub struct DSS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn _4_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_4_BIT_TRANSFER)
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn _5_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_5_BIT_TRANSFER)
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn _6_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_6_BIT_TRANSFER)
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn _7_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_7_BIT_TRANSFER)
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn _8_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_8_BIT_TRANSFER)
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn _9_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_9_BIT_TRANSFER)
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn _10_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_10_BIT_TRANSFER)
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn _11_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_11_BIT_TRANSFER)
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn _12_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_12_BIT_TRANSFER)
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn _13_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_13_BIT_TRANSFER)
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn _14_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_14_BIT_TRANSFER)
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn _15_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_15_BIT_TRANSFER)
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn _16_bit_transfer(self) -> &'a mut W {
        self.variant(DSS_A::_16_BIT_TRANSFER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Frame Format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FRF_A {
    #[doc = "0: SPI"]
    SPI = 0,
    #[doc = "1: TI"]
    TI = 1,
    #[doc = "2: Microwire"]
    MICROWIRE = 2,
    #[doc = "3: This combination is not supported and should not be used."]
    THIS_COMBINATION_IS_ = 3,
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<u8, FRF_A>;
impl FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            0 => FRF_A::SPI,
            1 => FRF_A::TI,
            2 => FRF_A::MICROWIRE,
            3 => FRF_A::THIS_COMBINATION_IS_,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == FRF_A::SPI
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::TI
    }
    #[doc = "Checks if the value of the field is `MICROWIRE`"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == FRF_A::MICROWIRE
    }
    #[doc = "Checks if the value of the field is `THIS_COMBINATION_IS_`"]
    #[inline(always)]
    pub fn is_this_combination_is_(&self) -> bool {
        *self == FRF_A::THIS_COMBINATION_IS_
    }
}
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(FRF_A::SPI)
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut W {
        self.variant(FRF_A::MICROWIRE)
    }
    #[doc = "This combination is not supported and should not be used."]
    #[inline(always)]
    pub fn this_combination_is_(self) -> &'a mut W {
        self.variant(FRF_A::THIS_COMBINATION_IS_)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Clock Out Polarity. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SSP controller maintains the bus clock low between frames."]
    BUS_LOW = 0,
    #[doc = "1: SSP controller maintains the bus clock high between frames."]
    BUS_HIGH = 1,
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
            false => CPOL_A::BUS_LOW,
            true => CPOL_A::BUS_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_LOW`"]
    #[inline(always)]
    pub fn is_bus_low(&self) -> bool {
        *self == CPOL_A::BUS_LOW
    }
    #[doc = "Checks if the value of the field is `BUS_HIGH`"]
    #[inline(always)]
    pub fn is_bus_high(&self) -> bool {
        *self == CPOL_A::BUS_HIGH
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
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn bus_low(self) -> &'a mut W {
        self.variant(CPOL_A::BUS_LOW)
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn bus_high(self) -> &'a mut W {
        self.variant(CPOL_A::BUS_HIGH)
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
#[doc = "Clock Out Phase. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRST_CLOCK = 0,
    #[doc = "1: SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECOND_CLOCK = 1,
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
            false => CPHA_A::FIRST_CLOCK,
            true => CPHA_A::SECOND_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST_CLOCK`"]
    #[inline(always)]
    pub fn is_first_clock(&self) -> bool {
        *self == CPHA_A::FIRST_CLOCK
    }
    #[doc = "Checks if the value of the field is `SECOND_CLOCK`"]
    #[inline(always)]
    pub fn is_second_clock(&self) -> bool {
        *self == CPHA_A::SECOND_CLOCK
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
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn first_clock(self) -> &'a mut W {
        self.variant(CPHA_A::FIRST_CLOCK)
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn second_clock(self) -> &'a mut W {
        self.variant(CPHA_A::SECOND_CLOCK)
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
#[doc = "Reader of field `SCR`"]
pub type SCR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCR`"]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&mut self) -> DSS_W {
        DSS_W { w: self }
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
}
