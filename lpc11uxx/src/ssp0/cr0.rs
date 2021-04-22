#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR0_SPEC>> for R {
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CR0_SPEC>> for W {
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `DSS` reader - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub struct DSS_R(crate::FieldReader<u8, DSS_A>);
impl DSS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSS_A> {
        match self.bits {
            3 => Some(DSS_A::_4_BIT_TRANSFER),
            4 => Some(DSS_A::_5_BIT_TRANSFER),
            5 => Some(DSS_A::_6_BIT_TRANSFER),
            6 => Some(DSS_A::_7_BIT_TRANSFER),
            7 => Some(DSS_A::_8_BIT_TRANSFER),
            8 => Some(DSS_A::_9_BIT_TRANSFER),
            9 => Some(DSS_A::_10_BIT_TRANSFER),
            10 => Some(DSS_A::_11_BIT_TRANSFER),
            11 => Some(DSS_A::_12_BIT_TRANSFER),
            12 => Some(DSS_A::_13_BIT_TRANSFER),
            13 => Some(DSS_A::_14_BIT_TRANSFER),
            14 => Some(DSS_A::_15_BIT_TRANSFER),
            15 => Some(DSS_A::_16_BIT_TRANSFER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_4_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_4_bit_transfer(&self) -> bool {
        **self == DSS_A::_4_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_5_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_5_bit_transfer(&self) -> bool {
        **self == DSS_A::_5_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_6_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_6_bit_transfer(&self) -> bool {
        **self == DSS_A::_6_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_7_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_7_bit_transfer(&self) -> bool {
        **self == DSS_A::_7_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_8_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_8_bit_transfer(&self) -> bool {
        **self == DSS_A::_8_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_9_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_9_bit_transfer(&self) -> bool {
        **self == DSS_A::_9_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_10_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_10_bit_transfer(&self) -> bool {
        **self == DSS_A::_10_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_11_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_11_bit_transfer(&self) -> bool {
        **self == DSS_A::_11_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_12_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_12_bit_transfer(&self) -> bool {
        **self == DSS_A::_12_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_13_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_13_bit_transfer(&self) -> bool {
        **self == DSS_A::_13_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_14_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_14_bit_transfer(&self) -> bool {
        **self == DSS_A::_14_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_15_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_15_bit_transfer(&self) -> bool {
        **self == DSS_A::_15_BIT_TRANSFER
    }
    #[doc = "Checks if the value of the field is `_16_BIT_TRANSFER`"]
    #[inline(always)]
    pub fn is_16_bit_transfer(&self) -> bool {
        **self == DSS_A::_16_BIT_TRANSFER
    }
}
impl core::ops::Deref for DSS_R {
    type Target = crate::FieldReader<u8, DSS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSS` writer - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
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
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
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
}
impl From<FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FRF` reader - Frame Format."]
pub struct FRF_R(crate::FieldReader<u8, FRF_A>);
impl FRF_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            0 => FRF_A::SPI,
            1 => FRF_A::TI,
            2 => FRF_A::MICROWIRE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == FRF_A::SPI
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        **self == FRF_A::TI
    }
    #[doc = "Checks if the value of the field is `MICROWIRE`"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        **self == FRF_A::MICROWIRE
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<u8, FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRF` writer - Frame Format."]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Clock Out Polarity. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: SPI controller maintains the bus clock low between frames."]
    LOW = 0,
    #[doc = "1: SPI controller maintains the bus clock high between frames."]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Out Polarity. This bit is only used in SPI mode."]
pub struct CPOL_R(crate::FieldReader<bool, CPOL_A>);
impl CPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CPOL_A::HIGH
    }
}
impl core::ops::Deref for CPOL_R {
    type Target = crate::FieldReader<bool, CPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPOL` writer - Clock Out Polarity. This bit is only used in SPI mode."]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "SPI controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Clock Out Phase. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: SPI controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FIRSTCLOCK = 0,
    #[doc = "1: SPI controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SECONDCLOCK = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Out Phase. This bit is only used in SPI mode."]
pub struct CPHA_R(crate::FieldReader<bool, CPHA_A>);
impl CPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRSTCLOCK,
            true => CPHA_A::SECONDCLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `FIRSTCLOCK`"]
    #[inline(always)]
    pub fn is_firstclock(&self) -> bool {
        **self == CPHA_A::FIRSTCLOCK
    }
    #[doc = "Checks if the value of the field is `SECONDCLOCK`"]
    #[inline(always)]
    pub fn is_secondclock(&self) -> bool {
        **self == CPHA_A::SECONDCLOCK
    }
}
impl core::ops::Deref for CPHA_R {
    type Target = crate::FieldReader<bool, CPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPHA` writer - Clock Out Phase. This bit is only used in SPI mode."]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn firstclock(self) -> &'a mut W {
        self.variant(CPHA_A::FIRSTCLOCK)
    }
    #[doc = "SPI controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn secondclock(self) -> &'a mut W {
        self.variant(CPHA_A::SECONDCLOCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `SCR` reader - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub struct SCR_R(crate::FieldReader<u8, u8>);
impl SCR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCR` writer - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub struct SCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
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
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
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
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W {
        SCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
