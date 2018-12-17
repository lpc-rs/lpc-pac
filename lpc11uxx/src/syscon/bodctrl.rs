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
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    LEVEL_0_THE_RESET_A,
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    LEVEL_1_THE_RESET_A,
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    LEVEL_2_THE_RESET_A,
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    LEVEL_3_THE_RESET_A,
}
impl BODRSTLEVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODRSTLEVR::LEVEL_0_THE_RESET_A => 0,
            BODRSTLEVR::LEVEL_1_THE_RESET_A => 1,
            BODRSTLEVR::LEVEL_2_THE_RESET_A => 2,
            BODRSTLEVR::LEVEL_3_THE_RESET_A => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODRSTLEVR {
        match value {
            0 => BODRSTLEVR::LEVEL_0_THE_RESET_A,
            1 => BODRSTLEVR::LEVEL_1_THE_RESET_A,
            2 => BODRSTLEVR::LEVEL_2_THE_RESET_A,
            3 => BODRSTLEVR::LEVEL_3_THE_RESET_A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_THE_RESET_A`"]
    #[inline]
    pub fn is_level_0_the_reset_a(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_0_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_1_THE_RESET_A`"]
    #[inline]
    pub fn is_level_1_the_reset_a(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_1_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_THE_RESET_A`"]
    #[inline]
    pub fn is_level_2_the_reset_a(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_2_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_THE_RESET_A`"]
    #[inline]
    pub fn is_level_3_the_reset_a(&self) -> bool {
        *self == BODRSTLEVR::LEVEL_3_THE_RESET_A
    }
}
#[doc = "Possible values of the field `BODINTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTVALR {
    #[doc = "Level 0: Reserved."]
    LEVEL_0_RESERVED_,
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    LEVEL_1THE_INTERRUP,
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    LEVEL_2_THE_INTERRU,
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    LEVEL_3_THE_INTERRU,
}
impl BODINTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BODINTVALR::LEVEL_0_RESERVED_ => 0,
            BODINTVALR::LEVEL_1THE_INTERRUP => 1,
            BODINTVALR::LEVEL_2_THE_INTERRU => 2,
            BODINTVALR::LEVEL_3_THE_INTERRU => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BODINTVALR {
        match value {
            0 => BODINTVALR::LEVEL_0_RESERVED_,
            1 => BODINTVALR::LEVEL_1THE_INTERRUP,
            2 => BODINTVALR::LEVEL_2_THE_INTERRU,
            3 => BODINTVALR::LEVEL_3_THE_INTERRU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_RESERVED_`"]
    #[inline]
    pub fn is_level_0_reserved_(&self) -> bool {
        *self == BODINTVALR::LEVEL_0_RESERVED_
    }
    #[doc = "Checks if the value of the field is `LEVEL_1THE_INTERRUP`"]
    #[inline]
    pub fn is_level_1the_interrup(&self) -> bool {
        *self == BODINTVALR::LEVEL_1THE_INTERRUP
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_THE_INTERRU`"]
    #[inline]
    pub fn is_level_2_the_interru(&self) -> bool {
        *self == BODINTVALR::LEVEL_2_THE_INTERRU
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_THE_INTERRU`"]
    #[inline]
    pub fn is_level_3_the_interru(&self) -> bool {
        *self == BODINTVALR::LEVEL_3_THE_INTERRU
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
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    LEVEL_0_THE_RESET_A,
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    LEVEL_1_THE_RESET_A,
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    LEVEL_2_THE_RESET_A,
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    LEVEL_3_THE_RESET_A,
}
impl BODRSTLEVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODRSTLEVW::LEVEL_0_THE_RESET_A => 0,
            BODRSTLEVW::LEVEL_1_THE_RESET_A => 1,
            BODRSTLEVW::LEVEL_2_THE_RESET_A => 2,
            BODRSTLEVW::LEVEL_3_THE_RESET_A => 3,
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
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    #[inline]
    pub fn level_0_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_0_THE_RESET_A)
    }
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    #[inline]
    pub fn level_1_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_1_THE_RESET_A)
    }
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    #[inline]
    pub fn level_2_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_2_THE_RESET_A)
    }
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    #[inline]
    pub fn level_3_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEVW::LEVEL_3_THE_RESET_A)
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
    #[doc = "Level 0: Reserved."]
    LEVEL_0_RESERVED_,
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    LEVEL_1THE_INTERRUP,
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    LEVEL_2_THE_INTERRU,
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    LEVEL_3_THE_INTERRU,
}
impl BODINTVALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BODINTVALW::LEVEL_0_RESERVED_ => 0,
            BODINTVALW::LEVEL_1THE_INTERRUP => 1,
            BODINTVALW::LEVEL_2_THE_INTERRU => 2,
            BODINTVALW::LEVEL_3_THE_INTERRU => 3,
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
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Level 0: Reserved."]
    #[inline]
    pub fn level_0_reserved_(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_0_RESERVED_)
    }
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    #[inline]
    pub fn level_1the_interrup(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_1THE_INTERRUP)
    }
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    #[inline]
    pub fn level_2_the_interru(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_2_THE_INTERRU)
    }
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    #[inline]
    pub fn level_3_the_interru(self) -> &'a mut W {
        self.variant(BODINTVALW::LEVEL_3_THE_INTERRU)
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
