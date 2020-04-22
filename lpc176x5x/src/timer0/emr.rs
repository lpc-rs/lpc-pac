#[doc = "Reader of register EMR"]
pub type R = crate::R<u32, super::EMR>;
#[doc = "Writer for register EMR"]
pub type W = crate::W<u32, super::EMR>;
#[doc = "Register EMR `reset()`'s with value 0"]
impl crate::ResetValue for super::EMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM0`"]
pub type EM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM0`"]
pub struct EM0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM0_W<'a> {
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
#[doc = "Reader of field `EM1`"]
pub type EM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM1`"]
pub struct EM1_W<'a> {
    w: &'a mut W,
}
impl<'a> EM1_W<'a> {
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
#[doc = "Reader of field `EM2`"]
pub type EM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM2`"]
pub struct EM2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2_W<'a> {
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
#[doc = "Reader of field `EM3`"]
pub type EM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM3`"]
pub struct EM3_W<'a> {
    w: &'a mut W,
}
impl<'a> EM3_W<'a> {
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
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC0_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON = 3,
}
impl From<EMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMC0`"]
pub type EMC0_R = crate::R<u8, EMC0_A>;
impl EMC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC0_A {
        match self.bits {
            0 => EMC0_A::DO_NOTHING_,
            1 => EMC0_A::CLEAR_THE_CORRESPOND,
            2 => EMC0_A::SET_THE_CORRESPONDIN,
            3 => EMC0_A::TOGGLE_THE_CORRESPON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC0_A::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC0_A::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC0_A::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC0_A::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Write proxy for field `EMC0`"]
pub struct EMC0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC0_A::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC0_A::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC0_A::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC0_A::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC1_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON = 3,
}
impl From<EMC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMC1`"]
pub type EMC1_R = crate::R<u8, EMC1_A>;
impl EMC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC1_A {
        match self.bits {
            0 => EMC1_A::DO_NOTHING_,
            1 => EMC1_A::CLEAR_THE_CORRESPOND,
            2 => EMC1_A::SET_THE_CORRESPONDIN,
            3 => EMC1_A::TOGGLE_THE_CORRESPON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC1_A::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC1_A::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC1_A::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC1_A::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Write proxy for field `EMC1`"]
pub struct EMC1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC1_A::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC1_A::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC1_A::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC1_A::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC2_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON = 3,
}
impl From<EMC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMC2`"]
pub type EMC2_R = crate::R<u8, EMC2_A>;
impl EMC2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC2_A {
        match self.bits {
            0 => EMC2_A::DO_NOTHING_,
            1 => EMC2_A::CLEAR_THE_CORRESPOND,
            2 => EMC2_A::SET_THE_CORRESPONDIN,
            3 => EMC2_A::TOGGLE_THE_CORRESPON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC2_A::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC2_A::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC2_A::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC2_A::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Write proxy for field `EMC2`"]
pub struct EMC2_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC2_A::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC2_A::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC2_A::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC2_A::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC3_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    CLEAR_THE_CORRESPOND = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SET_THE_CORRESPONDIN = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    TOGGLE_THE_CORRESPON = 3,
}
impl From<EMC3_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EMC3`"]
pub type EMC3_R = crate::R<u8, EMC3_A>;
impl EMC3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC3_A {
        match self.bits {
            0 => EMC3_A::DO_NOTHING_,
            1 => EMC3_A::CLEAR_THE_CORRESPOND,
            2 => EMC3_A::SET_THE_CORRESPONDIN,
            3 => EMC3_A::TOGGLE_THE_CORRESPON,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING_`"]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == EMC3_A::DO_NOTHING_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == EMC3_A::CLEAR_THE_CORRESPOND
    }
    #[doc = "Checks if the value of the field is `SET_THE_CORRESPONDIN`"]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == EMC3_A::SET_THE_CORRESPONDIN
    }
    #[doc = "Checks if the value of the field is `TOGGLE_THE_CORRESPON`"]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == EMC3_A::TOGGLE_THE_CORRESPON
    }
}
#[doc = "Write proxy for field `EMC3`"]
pub struct EMC3_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut W {
        self.variant(EMC3_A::DO_NOTHING_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(EMC3_A::CLEAR_THE_CORRESPOND)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut W {
        self.variant(EMC3_A::SET_THE_CORRESPONDIN)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut W {
        self.variant(EMC3_A::TOGGLE_THE_CORRESPON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&self) -> EM3_R {
        EM3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> EMC0_R {
        EMC0_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> EMC1_R {
        EMC1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> EMC2_R {
        EMC2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> EMC3_R {
        EMC3_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W { w: self }
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W { w: self }
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W { w: self }
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&mut self) -> EM3_W {
        EM3_W { w: self }
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&mut self) -> EMC0_W {
        EMC0_W { w: self }
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&mut self) -> EMC1_W {
        EMC1_W { w: self }
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&mut self) -> EMC2_W {
        EMC2_W { w: self }
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&mut self) -> EMC3_W {
        EMC3_W { w: self }
    }
}
