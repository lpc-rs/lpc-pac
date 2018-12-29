#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVCMDSTAT {
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
#[doc = r" Value of the field"]
pub struct DEV_ADDRR {
    bits: u8,
}
impl DEV_ADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DEV_ENR {
    bits: bool,
}
impl DEV_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct SETUPR {
    bits: bool,
}
impl SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `PLL_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_ONR {
    #[doc = "USB_NeedClk functional"]
    USB_NEEDCLK_FUNCTION,
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    USB_NEEDCLK_ALWAYS_1,
}
impl PLL_ONR {
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
            PLL_ONR::USB_NEEDCLK_FUNCTION => false,
            PLL_ONR::USB_NEEDCLK_ALWAYS_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLL_ONR {
        match value {
            false => PLL_ONR::USB_NEEDCLK_FUNCTION,
            true => PLL_ONR::USB_NEEDCLK_ALWAYS_1,
        }
    }
    #[doc = "Checks if the value of the field is `USB_NEEDCLK_FUNCTION`"]
    #[inline]
    pub fn is_usb_needclk_function(&self) -> bool {
        *self == PLL_ONR::USB_NEEDCLK_FUNCTION
    }
    #[doc = "Checks if the value of the field is `USB_NEEDCLK_ALWAYS_1`"]
    #[inline]
    pub fn is_usb_needclk_always_1(&self) -> bool {
        *self == PLL_ONR::USB_NEEDCLK_ALWAYS_1
    }
}
#[doc = "Possible values of the field `LPM_SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_SUPR {
    #[doc = "LPM not supported."]
    LPM_NOT_SUPPORTED_,
    #[doc = "LPM supported."]
    LPM_SUPPORTED_,
}
impl LPM_SUPR {
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
            LPM_SUPR::LPM_NOT_SUPPORTED_ => false,
            LPM_SUPR::LPM_SUPPORTED_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPM_SUPR {
        match value {
            false => LPM_SUPR::LPM_NOT_SUPPORTED_,
            true => LPM_SUPR::LPM_SUPPORTED_,
        }
    }
    #[doc = "Checks if the value of the field is `LPM_NOT_SUPPORTED_`"]
    #[inline]
    pub fn is_lpm_not_supported_(&self) -> bool {
        *self == LPM_SUPR::LPM_NOT_SUPPORTED_
    }
    #[doc = "Checks if the value of the field is `LPM_SUPPORTED_`"]
    #[inline]
    pub fn is_lpm_supported_(&self) -> bool {
        *self == LPM_SUPR::LPM_SUPPORTED_
    }
}
#[doc = "Possible values of the field `INTONNAK_AO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AOR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_AOR {
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
            INTONNAK_AOR::ACKNOW => false,
            INTONNAK_AOR::ACKNOW_NAK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTONNAK_AOR {
        match value {
            false => INTONNAK_AOR::ACKNOW,
            true => INTONNAK_AOR::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_AOR::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_AOR::ACKNOW_NAK
    }
}
#[doc = "Possible values of the field `INTONNAK_AI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AIR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_AIR {
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
            INTONNAK_AIR::ACKNOW => false,
            INTONNAK_AIR::ACKNOW_NAK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTONNAK_AIR {
        match value {
            false => INTONNAK_AIR::ACKNOW,
            true => INTONNAK_AIR::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_AIR::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_AIR::ACKNOW_NAK
    }
}
#[doc = "Possible values of the field `INTONNAK_CO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_COR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_COR {
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
            INTONNAK_COR::ACKNOW => false,
            INTONNAK_COR::ACKNOW_NAK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTONNAK_COR {
        match value {
            false => INTONNAK_COR::ACKNOW,
            true => INTONNAK_COR::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_COR::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_COR::ACKNOW_NAK
    }
}
#[doc = "Possible values of the field `INTONNAK_CI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_CIR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_CIR {
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
            INTONNAK_CIR::ACKNOW => false,
            INTONNAK_CIR::ACKNOW_NAK => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTONNAK_CIR {
        match value {
            false => INTONNAK_CIR::ACKNOW,
            true => INTONNAK_CIR::ACKNOW_NAK,
        }
    }
    #[doc = "Checks if the value of the field is `ACKNOW`"]
    #[inline]
    pub fn is_acknow(&self) -> bool {
        *self == INTONNAK_CIR::ACKNOW
    }
    #[doc = "Checks if the value of the field is `ACKNOW_NAK`"]
    #[inline]
    pub fn is_acknow_nak(&self) -> bool {
        *self == INTONNAK_CIR::ACKNOW_NAK
    }
}
#[doc = r" Value of the field"]
pub struct DCONR {
    bits: bool,
}
impl DCONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DSUSR {
    bits: bool,
}
impl DSUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LPM_SUSR {
    bits: bool,
}
impl LPM_SUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LPM_REWPR {
    bits: bool,
}
impl LPM_REWPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DCON_CR {
    bits: bool,
}
impl DCON_CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DSUS_CR {
    bits: bool,
}
impl DSUS_CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct DRES_CR {
    bits: bool,
}
impl DRES_CR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct VBUSDEBOUNCEDR {
    bits: bool,
}
impl VBUSDEBOUNCEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Proxy"]
pub struct _DEV_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DEV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SETUPW<'a> {
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
#[doc = "Values that can be written to the field `PLL_ON`"]
pub enum PLL_ONW {
    #[doc = "USB_NeedClk functional"]
    USB_NEEDCLK_FUNCTION,
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    USB_NEEDCLK_ALWAYS_1,
}
impl PLL_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_ONW::USB_NEEDCLK_FUNCTION => false,
            PLL_ONW::USB_NEEDCLK_ALWAYS_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLL_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_ONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLL_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB_NeedClk functional"]
    #[inline]
    pub fn usb_needclk_function(self) -> &'a mut W {
        self.variant(PLL_ONW::USB_NEEDCLK_FUNCTION)
    }
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    #[inline]
    pub fn usb_needclk_always_1(self) -> &'a mut W {
        self.variant(PLL_ONW::USB_NEEDCLK_ALWAYS_1)
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
#[doc = "Values that can be written to the field `LPM_SUP`"]
pub enum LPM_SUPW {
    #[doc = "LPM not supported."]
    LPM_NOT_SUPPORTED_,
    #[doc = "LPM supported."]
    LPM_SUPPORTED_,
}
impl LPM_SUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPM_SUPW::LPM_NOT_SUPPORTED_ => false,
            LPM_SUPW::LPM_SUPPORTED_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPM_SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPM_SUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPM not supported."]
    #[inline]
    pub fn lpm_not_supported_(self) -> &'a mut W {
        self.variant(LPM_SUPW::LPM_NOT_SUPPORTED_)
    }
    #[doc = "LPM supported."]
    #[inline]
    pub fn lpm_supported_(self) -> &'a mut W {
        self.variant(LPM_SUPW::LPM_SUPPORTED_)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTONNAK_AO`"]
pub enum INTONNAK_AOW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_AOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AOW::ACKNOW => false,
            INTONNAK_AOW::ACKNOW_NAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTONNAK_AOW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTONNAK_AOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_AOW::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_AOW::ACKNOW_NAK)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTONNAK_AI`"]
pub enum INTONNAK_AIW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AIW::ACKNOW => false,
            INTONNAK_AIW::ACKNOW_NAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTONNAK_AIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTONNAK_AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_AIW::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_AIW::ACKNOW_NAK)
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
#[doc = "Values that can be written to the field `INTONNAK_CO`"]
pub enum INTONNAK_COW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_COW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_COW::ACKNOW => false,
            INTONNAK_COW::ACKNOW_NAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTONNAK_COW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_COW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTONNAK_COW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_COW::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_COW::ACKNOW_NAK)
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
#[doc = "Values that can be written to the field `INTONNAK_CI`"]
pub enum INTONNAK_CIW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ACKNOW,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    ACKNOW_NAK,
}
impl INTONNAK_CIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_CIW::ACKNOW => false,
            INTONNAK_CIW::ACKNOW_NAK => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTONNAK_CIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_CIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTONNAK_CIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline]
    pub fn acknow(self) -> &'a mut W {
        self.variant(INTONNAK_CIW::ACKNOW)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline]
    pub fn acknow_nak(self) -> &'a mut W {
        self.variant(INTONNAK_CIW::ACKNOW_NAK)
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
#[doc = r" Proxy"]
pub struct _DCONW<'a> {
    w: &'a mut W,
}
impl<'a> _DCONW<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUSW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_SUSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUSW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LPM_REWPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_REWPW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCON_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DCON_CW<'a> {
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DSUS_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUS_CW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DRES_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DRES_CW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VBUSDEBOUNCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSDEBOUNCEDW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline]
    pub fn dev_addr(&self) -> DEV_ADDRR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DEV_ADDRR { bits }
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline]
    pub fn dev_en(&self) -> DEV_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DEV_ENR { bits }
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline]
    pub fn setup(&self) -> SETUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SETUPR { bits }
    }
    #[doc = "Bit 9 - Always PLL Clock on:"]
    #[inline]
    pub fn pll_on(&self) -> PLL_ONR {
        PLL_ONR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline]
    pub fn lpm_sup(&self) -> LPM_SUPR {
        LPM_SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline]
    pub fn intonnak_ao(&self) -> INTONNAK_AOR {
        INTONNAK_AOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline]
    pub fn intonnak_ai(&self) -> INTONNAK_AIR {
        INTONNAK_AIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline]
    pub fn intonnak_co(&self) -> INTONNAK_COR {
        INTONNAK_COR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline]
    pub fn intonnak_ci(&self) -> INTONNAK_CIR {
        INTONNAK_CIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline]
    pub fn dcon(&self) -> DCONR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCONR { bits }
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline]
    pub fn dsus(&self) -> DSUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSUSR { bits }
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline]
    pub fn lpm_sus(&self) -> LPM_SUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_SUSR { bits }
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline]
    pub fn lpm_rewp(&self) -> LPM_REWPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LPM_REWPR { bits }
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline]
    pub fn dcon_c(&self) -> DCON_CR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DCON_CR { bits }
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline]
    pub fn dsus_c(&self) -> DSUS_CR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSUS_CR { bits }
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline]
    pub fn dres_c(&self) -> DRES_CR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DRES_CR { bits }
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline]
    pub fn vbusdebounced(&self) -> VBUSDEBOUNCEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSDEBOUNCEDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2048 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline]
    pub fn dev_addr(&mut self) -> _DEV_ADDRW {
        _DEV_ADDRW { w: self }
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline]
    pub fn dev_en(&mut self) -> _DEV_ENW {
        _DEV_ENW { w: self }
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline]
    pub fn setup(&mut self) -> _SETUPW {
        _SETUPW { w: self }
    }
    #[doc = "Bit 9 - Always PLL Clock on:"]
    #[inline]
    pub fn pll_on(&mut self) -> _PLL_ONW {
        _PLL_ONW { w: self }
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline]
    pub fn lpm_sup(&mut self) -> _LPM_SUPW {
        _LPM_SUPW { w: self }
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline]
    pub fn intonnak_ao(&mut self) -> _INTONNAK_AOW {
        _INTONNAK_AOW { w: self }
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline]
    pub fn intonnak_ai(&mut self) -> _INTONNAK_AIW {
        _INTONNAK_AIW { w: self }
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline]
    pub fn intonnak_co(&mut self) -> _INTONNAK_COW {
        _INTONNAK_COW { w: self }
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline]
    pub fn intonnak_ci(&mut self) -> _INTONNAK_CIW {
        _INTONNAK_CIW { w: self }
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline]
    pub fn dcon(&mut self) -> _DCONW {
        _DCONW { w: self }
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline]
    pub fn dsus(&mut self) -> _DSUSW {
        _DSUSW { w: self }
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline]
    pub fn lpm_sus(&mut self) -> _LPM_SUSW {
        _LPM_SUSW { w: self }
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline]
    pub fn lpm_rewp(&mut self) -> _LPM_REWPW {
        _LPM_REWPW { w: self }
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline]
    pub fn dcon_c(&mut self) -> _DCON_CW {
        _DCON_CW { w: self }
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline]
    pub fn dsus_c(&mut self) -> _DSUS_CW {
        _DSUS_CW { w: self }
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline]
    pub fn dres_c(&mut self) -> _DRES_CW {
        _DRES_CW { w: self }
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline]
    pub fn vbusdebounced(&mut self) -> _VBUSDEBOUNCEDW {
        _VBUSDEBOUNCEDW { w: self }
    }
}
