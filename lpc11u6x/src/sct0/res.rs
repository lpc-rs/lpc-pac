#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RES {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
impl crate::ToBits<u8> for O0RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O0RESR::NO_CHANGE => 0,
            O0RESR::SET_OUTPUT_OR_CLEAR => 1,
            O0RESR::CLEAR_OUTPUT_OR_SET => 2,
            O0RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O0RES_R = crate::FR<u8, O0RESR>;
impl O0RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O0RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline(always)]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O0RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline(always)]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O0RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O0RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O0RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O0RESW::NO_CHANGE => 0,
            O0RESW::SET_OUTPUT_OR_CLEAR => 1,
            O0RESW::CLEAR_OUTPUT_OR_SET => 2,
            O0RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O0RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O0RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O0RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O0RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O0RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR0 field)."]
    #[inline(always)]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O0RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O0RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
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
impl crate::ToBits<u8> for O1RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O1RESR::NO_CHANGE => 0,
            O1RESR::SET_OUTPUT_OR_CLEAR => 1,
            O1RESR::CLEAR_OUTPUT_OR_SET => 2,
            O1RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O1RES_R = crate::FR<u8, O1RESR>;
impl O1RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O1RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline(always)]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O1RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline(always)]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O1RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O1RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O1RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O1RESW::NO_CHANGE => 0,
            O1RESW::SET_OUTPUT_OR_CLEAR => 1,
            O1RESW::CLEAR_OUTPUT_OR_SET => 2,
            O1RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O1RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O1RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O1RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O1RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O1RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR1 field)."]
    #[inline(always)]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O1RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O1RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
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
impl crate::ToBits<u8> for O2RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O2RESR::NO_CHANGE => 0,
            O2RESR::SET_OUTPUT_OR_CLEAR => 1,
            O2RESR::CLEAR_OUTPUT_N_OR_S => 2,
            O2RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O2RES_R = crate::FR<u8, O2RESR>;
impl O2RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O2RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline(always)]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O2RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_N_OR_S`"]
    #[inline(always)]
    pub fn is_clear_output_n_or_s(&self) -> bool {
        *self == O2RESR::CLEAR_OUTPUT_N_OR_S
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O2RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O2RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O2RESW::NO_CHANGE => 0,
            O2RESW::SET_OUTPUT_OR_CLEAR => 1,
            O2RESW::CLEAR_OUTPUT_N_OR_S => 2,
            O2RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O2RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O2RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O2RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O2RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O2RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output n (or set based on the SETCLR2 field)."]
    #[inline(always)]
    pub fn clear_output_n_or_s(self) -> &'a mut W {
        self.variant(O2RESW::CLEAR_OUTPUT_N_OR_S)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O2RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
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
impl crate::ToBits<u8> for O3RESR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            O3RESR::NO_CHANGE => 0,
            O3RESR::SET_OUTPUT_OR_CLEAR => 1,
            O3RESR::CLEAR_OUTPUT_OR_SET => 2,
            O3RESR::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type O3RES_R = crate::FR<u8, O3RESR>;
impl O3RES_R {
    #[doc = "Checks if the value of the field is `NO_CHANGE`"]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == O3RESR::NO_CHANGE
    }
    #[doc = "Checks if the value of the field is `SET_OUTPUT_OR_CLEAR`"]
    #[inline(always)]
    pub fn is_set_output_or_clear(&self) -> bool {
        *self == O3RESR::SET_OUTPUT_OR_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR_OUTPUT_OR_SET`"]
    #[inline(always)]
    pub fn is_clear_output_or_set(&self) -> bool {
        *self == O3RESR::CLEAR_OUTPUT_OR_SET
    }
    #[doc = "Checks if the value of the field is `TOGGLE_OUTPUT`"]
    #[inline(always)]
    pub fn is_toggle_output(&self) -> bool {
        *self == O3RESR::TOGGLE_OUTPUT
    }
}
#[doc = "Values that can be written to the field `O3RES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            O3RESW::NO_CHANGE => 0,
            O3RESW::SET_OUTPUT_OR_CLEAR => 1,
            O3RESW::CLEAR_OUTPUT_OR_SET => 2,
            O3RESW::TOGGLE_OUTPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _O3RESW<'a> {
    w: &'a mut W,
}
impl<'a> _O3RESW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: O3RESW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut W {
        self.variant(O3RESW::NO_CHANGE)
    }
    #[doc = "Set output (or clear based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn set_output_or_clear(self) -> &'a mut W {
        self.variant(O3RESW::SET_OUTPUT_OR_CLEAR)
    }
    #[doc = "Clear output (or set based on the SETCLR3 field)."]
    #[inline(always)]
    pub fn clear_output_or_set(self) -> &'a mut W {
        self.variant(O3RESW::CLEAR_OUTPUT_OR_SET)
    }
    #[doc = "Toggle output."]
    #[inline(always)]
    pub fn toggle_output(self) -> &'a mut W {
        self.variant(O3RESW::TOGGLE_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&self) -> O0RES_R {
        O0RES_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&self) -> O1RES_R {
        O1RES_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&self) -> O2RES_R {
        O2RES_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&self) -> O3RES_R {
        O3RES_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Effect of simultaneous set and clear on output 0."]
    #[inline(always)]
    pub fn o0res(&mut self) -> _O0RESW {
        _O0RESW { w: self }
    }
    #[doc = "Bits 2:3 - Effect of simultaneous set and clear on output 1."]
    #[inline(always)]
    pub fn o1res(&mut self) -> _O1RESW {
        _O1RESW { w: self }
    }
    #[doc = "Bits 4:5 - Effect of simultaneous set and clear on output 2."]
    #[inline(always)]
    pub fn o2res(&mut self) -> _O2RESW {
        _O2RESW { w: self }
    }
    #[doc = "Bits 6:7 - Effect of simultaneous set and clear on output 3."]
    #[inline(always)]
    pub fn o3res(&mut self) -> _O3RESW {
        _O3RESW { w: self }
    }
}
