#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN0_A {
    #[doc = "0: Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN0_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN0`"]
pub type ADINTEN0_R = crate::R<bool, ADINTEN0_A>;
impl ADINTEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN0_A {
        match self.bits {
            false => ADINTEN0_A::DISABLE,
            true => ADINTEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN0_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN0`"]
pub struct ADINTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN0_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN1_A {
    #[doc = "0: Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN1_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN1`"]
pub type ADINTEN1_R = crate::R<bool, ADINTEN1_A>;
impl ADINTEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN1_A {
        match self.bits {
            false => ADINTEN1_A::DISABLE,
            true => ADINTEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN1_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN1`"]
pub struct ADINTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN1_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN2_A {
    #[doc = "0: Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN2_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN2`"]
pub type ADINTEN2_R = crate::R<bool, ADINTEN2_A>;
impl ADINTEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN2_A {
        match self.bits {
            false => ADINTEN2_A::DISABLE,
            true => ADINTEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN2_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN2`"]
pub struct ADINTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN2_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN2_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN3_A {
    #[doc = "0: Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN3_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN3`"]
pub type ADINTEN3_R = crate::R<bool, ADINTEN3_A>;
impl ADINTEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN3_A {
        match self.bits {
            false => ADINTEN3_A::DISABLE,
            true => ADINTEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN3_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN3`"]
pub struct ADINTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN3_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN3_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN4_A {
    #[doc = "0: Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN4_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN4`"]
pub type ADINTEN4_R = crate::R<bool, ADINTEN4_A>;
impl ADINTEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN4_A {
        match self.bits {
            false => ADINTEN4_A::DISABLE,
            true => ADINTEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN4_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN4`"]
pub struct ADINTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN4_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN4_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN5_A {
    #[doc = "0: Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN5_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN5`"]
pub type ADINTEN5_R = crate::R<bool, ADINTEN5_A>;
impl ADINTEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN5_A {
        match self.bits {
            false => ADINTEN5_A::DISABLE,
            true => ADINTEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN5_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN5`"]
pub struct ADINTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN5_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN5_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN6_A {
    #[doc = "0: Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN6_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN6`"]
pub type ADINTEN6_R = crate::R<bool, ADINTEN6_A>;
impl ADINTEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN6_A {
        match self.bits {
            false => ADINTEN6_A::DISABLE,
            true => ADINTEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN6_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN6`"]
pub struct ADINTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN6_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN6_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN7_A {
    #[doc = "0: Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE = 0,
    #[doc = "1: Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE = 1,
}
impl From<ADINTEN7_A> for bool {
    #[inline(always)]
    fn from(variant: ADINTEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADINTEN7`"]
pub type ADINTEN7_R = crate::R<bool, ADINTEN7_A>;
impl ADINTEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADINTEN7_A {
        match self.bits {
            false => ADINTEN7_A::DISABLE,
            true => ADINTEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN7_A::ENABLE
    }
}
#[doc = "Write proxy for field `ADINTEN7`"]
pub struct ADINTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADINTEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADINTEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN7_A::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN7_A::ENABLE)
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
#[doc = "Interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADGINTEN_A {
    #[doc = "0: Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS = 0,
    #[doc = "1: The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL = 1,
}
impl From<ADGINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADGINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADGINTEN`"]
pub type ADGINTEN_R = crate::R<bool, ADGINTEN_A>;
impl ADGINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADGINTEN_A {
        match self.bits {
            false => ADGINTEN_A::CHANNELS,
            true => ADGINTEN_A::GLOBAL,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS`"]
    #[inline(always)]
    pub fn is_channels(&self) -> bool {
        *self == ADGINTEN_A::CHANNELS
    }
    #[doc = "Checks if the value of the field is `GLOBAL`"]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == ADGINTEN_A::GLOBAL
    }
}
#[doc = "Write proxy for field `ADGINTEN`"]
pub struct ADGINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADGINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADGINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn channels(self) -> &'a mut W {
        self.variant(ADGINTEN_A::CHANNELS)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn global(self) -> &'a mut W {
        self.variant(ADGINTEN_A::GLOBAL)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&self) -> ADINTEN0_R {
        ADINTEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&self) -> ADINTEN1_R {
        ADINTEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&self) -> ADINTEN2_R {
        ADINTEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&self) -> ADINTEN3_R {
        ADINTEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&self) -> ADINTEN4_R {
        ADINTEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&self) -> ADINTEN5_R {
        ADINTEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&self) -> ADINTEN6_R {
        ADINTEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&self) -> ADINTEN7_R {
        ADINTEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&self) -> ADGINTEN_R {
        ADGINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&mut self) -> ADINTEN0_W {
        ADINTEN0_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&mut self) -> ADINTEN1_W {
        ADINTEN1_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&mut self) -> ADINTEN2_W {
        ADINTEN2_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&mut self) -> ADINTEN3_W {
        ADINTEN3_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&mut self) -> ADINTEN4_W {
        ADINTEN4_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&mut self) -> ADINTEN5_W {
        ADINTEN5_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&mut self) -> ADINTEN6_W {
        ADINTEN6_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&mut self) -> ADINTEN7_W {
        ADINTEN7_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&mut self) -> ADGINTEN_W {
        ADGINTEN_W { w: self }
    }
}
