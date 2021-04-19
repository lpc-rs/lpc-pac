#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CR_SPEC>> for R {
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl core::convert::From<crate::W<CR_SPEC>> for W {
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
pub struct SEL_R(crate::FieldReader<u8, u8>);
impl SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 276) must be set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_A {
    #[doc = "0: Software-controlled mode: Conversions are software-controlled and require 11 clocks."]
    SOFTWARE_CONTROLLED = 0,
    #[doc = "1: Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher  bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    HARDWARE_SCAN = 1,
}
impl From<BURST_A> for bool {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 276) must be set to 0."]
pub struct BURST_R(crate::FieldReader<bool, BURST_A>);
impl BURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            false => BURST_A::SOFTWARE_CONTROLLED,
            true => BURST_A::HARDWARE_SCAN,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE_CONTROLLED`"]
    #[inline(always)]
    pub fn is_software_controlled(&self) -> bool {
        **self == BURST_A::SOFTWARE_CONTROLLED
    }
    #[doc = "Checks if the value of the field is `HARDWARE_SCAN`"]
    #[inline(always)]
    pub fn is_hardware_scan(&self) -> bool {
        **self == BURST_A::HARDWARE_SCAN
    }
}
impl core::ops::Deref for BURST_R {
    type Target = crate::FieldReader<bool, BURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST` writer - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 276) must be set to 0."]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Software-controlled mode: Conversions are software-controlled and require 11 clocks."]
    #[inline(always)]
    pub fn software_controlled(self) -> &'a mut W {
        self.variant(BURST_A::SOFTWARE_CONTROLLED)
    }
    #[doc = "Hardware scan mode: The AD converter does repeated conversions at the rate selected by the CLKS field, scanning (if necessary) through the pins selected by 1s in the SEL field. The first conversion after the start corresponds to the least-significant bit set to 1 in the SEL field, then the next higher bits (pins) set to 1 are scanned if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion in progress when this bit is cleared will be completed. Important: START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn hardware_scan(self) -> &'a mut W {
        self.variant(BURST_A::HARDWARE_SCAN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: 11 clocks / 10 bits"]
    _11_CLOCKS_10_BITS = 0,
    #[doc = "1: 10 clocks / 9 bits"]
    _10_CLOCKS_9_BITS = 1,
    #[doc = "2: 9 clocks / 8 bits"]
    _9_CLOCKS_8_BITS = 2,
    #[doc = "3: 8 clocks / 7 bits"]
    _8_CLOCKS_7_BITS = 3,
    #[doc = "4: 7 clocks / 6 bits"]
    _7_CLOCKS_6_BITS = 4,
    #[doc = "5: 6 clocks / 5 bits"]
    _6_CLOCKS_5_BITS = 5,
    #[doc = "6: 5 clocks / 4 bits"]
    _5_CLOCKS_4_BITS = 6,
    #[doc = "7: 4 clocks / 3 bits"]
    _4_CLOCKS_3_BITS = 7,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKS` reader - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
pub struct CLKS_R(crate::FieldReader<u8, CLKS_A>);
impl CLKS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::_11_CLOCKS_10_BITS,
            1 => CLKS_A::_10_CLOCKS_9_BITS,
            2 => CLKS_A::_9_CLOCKS_8_BITS,
            3 => CLKS_A::_8_CLOCKS_7_BITS,
            4 => CLKS_A::_7_CLOCKS_6_BITS,
            5 => CLKS_A::_6_CLOCKS_5_BITS,
            6 => CLKS_A::_5_CLOCKS_4_BITS,
            7 => CLKS_A::_4_CLOCKS_3_BITS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_11_CLOCKS_10_BITS`"]
    #[inline(always)]
    pub fn is_11_clocks_10_bits(&self) -> bool {
        **self == CLKS_A::_11_CLOCKS_10_BITS
    }
    #[doc = "Checks if the value of the field is `_10_CLOCKS_9_BITS`"]
    #[inline(always)]
    pub fn is_10_clocks_9_bits(&self) -> bool {
        **self == CLKS_A::_10_CLOCKS_9_BITS
    }
    #[doc = "Checks if the value of the field is `_9_CLOCKS_8_BITS`"]
    #[inline(always)]
    pub fn is_9_clocks_8_bits(&self) -> bool {
        **self == CLKS_A::_9_CLOCKS_8_BITS
    }
    #[doc = "Checks if the value of the field is `_8_CLOCKS_7_BITS`"]
    #[inline(always)]
    pub fn is_8_clocks_7_bits(&self) -> bool {
        **self == CLKS_A::_8_CLOCKS_7_BITS
    }
    #[doc = "Checks if the value of the field is `_7_CLOCKS_6_BITS`"]
    #[inline(always)]
    pub fn is_7_clocks_6_bits(&self) -> bool {
        **self == CLKS_A::_7_CLOCKS_6_BITS
    }
    #[doc = "Checks if the value of the field is `_6_CLOCKS_5_BITS`"]
    #[inline(always)]
    pub fn is_6_clocks_5_bits(&self) -> bool {
        **self == CLKS_A::_6_CLOCKS_5_BITS
    }
    #[doc = "Checks if the value of the field is `_5_CLOCKS_4_BITS`"]
    #[inline(always)]
    pub fn is_5_clocks_4_bits(&self) -> bool {
        **self == CLKS_A::_5_CLOCKS_4_BITS
    }
    #[doc = "Checks if the value of the field is `_4_CLOCKS_3_BITS`"]
    #[inline(always)]
    pub fn is_4_clocks_3_bits(&self) -> bool {
        **self == CLKS_A::_4_CLOCKS_3_BITS
    }
}
impl core::ops::Deref for CLKS_R {
    type Target = crate::FieldReader<u8, CLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKS` writer - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
pub struct CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "11 clocks / 10 bits"]
    #[inline(always)]
    pub fn _11_clocks_10_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_11_CLOCKS_10_BITS)
    }
    #[doc = "10 clocks / 9 bits"]
    #[inline(always)]
    pub fn _10_clocks_9_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_10_CLOCKS_9_BITS)
    }
    #[doc = "9 clocks / 8 bits"]
    #[inline(always)]
    pub fn _9_clocks_8_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_9_CLOCKS_8_BITS)
    }
    #[doc = "8 clocks / 7 bits"]
    #[inline(always)]
    pub fn _8_clocks_7_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_8_CLOCKS_7_BITS)
    }
    #[doc = "7 clocks / 6 bits"]
    #[inline(always)]
    pub fn _7_clocks_6_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_7_CLOCKS_6_BITS)
    }
    #[doc = "6 clocks / 5 bits"]
    #[inline(always)]
    pub fn _6_clocks_5_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_6_CLOCKS_5_BITS)
    }
    #[doc = "5 clocks / 4 bits"]
    #[inline(always)]
    pub fn _5_clocks_4_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_5_CLOCKS_4_BITS)
    }
    #[doc = "4 clocks / 3 bits"]
    #[inline(always)]
    pub fn _4_clocks_3_bits(self) -> &'a mut W {
        self.variant(CLKS_A::_4_CLOCKS_3_BITS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 17)) | ((value as u32 & 0x07) << 17);
        self.w
    }
}
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum START_A {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)."]
    NO_START_THIS_VALUE = 0,
    #[doc = "1: Start conversion now."]
    START_CONVERSION_NOW = 1,
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    PIO0_2 = 2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    PIO1_5 = 3,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    CT32B0_MAT0 = 4,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    CT32B0_MAT1 = 5,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    CT16B0_MAT0 = 6,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    CT16B0_MAT1 = 7,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
#[doc = "Field `START` reader - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub struct START_R(crate::FieldReader<u8, START_A>);
impl START_R {
    pub(crate) fn new(bits: u8) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            0 => START_A::NO_START_THIS_VALUE,
            1 => START_A::START_CONVERSION_NOW,
            2 => START_A::PIO0_2,
            3 => START_A::PIO1_5,
            4 => START_A::CT32B0_MAT0,
            5 => START_A::CT32B0_MAT1,
            6 => START_A::CT16B0_MAT0,
            7 => START_A::CT16B0_MAT1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_THIS_VALUE`"]
    #[inline(always)]
    pub fn is_no_start_this_value(&self) -> bool {
        **self == START_A::NO_START_THIS_VALUE
    }
    #[doc = "Checks if the value of the field is `START_CONVERSION_NOW`"]
    #[inline(always)]
    pub fn is_start_conversion_now(&self) -> bool {
        **self == START_A::START_CONVERSION_NOW
    }
    #[doc = "Checks if the value of the field is `PIO0_2`"]
    #[inline(always)]
    pub fn is_pio0_2(&self) -> bool {
        **self == START_A::PIO0_2
    }
    #[doc = "Checks if the value of the field is `PIO1_5`"]
    #[inline(always)]
    pub fn is_pio1_5(&self) -> bool {
        **self == START_A::PIO1_5
    }
    #[doc = "Checks if the value of the field is `CT32B0_MAT0`"]
    #[inline(always)]
    pub fn is_ct32b0_mat0(&self) -> bool {
        **self == START_A::CT32B0_MAT0
    }
    #[doc = "Checks if the value of the field is `CT32B0_MAT1`"]
    #[inline(always)]
    pub fn is_ct32b0_mat1(&self) -> bool {
        **self == START_A::CT32B0_MAT1
    }
    #[doc = "Checks if the value of the field is `CT16B0_MAT0`"]
    #[inline(always)]
    pub fn is_ct16b0_mat0(&self) -> bool {
        **self == START_A::CT16B0_MAT0
    }
    #[doc = "Checks if the value of the field is `CT16B0_MAT1`"]
    #[inline(always)]
    pub fn is_ct16b0_mat1(&self) -> bool {
        **self == START_A::CT16B0_MAT1
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<u8, START_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn no_start_this_value(self) -> &'a mut W {
        self.variant(START_A::NO_START_THIS_VALUE)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start_conversion_now(self) -> &'a mut W {
        self.variant(START_A::START_CONVERSION_NOW)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO0_2/SSEL/CT16B0_CAP0."]
    #[inline(always)]
    pub fn pio0_2(self) -> &'a mut W {
        self.variant(START_A::PIO0_2)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on PIO1_5/DIR/CT32B0_CAP0."]
    #[inline(always)]
    pub fn pio1_5(self) -> &'a mut W {
        self.variant(START_A::PIO1_5)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn ct32b0_mat0(self) -> &'a mut W {
        self.variant(START_A::CT32B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT32B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn ct32b0_mat1(self) -> &'a mut W {
        self.variant(START_A::CT32B0_MAT1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT0\\[1\\]."]
    #[inline(always)]
    pub fn ct16b0_mat0(self) -> &'a mut W {
        self.variant(START_A::CT16B0_MAT0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on CT16B0_MAT1\\[1\\]."]
    #[inline(always)]
    pub fn ct16b0_mat1(self) -> &'a mut W {
        self.variant(START_A::CT16B0_MAT1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING = 0,
    #[doc = "1: Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLING = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - This bit is significant only when the START field contains 010-111. In these cases:"]
pub struct EDGE_R(crate::FieldReader<bool, EDGE_A>);
impl EDGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::RISING,
            true => EDGE_A::FALLING,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == EDGE_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == EDGE_A::FALLING
    }
}
impl core::ops::Deref for EDGE_R {
    type Target = crate::FieldReader<bool, EDGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDGE` writer - This bit is significant only when the START field contains 010-111. In these cases:"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGE_A::RISING)
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGE_A::FALLING)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 276) must be set to 0."]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 17) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects which of the AD7:0 pins is (are) to be sampled and converted. Bit 0 selects Pin AD0, bit 1 selects pin AD1,..., and bit 7 selects pin AD7. In software-controlled mode (BURST = 0), only one channel can be selected, i.e. only one of these bits should be 1. In hardware scan mode (BURST = 1), any numbers of channels can be selected, i.e any or all bits can be set to 1. If all bits are set to 0, channel 0 is selected automatically (SEL = 0x01)."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by CLKDIV +1 to produce the clock for the ADC, which should be less than or equal to 4.5 MHz. Typically, software should program the smallest value in this field that yields a clock of 4.5 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 16 - Burst mode If BURST is set to 1, the ADGINTEN bit in the INTEN register (Table 276) must be set to 0."]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bits 17:19 - This field selects the number of clocks used for each conversion in Burst mode, and the number of bits of accuracy of the result in the LS bits of ADDR, between 11 clocks (10 bits) and 4 clocks (3 bits)."]
    #[inline(always)]
    pub fn clks(&mut self) -> CLKS_W {
        CLKS_W { w: self }
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A/D Control Register. The CR register must be written to select the operating mode before A/D conversion can occur.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
