#[doc = "Register `PDAWAKECFG` reader"]
pub struct R(crate::R<PDAWAKECFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDAWAKECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDAWAKECFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDAWAKECFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDAWAKECFG` writer"]
pub struct W(crate::W<PDAWAKECFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDAWAKECFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDAWAKECFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDAWAKECFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FRO oscillator output wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FROOUT_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<FROOUT_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FROOUT_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FROOUT_PD` reader - FRO oscillator output wake-up configuration"]
pub struct FROOUT_PD_R(crate::FieldReader<bool, FROOUT_PD_A>);
impl FROOUT_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FROOUT_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FROOUT_PD_A {
        match self.bits {
            false => FROOUT_PD_A::POWERED,
            true => FROOUT_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == FROOUT_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == FROOUT_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for FROOUT_PD_R {
    type Target = crate::FieldReader<bool, FROOUT_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FROOUT_PD` writer - FRO oscillator output wake-up configuration"]
pub struct FROOUT_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FROOUT_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FROOUT_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FROOUT_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FROOUT_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "FRO oscillator power-down wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRO_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<FRO_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FRO_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRO_PD` reader - FRO oscillator power-down wake-up configuration"]
pub struct FRO_PD_R(crate::FieldReader<bool, FRO_PD_A>);
impl FRO_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRO_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRO_PD_A {
        match self.bits {
            false => FRO_PD_A::POWERED,
            true => FRO_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == FRO_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == FRO_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for FRO_PD_R {
    type Target = crate::FieldReader<bool, FRO_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRO_PD` writer - FRO oscillator power-down wake-up configuration"]
pub struct FRO_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FRO_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRO_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FRO_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FRO_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Flash wake-up configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<FLASH_PD_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH_PD` reader - Flash wake-up configuration"]
pub struct FLASH_PD_R(crate::FieldReader<bool, FLASH_PD_A>);
impl FLASH_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_PD_A {
        match self.bits {
            false => FLASH_PD_A::POWERED,
            true => FLASH_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == FLASH_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == FLASH_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for FLASH_PD_R {
    type Target = crate::FieldReader<bool, FLASH_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH_PD` writer - Flash wake-up configuration"]
pub struct FLASH_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(FLASH_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "BOD wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOD_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<BOD_PD_A> for bool {
    #[inline(always)]
    fn from(variant: BOD_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOD_PD` reader - BOD wake-up configuration"]
pub struct BOD_PD_R(crate::FieldReader<bool, BOD_PD_A>);
impl BOD_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOD_PD_A {
        match self.bits {
            false => BOD_PD_A::POWERED,
            true => BOD_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == BOD_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == BOD_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for BOD_PD_R {
    type Target = crate::FieldReader<bool, BOD_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD_PD` writer - BOD wake-up configuration"]
pub struct BOD_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BOD_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(BOD_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "ADC wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<ADC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` reader - ADC wake-up configuration"]
pub struct ADC_PD_R(crate::FieldReader<bool, ADC_PD_A>);
impl ADC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_PD_A {
        match self.bits {
            false => ADC_PD_A::POWERED,
            true => ADC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == ADC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == ADC_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for ADC_PD_R {
    type Target = crate::FieldReader<bool, ADC_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_PD` writer - ADC wake-up configuration"]
pub struct ADC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(ADC_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Crystal oscillator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSOSC_PD_A {
    #[doc = "0: powered"]
    POWERED = 0,
    #[doc = "1: powered down"]
    POWERED_DOWN = 1,
}
impl From<SYSOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSOSC_PD` reader - Crystal oscillator wake-up configuration"]
pub struct SYSOSC_PD_R(crate::FieldReader<bool, SYSOSC_PD_A>);
impl SYSOSC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSOSC_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSOSC_PD_A {
        match self.bits {
            false => SYSOSC_PD_A::POWERED,
            true => SYSOSC_PD_A::POWERED_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWERED`"]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        **self == SYSOSC_PD_A::POWERED
    }
    #[doc = "Checks if the value of the field is `POWERED_DOWN`"]
    #[inline(always)]
    pub fn is_powered_down(&self) -> bool {
        **self == SYSOSC_PD_A::POWERED_DOWN
    }
}
impl core::ops::Deref for SYSOSC_PD_R {
    type Target = crate::FieldReader<bool, SYSOSC_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSOSC_PD` writer - Crystal oscillator wake-up configuration"]
pub struct SYSOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSOSC_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "powered"]
    #[inline(always)]
    pub fn powered(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED)
    }
    #[doc = "powered down"]
    #[inline(always)]
    pub fn powered_down(self) -> &'a mut W {
        self.variant(SYSOSC_PD_A::POWERED_DOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTOSC_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<WDTOSC_PD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTOSC_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTOSC_PD` reader - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
pub struct WDTOSC_PD_R(crate::FieldReader<bool, WDTOSC_PD_A>);
impl WDTOSC_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTOSC_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTOSC_PD_A {
        match self.bits {
            false => WDTOSC_PD_A::DISABLED,
            true => WDTOSC_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WDTOSC_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WDTOSC_PD_A::ENABLED
    }
}
impl core::ops::Deref for WDTOSC_PD_R {
    type Target = crate::FieldReader<bool, WDTOSC_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTOSC_PD` writer - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
pub struct WDTOSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTOSC_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTOSC_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDTOSC_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "System PLL wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSPLL_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SYSPLL_PD_A> for bool {
    #[inline(always)]
    fn from(variant: SYSPLL_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLL_PD` reader - System PLL wake-up configuration"]
pub struct SYSPLL_PD_R(crate::FieldReader<bool, SYSPLL_PD_A>);
impl SYSPLL_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSPLL_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSPLL_PD_A {
        match self.bits {
            false => SYSPLL_PD_A::DISABLED,
            true => SYSPLL_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SYSPLL_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYSPLL_PD_A::ENABLED
    }
}
impl core::ops::Deref for SYSPLL_PD_R {
    type Target = crate::FieldReader<bool, SYSPLL_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSPLL_PD` writer - System PLL wake-up configuration"]
pub struct SYSPLL_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSPLL_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSPLL_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSPLL_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "VREF2 wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREF2_PD_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<VREF2_PD_A> for bool {
    #[inline(always)]
    fn from(variant: VREF2_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREF2_PD` reader - VREF2 wake-up configuration"]
pub struct VREF2_PD_R(crate::FieldReader<bool, VREF2_PD_A>);
impl VREF2_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREF2_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF2_PD_A {
        match self.bits {
            false => VREF2_PD_A::DISABLED,
            true => VREF2_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == VREF2_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == VREF2_PD_A::ENABLED
    }
}
impl core::ops::Deref for VREF2_PD_R {
    type Target = crate::FieldReader<bool, VREF2_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREF2_PD` writer - VREF2 wake-up configuration"]
pub struct VREF2_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> VREF2_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREF2_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREF2_PD_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREF2_PD_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "DAC0 wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - DAC0 wake-up configuration"]
pub struct DAC0_R(crate::FieldReader<bool, DAC0_A>);
impl DAC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::DISABLED,
            true => DAC0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DAC0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DAC0_A::ENABLED
    }
}
impl core::ops::Deref for DAC0_R {
    type Target = crate::FieldReader<bool, DAC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC0` writer - DAC0 wake-up configuration"]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "DAC1 wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAC1_A> for bool {
    #[inline(always)]
    fn from(variant: DAC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC1` reader - DAC1 wake-up configuration"]
pub struct DAC1_R(crate::FieldReader<bool, DAC1_A>);
impl DAC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC1_A {
        match self.bits {
            false => DAC1_A::DISABLED,
            true => DAC1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DAC1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DAC1_A::ENABLED
    }
}
impl core::ops::Deref for DAC1_R {
    type Target = crate::FieldReader<bool, DAC1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC1` writer - DAC1 wake-up configuration"]
pub struct DAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAC1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAC1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Analog comparator wake-up configuration\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` reader - Analog comparator wake-up configuration"]
pub struct ACMP_R(crate::FieldReader<bool, ACMP_A>);
impl ACMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::DISABLED,
            true => ACMP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ACMP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACMP_A::ENABLED
    }
}
impl core::ops::Deref for ACMP_R {
    type Target = crate::FieldReader<bool, ACMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP` writer - Analog comparator wake-up configuration"]
pub struct ACMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACMP_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACMP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FRO oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn froout_pd(&self) -> FROOUT_PD_R {
        FROOUT_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FRO oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn fro_pd(&self) -> FRO_PD_R {
        FRO_PD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&self) -> FLASH_PD_R {
        FLASH_PD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&self) -> BOD_PD_R {
        BOD_PD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&self) -> SYSOSC_PD_R {
        SYSOSC_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline(always)]
    pub fn wdtosc_pd(&self) -> WDTOSC_PD_R {
        WDTOSC_PD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&self) -> SYSPLL_PD_R {
        SYSPLL_PD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VREF2 wake-up configuration"]
    #[inline(always)]
    pub fn vref2_pd(&self) -> VREF2_PD_R {
        VREF2_PD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DAC0 wake-up configuration"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DAC1 wake-up configuration"]
    #[inline(always)]
    pub fn dac1(&self) -> DAC1_R {
        DAC1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRO oscillator output wake-up configuration"]
    #[inline(always)]
    pub fn froout_pd(&mut self) -> FROOUT_PD_W {
        FROOUT_PD_W { w: self }
    }
    #[doc = "Bit 1 - FRO oscillator power-down wake-up configuration"]
    #[inline(always)]
    pub fn fro_pd(&mut self) -> FRO_PD_W {
        FRO_PD_W { w: self }
    }
    #[doc = "Bit 2 - Flash wake-up configuration"]
    #[inline(always)]
    pub fn flash_pd(&mut self) -> FLASH_PD_W {
        FLASH_PD_W { w: self }
    }
    #[doc = "Bit 3 - BOD wake-up configuration"]
    #[inline(always)]
    pub fn bod_pd(&mut self) -> BOD_PD_W {
        BOD_PD_W { w: self }
    }
    #[doc = "Bit 4 - ADC wake-up configuration"]
    #[inline(always)]
    pub fn adc_pd(&mut self) -> ADC_PD_W {
        ADC_PD_W { w: self }
    }
    #[doc = "Bit 5 - Crystal oscillator wake-up configuration"]
    #[inline(always)]
    pub fn sysosc_pd(&mut self) -> SYSOSC_PD_W {
        SYSOSC_PD_W { w: self }
    }
    #[doc = "Bit 6 - Watchdog oscillator wake-up configuration. Changing this bit to powered-down has no effect when the LOCK bit in the WWDT MOD register is set. In this case, the watchdog oscillator is always running"]
    #[inline(always)]
    pub fn wdtosc_pd(&mut self) -> WDTOSC_PD_W {
        WDTOSC_PD_W { w: self }
    }
    #[doc = "Bit 7 - System PLL wake-up configuration"]
    #[inline(always)]
    pub fn syspll_pd(&mut self) -> SYSPLL_PD_W {
        SYSPLL_PD_W { w: self }
    }
    #[doc = "Bit 10 - VREF2 wake-up configuration"]
    #[inline(always)]
    pub fn vref2_pd(&mut self) -> VREF2_PD_W {
        VREF2_PD_W { w: self }
    }
    #[doc = "Bit 13 - DAC0 wake-up configuration"]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
    #[doc = "Bit 14 - DAC1 wake-up configuration"]
    #[inline(always)]
    pub fn dac1(&mut self) -> DAC1_W {
        DAC1_W { w: self }
    }
    #[doc = "Bit 15 - Analog comparator wake-up configuration"]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W {
        ACMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdawakecfg](index.html) module"]
pub struct PDAWAKECFG_SPEC;
impl crate::RegisterSpec for PDAWAKECFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdawakecfg::R](R) reader structure"]
impl crate::Readable for PDAWAKECFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdawakecfg::W](W) writer structure"]
impl crate::Writable for PDAWAKECFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDAWAKECFG to value 0xedf8"]
impl crate::Resettable for PDAWAKECFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xedf8
    }
}
