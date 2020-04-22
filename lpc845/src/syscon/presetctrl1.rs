#[doc = "Reader of register PRESETCTRL1"]
pub type R = crate::R<u32, super::PRESETCTRL1>;
#[doc = "Writer for register PRESETCTRL1"]
pub type W = crate::W<u32, super::PRESETCTRL1>;
#[doc = "Register PRESETCTRL1 `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PRESETCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Capacitive touch reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAPT_RST_N_A {
    #[doc = "0: Assert the capacitive touch reset."]
    ASSERT = 0,
    #[doc = "1: Clear the capacitive touch reset."]
    CLEAR = 1,
}
impl From<CAPT_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: CAPT_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CAPT_RST_N`"]
pub type CAPT_RST_N_R = crate::R<bool, CAPT_RST_N_A>;
impl CAPT_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPT_RST_N_A {
        match self.bits {
            false => CAPT_RST_N_A::ASSERT,
            true => CAPT_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == CAPT_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CAPT_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `CAPT_RST_N`"]
pub struct CAPT_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPT_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPT_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the capacitive touch reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(CAPT_RST_N_A::ASSERT)
    }
    #[doc = "Clear the capacitive touch reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAPT_RST_N_A::CLEAR)
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
#[doc = "DAC1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_RST_N_A {
    #[doc = "0: Assert the DAC1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the DAC1 reset."]
    CLEAR = 1,
}
impl From<DAC1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DAC1_RST_N`"]
pub type DAC1_RST_N_R = crate::R<bool, DAC1_RST_N_A>;
impl DAC1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_RST_N_A {
        match self.bits {
            false => DAC1_RST_N_A::ASSERT,
            true => DAC1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == DAC1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == DAC1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `DAC1_RST_N`"]
pub struct DAC1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the DAC1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(DAC1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the DAC1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DAC1_RST_N_A::CLEAR)
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
#[doc = "Fractional baud rate generator 0 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG0_RST_N_A {
    #[doc = "0: Assert the FRG0 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the FRG0 reset."]
    CLEAR = 1,
}
impl From<FRG0_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FRG0_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRG0_RST_N`"]
pub type FRG0_RST_N_R = crate::R<bool, FRG0_RST_N_A>;
impl FRG0_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRG0_RST_N_A {
        match self.bits {
            false => FRG0_RST_N_A::ASSERT,
            true => FRG0_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == FRG0_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FRG0_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `FRG0_RST_N`"]
pub struct FRG0_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FRG0_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRG0_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the FRG0 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG0_RST_N_A::ASSERT)
    }
    #[doc = "Clear the FRG0 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG0_RST_N_A::CLEAR)
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
#[doc = "Fractional baud rate generator 1 reset control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG1_RST_N_A {
    #[doc = "0: Assert the FRG1 reset."]
    ASSERT = 0,
    #[doc = "1: Clear the FRG1 reset."]
    CLEAR = 1,
}
impl From<FRG1_RST_N_A> for bool {
    #[inline(always)]
    fn from(variant: FRG1_RST_N_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRG1_RST_N`"]
pub type FRG1_RST_N_R = crate::R<bool, FRG1_RST_N_A>;
impl FRG1_RST_N_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRG1_RST_N_A {
        match self.bits {
            false => FRG1_RST_N_A::ASSERT,
            true => FRG1_RST_N_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        *self == FRG1_RST_N_A::ASSERT
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == FRG1_RST_N_A::CLEAR
    }
}
#[doc = "Write proxy for field `FRG1_RST_N`"]
pub struct FRG1_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FRG1_RST_N_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRG1_RST_N_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Assert the FRG1 reset."]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(FRG1_RST_N_A::ASSERT)
    }
    #[doc = "Clear the FRG1 reset."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FRG1_RST_N_A::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline(always)]
    pub fn capt_rst_n(&self) -> CAPT_RST_N_R {
        CAPT_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline(always)]
    pub fn dac1_rst_n(&self) -> DAC1_RST_N_R {
        DAC1_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline(always)]
    pub fn frg0_rst_n(&self) -> FRG0_RST_N_R {
        FRG0_RST_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline(always)]
    pub fn frg1_rst_n(&self) -> FRG1_RST_N_R {
        FRG1_RST_N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capacitive touch reset control"]
    #[inline(always)]
    pub fn capt_rst_n(&mut self) -> CAPT_RST_N_W {
        CAPT_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - DAC1 reset control"]
    #[inline(always)]
    pub fn dac1_rst_n(&mut self) -> DAC1_RST_N_W {
        DAC1_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - Fractional baud rate generator 0 reset control"]
    #[inline(always)]
    pub fn frg0_rst_n(&mut self) -> FRG0_RST_N_W {
        FRG0_RST_N_W { w: self }
    }
    #[doc = "Bit 4 - Fractional baud rate generator 1 reset control"]
    #[inline(always)]
    pub fn frg1_rst_n(&mut self) -> FRG1_RST_N_W {
        FRG1_RST_N_W { w: self }
    }
}
