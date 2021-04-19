#[doc = "Register `EMR` reader"]
pub struct R(crate::R<EMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EMR_SPEC>> for R {
    fn from(reader: crate::R<EMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR` writer"]
pub struct W(crate::W<EMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR_SPEC>;
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
impl core::convert::From<crate::W<EMR_SPEC>> for W {
    fn from(writer: crate::W<EMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM0` reader - External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
pub struct EM0_R(crate::FieldReader<bool, bool>);
impl EM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM0` writer - External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `EM1` reader - External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
pub struct EM1_R(crate::FieldReader<bool, bool>);
impl EM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM1` writer - External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `EM2` reader - External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
pub struct EM2_R(crate::FieldReader<bool, bool>);
impl EM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM2` writer - External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EM3` reader - External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
pub struct EM3_R(crate::FieldReader<bool, bool>);
impl EM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM3` writer - External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC0_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear. Clear the corresponding External Match bit/output to 0 (MAT0 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set. Set the corresponding External Match bit/output to 1 (MAT0 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC0_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC0` reader - External Match Control 0. Determines the functionality of External Match 0."]
pub struct EMC0_R(crate::FieldReader<u8, EMC0_A>);
impl EMC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC0_A {
        match self.bits {
            0 => EMC0_A::DO_NOTHING,
            1 => EMC0_A::CLEAR,
            2 => EMC0_A::SET,
            3 => EMC0_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        **self == EMC0_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == EMC0_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == EMC0_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == EMC0_A::TOGGLE
    }
}
impl core::ops::Deref for EMC0_R {
    type Target = crate::FieldReader<u8, EMC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC0` writer - External Match Control 0. Determines the functionality of External Match 0."]
pub struct EMC0_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC0_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC0_A::DO_NOTHING)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT0 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC0_A::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT0 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC0_A::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC0_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC1_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear. Clear the corresponding External Match bit/output to 0 (MAT1 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set. Set the corresponding External Match bit/output to 1 (MAT1 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC1_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC1` reader - External Match Control 1. Determines the functionality of External Match 1."]
pub struct EMC1_R(crate::FieldReader<u8, EMC1_A>);
impl EMC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC1_A {
        match self.bits {
            0 => EMC1_A::DO_NOTHING,
            1 => EMC1_A::CLEAR,
            2 => EMC1_A::SET,
            3 => EMC1_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        **self == EMC1_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == EMC1_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == EMC1_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == EMC1_A::TOGGLE
    }
}
impl core::ops::Deref for EMC1_R {
    type Target = crate::FieldReader<u8, EMC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC1` writer - External Match Control 1. Determines the functionality of External Match 1."]
pub struct EMC1_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC1_A::DO_NOTHING)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT1 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC1_A::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT1 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC1_A::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC1_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC2_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear. Clear the corresponding External Match bit/output to 0 (MAT2 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set. Set the corresponding External Match bit/output to 1 (MAT2 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC2_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC2` reader - External Match Control 2. Determines the functionality of External Match 2."]
pub struct EMC2_R(crate::FieldReader<u8, EMC2_A>);
impl EMC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMC2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC2_A {
        match self.bits {
            0 => EMC2_A::DO_NOTHING,
            1 => EMC2_A::CLEAR,
            2 => EMC2_A::SET,
            3 => EMC2_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        **self == EMC2_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == EMC2_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == EMC2_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == EMC2_A::TOGGLE
    }
}
impl core::ops::Deref for EMC2_R {
    type Target = crate::FieldReader<u8, EMC2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC2` writer - External Match Control 2. Determines the functionality of External Match 2."]
pub struct EMC2_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC2_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC2_A::DO_NOTHING)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT2 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC2_A::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT2 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC2_A::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC2_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EMC3_A {
    #[doc = "0: Do Nothing."]
    DO_NOTHING = 0,
    #[doc = "1: Clear. Clear the corresponding External Match bit/output to 0 (MAT3 pin is LOW if pinned out)."]
    CLEAR = 1,
    #[doc = "2: Set. Set the corresponding External Match bit/output to 1 (MAT3 pin is HIGH if pinned out)."]
    SET = 2,
    #[doc = "3: Toggle. Toggle the corresponding External Match bit/output."]
    TOGGLE = 3,
}
impl From<EMC3_A> for u8 {
    #[inline(always)]
    fn from(variant: EMC3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EMC3` reader - External Match Control 3. Determines the functionality of External Match 3."]
pub struct EMC3_R(crate::FieldReader<u8, EMC3_A>);
impl EMC3_R {
    pub(crate) fn new(bits: u8) -> Self {
        EMC3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EMC3_A {
        match self.bits {
            0 => EMC3_A::DO_NOTHING,
            1 => EMC3_A::CLEAR,
            2 => EMC3_A::SET,
            3 => EMC3_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DO_NOTHING`"]
    #[inline(always)]
    pub fn is_do_nothing(&self) -> bool {
        **self == EMC3_A::DO_NOTHING
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == EMC3_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == EMC3_A::SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == EMC3_A::TOGGLE
    }
}
impl core::ops::Deref for EMC3_R {
    type Target = crate::FieldReader<u8, EMC3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMC3` writer - External Match Control 3. Determines the functionality of External Match 3."]
pub struct EMC3_W<'a> {
    w: &'a mut W,
}
impl<'a> EMC3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMC3_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut W {
        self.variant(EMC3_A::DO_NOTHING)
    }
    #[doc = "Clear. Clear the corresponding External Match bit/output to 0 (MAT3 pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EMC3_A::CLEAR)
    }
    #[doc = "Set. Set the corresponding External Match bit/output to 1 (MAT3 pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(EMC3_A::SET)
    }
    #[doc = "Toggle. Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(EMC3_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em0(&self) -> EM0_R {
        EM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em1(&self) -> EM1_R {
        EM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em2(&self) -> EM2_R {
        EM2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
    #[doc = "Bit 0 - External Match 0. This bit reflects the state of output MAT0, whether or not this output is connected to a pin. When a match occurs between the TC and MR0, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[5:4\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em0(&mut self) -> EM0_W {
        EM0_W { w: self }
    }
    #[doc = "Bit 1 - External Match 1. This bit reflects the state of output MAT1, whether or not this output is connected to a pin. When a match occurs between the TC and MR1, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[7:6\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em1(&mut self) -> EM1_W {
        EM1_W { w: self }
    }
    #[doc = "Bit 2 - External Match 2. This bit reflects the state of output MAT2, whether or not this output is connected to a pin. When a match occurs between the TC and MR2, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by EMR\\[9:8\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
    #[inline(always)]
    pub fn em2(&mut self) -> EM2_W {
        EM2_W { w: self }
    }
    #[doc = "Bit 3 - External Match 3. This bit reflects the state of output MAT3, whether or not this output is connected to a pin. When a match occurs between the TC and MR3, this bit can either toggle, go LOW, go HIGH, or do nothing, as selected by MR\\[11:10\\]. This bit is driven to the MAT pins if the match function is selected via IOCON. 0 = LOW. 1 = HIGH."]
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
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Match Register. The EMR controls the match function and the external match pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr](index.html) module"]
pub struct EMR_SPEC;
impl crate::RegisterSpec for EMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr::R](R) reader structure"]
impl crate::Readable for EMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr::W](W) writer structure"]
impl crate::Writable for EMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
