#[doc = "Register `SYSAHBCLKCTRL` reader"]
pub struct R(crate::R<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSAHBCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSAHBCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSAHBCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSAHBCLKCTRL` writer"]
pub struct W(crate::W<SYSAHBCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSAHBCLKCTRL_SPEC>;
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
impl From<crate::W<SYSAHBCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSAHBCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYS_A {
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SYS_A> for bool {
    #[inline(always)]
    fn from(variant: SYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYS` reader - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
pub struct SYS_R(crate::FieldReader<bool, SYS_A>);
impl SYS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYS_A {
        match self.bits {
            true => SYS_A::ENABLED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SYS_A::ENABLED
    }
}
impl core::ops::Deref for SYS_R {
    type Target = crate::FieldReader<bool, SYS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS` writer - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
pub struct SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYS_A::ENABLED)
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
#[doc = "Enables clock for ROM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ROM_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => ROM_A::DISABLED,
            true => ROM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ROM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ROM_A::ENABLED
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ROM_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ROM_A::ENABLED)
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
#[doc = "Enables clock for RAM.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RAM0_A> for bool {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM0` reader - Enables clock for RAM."]
pub struct RAM0_R(crate::FieldReader<bool, RAM0_A>);
impl RAM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_A {
        match self.bits {
            false => RAM0_A::DISABLED,
            true => RAM0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RAM0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RAM0_A::ENABLED
    }
}
impl core::ops::Deref for RAM0_R {
    type Target = crate::FieldReader<bool, RAM0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM0` writer - Enables clock for RAM."]
pub struct RAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM0_A::ENABLED)
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
#[doc = "Enables clock for flash register interface.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHREG_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FLASHREG_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHREG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHREG` reader - Enables clock for flash register interface."]
pub struct FLASHREG_R(crate::FieldReader<bool, FLASHREG_A>);
impl FLASHREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHREG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHREG_A {
        match self.bits {
            false => FLASHREG_A::DISABLED,
            true => FLASHREG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLASHREG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FLASHREG_A::ENABLED
    }
}
impl core::ops::Deref for FLASHREG_R {
    type Target = crate::FieldReader<bool, FLASHREG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHREG` writer - Enables clock for flash register interface."]
pub struct FLASHREG_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHREG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHREG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHREG_A::ENABLED)
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
#[doc = "Enables clock for flash array access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASHARRAY_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<FLASHARRAY_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHARRAY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHARRAY` reader - Enables clock for flash array access."]
pub struct FLASHARRAY_R(crate::FieldReader<bool, FLASHARRAY_A>);
impl FLASHARRAY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLASHARRAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHARRAY_A {
        match self.bits {
            false => FLASHARRAY_A::DISABLED,
            true => FLASHARRAY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FLASHARRAY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FLASHARRAY_A::ENABLED
    }
}
impl core::ops::Deref for FLASHARRAY_R {
    type Target = crate::FieldReader<bool, FLASHARRAY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLASHARRAY` writer - Enables clock for flash array access."]
pub struct FLASHARRAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHARRAY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHARRAY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FLASHARRAY_A::ENABLED)
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
#[doc = "Enables clock for I2C.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<I2C_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` reader - Enables clock for I2C."]
pub struct I2C_R(crate::FieldReader<bool, I2C_A>);
impl I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_A {
        match self.bits {
            false => I2C_A::DISABLED,
            true => I2C_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2C_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2C_A::ENABLED
    }
}
impl core::ops::Deref for I2C_R {
    type Target = crate::FieldReader<bool, I2C_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C` writer - Enables clock for I2C."]
pub struct I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2C_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2C_A::ENABLED)
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
#[doc = "Enables clock for GPIO port registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GPIO_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO port registers."]
pub struct GPIO_R(crate::FieldReader<bool, GPIO_A>);
impl GPIO_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_A {
        match self.bits {
            false => GPIO_A::DISABLED,
            true => GPIO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GPIO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GPIO_A::ENABLED
    }
}
impl core::ops::Deref for GPIO_R {
    type Target = crate::FieldReader<bool, GPIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO port registers."]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIO_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIO_A::ENABLED)
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
#[doc = "Enables clock for 16-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT16B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B0` reader - Enables clock for 16-bit counter/timer 0."]
pub struct CT16B0_R(crate::FieldReader<bool, CT16B0_A>);
impl CT16B0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT16B0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B0_A {
        match self.bits {
            false => CT16B0_A::DISABLED,
            true => CT16B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CT16B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CT16B0_A::ENABLED
    }
}
impl core::ops::Deref for CT16B0_R {
    type Target = crate::FieldReader<bool, CT16B0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT16B0` writer - Enables clock for 16-bit counter/timer 0."]
pub struct CT16B0_W<'a> {
    w: &'a mut W,
}
impl<'a> CT16B0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B0_A::ENABLED)
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
#[doc = "Enables clock for 16-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT16B1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT16B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT16B1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT16B1` reader - Enables clock for 16-bit counter/timer 1."]
pub struct CT16B1_R(crate::FieldReader<bool, CT16B1_A>);
impl CT16B1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT16B1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT16B1_A {
        match self.bits {
            false => CT16B1_A::DISABLED,
            true => CT16B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CT16B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CT16B1_A::ENABLED
    }
}
impl core::ops::Deref for CT16B1_R {
    type Target = crate::FieldReader<bool, CT16B1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT16B1` writer - Enables clock for 16-bit counter/timer 1."]
pub struct CT16B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CT16B1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT16B1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT16B1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT16B1_A::ENABLED)
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
#[doc = "Enables clock for 32-bit counter/timer 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT32B0_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32B0` reader - Enables clock for 32-bit counter/timer 0."]
pub struct CT32B0_R(crate::FieldReader<bool, CT32B0_A>);
impl CT32B0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT32B0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B0_A {
        match self.bits {
            false => CT32B0_A::DISABLED,
            true => CT32B0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CT32B0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CT32B0_A::ENABLED
    }
}
impl core::ops::Deref for CT32B0_R {
    type Target = crate::FieldReader<bool, CT32B0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT32B0` writer - Enables clock for 32-bit counter/timer 0."]
pub struct CT32B0_W<'a> {
    w: &'a mut W,
}
impl<'a> CT32B0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B0_A::ENABLED)
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
#[doc = "Enables clock for 32-bit counter/timer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CT32B1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<CT32B1_A> for bool {
    #[inline(always)]
    fn from(variant: CT32B1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT32B1` reader - Enables clock for 32-bit counter/timer 1."]
pub struct CT32B1_R(crate::FieldReader<bool, CT32B1_A>);
impl CT32B1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CT32B1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CT32B1_A {
        match self.bits {
            false => CT32B1_A::DISABLED,
            true => CT32B1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CT32B1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CT32B1_A::ENABLED
    }
}
impl core::ops::Deref for CT32B1_R {
    type Target = crate::FieldReader<bool, CT32B1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CT32B1` writer - Enables clock for 32-bit counter/timer 1."]
pub struct CT32B1_W<'a> {
    w: &'a mut W,
}
impl<'a> CT32B1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CT32B1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CT32B1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CT32B1_A::ENABLED)
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
#[doc = "Enables clock for SSP0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SSP0_A> for bool {
    #[inline(always)]
    fn from(variant: SSP0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP0` reader - Enables clock for SSP0."]
pub struct SSP0_R(crate::FieldReader<bool, SSP0_A>);
impl SSP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP0_A {
        match self.bits {
            false => SSP0_A::DISABLED,
            true => SSP0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSP0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSP0_A::ENABLED
    }
}
impl core::ops::Deref for SSP0_R {
    type Target = crate::FieldReader<bool, SSP0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP0` writer - Enables clock for SSP0."]
pub struct SSP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP0_A::ENABLED)
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
#[doc = "Enables clock for UART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USART_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USART_A> for bool {
    #[inline(always)]
    fn from(variant: USART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USART` reader - Enables clock for UART."]
pub struct USART_R(crate::FieldReader<bool, USART_A>);
impl USART_R {
    pub(crate) fn new(bits: bool) -> Self {
        USART_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART_A {
        match self.bits {
            false => USART_A::DISABLED,
            true => USART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USART_A::ENABLED
    }
}
impl core::ops::Deref for USART_R {
    type Target = crate::FieldReader<bool, USART_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART` writer - Enables clock for UART."]
pub struct USART_W<'a> {
    w: &'a mut W,
}
impl<'a> USART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USART_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USART_A::ENABLED)
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
#[doc = "Enables clock for ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => ADC_A::DISABLED,
            true => ADC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ADC_A::ENABLED
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADC_A::ENABLED)
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
#[doc = "Enables clock to the USB register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USB_A> for bool {
    #[inline(always)]
    fn from(variant: USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB` reader - Enables clock to the USB register interface."]
pub struct USB_R(crate::FieldReader<bool, USB_A>);
impl USB_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USB_A {
        match self.bits {
            false => USB_A::DISABLED,
            true => USB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USB_A::ENABLED
    }
}
impl core::ops::Deref for USB_R {
    type Target = crate::FieldReader<bool, USB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB` writer - Enables clock to the USB register interface."]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USB_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USB_A::ENABLED)
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
#[doc = "Enables clock for WWDT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WWDT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
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
            false => WWDT_A::DISABLED,
            true => WWDT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == WWDT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == WWDT_A::ENABLED
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WWDT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WWDT_A::ENABLED)
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
#[doc = "Enables clock for I/O configuration block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables clock for I/O configuration block."]
pub struct IOCON_R(crate::FieldReader<bool, IOCON_A>);
impl IOCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::DISABLED,
            true => IOCON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == IOCON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == IOCON_A::ENABLED
    }
}
impl core::ops::Deref for IOCON_R {
    type Target = crate::FieldReader<bool, IOCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON` writer - Enables clock for I/O configuration block."]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IOCON_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IOCON_A::ENABLED)
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
#[doc = "Enables clock for SSP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSP1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SSP1_A> for bool {
    #[inline(always)]
    fn from(variant: SSP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSP1` reader - Enables clock for SSP1."]
pub struct SSP1_R(crate::FieldReader<bool, SSP1_A>);
impl SSP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSP1_A {
        match self.bits {
            false => SSP1_A::DISABLED,
            true => SSP1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SSP1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SSP1_A::ENABLED
    }
}
impl core::ops::Deref for SSP1_R {
    type Target = crate::FieldReader<bool, SSP1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP1` writer - Enables clock for SSP1."]
pub struct SSP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSP1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSP1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSP1_A::ENABLED)
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
#[doc = "Enables clock to GPIO Pin interrupts register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PINT_A> for bool {
    #[inline(always)]
    fn from(variant: PINT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINT` reader - Enables clock to GPIO Pin interrupts register interface."]
pub struct PINT_R(crate::FieldReader<bool, PINT_A>);
impl PINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINT_A {
        match self.bits {
            false => PINT_A::DISABLED,
            true => PINT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PINT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PINT_A::ENABLED
    }
}
impl core::ops::Deref for PINT_R {
    type Target = crate::FieldReader<bool, PINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINT` writer - Enables clock to GPIO Pin interrupts register interface."]
pub struct PINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PINT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINT_A::ENABLED)
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
#[doc = "Enables clock to GPIO GROUP0 interrupt register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP0INT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GROUP0INT_A> for bool {
    #[inline(always)]
    fn from(variant: GROUP0INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GROUP0INT` reader - Enables clock to GPIO GROUP0 interrupt register interface."]
pub struct GROUP0INT_R(crate::FieldReader<bool, GROUP0INT_A>);
impl GROUP0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GROUP0INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROUP0INT_A {
        match self.bits {
            false => GROUP0INT_A::DISABLED,
            true => GROUP0INT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GROUP0INT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GROUP0INT_A::ENABLED
    }
}
impl core::ops::Deref for GROUP0INT_R {
    type Target = crate::FieldReader<bool, GROUP0INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROUP0INT` writer - Enables clock to GPIO GROUP0 interrupt register interface."]
pub struct GROUP0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP0INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP0INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP0INT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP0INT_A::ENABLED)
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
#[doc = "Enables clock to GPIO GROUP1 interrupt register interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GROUP1INT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<GROUP1INT_A> for bool {
    #[inline(always)]
    fn from(variant: GROUP1INT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GROUP1INT` reader - Enables clock to GPIO GROUP1 interrupt register interface."]
pub struct GROUP1INT_R(crate::FieldReader<bool, GROUP1INT_A>);
impl GROUP1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        GROUP1INT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GROUP1INT_A {
        match self.bits {
            false => GROUP1INT_A::DISABLED,
            true => GROUP1INT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == GROUP1INT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == GROUP1INT_A::ENABLED
    }
}
impl core::ops::Deref for GROUP1INT_R {
    type Target = crate::FieldReader<bool, GROUP1INT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GROUP1INT` writer - Enables clock to GPIO GROUP1 interrupt register interface."]
pub struct GROUP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> GROUP1INT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GROUP1INT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GROUP1INT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GROUP1INT_A::ENABLED)
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
#[doc = "Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<RAM1_A> for bool {
    #[inline(always)]
    fn from(variant: RAM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM1` reader - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
pub struct RAM1_R(crate::FieldReader<bool, RAM1_A>);
impl RAM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAM1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM1_A {
        match self.bits {
            false => RAM1_A::DISABLED,
            true => RAM1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RAM1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RAM1_A::ENABLED
    }
}
impl core::ops::Deref for RAM1_R {
    type Target = crate::FieldReader<bool, RAM1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAM1` writer - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
pub struct RAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RAM1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RAM1_A::ENABLED)
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
#[doc = "Enables USB SRAM block at address 0x2000 4000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRAM_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBRAM_A> for bool {
    #[inline(always)]
    fn from(variant: USBRAM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRAM` reader - Enables USB SRAM block at address 0x2000 4000."]
pub struct USBRAM_R(crate::FieldReader<bool, USBRAM_A>);
impl USBRAM_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBRAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRAM_A {
        match self.bits {
            false => USBRAM_A::DISABLED,
            true => USBRAM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == USBRAM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == USBRAM_A::ENABLED
    }
}
impl core::ops::Deref for USBRAM_R {
    type Target = crate::FieldReader<bool, USBRAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBRAM` writer - Enables USB SRAM block at address 0x2000 4000."]
pub struct USBRAM_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRAM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRAM_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBRAM_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBRAM_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&self) -> FLASHARRAY_R {
        FLASHARRAY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&self) -> I2C_R {
        I2C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&self) -> CT16B0_R {
        CT16B0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&self) -> CT16B1_R {
        CT16B1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&self) -> CT32B0_R {
        CT32B0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&self) -> CT32B1_R {
        CT32B1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    pub fn ssp0(&self) -> SSP0_R {
        SSP0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline(always)]
    pub fn usart(&self) -> USART_R {
        USART_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    pub fn ssp1(&self) -> SSP1_R {
        SSP1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline(always)]
    pub fn pint(&self) -> PINT_R {
        PINT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    pub fn group0int(&self) -> GROUP0INT_R {
        GROUP0INT_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    pub fn group1int(&self) -> GROUP1INT_R {
        GROUP1INT_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline(always)]
    pub fn usbram(&self) -> USBRAM_R {
        USBRAM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the AHB, the APB bridge, the Cortex-M0 FCLK and HCLK, SysCon, and the PMU. This bit is read only and always reads as 1."]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W {
        SYS_W { w: self }
    }
    #[doc = "Bit 1 - Enables clock for ROM."]
    #[inline(always)]
    pub fn rom(&mut self) -> ROM_W {
        ROM_W { w: self }
    }
    #[doc = "Bit 2 - Enables clock for RAM."]
    #[inline(always)]
    pub fn ram0(&mut self) -> RAM0_W {
        RAM0_W { w: self }
    }
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> FLASHREG_W {
        FLASHREG_W { w: self }
    }
    #[doc = "Bit 4 - Enables clock for flash array access."]
    #[inline(always)]
    pub fn flasharray(&mut self) -> FLASHARRAY_W {
        FLASHARRAY_W { w: self }
    }
    #[doc = "Bit 5 - Enables clock for I2C."]
    #[inline(always)]
    pub fn i2c(&mut self) -> I2C_W {
        I2C_W { w: self }
    }
    #[doc = "Bit 6 - Enables clock for GPIO port registers."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 7 - Enables clock for 16-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct16b0(&mut self) -> CT16B0_W {
        CT16B0_W { w: self }
    }
    #[doc = "Bit 8 - Enables clock for 16-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct16b1(&mut self) -> CT16B1_W {
        CT16B1_W { w: self }
    }
    #[doc = "Bit 9 - Enables clock for 32-bit counter/timer 0."]
    #[inline(always)]
    pub fn ct32b0(&mut self) -> CT32B0_W {
        CT32B0_W { w: self }
    }
    #[doc = "Bit 10 - Enables clock for 32-bit counter/timer 1."]
    #[inline(always)]
    pub fn ct32b1(&mut self) -> CT32B1_W {
        CT32B1_W { w: self }
    }
    #[doc = "Bit 11 - Enables clock for SSP0."]
    #[inline(always)]
    pub fn ssp0(&mut self) -> SSP0_W {
        SSP0_W { w: self }
    }
    #[doc = "Bit 12 - Enables clock for UART."]
    #[inline(always)]
    pub fn usart(&mut self) -> USART_W {
        USART_W { w: self }
    }
    #[doc = "Bit 13 - Enables clock for ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 14 - Enables clock to the USB register interface."]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 15 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 16 - Enables clock for I/O configuration block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 18 - Enables clock for SSP1."]
    #[inline(always)]
    pub fn ssp1(&mut self) -> SSP1_W {
        SSP1_W { w: self }
    }
    #[doc = "Bit 19 - Enables clock to GPIO Pin interrupts register interface."]
    #[inline(always)]
    pub fn pint(&mut self) -> PINT_W {
        PINT_W { w: self }
    }
    #[doc = "Bit 23 - Enables clock to GPIO GROUP0 interrupt register interface."]
    #[inline(always)]
    pub fn group0int(&mut self) -> GROUP0INT_W {
        GROUP0INT_W { w: self }
    }
    #[doc = "Bit 24 - Enables clock to GPIO GROUP1 interrupt register interface."]
    #[inline(always)]
    pub fn group1int(&mut self) -> GROUP1INT_W {
        GROUP1INT_W { w: self }
    }
    #[doc = "Bit 26 - Enables SRAM1 block at address 0x2000 0000. See Section 3.1 for availability of this bit."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W {
        RAM1_W { w: self }
    }
    #[doc = "Bit 27 - Enables USB SRAM block at address 0x2000 4000."]
    #[inline(always)]
    pub fn usbram(&mut self) -> USBRAM_W {
        USBRAM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysahbclkctrl](index.html) module"]
pub struct SYSAHBCLKCTRL_SPEC;
impl crate::RegisterSpec for SYSAHBCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysahbclkctrl::R](R) reader structure"]
impl crate::Readable for SYSAHBCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysahbclkctrl::W](W) writer structure"]
impl crate::Writable for SYSAHBCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0x3f"]
impl crate::Resettable for SYSAHBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
