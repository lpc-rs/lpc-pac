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
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP0RER::ENABLE => true,
            CAP0RER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0RER {
        match value {
            true => CAP0RER::ENABLE,
            false => CAP0RER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP0RER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP0RER::DISABLE
    }
}
#[doc = "Possible values of the field `CAP0FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0FER {
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP0FER::ENABLE => true,
            CAP0FER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0FER {
        match value {
            true => CAP0FER::ENABLE,
            false => CAP0FER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP0FER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP0FER::DISABLE
    }
}
#[doc = "Possible values of the field `CAP0I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP0IR {
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP0IR::ENABLE => true,
            CAP0IR::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP0IR {
        match value {
            true => CAP0IR::ENABLE,
            false => CAP0IR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP0IR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP0IR::DISABLE
    }
}
#[doc = "Possible values of the field `CAP1RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1RER {
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP1RER::ENABLE => true,
            CAP1RER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1RER {
        match value {
            true => CAP1RER::ENABLE,
            false => CAP1RER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP1RER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP1RER::DISABLE
    }
}
#[doc = "Possible values of the field `CAP1FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1FER {
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP1FER::ENABLE => true,
            CAP1FER::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1FER {
        match value {
            true => CAP1FER::ENABLE,
            false => CAP1FER::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP1FER::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP1FER::DISABLE
    }
}
#[doc = "Possible values of the field `CAP1I`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAP1IR {
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
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
            CAP1IR::ENABLE => true,
            CAP1IR::DISABLE => false,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CAP1IR {
        match value {
            true => CAP1IR::ENABLE,
            false => CAP1IR::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == CAP1IR::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == CAP1IR::DISABLE
    }
}
#[doc = "Values that can be written to the field `CAP0RE`"]
pub enum CAP0REW {
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0REW::ENABLE => true,
            CAP0REW::DISABLE => false,
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
    #[doc = "A sequence of 0 then 1 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0REW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0REW::DISABLE)
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
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0FEW::ENABLE => true,
            CAP0FEW::DISABLE => false,
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
    #[doc = "A sequence of 1 then 0 on CAPn.0 will cause CR0 to be loaded with the contents of TC."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0FEW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0FEW::DISABLE)
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
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP0IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP0IW::ENABLE => true,
            CAP0IW::DISABLE => false,
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
    #[doc = "A CR0 load due to a CAPn.0 event will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP0IW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP0IW::DISABLE)
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
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1REW::ENABLE => true,
            CAP1REW::DISABLE => false,
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
    #[doc = "A sequence of 0 then 1 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1REW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1REW::DISABLE)
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
#[doc = "Values that can be written to the field `CAP1FE`"]
pub enum CAP1FEW {
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1FEW::ENABLE => true,
            CAP1FEW::DISABLE => false,
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
    #[doc = "A sequence of 1 then 0 on CAPn.1 will cause CR1 to be loaded with the contents of TC."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1FEW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1FEW::DISABLE)
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
#[doc = "Values that can be written to the field `CAP1I`"]
pub enum CAP1IW {
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    ENABLE,
    #[doc = "This feature is disabled."]
    DISABLE,
}
impl CAP1IW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CAP1IW::ENABLE => true,
            CAP1IW::DISABLE => false,
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
    #[doc = "A CR1 load due to a CAPn.1 event will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(CAP1IW::ENABLE)
    }
    #[doc = "This feature is disabled."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(CAP1IW::DISABLE)
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
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline]
    pub fn cap0re(&self) -> CAP0RER {
        CAP0RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline]
    pub fn cap0fe(&self) -> CAP0FER {
        CAP0FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline]
    pub fn cap0i(&self) -> CAP0IR {
        CAP0IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline]
    pub fn cap1re(&self) -> CAP1RER {
        CAP1RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline]
    pub fn cap1fe(&self) -> CAP1FER {
        CAP1FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline]
    pub fn cap1i(&self) -> CAP1IR {
        CAP1IR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Capture on CAPn.0 rising edge"]
    #[inline]
    pub fn cap0re(&mut self) -> _CAP0REW {
        _CAP0REW { w: self }
    }
    #[doc = "Bit 1 - Capture on CAPn.0 falling edge"]
    #[inline]
    pub fn cap0fe(&mut self) -> _CAP0FEW {
        _CAP0FEW { w: self }
    }
    #[doc = "Bit 2 - Interrupt on CAPn.0 event"]
    #[inline]
    pub fn cap0i(&mut self) -> _CAP0IW {
        _CAP0IW { w: self }
    }
    #[doc = "Bit 3 - Capture on CAPn.1 rising edge"]
    #[inline]
    pub fn cap1re(&mut self) -> _CAP1REW {
        _CAP1REW { w: self }
    }
    #[doc = "Bit 4 - Capture on CAPn.1 falling edge"]
    #[inline]
    pub fn cap1fe(&mut self) -> _CAP1FEW {
        _CAP1FEW { w: self }
    }
    #[doc = "Bit 5 - Interrupt on CAPn.1 event"]
    #[inline]
    pub fn cap1i(&mut self) -> _CAP1IW {
        _CAP1IW { w: self }
    }
}
