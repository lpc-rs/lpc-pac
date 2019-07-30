#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PRESETCTRL {
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
#[doc = "Possible values of the field `SSP0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_NR {
    #[doc = "Reset. Resets the SSP0 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    CLEAR_RESET,
}
impl crate::ToBits<bool> for SSP0_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSP0_RST_NR::RESET => false,
            SSP0_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSP0_RST_N_R = crate::FR<bool, SSP0_RST_NR>;
impl SSP0_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SSP0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == SSP0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `SSP0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_RST_NW {
    #[doc = "Reset. Resets the SSP0 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP0_RST_NW::RESET => false,
            SSP0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSP0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP0_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SSP0 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::RESET)
    }
    #[doc = "Clear reset. SSP0 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SSP0_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
impl crate::ToBits<bool> for I2C0_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C0_RST_NR::RESET => false,
            I2C0_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C0_RST_N_R = crate::FR<bool, I2C0_RST_NR>;
impl I2C0_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == I2C0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `I2C0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_RST_NW {
    #[doc = "Reset. Resets the I2C0 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C0 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C0_RST_NW::RESET => false,
            I2C0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C0_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the I2C0 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::RESET)
    }
    #[doc = "Clear reset. I2C0 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(I2C0_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
impl crate::ToBits<bool> for SSP1_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SSP1_RST_NR::RESET => false,
            SSP1_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SSP1_RST_N_R = crate::FR<bool, SSP1_RST_NR>;
impl SSP1_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SSP1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == SSP1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `SSP1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_RST_NW {
    #[doc = "Reset. Resets the SSP1 peripheral."]
    RESET,
    #[doc = "Clear reset. SSP1 reset de-asserted."]
    CLEAR_RESET,
}
impl SSP1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SSP1_RST_NW::RESET => false,
            SSP1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSP1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SSP1_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SSP1 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::RESET)
    }
    #[doc = "Clear reset. SSP1 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SSP1_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
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
impl crate::ToBits<bool> for I2C1_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            I2C1_RST_NR::RESET => false,
            I2C1_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type I2C1_RST_N_R = crate::FR<bool, I2C1_RST_NR>;
impl I2C1_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == I2C1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == I2C1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `I2C1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_RST_NW {
    #[doc = "Reset. Resets the I2C1 peripheral."]
    RESET,
    #[doc = "Clear reset. I2C1 reset de-asserted."]
    CLEAR_RESET,
}
impl I2C1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            I2C1_RST_NW::RESET => false,
            I2C1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _I2C1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C1_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the I2C1 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::RESET)
    }
    #[doc = "Clear reset. I2C1 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(I2C1_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
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
impl crate::ToBits<bool> for FRG_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            FRG_RST_NR::RESET => false,
            FRG_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type FRG_RST_N_R = crate::FR<bool, FRG_RST_NR>;
impl FRG_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == FRG_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == FRG_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `FRG_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRG_RST_NW {
    #[doc = "Reset. Resets the FRG peripheral."]
    RESET,
    #[doc = "Clear reset. FRG reset de-asserted."]
    CLEAR_RESET,
}
impl FRG_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            FRG_RST_NW::RESET => false,
            FRG_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FRG_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _FRG_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRG_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the FRG peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(FRG_RST_NW::RESET)
    }
    #[doc = "Clear reset. FRG reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(FRG_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
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
impl crate::ToBits<bool> for USART1_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART1_RST_NR::RESET => false,
            USART1_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART1_RST_N_R = crate::FR<bool, USART1_RST_NR>;
impl USART1_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `USART1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART1_RST_NW {
    #[doc = "Reset. Resets the USART1 peripheral."]
    RESET,
    #[doc = "Clear reset. USART1 reset de-asserted."]
    CLEAR_RESET,
}
impl USART1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART1_RST_NW::RESET => false,
            USART1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART1_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART1 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART1_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART1 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART1_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
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
impl crate::ToBits<bool> for USART2_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART2_RST_NR::RESET => false,
            USART2_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART2_RST_N_R = crate::FR<bool, USART2_RST_NR>;
impl USART2_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART2_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART2_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `USART2_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART2_RST_NW {
    #[doc = "Reset. Resets the USART2 peripheral."]
    RESET,
    #[doc = "Clear reset. USART2 reset de-asserted."]
    CLEAR_RESET,
}
impl USART2_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART2_RST_NW::RESET => false,
            USART2_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART2_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART2_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART2_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART2 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART2_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART2 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART2_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
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
impl crate::ToBits<bool> for USART3_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART3_RST_NR::RESET => false,
            USART3_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART3_RST_N_R = crate::FR<bool, USART3_RST_NR>;
impl USART3_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART3_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART3_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `USART3_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART3_RST_NW {
    #[doc = "Reset. Resets the USART3 peripheral."]
    RESET,
    #[doc = "Clear reset. USART3 reset de-asserted."]
    CLEAR_RESET,
}
impl USART3_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART3_RST_NW::RESET => false,
            USART3_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART3_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART3_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART3_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART3 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART3_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART3 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART3_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
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
impl crate::ToBits<bool> for USART4_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            USART4_RST_NR::RESET => false,
            USART4_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type USART4_RST_N_R = crate::FR<bool, USART4_RST_NR>;
impl USART4_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == USART4_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == USART4_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `USART4_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART4_RST_NW {
    #[doc = "Reset. Resets the USART4 peripheral."]
    RESET,
    #[doc = "Clear reset. USART4 reset de-asserted."]
    CLEAR_RESET,
}
impl USART4_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            USART4_RST_NW::RESET => false,
            USART4_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USART4_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _USART4_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART4_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the USART4 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(USART4_RST_NW::RESET)
    }
    #[doc = "Clear reset. USART4 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(USART4_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
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
impl crate::ToBits<bool> for SCT0_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCT0_RST_NR::RESET => false,
            SCT0_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCT0_RST_N_R = crate::FR<bool, SCT0_RST_NR>;
impl SCT0_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SCT0_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == SCT0_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `SCT0_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT0_RST_NW {
    #[doc = "Reset. Resets the SCT0 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT0 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT0_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT0_RST_NW::RESET => false,
            SCT0_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCT0_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT0_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT0_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SCT0 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SCT0_RST_NW::RESET)
    }
    #[doc = "Clear reset. SCT0 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SCT0_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
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
impl crate::ToBits<bool> for SCT1_RST_NR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCT1_RST_NR::RESET => false,
            SCT1_RST_NR::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCT1_RST_N_R = crate::FR<bool, SCT1_RST_NR>;
impl SCT1_RST_N_R {
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SCT1_RST_NR::RESET
    }
    #[doc = "Checks if the value of the field is `CLEAR_RESET`"]
    #[inline(always)]
    pub fn is_clear_reset(&self) -> bool {
        *self == SCT1_RST_NR::CLEAR_RESET
    }
}
#[doc = "Values that can be written to the field `SCT1_RST_N`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT1_RST_NW {
    #[doc = "Reset. Resets the SCT1 peripheral."]
    RESET,
    #[doc = "Clear reset. SCT1 reset de-asserted."]
    CLEAR_RESET,
}
impl SCT1_RST_NW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCT1_RST_NW::RESET => false,
            SCT1_RST_NW::CLEAR_RESET => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCT1_RST_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SCT1_RST_NW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT1_RST_NW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset. Resets the SCT1 peripheral."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SCT1_RST_NW::RESET)
    }
    #[doc = "Clear reset. SCT1 reset de-asserted."]
    #[inline(always)]
    pub fn clear_reset(self) -> &'a mut W {
        self.variant(SCT1_RST_NW::CLEAR_RESET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&self) -> SSP0_RST_N_R {
        SSP0_RST_N_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C0 reset control"]
    #[inline(always)]
    pub fn i2c0_rst_n(&self) -> I2C0_RST_N_R {
        I2C0_RST_N_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&self) -> SSP1_RST_N_R {
        SSP1_RST_N_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C1 reset control"]
    #[inline(always)]
    pub fn i2c1_rst_n(&self) -> I2C1_RST_N_R {
        I2C1_RST_N_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FRG reset control"]
    #[inline(always)]
    pub fn frg_rst_n(&self) -> FRG_RST_N_R {
        FRG_RST_N_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USART1 reset control"]
    #[inline(always)]
    pub fn usart1_rst_n(&self) -> USART1_RST_N_R {
        USART1_RST_N_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - USART2 reset control"]
    #[inline(always)]
    pub fn usart2_rst_n(&self) -> USART2_RST_N_R {
        USART2_RST_N_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - USART3 reset control"]
    #[inline(always)]
    pub fn usart3_rst_n(&self) -> USART3_RST_N_R {
        USART3_RST_N_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USART4 reset control"]
    #[inline(always)]
    pub fn usart4_rst_n(&self) -> USART4_RST_N_R {
        USART4_RST_N_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SCT0 reset control"]
    #[inline(always)]
    pub fn sct0_rst_n(&self) -> SCT0_RST_N_R {
        SCT0_RST_N_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SCT1 reset control"]
    #[inline(always)]
    pub fn sct1_rst_n(&self) -> SCT1_RST_N_R {
        SCT1_RST_N_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSP0 reset control"]
    #[inline(always)]
    pub fn ssp0_rst_n(&mut self) -> _SSP0_RST_NW {
        _SSP0_RST_NW { w: self }
    }
    #[doc = "Bit 1 - I2C0 reset control"]
    #[inline(always)]
    pub fn i2c0_rst_n(&mut self) -> _I2C0_RST_NW {
        _I2C0_RST_NW { w: self }
    }
    #[doc = "Bit 2 - SSP1 reset control"]
    #[inline(always)]
    pub fn ssp1_rst_n(&mut self) -> _SSP1_RST_NW {
        _SSP1_RST_NW { w: self }
    }
    #[doc = "Bit 3 - I2C1 reset control"]
    #[inline(always)]
    pub fn i2c1_rst_n(&mut self) -> _I2C1_RST_NW {
        _I2C1_RST_NW { w: self }
    }
    #[doc = "Bit 4 - FRG reset control"]
    #[inline(always)]
    pub fn frg_rst_n(&mut self) -> _FRG_RST_NW {
        _FRG_RST_NW { w: self }
    }
    #[doc = "Bit 5 - USART1 reset control"]
    #[inline(always)]
    pub fn usart1_rst_n(&mut self) -> _USART1_RST_NW {
        _USART1_RST_NW { w: self }
    }
    #[doc = "Bit 6 - USART2 reset control"]
    #[inline(always)]
    pub fn usart2_rst_n(&mut self) -> _USART2_RST_NW {
        _USART2_RST_NW { w: self }
    }
    #[doc = "Bit 7 - USART3 reset control"]
    #[inline(always)]
    pub fn usart3_rst_n(&mut self) -> _USART3_RST_NW {
        _USART3_RST_NW { w: self }
    }
    #[doc = "Bit 8 - USART4 reset control"]
    #[inline(always)]
    pub fn usart4_rst_n(&mut self) -> _USART4_RST_NW {
        _USART4_RST_NW { w: self }
    }
    #[doc = "Bit 9 - SCT0 reset control"]
    #[inline(always)]
    pub fn sct0_rst_n(&mut self) -> _SCT0_RST_NW {
        _SCT0_RST_NW { w: self }
    }
    #[doc = "Bit 10 - SCT1 reset control"]
    #[inline(always)]
    pub fn sct1_rst_n(&mut self) -> _SCT1_RST_NW {
        _SCT1_RST_NW { w: self }
    }
}
