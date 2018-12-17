#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CCR {
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
#[doc = "Possible values of the field `CAP0RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0RER {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0RER {
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
            CAP0RER::ENABLED_ => true,
            CAP0RER::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0RER {
        match value {
            true => CAP0RER::ENABLED_,
            false => CAP0RER::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP0RER::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP0RER::DISABLED_
    }
}
#[doc = "Possible values of the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FER {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0FER {
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
            CAP0FER::ENABLED_ => true,
            CAP0FER::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0FER {
        match value {
            true => CAP0FER::ENABLED_,
            false => CAP0FER::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP0FER::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP0FER::DISABLED_
    }
}
#[doc = "Possible values of the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IR {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0IR {
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
            CAP0IR::ENABLED_ => true,
            CAP0IR::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0IR {
        match value {
            true => CAP0IR::ENABLED_,
            false => CAP0IR::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP0IR::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP0IR::DISABLED_
    }
}
#[doc = "Possible values of the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RER {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1RER {
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
            CAP1RER::ENABLED_ => true,
            CAP1RER::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1RER {
        match value {
            true => CAP1RER::ENABLED_,
            false => CAP1RER::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP1RER::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP1RER::DISABLED_
    }
}
#[doc = "Possible values of the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FER {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1FER {
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
            CAP1FER::ENABLED_ => true,
            CAP1FER::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1FER {
        match value {
            true => CAP1FER::ENABLED_,
            false => CAP1FER::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP1FER::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP1FER::DISABLED_
    }
}
#[doc = "Possible values of the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IR {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1IR {
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
            CAP1IR::ENABLED_ => true,
            CAP1IR::DISABLED_ => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1IR {
        match value {
            true => CAP1IR::ENABLED_,
            false => CAP1IR::DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED_`"]
    #[inline]
    pub fn is_enabled_(&self) -> bool {
        *self == CAP1IR::ENABLED_
    }
    #[doc = "Checks if the value of the field is `DISABLED_`"]
    #[inline]
    pub fn is_disabled_(&self) -> bool {
        *self == CAP1IR::DISABLED_
    }
}
#[doc = "Values that can be written to the field `CAP0RE`"]
pub enum CAP0REW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0REW::ENABLED_ => true,
            CAP0REW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP0REW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP0REW::DISABLED_)
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
#[doc = "Values that can be written to the field `CAP0FE`"]
pub enum CAP0FEW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0FEW::ENABLED_ => true,
            CAP0FEW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP0FEW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP0FEW::DISABLED_)
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
#[doc = "Values that can be written to the field `CAP0I`"]
pub enum CAP0IW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0IW::ENABLED_ => true,
            CAP0IW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP0IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP0IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP0IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP0IW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP0IW::DISABLED_)
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
#[doc = "Values that can be written to the field `CAP1RE`"]
pub enum CAP1REW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1REW::ENABLED_ => true,
            CAP1REW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1REW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP1REW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP1REW::DISABLED_)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAP1FE`"]
pub enum CAP1FEW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1FEW::ENABLED_ => true,
            CAP1FEW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1FEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP1FEW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP1FEW::DISABLED_)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CAP1I`"]
pub enum CAP1IW {
    #[doc = "Enabled."]
    ENABLED_,
    #[doc = "Disabled."]
    DISABLED_,
}
impl CAP1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1IW::ENABLED_ => true,
            CAP1IW::DISABLED_ => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CAP1IW<'a> {
    w: &'a mut W,
}
impl<'a> _CAP1IW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CAP1IW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Enabled."]
    #[inline]
    pub fn enabled_(self) -> &'a mut W {
        self.variant(CAP1IW::ENABLED_)
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled_(self) -> &'a mut W {
        self.variant(CAP1IW::DISABLED_)
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap0re(&self) -> CAP0RER {
        CAP0RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap0fe(&self) -> CAP0FER {
        CAP0FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
    #[inline]
    pub fn cap0i(&self) -> CAP0IR {
        CAP0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap1re(&self) -> CAP1RER {
        CAP1RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap1fe(&self) -> CAP1FER {
        CAP1FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
    #[inline]
    pub fn cap1i(&self) -> CAP1IR {
        CAP1IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Capture on CT32B0_CAP0 rising edge: a sequence of 0 then 1 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap0re(&mut self) -> _CAP0REW {
        _CAP0REW { w: self }
    }
    #[doc = "Bit 1 - Capture on CT32B0_CAP0 falling edge: a sequence of 1 then 0 on CT32B0_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap0fe(&mut self) -> _CAP0FEW {
        _CAP0FEW { w: self }
    }
    #[doc = "Bit 2 - Interrupt on CT32B0_CAP0 event: a CR0 load due to a CT32B0_CAP0 event will generate an interrupt."]
    #[inline]
    pub fn cap0i(&mut self) -> _CAP0IW {
        _CAP0IW { w: self }
    }
    #[doc = "Bit 6 - Capture on CT32B0_CAP1 rising edge: a sequence of 0 then 1 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap1re(&mut self) -> _CAP1REW {
        _CAP1REW { w: self }
    }
    #[doc = "Bit 7 - Capture on CT32B0_CAP1 falling edge: a sequence of 1 then 0 on CT32B0_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn cap1fe(&mut self) -> _CAP1FEW {
        _CAP1FEW { w: self }
    }
    #[doc = "Bit 8 - Interrupt on CT32B0_CAP1 event: a CR1 load due to a CT32B0_CAP1 event will generate an interrupt."]
    #[inline]
    pub fn cap1i(&mut self) -> _CAP1IW {
        _CAP1IW { w: self }
    }
}
