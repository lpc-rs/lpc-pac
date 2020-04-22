#[doc = "Reader of register RES"]
pub type R = crate::R<u32, super::RES>;
#[doc = "Writer for register RES"]
pub type W = crate::W<u32, super::RES>;
#[doc = "Register RES `reset()`'s with value 0"]
impl crate::ResetValue for super::RES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `O0RES`"]
pub type O0RES_R = crate::R<u8, O0RES_A>;
impl O0RES_R {
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
        *self == O0RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O0RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O0RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O0RES`"]
pub struct O0RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O0RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O0RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
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
#[doc = "Reader of field `O1RES`"]
pub type O1RES_R = crate::R<u8, O1RES_A>;
impl O1RES_R {
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
        *self == O1RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O1RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O1RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O1RES`"]
pub struct O1RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O1RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O1RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
#[doc = "Reader of field `O2RES`"]
pub type O2RES_R = crate::R<u8, O2RES_A>;
impl O2RES_R {
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
        *self == O2RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O2RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O2RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O2RES`"]
pub struct O2RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O2RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O2RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
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
#[doc = "Reader of field `O3RES`"]
pub type O3RES_R = crate::R<u8, O3RES_A>;
impl O3RES_R {
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
        *self == O3RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O3RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O3RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O3RES`"]
pub struct O3RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O3RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O3RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
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
#[doc = "Reader of field `O4RES`"]
pub type O4RES_R = crate::R<u8, O4RES_A>;
impl O4RES_R {
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
        *self == O4RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O4RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O4RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O4RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O4RES`"]
pub struct O4RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O4RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O4RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
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
#[doc = "Reader of field `O5RES`"]
pub type O5RES_R = crate::R<u8, O5RES_A>;
impl O5RES_R {
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
        *self == O5RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O5RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O5RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O5RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O5RES`"]
pub struct O5RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O5RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O5RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
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
#[doc = "Reader of field `O6RES`"]
pub type O6RES_R = crate::R<u8, O6RES_A>;
impl O6RES_R {
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
        *self == O6RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O6RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O6RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O6RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O6RES`"]
pub struct O6RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O6RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O6RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O7RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output n (or set based on the SETCLR7 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O7RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O7RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O7RES`"]
pub type O7RES_R = crate::R<u8, O7RES_A>;
impl O7RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O7RES_A {
        match self.bits {
            0 => O7RES_A::NO_CHANGE,
            1 => O7RES_A::SET,
            2 => O7RES_A::CLEAR,
            3 => O7RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O7RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O7RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O7RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O7RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O7RES`"]
pub struct O7RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O7RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O7RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O7RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR7 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O7RES_A::SET)
    }
    #[doc = "Clear output n (or set based on the SETCLR7 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O7RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O7RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O8RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR8 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O8RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O8RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O8RES`"]
pub type O8RES_R = crate::R<u8, O8RES_A>;
impl O8RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O8RES_A {
        match self.bits {
            0 => O8RES_A::NO_CHANGE,
            1 => O8RES_A::SET,
            2 => O8RES_A::CLEAR,
            3 => O8RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O8RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O8RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O8RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O8RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O8RES`"]
pub struct O8RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O8RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O8RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O8RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR8 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O8RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR8 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O8RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O8RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O9RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR9 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O9RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O9RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O9RES`"]
pub type O9RES_R = crate::R<u8, O9RES_A>;
impl O9RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O9RES_A {
        match self.bits {
            0 => O9RES_A::NO_CHANGE,
            1 => O9RES_A::SET,
            2 => O9RES_A::CLEAR,
            3 => O9RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O9RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O9RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O9RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O9RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O9RES`"]
pub struct O9RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O9RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O9RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O9RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR9 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O9RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR9 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O9RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O9RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O10RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR10 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O10RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O10RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O10RES`"]
pub type O10RES_R = crate::R<u8, O10RES_A>;
impl O10RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O10RES_A {
        match self.bits {
            0 => O10RES_A::NO_CHANGE,
            1 => O10RES_A::SET,
            2 => O10RES_A::CLEAR,
            3 => O10RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O10RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O10RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O10RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O10RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O10RES`"]
pub struct O10RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O10RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O10RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O10RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR10 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O10RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR10 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O10RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O10RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O11RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR11 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O11RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O11RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O11RES`"]
pub type O11RES_R = crate::R<u8, O11RES_A>;
impl O11RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O11RES_A {
        match self.bits {
            0 => O11RES_A::NO_CHANGE,
            1 => O11RES_A::SET,
            2 => O11RES_A::CLEAR,
            3 => O11RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O11RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O11RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O11RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O11RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O11RES`"]
pub struct O11RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O11RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O11RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O11RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR11 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O11RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR11 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O11RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O11RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O12RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR12 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O12RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O12RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O12RES`"]
pub type O12RES_R = crate::R<u8, O12RES_A>;
impl O12RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O12RES_A {
        match self.bits {
            0 => O12RES_A::NO_CHANGE,
            1 => O12RES_A::SET,
            2 => O12RES_A::CLEAR,
            3 => O12RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O12RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O12RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O12RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O12RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O12RES`"]
pub struct O12RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O12RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O12RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O12RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR12 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O12RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR12 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O12RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O12RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O13RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR13 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O13RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O13RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O13RES`"]
pub type O13RES_R = crate::R<u8, O13RES_A>;
impl O13RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O13RES_A {
        match self.bits {
            0 => O13RES_A::NO_CHANGE,
            1 => O13RES_A::SET,
            2 => O13RES_A::CLEAR,
            3 => O13RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O13RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O13RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O13RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O13RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O13RES`"]
pub struct O13RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O13RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O13RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O13RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR13 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O13RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR13 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O13RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O13RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O14RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR14 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O14RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O14RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O14RES`"]
pub type O14RES_R = crate::R<u8, O14RES_A>;
impl O14RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O14RES_A {
        match self.bits {
            0 => O14RES_A::NO_CHANGE,
            1 => O14RES_A::SET,
            2 => O14RES_A::CLEAR,
            3 => O14RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O14RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O14RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O14RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O14RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O14RES`"]
pub struct O14RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O14RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O14RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O14RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR14 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O14RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR14 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O14RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O14RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Effect of simultaneous set and clear on output 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum O15RES_A {
    #[doc = "0: No change."]
    NO_CHANGE = 0,
    #[doc = "1: Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    SET = 1,
    #[doc = "2: Clear output (or set based on the SETCLR15 field)."]
    CLEAR = 2,
    #[doc = "3: Toggle output."]
    TOGGLE_OUTPUT = 3,
}
impl From<O15RES_A> for u8 {
    #[inline(always)]
    fn from(variant: O15RES_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `O15RES`"]
pub type O15RES_R = crate::R<u8, O15RES_A>;
impl O15RES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> O15RES_A {
        match self.bits {
            0 => O15RES_A::NO_CHANGE,
            1 => O15RES_A::SET,
            2 => O15RES_A::CLEAR,
            3 => O15RES_A::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O15RES_A::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == O15RES_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == O15RES_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O15RES_A::TOGGLE_OUTPUT
    }
}
#[doc = "Write proxy for field `O15RES`"]
pub struct O15RES_W<'a> {
    w: &'a mut W,
}
impl<'a> O15RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O15RES_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O15RES_A::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR15 field in the OUTPUTDIRCTRL register)."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(O15RES_A::SET)
    }
    #[doc = "Clear output (or set based on the SETCLR15 field)."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(O15RES_A::CLEAR)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O15RES_A::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
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
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub fn o7res(&self) -> O7RES_R {
        O7RES_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub fn o8res(&self) -> O8RES_R {
        O8RES_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub fn o9res(&self) -> O9RES_R {
        O9RES_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub fn o10res(&self) -> O10RES_R {
        O10RES_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub fn o11res(&self) -> O11RES_R {
        O11RES_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub fn o12res(&self) -> O12RES_R {
        O12RES_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub fn o13res(&self) -> O13RES_R {
        O13RES_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub fn o14res(&self) -> O14RES_R {
        O14RES_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub fn o15res(&self) -> O15RES_R {
        O15RES_R::new(((self.bits >> 30) & 0x03) as u8)
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
    #[doc = "Bits 14:15 - Effect of simultaneous set and clear on output 7."]
    #[inline(always)]
    pub fn o7res(&mut self) -> O7RES_W {
        O7RES_W { w: self }
    }
    #[doc = "Bits 16:17 - Effect of simultaneous set and clear on output 8."]
    #[inline(always)]
    pub fn o8res(&mut self) -> O8RES_W {
        O8RES_W { w: self }
    }
    #[doc = "Bits 18:19 - Effect of simultaneous set and clear on output 9."]
    #[inline(always)]
    pub fn o9res(&mut self) -> O9RES_W {
        O9RES_W { w: self }
    }
    #[doc = "Bits 20:21 - Effect of simultaneous set and clear on output 10."]
    #[inline(always)]
    pub fn o10res(&mut self) -> O10RES_W {
        O10RES_W { w: self }
    }
    #[doc = "Bits 22:23 - Effect of simultaneous set and clear on output 11."]
    #[inline(always)]
    pub fn o11res(&mut self) -> O11RES_W {
        O11RES_W { w: self }
    }
    #[doc = "Bits 24:25 - Effect of simultaneous set and clear on output 12."]
    #[inline(always)]
    pub fn o12res(&mut self) -> O12RES_W {
        O12RES_W { w: self }
    }
    #[doc = "Bits 26:27 - Effect of simultaneous set and clear on output 13."]
    #[inline(always)]
    pub fn o13res(&mut self) -> O13RES_W {
        O13RES_W { w: self }
    }
    #[doc = "Bits 28:29 - Effect of simultaneous set and clear on output 14."]
    #[inline(always)]
    pub fn o14res(&mut self) -> O14RES_W {
        O14RES_W { w: self }
    }
    #[doc = "Bits 30:31 - Effect of simultaneous set and clear on output 15."]
    #[inline(always)]
    pub fn o15res(&mut self) -> O15RES_W {
        O15RES_W { w: self }
    }
}
