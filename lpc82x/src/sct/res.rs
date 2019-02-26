#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RES {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `O0RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O0RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O0RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O0RESR::NO_CHANGE => 0,
            O0RESR::SET_OUTPUT_OR_CLEAR => 1,
            O0RESR::CLEAR_OUTPUT_OR_SET => 2,
            O0RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O0RESR {
        match value {
            0 => O0RESR::NO_CHANGE,
            1 => O0RESR::SET_OUTPUT_OR_CLEAR,
            2 => O0RESR::CLEAR_OUTPUT_OR_SET,
            3 => O0RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O0RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O0RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O0RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O1RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O1RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O1RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O1RESR::NO_CHANGE => 0,
            O1RESR::SET_OUTPUT_OR_CLEAR => 1,
            O1RESR::CLEAR_OUTPUT_OR_SET => 2,
            O1RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O1RESR {
        match value {
            0 => O1RESR::NO_CHANGE,
            1 => O1RESR::SET_OUTPUT_OR_CLEAR,
            2 => O1RESR::CLEAR_OUTPUT_OR_SET,
            3 => O1RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O1RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O1RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O1RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O2RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O2RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR_OUTPUT_N_OR_S,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O2RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O2RESR::NO_CHANGE => 0,
            O2RESR::SET_OUTPUT_OR_CLEAR => 1,
            O2RESR::CLEAR_OUTPUT_N_OR_S => 2,
            O2RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O2RESR {
        match value {
            0 => O2RESR::NO_CHANGE,
            1 => O2RESR::SET_OUTPUT_OR_CLEAR,
            2 => O2RESR::CLEAR_OUTPUT_N_OR_S,
            3 => O2RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O2RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O2RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_N_OR_S`"]
    #[inline]
    pub fn is_clear_output_n_or_s(&self) -> bool {
        *self == O2RESR::CLEAR_OUTPUT_N_OR_S
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O3RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O3RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O3RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O3RESR::NO_CHANGE => 0,
            O3RESR::SET_OUTPUT_OR_CLEAR => 1,
            O3RESR::CLEAR_OUTPUT_OR_SET => 2,
            O3RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O3RESR {
        match value {
            0 => O3RESR::NO_CHANGE,
            1 => O3RESR::SET_OUTPUT_OR_CLEAR,
            2 => O3RESR::CLEAR_OUTPUT_OR_SET,
            3 => O3RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O3RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O3RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O3RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O4RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O4RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O4RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O4RESR::NO_CHANGE => 0,
            O4RESR::SET_OUTPUT_OR_CLEAR => 1,
            O4RESR::CLEAR_OUTPUT_OR_SET => 2,
            O4RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O4RESR {
        match value {
            0 => O4RESR::NO_CHANGE,
            1 => O4RESR::SET_OUTPUT_OR_CLEAR,
            2 => O4RESR::CLEAR_OUTPUT_OR_SET,
            3 => O4RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O4RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O4RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O4RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O4RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Possible values of the field `O5RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum O5RESR {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O5RESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            O5RESR::NO_CHANGE => 0,
            O5RESR::SET_OUTPUT_OR_CLEAR => 1,
            O5RESR::CLEAR_OUTPUT_OR_SET => 2,
            O5RESR::TOGGLE_OUTPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> O5RESR {
        match value {
            0 => O5RESR::NO_CHANGE,
            1 => O5RESR::SET_OUTPUT_OR_CLEAR,
            2 => O5RESR::CLEAR_OUTPUT_OR_SET,
            3 => O5RESR::TOGGLE_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline]
    pub fn is_no_change(&self) -> bool {
        *self == O5RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O5RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O5RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline]
    pub fn is_toggle_output(&self) -> bool {
        *self == O5RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O0RES`"]
pub enum O0RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR0 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O0RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O0RESW::NO_CHANGE => 0,
            O0RESW::SET_OUTPUT_OR_CLEAR => 1,
            O0RESW::CLEAR_OUTPUT_OR_SET => 2,
            O0RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O0RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O0RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O0RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O0RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O0RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O1RES`"]
pub enum O1RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR1 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O1RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O1RESW::NO_CHANGE => 0,
            O1RESW::SET_OUTPUT_OR_CLEAR => 1,
            O1RESW::CLEAR_OUTPUT_OR_SET => 2,
            O1RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O1RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O1RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O1RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O1RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O1RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O2RES`"]
pub enum O2RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR2 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    CLEAR_OUTPUT_N_OR_S,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O2RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O2RESW::NO_CHANGE => 0,
            O2RESW::SET_OUTPUT_OR_CLEAR => 1,
            O2RESW::CLEAR_OUTPUT_N_OR_S => 2,
            O2RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O2RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O2RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O2RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O2RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline]
    pub fn clear_output_n_or_s(self) -> &'a mut W {
        self.variant(O2RESW::CLEAR_OUTPUT_N_OR_S)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O3RES`"]
pub enum O3RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR3 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O3RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O3RESW::NO_CHANGE => 0,
            O3RESW::SET_OUTPUT_OR_CLEAR => 1,
            O3RESW::CLEAR_OUTPUT_OR_SET => 2,
            O3RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O3RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O3RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O3RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O3RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O3RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O4RES`"]
pub enum O4RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR4 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O4RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O4RESW::NO_CHANGE => 0,
            O4RESW::SET_OUTPUT_OR_CLEAR => 1,
            O4RESW::CLEAR_OUTPUT_OR_SET => 2,
            O4RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O4RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O4RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O4RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O4RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR4 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O4RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR4 field)."]
    #[inline]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O4RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O4RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `O5RES`"]
pub enum O5RESW {
    #[doc = "No change."]
    NO_CHANGE,
    #[doc = "Set output (or clear based on the SETCLR5 field)."]
    SET_OUTPUT_OR_CLEAR,
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    CLEAR_OUTPUT_OR_SET,
    #[doc = "Toggle output."]
    TOGGLE_OUTPUT,
}
impl O5RESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            O5RESW::NO_CHANGE => 0,
            O5RESW::SET_OUTPUT_OR_CLEAR => 1,
            O5RESW::CLEAR_OUTPUT_OR_SET => 2,
            O5RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _O5RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O5RESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: O5RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O5RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR5 field)."]
    #[inline]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O5RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR5 field)."]
    #[inline]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O5RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O5RESW::TOGGLE_OUTPUT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline]
    pub fn o0res(&self) -> O0RESR {
        O0RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline]
    pub fn o1res(&self) -> O1RESR {
        O1RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline]
    pub fn o2res(&self) -> O2RESR {
        O2RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline]
    pub fn o3res(&self) -> O3RESR {
        O3RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline]
    pub fn o4res(&self) -> O4RESR {
        O4RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline]
    pub fn o5res(&self) -> O5RESR {
        O5RESR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline]
    pub fn o0res(&mut self) -> _O0RESW {
        _O0RESW { w: self }
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline]
    pub fn o1res(&mut self) -> _O1RESW {
        _O1RESW { w: self }
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline]
    pub fn o2res(&mut self) -> _O2RESW {
        _O2RESW { w: self }
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline]
    pub fn o3res(&mut self) -> _O3RESW {
        _O3RESW { w: self }
    }
    #[doc = "Bits 8:9 - Effect of simultaneous set and clear on output 4."]
    #[inline]
    pub fn o4res(&mut self) -> _O4RESW {
        _O4RESW { w: self }
    }
    #[doc = "Bits 10:11 - Effect of simultaneous set and clear on output 5."]
    #[inline]
    pub fn o5res(&mut self) -> _O5RESW {
        _O5RESW { w: self }
    }
}
