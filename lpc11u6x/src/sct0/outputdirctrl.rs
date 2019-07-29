#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUTPUTDIRCTRL {
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
#[doc = "Possible values of the field `SETCLR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR0R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR0R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR0R::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR0R::SET_AND_CLEAR_ARE_RE => 2,
            SETCLR0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR0R {
        match value {
            0 => SETCLR0R::SET_AND_CLEAR_DO_NOT,
            1 => SETCLR0R::SET_AND_CLEAR_ARE_RE,
            2 => SETCLR0R::SET_AND_CLEAR_ARE_RE,
            i => SETCLR0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_ARE_RE
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR0R::SET_AND_CLEAR_ARE_RE
    }
}
#[doc = "Possible values of the field `SETCLR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR1R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR1R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR1R::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR1R::SET_AND_CLEAR_ARE_RE => 2,
            SETCLR1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR1R {
        match value {
            0 => SETCLR1R::SET_AND_CLEAR_DO_NOT,
            1 => SETCLR1R::SET_AND_CLEAR_ARE_RE,
            2 => SETCLR1R::SET_AND_CLEAR_ARE_RE,
            i => SETCLR1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_ARE_RE
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR1R::SET_AND_CLEAR_ARE_RE
    }
}
#[doc = "Possible values of the field `SETCLR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR2R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR2R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR2R::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR2R::SET_AND_CLEAR_ARE_RE => 2,
            SETCLR2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR2R {
        match value {
            0 => SETCLR2R::SET_AND_CLEAR_DO_NOT,
            1 => SETCLR2R::SET_AND_CLEAR_ARE_RE,
            2 => SETCLR2R::SET_AND_CLEAR_ARE_RE,
            i => SETCLR2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_ARE_RE
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR2R::SET_AND_CLEAR_ARE_RE
    }
}
#[doc = "Possible values of the field `SETCLR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SETCLR3R {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SETCLR3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SETCLR3R::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR3R::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR3R::SET_AND_CLEAR_ARE_RE => 2,
            SETCLR3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SETCLR3R {
        match value {
            0 => SETCLR3R::SET_AND_CLEAR_DO_NOT,
            1 => SETCLR3R::SET_AND_CLEAR_ARE_RE,
            2 => SETCLR3R::SET_AND_CLEAR_ARE_RE,
            i => SETCLR3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_DO_NOT`"]
    #[inline]
    pub fn is_set_and_clear_do_not(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_DO_NOT
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_ARE_RE
    }
    #[doc = "Checks if the value of the field is `SET_AND_CLEAR_ARE_RE`"]
    #[inline]
    pub fn is_set_and_clear_are_re(&self) -> bool {
        *self == SETCLR3R::SET_AND_CLEAR_ARE_RE
    }
}
#[doc = "Values that can be written to the field `SETCLR0`"]
pub enum SETCLR0W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
}
impl SETCLR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR0W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR0W::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR0W::SET_AND_CLEAR_ARE_RE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR0W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR0W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR1`"]
pub enum SETCLR1W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
}
impl SETCLR1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR1W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR1W::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR1W::SET_AND_CLEAR_ARE_RE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR1W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR1W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR2`"]
pub enum SETCLR2W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
}
impl SETCLR2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR2W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR2W::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR2W::SET_AND_CLEAR_ARE_RE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR2W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR2W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SETCLR3`"]
pub enum SETCLR3W {
    #[doc = "Set and clear do not depend on any counter."]
    SET_AND_CLEAR_DO_NOT,
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    SET_AND_CLEAR_ARE_RE,
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    SET_AND_CLEAR_ARE_RE,
}
impl SETCLR3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SETCLR3W::SET_AND_CLEAR_DO_NOT => 0,
            SETCLR3W::SET_AND_CLEAR_ARE_RE => 1,
            SETCLR3W::SET_AND_CLEAR_ARE_RE => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SETCLR3W<'a> {
    w: &'a mut W,
}
impl<'a> _SETCLR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SETCLR3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set and clear do not depend on any counter."]
    #[inline]
    pub fn set_and_clear_do_not(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_DO_NOT)
    }
    #[doc = "Set and clear are reversed when counter L or the unified counter is counting down."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = "Set and clear are reversed when counter H is counting down. Do not use if UNIFY = 1."]
    #[inline]
    pub fn set_and_clear_are_re(self) -> &'a mut W {
        self.variant(SETCLR3W::SET_AND_CLEAR_ARE_RE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&self) -> SETCLR0R {
        SETCLR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&self) -> SETCLR1R {
        SETCLR1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&self) -> SETCLR2R {
        SETCLR2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&self) -> SETCLR3R {
        SETCLR3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - Set/clear operation on output 0. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr0(&mut self) -> _SETCLR0W {
        _SETCLR0W { w: self }
    }
    #[doc = "Bits 2:3 - Set/clear operation on output 1. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr1(&mut self) -> _SETCLR1W {
        _SETCLR1W { w: self }
    }
    #[doc = "Bits 4:5 - Set/clear operation on output 2. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr2(&mut self) -> _SETCLR2W {
        _SETCLR2W { w: self }
    }
    #[doc = "Bits 6:7 - Set/clear operation on output 3. Value 0x3 is reserved. Do not program this value."]
    #[inline]
    pub fn setclr3(&mut self) -> _SETCLR3W {
        _SETCLR3W { w: self }
    }
}
