#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PDAWAKECFG {
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
#[doc = "Possible values of the field `FROOUT_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FROOUT_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FROOUT_PDR {
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
            FROOUT_PDR::POWERED => false,
            FROOUT_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FROOUT_PDR {
        match value {
            false => FROOUT_PDR::POWERED,
            true => FROOUT_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == FROOUT_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == FROOUT_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `FRO_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FRO_PDR {
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
            FRO_PDR::POWERED => false,
            FRO_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRO_PDR {
        match value {
            false => FRO_PDR::POWERED,
            true => FRO_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == FRO_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == FRO_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `FLASH_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FLASH_PDR {
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
            FLASH_PDR::POWERED => false,
            FLASH_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLASH_PDR {
        match value {
            false => FLASH_PDR::POWERED,
            true => FLASH_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == FLASH_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == FLASH_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `BOD_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl BOD_PDR {
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
            BOD_PDR::POWERED => false,
            BOD_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOD_PDR {
        match value {
            false => BOD_PDR::POWERED,
            true => BOD_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == BOD_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == BOD_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `ADC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl ADC_PDR {
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
            ADC_PDR::POWERED => false,
            ADC_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADC_PDR {
        match value {
            false => ADC_PDR::POWERED,
            true => ADC_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == ADC_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == ADC_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `SYSOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PDR {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl SYSOSC_PDR {
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
            SYSOSC_PDR::POWERED => false,
            SYSOSC_PDR::POWERED_DOWN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSOSC_PDR {
        match value {
            false => SYSOSC_PDR::POWERED,
            true => SYSOSC_PDR::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline]
    pub fn is_powered(&self) -> bool {
        *self == SYSOSC_PDR::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline]
    pub fn is_powered_down(&self) -> bool {
        *self == SYSOSC_PDR::POWERED_DOWN
    }
}
#[doc = "Possible values of the field `WDTOSC_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PDR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl WDTOSC_PDR {
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
            WDTOSC_PDR::DISABLED => false,
            WDTOSC_PDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDTOSC_PDR {
        match value {
            false => WDTOSC_PDR::DISABLED,
            true => WDTOSC_PDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == WDTOSC_PDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == WDTOSC_PDR::ENABLED
    }
}
#[doc = "Possible values of the field `SYSPLL_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PDR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYSPLL_PDR {
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
            SYSPLL_PDR::DISABLED => false,
            SYSPLL_PDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYSPLL_PDR {
        match value {
            false => SYSPLL_PDR::DISABLED,
            true => SYSPLL_PDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SYSPLL_PDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SYSPLL_PDR::ENABLED
    }
}
#[doc = "Possible values of the field `VREF2_PD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF2_PDR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl VREF2_PDR {
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
            VREF2_PDR::DISABLED => false,
            VREF2_PDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VREF2_PDR {
        match value {
            false => VREF2_PDR::DISABLED,
            true => VREF2_PDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == VREF2_PDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == VREF2_PDR::ENABLED
    }
}
#[doc = "Possible values of the field `DAC0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DAC0R {
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
            DAC0R::DISABLED => false,
            DAC0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC0R {
        match value {
            false => DAC0R::DISABLED,
            true => DAC0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DAC0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DAC0R::ENABLED
    }
}
#[doc = "Possible values of the field `DAC1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DAC1R {
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
            DAC1R::DISABLED => false,
            DAC1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DAC1R {
        match value {
            false => DAC1R::DISABLED,
            true => DAC1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DAC1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DAC1R::ENABLED
    }
}
#[doc = "Possible values of the field `ACMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMPR {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ACMPR {
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
            ACMPR::DISABLED => false,
            ACMPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACMPR {
        match value {
            false => ACMPR::DISABLED,
            true => ACMPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ACMPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ACMPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `FROOUT_PD`"]
pub enum FROOUT_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FROOUT_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FROOUT_PDW::POWERED => false,
            FROOUT_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FROOUT_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _FROOUT_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FROOUT_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(FROOUT_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FROOUT_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `FRO_PD`"]
pub enum FRO_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FRO_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRO_PDW::POWERED => false,
            FRO_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRO_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _FRO_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRO_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(FRO_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FRO_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `FLASH_PD`"]
pub enum FLASH_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl FLASH_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLASH_PDW::POWERED => false,
            FLASH_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLASH_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLASH_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `BOD_PD`"]
pub enum BOD_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl BOD_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOD_PDW::POWERED => false,
            BOD_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOD_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _BOD_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOD_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `ADC_PD`"]
pub enum ADC_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl ADC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADC_PDW::POWERED => false,
            ADC_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `SYSOSC_PD`"]
pub enum SYSOSC_PDW {
    #[doc = "powered"]
    POWERED,
    #[doc = "powered down"]
    POWERED_DOWN,
}
impl SYSOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSOSC_PDW::POWERED => false,
            SYSOSC_PDW::POWERED_DOWN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSOSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "powered"]
    #[inline]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED)
    }
    #[doc = "powered down"]
    #[inline]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PDW::POWERED_DOWN)
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
#[doc = "Values that can be written to the field `WDTOSC_PD`"]
pub enum WDTOSC_PDW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl WDTOSC_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDTOSC_PDW::DISABLED => false,
            WDTOSC_PDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDTOSC_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _WDTOSC_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDTOSC_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDTOSC_PDW::ENABLED)
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
#[doc = "Values that can be written to the field `SYSPLL_PD`"]
pub enum SYSPLL_PDW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl SYSPLL_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYSPLL_PDW::DISABLED => false,
            SYSPLL_PDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYSPLL_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSPLL_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYSPLL_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSPLL_PDW::ENABLED)
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
#[doc = "Values that can be written to the field `VREF2_PD`"]
pub enum VREF2_PDW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl VREF2_PDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VREF2_PDW::DISABLED => false,
            VREF2_PDW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VREF2_PDW<'a> {
    w: &'a mut W,
}
impl<'a> _VREF2_PDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VREF2_PDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREF2_PDW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREF2_PDW::ENABLED)
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
#[doc = "Values that can be written to the field `DAC0`"]
pub enum DAC0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DAC0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC0W::DISABLED => false,
            DAC0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC0W<'a> {
    w: &'a mut W,
}
impl<'a> _DAC0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC0W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC0W::ENABLED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DAC1`"]
pub enum DAC1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl DAC1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DAC1W::DISABLED => false,
            DAC1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAC1W<'a> {
    w: &'a mut W,
}
impl<'a> _DAC1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAC1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC1W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC1W::ENABLED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACMP`"]
pub enum ACMPW {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ACMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACMPW::DISABLED => false,
            ACMPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACMPW<'a> {
    w: &'a mut W,
}
impl<'a> _ACMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMPW::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMPW::ENABLED)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - FRO oscillator output wake-up configuration"]
    #[inline]
    pub fn froout_pd(&self) -> FROOUT_PDR {
        FROOUT_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FRO oscillator power-down wake-up configuration"]
    #[inline]
    pub fn fro_pd(&self) -> FRO_PDR {
        FRO_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline]
    pub fn flash_pd(&self) -> FLASH_PDR {
        FLASH_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline]
    pub fn bod_pd(&self) -> BOD_PDR {
        BOD_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline]
    pub fn adc_pd(&self) -> ADC_PDR {
        ADC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline]
    pub fn sysosc_pd(&self) -> SYSOSC_PDR {
        SYSOSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline]
    pub fn wdtosc_pd(&self) -> WDTOSC_PDR {
        WDTOSC_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline]
    pub fn syspll_pd(&self) -> SYSPLL_PDR {
        SYSPLL_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - VREF2 wake-up configuration"]
    #[inline]
    pub fn vref2_pd(&self) -> VREF2_PDR {
        VREF2_PDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - DAC0 wake-up configuration"]
    #[inline]
    pub fn dac0(&self) -> DAC0R {
        DAC0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - DAC1 wake-up configuration"]
    #[inline]
    pub fn dac1(&self) -> DAC1R {
        DAC1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline]
    pub fn acmp(&self) -> ACMPR {
        ACMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 60920 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - FRO oscillator output wake-up configuration"]
    #[inline]
    pub fn froout_pd(&mut self) -> _FROOUT_PDW {
        _FROOUT_PDW { w: self }
    }
    #[doc = "Bit 1 - FRO oscillator power-down wake-up configuration"]
    #[inline]
    pub fn fro_pd(&mut self) -> _FRO_PDW {
        _FRO_PDW { w: self }
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline]
    pub fn flash_pd(&mut self) -> _FLASH_PDW {
        _FLASH_PDW { w: self }
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline]
    pub fn bod_pd(&mut self) -> _BOD_PDW {
        _BOD_PDW { w: self }
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline]
    pub fn adc_pd(&mut self) -> _ADC_PDW {
        _ADC_PDW { w: self }
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline]
    pub fn sysosc_pd(&mut self) -> _SYSOSC_PDW {
        _SYSOSC_PDW { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline]
    pub fn wdtosc_pd(&mut self) -> _WDTOSC_PDW {
        _WDTOSC_PDW { w: self }
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline]
    pub fn syspll_pd(&mut self) -> _SYSPLL_PDW {
        _SYSPLL_PDW { w: self }
    }
    #[doc = "Bit 10 - VREF2 wake-up configuration"]
    #[inline]
    pub fn vref2_pd(&mut self) -> _VREF2_PDW {
        _VREF2_PDW { w: self }
    }
    #[doc = "Bit 13 - DAC0 wake-up configuration"]
    #[inline]
    pub fn dac0(&mut self) -> _DAC0W {
        _DAC0W { w: self }
    }
    #[doc = "Bit 14 - DAC1 wake-up configuration"]
    #[inline]
    pub fn dac1(&mut self) -> _DAC1W {
        _DAC1W { w: self }
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline]
    pub fn acmp(&mut self) -> _ACMPW {
        _ACMPW { w: self }
    }
}
