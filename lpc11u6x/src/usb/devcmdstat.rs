#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVCMDSTAT {
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
        0x0800
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Reader of the field"]
pub type DEV_ADDR_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DEV_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ADDRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DEV_EN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DEV_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DEV_ENW<'a> {
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
#[doc = r"Reader of the field"]
pub type SETUP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SETUPW<'a> {
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
#[doc = "Possible values of the field `PLL_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_ONR {
    #[doc = "USB_NeedClk functional"]
    USB_NEEDCLK_FUNCTION,
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    USB_NEEDCLK_ALWAYS_1,
}
impl crate::ToBits<bool> for PLL_ONR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PLL_ONR::USB_NEEDCLK_FUNCTION => false,
            PLL_ONR::USB_NEEDCLK_ALWAYS_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PLL_ON_R = crate::FR<bool, PLL_ONR>;
impl PLL_ON_R {
    #[doc = "Checks if the value of the field is `USB_NEEDCLK_FUNCTION`"]
    #[inline(always)]
    pub fn is_usb_needclk_function(&self) -> bool {
        *self == PLL_ONR::USB_NEEDCLK_FUNCTION
    }
    #[doc = "Checks if the value of the field is `USB_NEEDCLK_ALWAYS_1`"]
    #[inline(always)]
    pub fn is_usb_needclk_always_1(&self) -> bool {
        *self == PLL_ONR::USB_NEEDCLK_ALWAYS_1
    }
}
#[doc = "Values that can be written to the field `PLL_ON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLL_ONW {
    #[doc = "USB_NeedClk functional"]
    USB_NEEDCLK_FUNCTION,
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    USB_NEEDCLK_ALWAYS_1,
}
impl PLL_ONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PLL_ONW::USB_NEEDCLK_FUNCTION => false,
            PLL_ONW::USB_NEEDCLK_ALWAYS_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PLL_ONW<'a> {
    w: &'a mut W,
}
impl<'a> _PLL_ONW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLL_ONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "USB_NeedClk functional"]
    #[inline(always)]
    pub fn usb_needclk_function(self) -> &'a mut W {
        self.variant(PLL_ONW::USB_NEEDCLK_FUNCTION)
    }
    #[doc = "USB_NeedClk always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn usb_needclk_always_1(self) -> &'a mut W {
        self.variant(PLL_ONW::USB_NEEDCLK_ALWAYS_1)
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
#[doc = "Possible values of the field `LPM_SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_SUPR {
    #[doc = "LPM not supported."]
    LPM_NOT_SUPPORTED,
    #[doc = "LPM supported."]
    LPM_SUPPORTED,
}
impl crate::ToBits<bool> for LPM_SUPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            LPM_SUPR::LPM_NOT_SUPPORTED => false,
            LPM_SUPR::LPM_SUPPORTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type LPM_SUP_R = crate::FR<bool, LPM_SUPR>;
impl LPM_SUP_R {
    #[doc = "Checks if the value of the field is `LPM_NOT_SUPPORTED`"]
    #[inline(always)]
    pub fn is_lpm_not_supported(&self) -> bool {
        *self == LPM_SUPR::LPM_NOT_SUPPORTED
    }
    #[doc = "Checks if the value of the field is `LPM_SUPPORTED`"]
    #[inline(always)]
    pub fn is_lpm_supported(&self) -> bool {
        *self == LPM_SUPR::LPM_SUPPORTED
    }
}
#[doc = "Values that can be written to the field `LPM_SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_SUPW {
    #[doc = "LPM not supported."]
    LPM_NOT_SUPPORTED,
    #[doc = "LPM supported."]
    LPM_SUPPORTED,
}
impl LPM_SUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            LPM_SUPW::LPM_NOT_SUPPORTED => false,
            LPM_SUPW::LPM_SUPPORTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _LPM_SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_SUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn lpm_not_supported(self) -> &'a mut W {
        self.variant(LPM_SUPW::LPM_NOT_SUPPORTED)
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn lpm_supported(self) -> &'a mut W {
        self.variant(LPM_SUPW::LPM_SUPPORTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `INTONNAK_AO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AOR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl crate::ToBits<bool> for INTONNAK_AOR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AOR::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_AOR::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTONNAK_AO_R = crate::FR<bool, INTONNAK_AOR>;
impl INTONNAK_AO_R {
    #[doc = "Checks if the value of the field is `ONLY_ACKNOWLEDGED_PA`"]
    #[inline(always)]
    pub fn is_only_acknowledged_pa(&self) -> bool {
        *self == INTONNAK_AOR::ONLY_ACKNOWLEDGED_PA
    }
    #[doc = "Checks if the value of the field is `BOTH_ACKNOWLEDGED_AN`"]
    #[inline(always)]
    pub fn is_both_acknowledged_an(&self) -> bool {
        *self == INTONNAK_AOR::BOTH_ACKNOWLEDGED_AN
    }
}
#[doc = "Values that can be written to the field `INTONNAK_AO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AOW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl INTONNAK_AOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AOW::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_AOW::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTONNAK_AOW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_AOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn only_acknowledged_pa(self) -> &'a mut W {
        self.variant(INTONNAK_AOW::ONLY_ACKNOWLEDGED_PA)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn both_acknowledged_an(self) -> &'a mut W {
        self.variant(INTONNAK_AOW::BOTH_ACKNOWLEDGED_AN)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `INTONNAK_AI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AIR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl crate::ToBits<bool> for INTONNAK_AIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AIR::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_AIR::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTONNAK_AI_R = crate::FR<bool, INTONNAK_AIR>;
impl INTONNAK_AI_R {
    #[doc = "Checks if the value of the field is `ONLY_ACKNOWLEDGED_PA`"]
    #[inline(always)]
    pub fn is_only_acknowledged_pa(&self) -> bool {
        *self == INTONNAK_AIR::ONLY_ACKNOWLEDGED_PA
    }
    #[doc = "Checks if the value of the field is `BOTH_ACKNOWLEDGED_AN`"]
    #[inline(always)]
    pub fn is_both_acknowledged_an(&self) -> bool {
        *self == INTONNAK_AIR::BOTH_ACKNOWLEDGED_AN
    }
}
#[doc = "Values that can be written to the field `INTONNAK_AI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AIW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl INTONNAK_AIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_AIW::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_AIW::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTONNAK_AIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_AIW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_AIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn only_acknowledged_pa(self) -> &'a mut W {
        self.variant(INTONNAK_AIW::ONLY_ACKNOWLEDGED_PA)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn both_acknowledged_an(self) -> &'a mut W {
        self.variant(INTONNAK_AIW::BOTH_ACKNOWLEDGED_AN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `INTONNAK_CO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_COR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl crate::ToBits<bool> for INTONNAK_COR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTONNAK_COR::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_COR::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTONNAK_CO_R = crate::FR<bool, INTONNAK_COR>;
impl INTONNAK_CO_R {
    #[doc = "Checks if the value of the field is `ONLY_ACKNOWLEDGED_PA`"]
    #[inline(always)]
    pub fn is_only_acknowledged_pa(&self) -> bool {
        *self == INTONNAK_COR::ONLY_ACKNOWLEDGED_PA
    }
    #[doc = "Checks if the value of the field is `BOTH_ACKNOWLEDGED_AN`"]
    #[inline(always)]
    pub fn is_both_acknowledged_an(&self) -> bool {
        *self == INTONNAK_COR::BOTH_ACKNOWLEDGED_AN
    }
}
#[doc = "Values that can be written to the field `INTONNAK_CO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_COW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl INTONNAK_COW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_COW::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_COW::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTONNAK_COW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_COW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_COW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn only_acknowledged_pa(self) -> &'a mut W {
        self.variant(INTONNAK_COW::ONLY_ACKNOWLEDGED_PA)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn both_acknowledged_an(self) -> &'a mut W {
        self.variant(INTONNAK_COW::BOTH_ACKNOWLEDGED_AN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `INTONNAK_CI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_CIR {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl crate::ToBits<bool> for INTONNAK_CIR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INTONNAK_CIR::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_CIR::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INTONNAK_CI_R = crate::FR<bool, INTONNAK_CIR>;
impl INTONNAK_CI_R {
    #[doc = "Checks if the value of the field is `ONLY_ACKNOWLEDGED_PA`"]
    #[inline(always)]
    pub fn is_only_acknowledged_pa(&self) -> bool {
        *self == INTONNAK_CIR::ONLY_ACKNOWLEDGED_PA
    }
    #[doc = "Checks if the value of the field is `BOTH_ACKNOWLEDGED_AN`"]
    #[inline(always)]
    pub fn is_both_acknowledged_an(&self) -> bool {
        *self == INTONNAK_CIR::BOTH_ACKNOWLEDGED_AN
    }
}
#[doc = "Values that can be written to the field `INTONNAK_CI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_CIW {
    #[doc = "Only acknowledged packets generate an interrupt"]
    ONLY_ACKNOWLEDGED_PA,
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    BOTH_ACKNOWLEDGED_AN,
}
impl INTONNAK_CIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INTONNAK_CIW::ONLY_ACKNOWLEDGED_PA => false,
            INTONNAK_CIW::BOTH_ACKNOWLEDGED_AN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INTONNAK_CIW<'a> {
    w: &'a mut W,
}
impl<'a> _INTONNAK_CIW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_CIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn only_acknowledged_pa(self) -> &'a mut W {
        self.variant(INTONNAK_CIW::ONLY_ACKNOWLEDGED_PA)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn both_acknowledged_an(self) -> &'a mut W {
        self.variant(INTONNAK_CIW::BOTH_ACKNOWLEDGED_AN)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DCON_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCONW<'a> {
    w: &'a mut W,
}
impl<'a> _DCONW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DSUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DSUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPM_SUS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPM_SUSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_SUSW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LPM_REWP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LPM_REWPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPM_REWPW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DCON_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DCON_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DCON_CW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DSUS_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DSUS_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DSUS_CW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type DRES_C_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DRES_CW<'a> {
    w: &'a mut W,
}
impl<'a> _DRES_CW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type VBUSDEBOUNCED_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _VBUSDEBOUNCEDW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSDEBOUNCEDW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USB Clock/PLL control."]
    #[inline(always)]
    pub fn pll_on(&self) -> PLL_ON_R {
        PLL_ON_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPM Support."]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    pub fn vbusdebounced(&self) -> VBUSDEBOUNCED_R {
        VBUSDEBOUNCED_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> _DEV_ADDRW {
        _DEV_ADDRW { w: self }
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&mut self) -> _DEV_ENW {
        _DEV_ENW { w: self }
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&mut self) -> _SETUPW {
        _SETUPW { w: self }
    }
    #[doc = "Bit 9 - USB Clock/PLL control."]
    #[inline(always)]
    pub fn pll_on(&mut self) -> _PLL_ONW {
        _PLL_ONW { w: self }
    }
    #[doc = "Bit 11 - LPM Support."]
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> _LPM_SUPW {
        _LPM_SUPW { w: self }
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> _INTONNAK_AOW {
        _INTONNAK_AOW { w: self }
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> _INTONNAK_AIW {
        _INTONNAK_AIW { w: self }
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> _INTONNAK_COW {
        _INTONNAK_COW { w: self }
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> _INTONNAK_CIW {
        _INTONNAK_CIW { w: self }
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VbusDebounced bit is one."]
    #[inline(always)]
    pub fn dcon(&mut self) -> _DCONW {
        _DCONW { w: self }
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&mut self) -> _DSUSW {
        _DSUSW { w: self }
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10us has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> _LPM_SUSW {
        _LPM_SUSW { w: self }
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    pub fn lpm_rewp(&mut self) -> _LPM_REWPW {
        _LPM_REWPW { w: self }
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&mut self) -> _DCON_CW {
        _DCON_CW { w: self }
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&mut self) -> _DSUS_CW {
        _DSUS_CW { w: self }
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&mut self) -> _DRES_CW {
        _DRES_CW { w: self }
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    pub fn vbusdebounced(&mut self) -> _VBUSDEBOUNCEDW {
        _VBUSDEBOUNCEDW { w: self }
    }
}
