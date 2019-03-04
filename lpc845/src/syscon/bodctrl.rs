#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BODCTRL {
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
#[doc = "Possible values of the field `BODRSTLEV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTLEVR {
    #[doc = "Level 1"]
    LEVEL_1,
    #[doc = "Level 2"]
    LEVEL_2,
    #[doc = "Level 3"]
    LEVEL_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BODRSTLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODRSTLEVR::LEVEL_1 => 1,
            BODRSTLEVR::LEVEL_2 => 2,
            BODRSTLEVR::LEVEL_3 => 3,
            BODRSTLEVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODRSTLEVR {
        match value {
            1 => BODRSTLEVR::LEVEL_1,
            2 => BODRSTLEVR::LEVEL_2,
            3 => BODRSTLEVR::LEVEL_3,
            i => BODRSTLEVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline]
    pub fn is_level_1(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline]
    pub fn is_level_2(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline]
    pub fn is_level_3(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_3
    }
}
#[doc = "Possible values of the field `BODINTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTVALR {
    #[doc = "Level 1"]
    LEVEL_1,
    #[doc = "Level 2"]
    LEVEL_2,
    #[doc = "Level 3"]
    LEVEL_3,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BODINTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODINTVALR::LEVEL_1 => 1,
            BODINTVALR::LEVEL_2 => 2,
            BODINTVALR::LEVEL_3 => 3,
            BODINTVALR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODINTVALR {
        match value {
            1 => BODINTVALR::LEVEL_1,
            2 => BODINTVALR::LEVEL_2,
            3 => BODINTVALR::LEVEL_3,
            i => BODINTVALR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline]
    pub fn is_level_1(&self) -> bool {
        *self == BODINTVALR::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline]
    pub fn is_level_2(&self) -> bool {
        *self == BODINTVALR::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline]
    pub fn is_level_3(&self) -> bool {
        *self == BODINTVALR::LEVEL_3
    }
}
#[doc = "Possible values of the field `BODRSTENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENAR {
    #[doc = "Disable reset function."]
    DISABLE,
    #[doc = "Enable reset function."]
    ENABLE,
}
impl BODRSTENAR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BODRSTENAR::DISABLE => false,
            BODRSTENAR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRSTENAR {
        match value {
            false => BODRSTENAR::DISABLE,
            true => BODRSTENAR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == BODRSTENAR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENAR::ENABLE
    }
}
#[doc = "Values that can be written to the field `BODRSTLEV`"]
pub enum BODRSTLEVW {
    #[doc = "Level 1"]
    LEVEL_1,
    #[doc = "Level 2"]
    LEVEL_2,
    #[doc = "Level 3"]
    LEVEL_3,
}
impl BODRSTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVW::LEVEL_1 => 1,
            BODRSTLEVW::LEVEL_2 => 2,
            BODRSTLEVW::LEVEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODRSTLEVW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTLEVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODRSTLEVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Level 1"]
    #[inline]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_1)
    }
    #[doc = "Level 2"]
    #[inline]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_2)
    }
    #[doc = "Level 3"]
    #[inline]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_3)
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
#[doc = "Values that can be written to the field `BODINTVAL`"]
pub enum BODINTVALW {
    #[doc = "Level 1"]
    LEVEL_1,
    #[doc = "Level 2"]
    LEVEL_2,
    #[doc = "Level 3"]
    LEVEL_3,
}
impl BODINTVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODINTVALW::LEVEL_1 => 1,
            BODINTVALW::LEVEL_2 => 2,
            BODINTVALW::LEVEL_3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODINTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _BODINTVALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODINTVALW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Level 1"]
    #[inline]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_1)
    }
    #[doc = "Level 2"]
    #[inline]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_2)
    }
    #[doc = "Level 3"]
    #[inline]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_3)
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
#[doc = "Values that can be written to the field `BODRSTENA`"]
pub enum BODRSTENAW {
    #[doc = "Disable reset function."]
    DISABLE,
    #[doc = "Enable reset function."]
    ENABLE,
}
impl BODRSTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENAW::DISABLE => false,
            BODRSTENAW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BODRSTENAW<'a> {
    w: &'a mut W,
}
impl<'a> _BODRSTENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BODRSTENAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable reset function."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENAW::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENAW::ENABLE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline]
    pub fn bodrstlev(&self) -> BODRSTLEVR {
        BODRSTLEVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline]
    pub fn bodintval(&self) -> BODINTVALR {
        BODINTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline]
    pub fn bodrstena(&self) -> BODRSTENAR {
        BODRSTENAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline]
    pub fn bodrstlev(&mut self) -> _BODRSTLEVW {
        _BODRSTLEVW { w: self }
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline]
    pub fn bodintval(&mut self) -> _BODINTVALW {
        _BODINTVALW { w: self }
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline]
    pub fn bodrstena(&mut self) -> _BODRSTENAW {
        _BODRSTENAW { w: self }
    }
}
