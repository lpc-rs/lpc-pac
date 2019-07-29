#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL {
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
#[doc = "Possible values of the field `SSP0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_NR {
    #[doc = "Reset. Resets the SSP0 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP0_RST_NR {
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
            SSP0_RST_NR::RESET => false,
            SSP0_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP0_RST_NR {
        match value {
            false => SSP0_RST_NR::RESET,
            true => SSP0_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SSP0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == SSP0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `I2C0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_NR {
    #[doc = "Reset. Resets the I2C0 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C0 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C0_RST_NR {
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
            I2C0_RST_NR::RESET => false,
            I2C0_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C0_RST_NR {
        match value {
            false => I2C0_RST_NR::RESET,
            true => I2C0_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == I2C0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == I2C0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `SSP1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_NR {
    #[doc = "Reset. Resets the SSP1 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP1 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP1_RST_NR {
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
            SSP1_RST_NR::RESET => false,
            SSP1_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSP1_RST_NR {
        match value {
            false => SSP1_RST_NR::RESET,
            true => SSP1_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SSP1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == SSP1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `I2C1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_RST_NR {
    #[doc = "Reset. Resets the I2C1 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C1 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C1_RST_NR {
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
            I2C1_RST_NR::RESET => false,
            I2C1_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> I2C1_RST_NR {
        match value {
            false => I2C1_RST_NR::RESET,
            true => I2C1_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == I2C1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == I2C1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `FRG_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG_RST_NR {
    #[doc = "Reset. Resets the FRG peripheral."]
    RESET,
    #[doc = "Clear reset. FRG reset de-asserted."]
    CLEAR_RESET,
}
impl FRG_RST_NR {
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
            FRG_RST_NR::RESET => false,
            FRG_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRG_RST_NR {
        match value {
            false => FRG_RST_NR::RESET,
            true => FRG_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == FRG_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == FRG_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `USART1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_RST_NR {
    #[doc = "Reset. Resets the USART1 peripheral."]
    RESET,
    #[doc = "Clear reset. USART1 reset de-asserted."]
    CLEAR_RESET,
}
impl USART1_RST_NR {
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
            USART1_RST_NR::RESET => false,
            USART1_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART1_RST_NR {
        match value {
            false => USART1_RST_NR::RESET,
            true => USART1_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `USART2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_RST_NR {
    #[doc = "Reset. Resets the USART2 peripheral."]
    RESET,
    #[doc = "Clear reset. USART2 reset de-asserted."]
    CLEAR_RESET,
}
impl USART2_RST_NR {
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
            USART2_RST_NR::RESET => false,
            USART2_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART2_RST_NR {
        match value {
            false => USART2_RST_NR::RESET,
            true => USART2_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART2_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART2_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `USART3_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3_RST_NR {
    #[doc = "Reset. Resets the USART3 peripheral."]
    RESET,
    #[doc = "Clear reset. USART3 reset de-asserted."]
    CLEAR_RESET,
}
impl USART3_RST_NR {
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
            USART3_RST_NR::RESET => false,
            USART3_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART3_RST_NR {
        match value {
            false => USART3_RST_NR::RESET,
            true => USART3_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART3_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART3_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `USART4_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART4_RST_NR {
    #[doc = "Reset. Resets the USART4 peripheral."]
    RESET,
    #[doc = "Clear reset. USART4 reset de-asserted."]
    CLEAR_RESET,
}
impl USART4_RST_NR {
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
            USART4_RST_NR::RESET => false,
            USART4_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> USART4_RST_NR {
        match value {
            false => USART4_RST_NR::RESET,
            true => USART4_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == USART4_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART4_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `SCT0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_RST_NR {
    #[doc = "Reset. Resets the SCT0 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT0 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT0_RST_NR {
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
            SCT0_RST_NR::RESET => false,
            SCT0_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT0_RST_NR {
        match value {
            false => SCT0_RST_NR::RESET,
            true => SCT0_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SCT0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == SCT0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Possible values of the field `SCT1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT1_RST_NR {
    #[doc = "Reset. Resets the SCT1 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT1 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT1_RST_NR {
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
            SCT1_RST_NR::RESET => false,
            SCT1_RST_NR::CLEAR_RESET => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCT1_RST_NR {
        match value {
            false => SCT1_RST_NR::RESET,
            true => SCT1_RST_NR::CLEAR_RESET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == SCT1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline]
    pub fn is_clear_reset(&self) -> bool {
        *self == SCT1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `SSP0_RST_N`"]
pub enum SSP0_RST_NW {
    #[doc = "Reset. Resets the SSP0 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0_RST_NW::RESET => false,
            SSP0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SSP0 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::RESET)
    }
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `I2C0_RST_N`"]
pub enum I2C0_RST_NW {
    #[doc = "Reset. Resets the I2C0 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C0 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_RST_NW::RESET => false,
            I2C0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the I2C0 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::RESET)
    }
    #[doc = "Clear reset. I2C0 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `SSP1_RST_N`"]
pub enum SSP1_RST_NW {
    #[doc = "Reset. Resets the SSP1 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP1 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1_RST_NW::RESET => false,
            SSP1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSP1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSP1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SSP1 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::RESET)
    }
    #[doc = "Clear reset. SSP1 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `I2C1_RST_N`"]
pub enum I2C1_RST_NW {
    #[doc = "Reset. Resets the I2C1 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C1 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1_RST_NW::RESET => false,
            I2C1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _I2C1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: I2C1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the I2C1 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::RESET)
    }
    #[doc = "Clear reset. I2C1 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `FRG_RST_N`"]
pub enum FRG_RST_NW {
    #[doc = "Reset. Resets the FRG peripheral."]
    RESET,
    #[doc = "Clear reset. FRG reset de-asserted."]
    CLEAR_RESET,
}
impl FRG_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRG_RST_NW::RESET => false,
            FRG_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRG_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FRG_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRG_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the FRG peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRG_RST_NW::RESET)
    }
    #[doc = "Clear reset. FRG reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(FRG_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `USART1_RST_N`"]
pub enum USART1_RST_NW {
    #[doc = "Reset. Resets the USART1 peripheral."]
    RESET,
    #[doc = "Clear reset. USART1 reset de-asserted."]
    CLEAR_RESET,
}
impl USART1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART1_RST_NW::RESET => false,
            USART1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART1 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART1_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART1 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART1_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `USART2_RST_N`"]
pub enum USART2_RST_NW {
    #[doc = "Reset. Resets the USART2 peripheral."]
    RESET,
    #[doc = "Clear reset. USART2 reset de-asserted."]
    CLEAR_RESET,
}
impl USART2_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART2_RST_NW::RESET => false,
            USART2_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART2_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART2_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART2 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART2 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART2_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `USART3_RST_N`"]
pub enum USART3_RST_NW {
    #[doc = "Reset. Resets the USART3 peripheral."]
    RESET,
    #[doc = "Clear reset. USART3 reset de-asserted."]
    CLEAR_RESET,
}
impl USART3_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART3_RST_NW::RESET => false,
            USART3_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART3_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART3_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART3 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART3_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART3 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART3_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `USART4_RST_N`"]
pub enum USART4_RST_NW {
    #[doc = "Reset. Resets the USART4 peripheral."]
    RESET,
    #[doc = "Clear reset. USART4 reset de-asserted."]
    CLEAR_RESET,
}
impl USART4_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            USART4_RST_NW::RESET => false,
            USART4_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _USART4_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART4_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: USART4_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART4 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART4_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART4 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART4_RST_NW::CLEAR_RESET)
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
#[doc = "Values that can be written to the field `SCT0_RST_N`"]
pub enum SCT0_RST_NW {
    #[doc = "Reset. Resets the SCT0 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT0 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT0_RST_NW::RESET => false,
            SCT0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SCT0 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SCT0_RST_NW::RESET)
    }
    #[doc = "Clear reset. SCT0 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SCT0_RST_NW::CLEAR_RESET)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCT1_RST_N`"]
pub enum SCT1_RST_NW {
    #[doc = "Reset. Resets the SCT1 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT1 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT1_RST_NW::RESET => false,
            SCT1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCT1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT1_RST_NW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCT1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SCT1 peripheral."]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(SCT1_RST_NW::RESET)
    }
    #[doc = "Clear reset. SCT1 reset de-asserted."]
    #[inline]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SCT1_RST_NW::CLEAR_RESET)
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
        const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_NR {
        SSP0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - I2C0 reset control"]
    #[inline]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_NR {
        I2C0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_NR {
        SSP1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - I2C1 reset control"]
    #[inline]
    pub fn i2c1_rst_n(&self) -> I2C1_RST_NR {
        I2C1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - FRG reset control"]
    #[inline]
    pub fn frg_rst_n(&self) -> FRG_RST_NR {
        FRG_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - USART1 reset control"]
    #[inline]
    pub fn usart1_rst_n(&self) -> USART1_RST_NR {
        USART1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - USART2 reset control"]
    #[inline]
    pub fn usart2_rst_n(&self) -> USART2_RST_NR {
        USART2_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - USART3 reset control"]
    #[inline]
    pub fn usart3_rst_n(&self) -> USART3_RST_NR {
        USART3_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - USART4 reset control"]
    #[inline]
    pub fn usart4_rst_n(&self) -> USART4_RST_NR {
        USART4_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SCT0 reset control"]
    #[inline]
    pub fn sct0_rst_n(&self) -> SCT0_RST_NR {
        SCT0_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - SCT1 reset control"]
    #[inline]
    pub fn sct1_rst_n(&self) -> SCT1_RST_NR {
        SCT1_RST_NR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline]
    pub fn ssp0_rst_n(&mut self) -> _SSP0_RST_NW {
        _SSP0_RST_NW { w: self }
    }
    #[doc = "Bit 1 - I2C0 reset control"]
    #[inline]
    pub fn i2c0_rst_n(&mut self) -> _I2C0_RST_NW {
        _I2C0_RST_NW { w: self }
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline]
    pub fn ssp1_rst_n(&mut self) -> _SSP1_RST_NW {
        _SSP1_RST_NW { w: self }
    }
    #[doc = "Bit 3 - I2C1 reset control"]
    #[inline]
    pub fn i2c1_rst_n(&mut self) -> _I2C1_RST_NW {
        _I2C1_RST_NW { w: self }
    }
    #[doc = "Bit 4 - FRG reset control"]
    #[inline]
    pub fn frg_rst_n(&mut self) -> _FRG_RST_NW {
        _FRG_RST_NW { w: self }
    }
    #[doc = "Bit 5 - USART1 reset control"]
    #[inline]
    pub fn usart1_rst_n(&mut self) -> _USART1_RST_NW {
        _USART1_RST_NW { w: self }
    }
    #[doc = "Bit 6 - USART2 reset control"]
    #[inline]
    pub fn usart2_rst_n(&mut self) -> _USART2_RST_NW {
        _USART2_RST_NW { w: self }
    }
    #[doc = "Bit 7 - USART3 reset control"]
    #[inline]
    pub fn usart3_rst_n(&mut self) -> _USART3_RST_NW {
        _USART3_RST_NW { w: self }
    }
    #[doc = "Bit 8 - USART4 reset control"]
    #[inline]
    pub fn usart4_rst_n(&mut self) -> _USART4_RST_NW {
        _USART4_RST_NW { w: self }
    }
    #[doc = "Bit 9 - SCT0 reset control"]
    #[inline]
    pub fn sct0_rst_n(&mut self) -> _SCT0_RST_NW {
        _SCT0_RST_NW { w: self }
    }
    #[doc = "Bit 10 - SCT1 reset control"]
    #[inline]
    pub fn sct1_rst_n(&mut self) -> _SCT1_RST_NW {
        _SCT1_RST_NW { w: self }
    }
}
