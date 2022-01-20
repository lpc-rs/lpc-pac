#[doc = "Register `RES` reader"]
pub struct R(crate::R<RES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RES` writer"]
pub struct W(crate::W<RES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RES_SPEC>;
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
impl From<crate::W<RES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Effect of simultaneous set and clear on output 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O0RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR0 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O0RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O0RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O0RES` reader - Effect of simultaneous set and clear on output 0."]
pub struct O0RES_R(crate::FieldReader<u8, O0RES_A>);
impl O0RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O0RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O0RES_A {
        match self.bits {
            0 => O0RES_A::NO_CHANGE,
            1 => O0RES_A::SET,
            2 => O0RES_A::CLEAR,
            3 => O0RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O0RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O0RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O0RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O0RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O0RES_R {
    type Target = crate::FieldReader<u8, O0RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O0RES` writer - Effect of simultaneous set and clear on output 0."]
pub struct O0RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O0RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O0RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O0RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O0RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O1RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR1 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O1RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O1RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O1RES` reader - Effect of simultaneous set and clear on output 1."]
pub struct O1RES_R(crate::FieldReader<u8, O1RES_A>);
impl O1RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O1RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O1RES_A {
        match self.bits {
            0 => O1RES_A::NO_CHANGE,
            1 => O1RES_A::SET,
            2 => O1RES_A::CLEAR,
            3 => O1RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O1RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O1RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O1RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O1RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O1RES_R {
    type Target = crate::FieldReader<u8, O1RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O1RES` writer - Effect of simultaneous set and clear on output 1."]
pub struct O1RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O1RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O1RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O1RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O1RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O2RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output n (or set based on the SETCLR2 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O2RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O2RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O2RES` reader - Effect of simultaneous set and clear on output 2."]
pub struct O2RES_R(crate::FieldReader<u8, O2RES_A>);
impl O2RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O2RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O2RES_A {
        match self.bits {
            0 => O2RES_A::NO_CHANGE,
            1 => O2RES_A::SET,
            2 => O2RES_A::CLEAR,
            3 => O2RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O2RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O2RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O2RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O2RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O2RES_R {
    type Target = crate::FieldReader<u8, O2RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O2RES` writer - Effect of simultaneous set and clear on output 2."]
pub struct O2RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O2RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O2RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O2RES_A::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O2RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O3RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR3 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O3RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O3RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O3RES` reader - Effect of simultaneous set and clear on output 3."]
pub struct O3RES_R(crate::FieldReader<u8, O3RES_A>);
impl O3RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O3RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O3RES_A {
        match self.bits {
            0 => O3RES_A::NO_CHANGE,
            1 => O3RES_A::SET,
            2 => O3RES_A::CLEAR,
            3 => O3RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O3RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O3RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O3RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O3RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O3RES_R {
    type Target = crate::FieldReader<u8, O3RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O3RES` writer - Effect of simultaneous set and clear on output 3."]
pub struct O3RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O3RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O3RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O3RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O3RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O4RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR4 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O4RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O4RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O4RES` reader - Effect of simultaneous set and clear on output 4."]
pub struct O4RES_R(crate::FieldReader<u8, O4RES_A>);
impl O4RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O4RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O4RES_A {
        match self.bits {
            0 => O4RES_A::NO_CHANGE,
            1 => O4RES_A::SET,
            2 => O4RES_A::CLEAR,
            3 => O4RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O4RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O4RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O4RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O4RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O4RES_R {
    type Target = crate::FieldReader<u8, O4RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O4RES` writer - Effect of simultaneous set and clear on output 4."]
pub struct O4RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O4RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O4RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O4RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR4 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O4RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O4RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O4RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O5RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR5 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O5RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O5RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O5RES` reader - Effect of simultaneous set and clear on output 5."]
pub struct O5RES_R(crate::FieldReader<u8, O5RES_A>);
impl O5RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O5RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O5RES_A {
        match self.bits {
            0 => O5RES_A::NO_CHANGE,
            1 => O5RES_A::SET,
            2 => O5RES_A::CLEAR,
            3 => O5RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O5RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O5RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O5RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O5RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O5RES_R {
    type Target = crate::FieldReader<u8, O5RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O5RES` writer - Effect of simultaneous set and clear on output 5."]
pub struct O5RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O5RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O5RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O5RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR5 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O5RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O5RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O5RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O6RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR6 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O6RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O6RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `O6RES` reader - Effect of simultaneous set and clear on output 6."]
pub struct O6RES_R(crate::FieldReader<u8, O6RES_A>);
impl O6RES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        O6RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O6RES_A {
        match self.bits {
            0 => O6RES_A::NO_CHANGE,
            1 => O6RES_A::SET,
            2 => O6RES_A::CLEAR,
            3 => O6RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        **self == O6RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == O6RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == O6RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        **self == O6RES_A::TOGGLE_OUTPUT
    }
}
impl core::ops::Deref for O6RES_R {
    type Target = crate::FieldReader<u8, O6RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O6RES` writer - Effect of simultaneous set and clear on output 6."]
pub struct O6RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O6RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O6RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O6RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR6 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O6RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR6 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O6RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O6RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&self) -> O0RES_R {
        O0RES_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&self) -> O1RES_R {
        O1RES_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&self) -> O2RES_R {
        O2RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&self) -> O3RES_R {
        O3RES_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub fn o4res(&self) -> O4RES_R {
        O4RES_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub fn o5res(&self) -> O5RES_R {
        O5RES_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub fn o6res(&self) -> O6RES_R {
        O6RES_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&mut self) -> O0RES_W {
        O0RES_W { w: self }
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&mut self) -> O1RES_W {
        O1RES_W { w: self }
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&mut self) -> O2RES_W {
        O2RES_W { w: self }
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&mut self) -> O3RES_W {
        O3RES_W { w: self }
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline(always)]
    pub fn o4res(&mut self) -> O4RES_W {
        O4RES_W { w: self }
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline(always)]
    pub fn o5res(&mut self) -> O5RES_W {
        O5RES_W { w: self }
    }
    #[doc = "Bits 12:13 - Effect of simultaneous set and clear on output 6."]
    #[inline(always)]
    pub fn o6res(&mut self) -> O6RES_W {
        O6RES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT conflict resolution register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](index.html) module"]
pub struct RES_SPEC;
impl crate::RegisterSpec for RES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [res::R](R) reader structure"]
impl crate::Readable for RES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [res::W](W) writer structure"]
impl crate::Writable for RES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RES to value 0"]
impl crate::Resettable for RES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
