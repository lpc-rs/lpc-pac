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
    #[doc = "Level 0."]
    LEVEL_0,
    #[doc = "Level 1."]
    LEVEL_1,
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODRSTLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODRSTLEVR::LEVEL_0 => 0,
            BODRSTLEVR::LEVEL_1 => 1,
            BODRSTLEVR::LEVEL_2 => 2,
            BODRSTLEVR::LEVEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODRSTLEVR {
        match value {
            0 => BODRSTLEVR::LEVEL_0,
            1 => BODRSTLEVR::LEVEL_1,
            2 => BODRSTLEVR::LEVEL_2,
            3 => BODRSTLEVR::LEVEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0`"]
    #[inline]
    pub fn is_level_0(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_0
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
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODINTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODINTVALR::LEVEL_2 => 2,
            BODINTVALR::LEVEL_3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODINTVALR {
        match value {
            2 => BODINTVALR::LEVEL_2,
            3 => BODINTVALR::LEVEL_3,
            _ => unreachable!(),
        }
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
    DISABLE_RESET_FUNCTI,
    #[doc = "Enable reset function."]
    ENABLE_RESET_FUNCTIO,
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
            BODRSTENAR::DISABLE_RESET_FUNCTI => false,
            BODRSTENAR::ENABLE_RESET_FUNCTIO => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BODRSTENAR {
        match value {
            false => BODRSTENAR::DISABLE_RESET_FUNCTI,
            true => BODRSTENAR::ENABLE_RESET_FUNCTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_RESET_FUNCTI`"]
    #[inline]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == BODRSTENAR::DISABLE_RESET_FUNCTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_FUNCTIO`"]
    #[inline]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == BODRSTENAR::ENABLE_RESET_FUNCTIO
    }
}
#[doc = "Values that can be written to the field `BODRSTLEV`"]
pub enum BODRSTLEVW {
    #[doc = "Level 0."]
    LEVEL_0,
    #[doc = "Level 1."]
    LEVEL_1,
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODRSTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVW::LEVEL_0 => 0,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0."]
    #[inline]
    pub fn level_0(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_0)
    }
    #[doc = "Level 1."]
    #[inline]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_1)
    }
    #[doc = "Level 2."]
    #[inline]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_2)
    }
    #[doc = "Level 3."]
    #[inline]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_3)
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
#[doc = "Values that can be written to the field `BODINTVAL`"]
pub enum BODINTVALW {
    #[doc = "Level 2."]
    LEVEL_2,
    #[doc = "Level 3."]
    LEVEL_3,
}
impl BODINTVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
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
    #[doc = "Level 2."]
    #[inline]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_2)
    }
    #[doc = "Level 3."]
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
    DISABLE_RESET_FUNCTI,
    #[doc = "Enable reset function."]
    ENABLE_RESET_FUNCTIO,
}
impl BODRSTENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BODRSTENAW::DISABLE_RESET_FUNCTI => false,
            BODRSTENAW::ENABLE_RESET_FUNCTIO => true,
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
    pub fn disable_reset_functi(self) -> &'a mut W {
        self.variant(BODRSTENAW::DISABLE_RESET_FUNCTI)
    }
    #[doc = "Enable reset function."]
    #[inline]
    pub fn enable_reset_functio(self) -> &'a mut W {
        self.variant(BODRSTENAW::ENABLE_RESET_FUNCTIO)
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
