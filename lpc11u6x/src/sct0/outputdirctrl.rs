#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUTDIRCTRL {
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
#[doc = "Possible values of the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl crate::ToBits<u8> for SETCLR0R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR0R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR0R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR0R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR0_R = crate::FR<u8, SETCLR0R>;
impl SETCLR0_R {
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline(always)]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_l(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_h(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H
    }
}
#[doc = "Values that can be written to the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl SETCLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR0W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR0W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR0W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline(always)]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_l(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_h(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl crate::ToBits<u8> for SETCLR1R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR1R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR1R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR1R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR1_R = crate::FR<u8, SETCLR1R>;
impl SETCLR1_R {
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline(always)]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_l(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_h(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H
    }
}
#[doc = "Values that can be written to the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl SETCLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR1W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR1W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR1W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline(always)]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_l(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_h(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl crate::ToBits<u8> for SETCLR2R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR2R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR2R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR2R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR2_R = crate::FR<u8, SETCLR2R>;
impl SETCLR2_R {
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline(always)]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_l(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_h(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H
    }
}
#[doc = "Values that can be written to the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl SETCLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR2W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR2W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR2W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline(always)]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_l(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_h(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl crate::ToBits<u8> for SETCLR3R {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SETCLR3R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR3R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR3R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SETCLR3_R = crate::FR<u8, SETCLR3R>;
impl SETCLR3_R {
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline(always)]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_l(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H`"]
    #[inline(always)]
    pub fn is_set_and_clear_are_reversed_when_counter_h(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H
    }
}
#[doc = "Values that can be written to the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H,
}
impl SETCLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR3W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR3W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L => 1,
            SETCLR3W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SETCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETCLR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline(always)]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_l(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_L)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline(always)]
    pub fn set_and_clear_are_reversed_when_counter_h(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_ARE_REVERSED_WHEN_COUNTER_H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&self) -> SETCLR0_R {
        SETCLR0_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&self) -> SETCLR1_R {
        SETCLR1_R::new(((self.bits() >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&self) -> SETCLR2_R {
        SETCLR2_R::new(((self.bits() >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&self) -> SETCLR3_R {
        SETCLR3_R::new(((self.bits() >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr0(&mut self) -> _SETCLR0W {
        _SETCLR0W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr1(&mut self) -> _SETCLR1W {
        _SETCLR1W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr2(&mut self) -> _SETCLR2W {
        _SETCLR2W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline(always)]
    pub fn setclr3(&mut self) -> _SETCLR3W {
        _SETCLR3W { w: self }
    }
}
