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
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BURST_A {
    #[doc = "1: The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    BURST = 1,
    #[doc = "0: Conversions are software controlled and require 31 clocks."]
    SW = 0,
}
impl From<BURST_A> for bool {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<bool, BURST_A>;
impl BURST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            true => BURST_A::BURST,
            false => BURST_A::SW,
        }
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == BURST_A::BURST
    }
    #[doc = "Checks if the value of the field is `SW`"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == BURST_A::SW
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(BURST_A::BURST)
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut W {
        self.variant(BURST_A::SW)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Power down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDN_A {
    #[doc = "1: The A/D converter is operational."]
    POWERED = 1,
    #[doc = "0: The A/D converter is in power-down mode."]
    POWERDOWN = 0,
}
impl From<PDN_A> for bool {
    #[inline(always)]
    fn from(variant: PDN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDN`"]
pub type PDN_R = crate::R<bool, PDN_A>;
impl PDN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDN_A {
        match self.bits {
            true => PDN_A::POWERED,
            false => PDN_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == PDN_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == PDN_A::POWERDOWN
    }
}
#[doc = "Write proxy for field `PDN`"]
pub struct PDN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(PDN_A::POWERED)
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PDN_A::POWERDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
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
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\]
pin."]
    P2_10 = 2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\]
pin."]
    P1_27 = 3,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    MAT0_1 = 4,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    MAT0_3 = 5,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    MAT1_0 = 6,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    MAT1_1 = 7,
}
impl From<START_A> for u8 {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u8, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            0 => START_A::NO_START_THIS_VALUE,
            1 => START_A::START_CONVERSION_NOW,
            2 => START_A::P2_10,
            3 => START_A::P1_27,
            4 => START_A::MAT0_1,
            5 => START_A::MAT0_3,
            6 => START_A::MAT1_0,
            7 => START_A::MAT1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_START_THIS_VALUE`"]
    #[inline(always)]
    pub fn is_no_start_this_value(&self) -> bool {
        *self == START_A::NO_START_THIS_VALUE
    }
    #[doc = "Checks if the value of the field is `START_CONVERSION_NOW`"]
    #[inline(always)]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == START_A::START_CONVERSION_NOW
    }
    #[doc = "Checks if the value of the field is `P2_10`"]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == START_A::P2_10
    }
    #[doc = "Checks if the value of the field is `P1_27`"]
    #[inline(always)]
    pub fn is_p1_27(&self) -> bool {
        *self == START_A::P1_27
    }
    #[doc = "Checks if the value of the field is `MAT0_1`"]
    #[inline(always)]
    pub fn is_mat0_1(&self) -> bool {
        *self == START_A::MAT0_1
    }
    #[doc = "Checks if the value of the field is `MAT0_3`"]
    #[inline(always)]
    pub fn is_mat0_3(&self) -> bool {
        *self == START_A::MAT0_3
    }
    #[doc = "Checks if the value of the field is `MAT1_0`"]
    #[inline(always)]
    pub fn is_mat1_0(&self) -> bool {
        *self == START_A::MAT1_0
    }
    #[doc = "Checks if the value of the field is `MAT1_1`"]
    #[inline(always)]
    pub fn is_mat1_1(&self) -> bool {
        *self == START_A::MAT1_1
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\]
pin."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut W {
        self.variant(START_A::P2_10)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\]
pin."]
    #[inline(always)]
    pub fn p1_27(self) -> &'a mut W {
        self.variant(START_A::P1_27)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat0_1(self) -> &'a mut W {
        self.variant(START_A::MAT0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn mat0_3(self) -> &'a mut W {
        self.variant(START_A::MAT0_3)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_0(self) -> &'a mut W {
        self.variant(START_A::MAT1_0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_1(self) -> &'a mut W {
        self.variant(START_A::MAT1_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "1: Start conversion on a falling edge on the selected CAP/MAT signal."]
    FALLLING = 1,
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    RISING = 0,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, EDGE_A>;
impl EDGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            true => EDGE_A::FALLLING,
            false => EDGE_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLLING`"]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == EDGE_A::FALLLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGE_A::RISING
    }
}
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDGE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut W {
        self.variant(EDGE_A::FALLLING)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGE_A::RISING)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&self) -> PDN_R {
        PDN_R::new(((self.bits >> 21) & 0x01) != 0)
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
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\]
pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&mut self) -> PDN_W {
        PDN_W { w: self }
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
}
