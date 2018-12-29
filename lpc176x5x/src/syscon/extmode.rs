#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTMODE {
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
#[doc = "Possible values of the field `EXTMODE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE0R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE0R {
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
            EXTMODE0R::LEVEL_SENSITIVE => false,
            EXTMODE0R::EDGE_SENSITIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTMODE0R {
        match value {
            false => EXTMODE0R::LEVEL_SENSITIVE,
            true => EXTMODE0R::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE0R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE0R::EDGE_SENSITIVE
    }
}
#[doc = "Possible values of the field `EXTMODE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE1R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE1R {
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
            EXTMODE1R::LEVEL_SENSITIVE => false,
            EXTMODE1R::EDGE_SENSITIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTMODE1R {
        match value {
            false => EXTMODE1R::LEVEL_SENSITIVE,
            true => EXTMODE1R::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE1R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE1R::EDGE_SENSITIVE
    }
}
#[doc = "Possible values of the field `EXTMODE2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE2R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE2R {
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
            EXTMODE2R::LEVEL_SENSITIVE => false,
            EXTMODE2R::EDGE_SENSITIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTMODE2R {
        match value {
            false => EXTMODE2R::LEVEL_SENSITIVE,
            true => EXTMODE2R::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE2R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE2R::EDGE_SENSITIVE
    }
}
#[doc = "Possible values of the field `EXTMODE3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE3R {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE3R {
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
            EXTMODE3R::LEVEL_SENSITIVE => false,
            EXTMODE3R::EDGE_SENSITIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTMODE3R {
        match value {
            false => EXTMODE3R::LEVEL_SENSITIVE,
            true => EXTMODE3R::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE3R::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE3R::EDGE_SENSITIVE
    }
}
#[doc = "Values that can be written to the field `EXTMODE0`"]
pub enum EXTMODE0W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE0W::LEVEL_SENSITIVE => false,
            EXTMODE0W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTMODE0W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTMODE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0W::EDGE_SENSITIVE)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTMODE1`"]
pub enum EXTMODE1W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE1W::LEVEL_SENSITIVE => false,
            EXTMODE1W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTMODE1W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTMODE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1W::EDGE_SENSITIVE)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTMODE2`"]
pub enum EXTMODE2W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE2W::LEVEL_SENSITIVE => false,
            EXTMODE2W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTMODE2W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTMODE2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2W::EDGE_SENSITIVE)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTMODE3`"]
pub enum EXTMODE3W {
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE,
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE,
}
impl EXTMODE3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTMODE3W::LEVEL_SENSITIVE => false,
            EXTMODE3W::EDGE_SENSITIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTMODE3W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTMODE3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTMODE3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3W::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3W::EDGE_SENSITIVE)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline]
    pub fn extmode0(&self) -> EXTMODE0R {
        EXTMODE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline]
    pub fn extmode1(&self) -> EXTMODE1R {
        EXTMODE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline]
    pub fn extmode2(&self) -> EXTMODE2R {
        EXTMODE2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline]
    pub fn extmode3(&self) -> EXTMODE3R {
        EXTMODE3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline]
    pub fn extmode0(&mut self) -> _EXTMODE0W {
        _EXTMODE0W { w: self }
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline]
    pub fn extmode1(&mut self) -> _EXTMODE1W {
        _EXTMODE1W { w: self }
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline]
    pub fn extmode2(&mut self) -> _EXTMODE2W {
        _EXTMODE2W { w: self }
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline]
    pub fn extmode3(&mut self) -> _EXTMODE3W {
        _EXTMODE3W { w: self }
    }
}
