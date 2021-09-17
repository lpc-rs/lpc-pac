#[doc = "Register `SYSAHBCLKCTRL0` reader"]
pub struct R(crate::R<SYSAHBCLKCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSAHBCLKCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSAHBCLKCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSAHBCLKCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSAHBCLKCTRL0` writer"]
pub struct W(crate::W<SYSAHBCLKCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSAHBCLKCTRL0_SPEC>;
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
impl From<crate::W<SYSAHBCLKCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSAHBCLKCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS` reader - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
pub struct SYS_R(crate::FieldReader<bool, bool>);
impl SYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS` writer - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
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
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ROM_A> for bool {
    #[inline(always)]
    fn from(variant: ROM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ROM` reader - Enables clock for ROM."]
pub struct ROM_R(crate::FieldReader<bool, ROM_A>);
impl ROM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::DISABLE,
            true => ROM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ROM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ROM_A::ENABLE
    }
}
impl core::ops::Deref for ROM_R {
    type Target = crate::FieldReader<bool, ROM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ROM` writer - Enables clock for ROM."]
pub struct ROM_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ROM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ROM_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ROM_A::ENABLE)
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
#[doc = "Enables clock for SRAM0 and SRAM1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<RAM0_1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0_1` reader - Enables clock for SRAM0 and SRAM1."]
pub struct RAM0_1_R(crate::FieldReader<bool, RAM0_1_A>);
impl RAM0_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_1_A {
        match self.bits {
            false => RAM0_1_A::DISABLE,
            true => RAM0_1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == RAM0_1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == RAM0_1_A::ENABLE
    }
}
impl core::ops::Deref for RAM0_1_R {
    type Target = crate::FieldReader<bool, RAM0_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0_1` writer - Enables clock for SRAM0 and SRAM1."]
pub struct RAM0_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RAM0_1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RAM0_1_A::ENABLE)
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
#[doc = "Enables clock for flash.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<FLASH_A> for bool {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASH` reader - Enables clock for flash."]
pub struct FLASH_R(crate::FieldReader<bool, FLASH_A>);
impl FLASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::DISABLE,
            true => FLASH_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FLASH_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FLASH_A::ENABLE
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<bool, FLASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASH` writer - Enables clock for flash."]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLASH_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLASH_A::ENABLE)
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
#[doc = "Enables clock for I2C0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C0_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - Enables clock for I2C0."]
pub struct I2C0_R(crate::FieldReader<bool, I2C0_A>);
impl I2C0_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::DISABLE,
            true => I2C0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == I2C0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == I2C0_A::ENABLE
    }
}
impl core::ops::Deref for I2C0_R {
    type Target = crate::FieldReader<bool, I2C0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C0` writer - Enables clock for I2C0."]
pub struct I2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C0_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C0_A::ENABLE)
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
#[doc = "Enables clock for GPIO0 port registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO0_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<GPIO0_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO0` reader - Enables clock for GPIO0 port registers."]
pub struct GPIO0_R(crate::FieldReader<bool, GPIO0_A>);
impl GPIO0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO0_A {
        match self.bits {
            false => GPIO0_A::DISABLE,
            true => GPIO0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO0_A::ENABLE
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, GPIO0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO0` writer - Enables clock for GPIO0 port registers."]
pub struct GPIO0_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO0_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO0_A::ENABLE)
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
#[doc = "Enables clock for switch matrix.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWM_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SWM_A> for bool {
    #[inline(always)]
    fn from(variant: SWM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWM` reader - Enables clock for switch matrix."]
pub struct SWM_R(crate::FieldReader<bool, SWM_A>);
impl SWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWM_A {
        match self.bits {
            false => SWM_A::DISABLE,
            true => SWM_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SWM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SWM_A::ENABLE
    }
}
impl core::ops::Deref for SWM_R {
    type Target = crate::FieldReader<bool, SWM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWM` writer - Enables clock for switch matrix."]
pub struct SWM_W<'a> {
    w: &'a mut W,
}
impl<'a> SWM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWM_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWM_A::ENABLE)
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
#[doc = "Enables clock for state configurable timer SCTimer/PWM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SCT_A> for bool {
    #[inline(always)]
    fn from(variant: SCT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCT` reader - Enables clock for state configurable timer SCTimer/PWM."]
pub struct SCT_R(crate::FieldReader<bool, SCT_A>);
impl SCT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::DISABLE,
            true => SCT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SCT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SCT_A::ENABLE
    }
}
impl core::ops::Deref for SCT_R {
    type Target = crate::FieldReader<bool, SCT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT` writer - Enables clock for state configurable timer SCTimer/PWM."]
pub struct SCT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SCT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SCT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Enables clock for self-wake-up timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<WKT_A> for bool {
    #[inline(always)]
    fn from(variant: WKT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKT` reader - Enables clock for self-wake-up timer."]
pub struct WKT_R(crate::FieldReader<bool, WKT_A>);
impl WKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_A {
        match self.bits {
            false => WKT_A::DISABLE,
            true => WKT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WKT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WKT_A::ENABLE
    }
}
impl core::ops::Deref for WKT_R {
    type Target = crate::FieldReader<bool, WKT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WKT` writer - Enables clock for self-wake-up timer."]
pub struct WKT_W<'a> {
    w: &'a mut W,
}
impl<'a> WKT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WKT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Enables clock for multi-rate timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MRT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<MRT_A> for bool {
    #[inline(always)]
    fn from(variant: MRT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRT` reader - Enables clock for multi-rate timer."]
pub struct MRT_R(crate::FieldReader<bool, MRT_A>);
impl MRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::DISABLE,
            true => MRT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MRT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MRT_A::ENABLE
    }
}
impl core::ops::Deref for MRT_R {
    type Target = crate::FieldReader<bool, MRT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT` writer - Enables clock for multi-rate timer."]
pub struct MRT_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MRT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MRT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MRT_A::ENABLE)
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
#[doc = "Enables clock for SPI0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI0_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - Enables clock for SPI0."]
pub struct SPI0_R(crate::FieldReader<bool, SPI0_A>);
impl SPI0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::DISABLE,
            true => SPI0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SPI0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SPI0_A::ENABLE
    }
}
impl core::ops::Deref for SPI0_R {
    type Target = crate::FieldReader<bool, SPI0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI0` writer - Enables clock for SPI0."]
pub struct SPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI0_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Enables clock for SPI1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` reader - Enables clock for SPI1."]
pub struct SPI1_R(crate::FieldReader<bool, SPI1_A>);
impl SPI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::DISABLE,
            true => SPI1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == SPI1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == SPI1_A::ENABLE
    }
}
impl core::ops::Deref for SPI1_R {
    type Target = crate::FieldReader<bool, SPI1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI1` writer - Enables clock for SPI1."]
pub struct SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPI1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPI1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Enables clock for CRC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRC_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC` reader - Enables clock for CRC."]
pub struct CRC_R(crate::FieldReader<bool, CRC_A>);
impl CRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::DISABLE,
            true => CRC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CRC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CRC_A::ENABLE
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<bool, CRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC` writer - Enables clock for CRC."]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CRC_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CRC_A::ENABLE)
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
#[doc = "Enables clock for UART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - Enables clock for UART0."]
pub struct UART0_R(crate::FieldReader<bool, UART0_A>);
impl UART0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::DISABLE,
            true => UART0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UART0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UART0_A::ENABLE
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, UART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0` writer - Enables clock for UART0."]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART0_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART0_A::ENABLE)
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
#[doc = "Enables clock for UART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` reader - Enables clock for UART1."]
pub struct UART1_R(crate::FieldReader<bool, UART1_A>);
impl UART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::DISABLE,
            true => UART1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UART1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UART1_A::ENABLE
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, UART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1` writer - Enables clock for UART1."]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART1_A::ENABLE)
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
#[doc = "Enables clock for UART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - Enables clock for UART2."]
pub struct UART2_R(crate::FieldReader<bool, UART2_A>);
impl UART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::DISABLE,
            true => UART2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UART2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UART2_A::ENABLE
    }
}
impl core::ops::Deref for UART2_R {
    type Target = crate::FieldReader<bool, UART2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2` writer - Enables clock for UART2."]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART2_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Enables clock for WWDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<WWDT_A> for bool {
    #[inline(always)]
    fn from(variant: WWDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDT` reader - Enables clock for WWDT."]
pub struct WWDT_R(crate::FieldReader<bool, WWDT_A>);
impl WWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WWDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::DISABLE,
            true => WWDT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == WWDT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == WWDT_A::ENABLE
    }
}
impl core::ops::Deref for WWDT_R {
    type Target = crate::FieldReader<bool, WWDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WWDT` writer - Enables clock for WWDT."]
pub struct WWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> WWDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WWDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Enables clock for IOCON.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables clock for IOCON."]
pub struct IOCON_R(crate::FieldReader<bool, IOCON_A>);
impl IOCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLE,
            true => IOCON_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == IOCON_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == IOCON_A::ENABLE
    }
}
impl core::ops::Deref for IOCON_R {
    type Target = crate::FieldReader<bool, IOCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON` writer - Enables clock for IOCON."]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Enables clock for analog comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` reader - Enables clock for analog comparator."]
pub struct ACMP_R(crate::FieldReader<bool, ACMP_A>);
impl ACMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::DISABLE,
            true => ACMP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ACMP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ACMP_A::ENABLE
    }
}
impl core::ops::Deref for ACMP_R {
    type Target = crate::FieldReader<bool, ACMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP` writer - Enables clock for analog comparator."]
pub struct ACMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACMP_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Enables clock for GPIO1 port registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<GPIO1_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO1` reader - Enables clock for GPIO1 port registers."]
pub struct GPIO1_R(crate::FieldReader<bool, GPIO1_A>);
impl GPIO1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO1_A {
        match self.bits {
            false => GPIO1_A::DISABLE,
            true => GPIO1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO1_A::ENABLE
    }
}
impl core::ops::Deref for GPIO1_R {
    type Target = crate::FieldReader<bool, GPIO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO1` writer - Enables clock for GPIO1 port registers."]
pub struct GPIO1_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Enables clock for I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - Enables clock for I2C1."]
pub struct I2C1_R(crate::FieldReader<bool, I2C1_A>);
impl I2C1_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::DISABLE,
            true => I2C1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == I2C1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == I2C1_A::ENABLE
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, I2C1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1` writer - Enables clock for I2C1."]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C1_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C1_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Enables clock for I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2` reader - Enables clock for I2C2."]
pub struct I2C2_R(crate::FieldReader<bool, I2C2_A>);
impl I2C2_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::DISABLE,
            true => I2C2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == I2C2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == I2C2_A::ENABLE
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, I2C2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2` writer - Enables clock for I2C2."]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C2_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C2_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Enables clock for I2C3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<I2C3_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3` reader - Enables clock for I2C3."]
pub struct I2C3_R(crate::FieldReader<bool, I2C3_A>);
impl I2C3_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_A {
        match self.bits {
            false => I2C3_A::DISABLE,
            true => I2C3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == I2C3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == I2C3_A::ENABLE
    }
}
impl core::ops::Deref for I2C3_R {
    type Target = crate::FieldReader<bool, I2C3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3` writer - Enables clock for I2C3."]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(I2C3_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(I2C3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Enables clock for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Enables clock for ADC."]
pub struct ADC_R(crate::FieldReader<bool, ADC_A>);
impl ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::DISABLE,
            true => ADC_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ADC_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ADC_A::ENABLE
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - Enables clock for ADC."]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADC_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADC_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enables clock for CTIMER.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIMER_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<CTIMER_A> for bool {
    #[inline(always)]
    fn from(variant: CTIMER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTIMER` reader - Enables clock for CTIMER."]
pub struct CTIMER_R(crate::FieldReader<bool, CTIMER_A>);
impl CTIMER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIMER_A {
        match self.bits {
            false => CTIMER_A::DISABLE,
            true => CTIMER_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == CTIMER_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == CTIMER_A::ENABLE
    }
}
impl core::ops::Deref for CTIMER_R {
    type Target = crate::FieldReader<bool, CTIMER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER` writer - Enables clock for CTIMER."]
pub struct CTIMER_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIMER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CTIMER_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CTIMER_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTB_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<MTB_A> for bool {
    #[inline(always)]
    fn from(variant: MTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTB` reader - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
pub struct MTB_R(crate::FieldReader<bool, MTB_A>);
impl MTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTB_A {
        match self.bits {
            false => MTB_A::DISABLE,
            true => MTB_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == MTB_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == MTB_A::ENABLE
    }
}
impl core::ops::Deref for MTB_R {
    type Target = crate::FieldReader<bool, MTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTB` writer - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
pub struct MTB_W<'a> {
    w: &'a mut W,
}
impl<'a> MTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MTB_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MTB_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Enables clock for DAC0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAC0_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - Enables clock for DAC0."]
pub struct DAC0_R(crate::FieldReader<bool, DAC0_A>);
impl DAC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAC0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::DISABLE,
            true => DAC0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DAC0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DAC0_A::ENABLE
    }
}
impl core::ops::Deref for DAC0_R {
    type Target = crate::FieldReader<bool, DAC0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC0` writer - Enables clock for DAC0."]
pub struct DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAC0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DAC0_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DAC0_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Enable clock for GPIO pin interrupt registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_INT_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<GPIO_INT_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO_INT` reader - Enable clock for GPIO pin interrupt registers"]
pub struct GPIO_INT_R(crate::FieldReader<bool, GPIO_INT_A>);
impl GPIO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_INT_A {
        match self.bits {
            false => GPIO_INT_A::DISABLE,
            true => GPIO_INT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == GPIO_INT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == GPIO_INT_A::ENABLE
    }
}
impl core::ops::Deref for GPIO_INT_R {
    type Target = crate::FieldReader<bool, GPIO_INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_INT` writer - Enable clock for GPIO pin interrupt registers"]
pub struct GPIO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(GPIO_INT_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(GPIO_INT_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Enables clock for DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Enables clock for DMA."]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DISABLE,
            true => DMA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DMA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DMA_A::ENABLE
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Enables clock for DMA."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMA_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMA_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enables clock for UART3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART3_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<UART3_A> for bool {
    #[inline(always)]
    fn from(variant: UART3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART3` reader - Enables clock for UART3."]
pub struct UART3_R(crate::FieldReader<bool, UART3_A>);
impl UART3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3_A {
        match self.bits {
            false => UART3_A::DISABLE,
            true => UART3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UART3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UART3_A::ENABLE
    }
}
impl core::ops::Deref for UART3_R {
    type Target = crate::FieldReader<bool, UART3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART3` writer - Enables clock for UART3."]
pub struct UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART3_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART3_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Enables clock for UART4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART4_A {
    #[doc = "0: disable"]
    DISABLE = 0,
    #[doc = "1: enable"]
    ENABLE = 1,
}
impl From<UART4_A> for bool {
    #[inline(always)]
    fn from(variant: UART4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART4` reader - Enables clock for UART4."]
pub struct UART4_R(crate::FieldReader<bool, UART4_A>);
impl UART4_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4_A {
        match self.bits {
            false => UART4_A::DISABLE,
            true => UART4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == UART4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == UART4_A::ENABLE
    }
}
impl core::ops::Deref for UART4_R {
    type Target = crate::FieldReader<bool, UART4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4` writer - Enables clock for UART4."]
pub struct UART4_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(UART4_A::DISABLE)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(UART4_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline(always)]
    pub fn ram0_1(&self) -> RAM0_1_R {
        RAM0_1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO0 port registers."]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline(always)]
    pub fn swm(&self) -> SWM_R {
        SWM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline(always)]
    pub fn sct(&self) -> SCT_R {
        SCT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline(always)]
    pub fn wkt(&self) -> WKT_R {
        WKT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables clock for UART0."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables clock for UART1."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables clock for UART2."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables clock for IOCON."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables clock for analog comparator."]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enables clock for GPIO1 port registers."]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables clock for I2C1."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables clock for I2C2."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables clock for I2C3."]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables clock for CTIMER."]
    #[inline(always)]
    pub fn ctimer(&self) -> CTIMER_R {
        CTIMER_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline(always)]
    pub fn mtb(&self) -> MTB_R {
        MTB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables clock for DAC0."]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable clock for GPIO pin interrupt registers"]
    #[inline(always)]
    pub fn gpio_int(&self) -> GPIO_INT_R {
        GPIO_INT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enables clock for UART3."]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables clock for UART4."]
    #[inline(always)]
    pub fn uart4(&self) -> UART4_R {
        UART4_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 2 - Enables clock for SRAM0 and SRAM1."]
    #[inline(always)]
    pub fn ram0_1(&mut self) -> RAM0_1_W {
        RAM0_1_W { w: self }
    }
    #[doc = "Bit 4 - Enables clock for flash."]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 5 - Enables clock for I2C0."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W {
        I2C0_W { w: self }
    }
    #[doc = "Bit 6 - Enables clock for GPIO0 port registers."]
    #[inline(always)]
    pub fn gpio0(&mut self) -> GPIO0_W {
        GPIO0_W { w: self }
    }
    #[doc = "Bit 7 - Enables clock for switch matrix."]
    #[inline(always)]
    pub fn swm(&mut self) -> SWM_W {
        SWM_W { w: self }
    }
    #[doc = "Bit 8 - Enables clock for state configurable timer SCTimer/PWM."]
    #[inline(always)]
    pub fn sct(&mut self) -> SCT_W {
        SCT_W { w: self }
    }
    #[doc = "Bit 9 - Enables clock for self-wake-up timer."]
    #[inline(always)]
    pub fn wkt(&mut self) -> WKT_W {
        WKT_W { w: self }
    }
    #[doc = "Bit 10 - Enables clock for multi-rate timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 11 - Enables clock for SPI0."]
    #[inline(always)]
    pub fn spi0(&mut self) -> SPI0_W {
        SPI0_W { w: self }
    }
    #[doc = "Bit 12 - Enables clock for SPI1."]
    #[inline(always)]
    pub fn spi1(&mut self) -> SPI1_W {
        SPI1_W { w: self }
    }
    #[doc = "Bit 13 - Enables clock for CRC."]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 14 - Enables clock for UART0."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 15 - Enables clock for UART1."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 16 - Enables clock for UART2."]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 18 - Enables clock for IOCON."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 19 - Enables clock for analog comparator."]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W {
        ACMP_W { w: self }
    }
    #[doc = "Bit 20 - Enables clock for GPIO1 port registers."]
    #[inline(always)]
    pub fn gpio1(&mut self) -> GPIO1_W {
        GPIO1_W { w: self }
    }
    #[doc = "Bit 21 - Enables clock for I2C1."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 22 - Enables clock for I2C2."]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 23 - Enables clock for I2C3."]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Bit 24 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 25 - Enables clock for CTIMER."]
    #[inline(always)]
    pub fn ctimer(&mut self) -> CTIMER_W {
        CTIMER_W { w: self }
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers. Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline(always)]
    pub fn mtb(&mut self) -> MTB_W {
        MTB_W { w: self }
    }
    #[doc = "Bit 27 - Enables clock for DAC0."]
    #[inline(always)]
    pub fn dac0(&mut self) -> DAC0_W {
        DAC0_W { w: self }
    }
    #[doc = "Bit 28 - Enable clock for GPIO pin interrupt registers"]
    #[inline(always)]
    pub fn gpio_int(&mut self) -> GPIO_INT_W {
        GPIO_INT_W { w: self }
    }
    #[doc = "Bit 29 - Enables clock for DMA."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 30 - Enables clock for UART3."]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART3_W {
        UART3_W { w: self }
    }
    #[doc = "Bit 31 - Enables clock for UART4."]
    #[inline(always)]
    pub fn uart4(&mut self) -> UART4_W {
        UART4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock group 0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl0](index.html) module"]
pub struct SYSAHBCLKCTRL0_SPEC;
impl crate::RegisterSpec for SYSAHBCLKCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysahbclkctrl0::R](R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl0::W](W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL0 to value 0x17"]
impl crate::Resettable for SYSAHBCLKCTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x17
    }
}
