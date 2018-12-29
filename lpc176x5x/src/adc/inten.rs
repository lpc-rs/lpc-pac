#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTEN {
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
#[doc = "Possible values of the field `ADINTEN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN0R {
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN0R {
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
            ADINTEN0R::DISABLE => false,
            ADINTEN0R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN0R {
        match value {
            false => ADINTEN0R::DISABLE,
            true => ADINTEN0R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN0R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN0R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN1R {
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN1R {
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
            ADINTEN1R::DISABLE => false,
            ADINTEN1R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN1R {
        match value {
            false => ADINTEN1R::DISABLE,
            true => ADINTEN1R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN1R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN1R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN2R {
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN2R {
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
            ADINTEN2R::DISABLE => false,
            ADINTEN2R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN2R {
        match value {
            false => ADINTEN2R::DISABLE,
            true => ADINTEN2R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN2R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN2R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN3R {
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN3R {
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
            ADINTEN3R::DISABLE => false,
            ADINTEN3R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN3R {
        match value {
            false => ADINTEN3R::DISABLE,
            true => ADINTEN3R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN3R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN3R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN4R {
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN4R {
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
            ADINTEN4R::DISABLE => false,
            ADINTEN4R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN4R {
        match value {
            false => ADINTEN4R::DISABLE,
            true => ADINTEN4R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN4R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN4R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN5R {
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN5R {
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
            ADINTEN5R::DISABLE => false,
            ADINTEN5R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN5R {
        match value {
            false => ADINTEN5R::DISABLE,
            true => ADINTEN5R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN5R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN5R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN6R {
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN6R {
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
            ADINTEN6R::DISABLE => false,
            ADINTEN6R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN6R {
        match value {
            false => ADINTEN6R::DISABLE,
            true => ADINTEN6R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN6R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN6R::ENABLE
    }
}
#[doc = "Possible values of the field `ADINTEN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADINTEN7R {
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN7R {
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
            ADINTEN7R::DISABLE => false,
            ADINTEN7R::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADINTEN7R {
        match value {
            false => ADINTEN7R::DISABLE,
            true => ADINTEN7R::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == ADINTEN7R::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == ADINTEN7R::ENABLE
    }
}
#[doc = "Possible values of the field `ADGINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADGINTENR {
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS,
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL,
}
impl ADGINTENR {
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
            ADGINTENR::CHANNELS => false,
            ADGINTENR::GLOBAL => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADGINTENR {
        match value {
            false => ADGINTENR::CHANNELS,
            true => ADGINTENR::GLOBAL,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNELS`"]
    #[inline]
    pub fn is_channels(&self) -> bool {
        *self == ADGINTENR::CHANNELS
    }
    #[doc = "Checks if the value of the field is `GLOBAL`"]
    #[inline]
    pub fn is_global(&self) -> bool {
        *self == ADGINTENR::GLOBAL
    }
}
#[doc = "Values that can be written to the field `ADINTEN0`"]
pub enum ADINTEN0W {
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN0W::DISABLE => false,
            ADINTEN0W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN0W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN0W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN1`"]
pub enum ADINTEN1W {
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN1W::DISABLE => false,
            ADINTEN1W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN1W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN1W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN2`"]
pub enum ADINTEN2W {
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN2W::DISABLE => false,
            ADINTEN2W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN2W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN2W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN3`"]
pub enum ADINTEN3W {
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN3W::DISABLE => false,
            ADINTEN3W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN3W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN3W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN4`"]
pub enum ADINTEN4W {
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN4W::DISABLE => false,
            ADINTEN4W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN4W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN4W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN5`"]
pub enum ADINTEN5W {
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN5W::DISABLE => false,
            ADINTEN5W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN5W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN5W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN6`"]
pub enum ADINTEN6W {
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN6W::DISABLE => false,
            ADINTEN6W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN6W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN6W::ENABLE)
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
#[doc = "Values that can be written to the field `ADINTEN7`"]
pub enum ADINTEN7W {
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    DISABLE,
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    ENABLE,
}
impl ADINTEN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADINTEN7W::DISABLE => false,
            ADINTEN7W::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADINTEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADINTEN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADINTEN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADINTEN7W::DISABLE)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADINTEN7W::ENABLE)
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
#[doc = "Values that can be written to the field `ADGINTEN`"]
pub enum ADGINTENW {
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    CHANNELS,
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    GLOBAL,
}
impl ADGINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADGINTENW::CHANNELS => false,
            ADGINTENW::GLOBAL => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADGINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ADGINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADGINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline]
    pub fn channels(self) -> &'a mut W {
        self.variant(ADGINTENW::CHANNELS)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline]
    pub fn global(self) -> &'a mut W {
        self.variant(ADGINTENW::GLOBAL)
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
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline]
    pub fn adinten0(&self) -> ADINTEN0R {
        ADINTEN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline]
    pub fn adinten1(&self) -> ADINTEN1R {
        ADINTEN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline]
    pub fn adinten2(&self) -> ADINTEN2R {
        ADINTEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline]
    pub fn adinten3(&self) -> ADINTEN3R {
        ADINTEN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline]
    pub fn adinten4(&self) -> ADINTEN4R {
        ADINTEN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline]
    pub fn adinten5(&self) -> ADINTEN5R {
        ADINTEN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline]
    pub fn adinten6(&self) -> ADINTEN6R {
        ADINTEN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline]
    pub fn adinten7(&self) -> ADINTEN7R {
        ADINTEN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline]
    pub fn adginten(&self) -> ADGINTENR {
        ADGINTENR::_from({
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
        W { bits: 256 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline]
    pub fn adinten0(&mut self) -> _ADINTEN0W {
        _ADINTEN0W { w: self }
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline]
    pub fn adinten1(&mut self) -> _ADINTEN1W {
        _ADINTEN1W { w: self }
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline]
    pub fn adinten2(&mut self) -> _ADINTEN2W {
        _ADINTEN2W { w: self }
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline]
    pub fn adinten3(&mut self) -> _ADINTEN3W {
        _ADINTEN3W { w: self }
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline]
    pub fn adinten4(&mut self) -> _ADINTEN4W {
        _ADINTEN4W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline]
    pub fn adinten5(&mut self) -> _ADINTEN5W {
        _ADINTEN5W { w: self }
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline]
    pub fn adinten6(&mut self) -> _ADINTEN6W {
        _ADINTEN6W { w: self }
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline]
    pub fn adinten7(&mut self) -> _ADINTEN7W {
        _ADINTEN7W { w: self }
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline]
    pub fn adginten(&mut self) -> _ADGINTENW {
        _ADGINTENW { w: self }
    }
}
