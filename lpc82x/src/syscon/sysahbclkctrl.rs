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
#[doc = "Field `SYS` reader - Enables the clock for the AHB, the APB bridge, the Cortex-M0+ core clocks, SYSCON, and the PMU. This bit is read only and always reads as 1."]
pub struct SYS_R(crate::FieldReader<bool, bool>);
impl SYS_R {
    #[inline(always)]
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
    #[doc = "0: Disable."]
    ROM_0 = 0,
    #[doc = "1: Enable."]
    ROM_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROM_A {
        match self.bits {
            false => ROM_A::ROM_0,
            true => ROM_A::ROM_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROM_0`"]
    #[inline(always)]
    pub fn is_rom_0(&self) -> bool {
        **self == ROM_A::ROM_0
    }
    #[doc = "Checks if the value of the field is `ROM_1`"]
    #[inline(always)]
    pub fn is_rom_1(&self) -> bool {
        **self == ROM_A::ROM_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn rom_0(self) -> &'a mut W {
        self.variant(ROM_A::ROM_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn rom_1(self) -> &'a mut W {
        self.variant(ROM_A::ROM_1)
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
    #[doc = "0: Disable."]
    RAM0_1_0 = 0,
    #[doc = "1: Enable."]
    RAM0_1_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RAM0_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM0_1_A {
        match self.bits {
            false => RAM0_1_A::RAM0_1_0,
            true => RAM0_1_A::RAM0_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0_1_0`"]
    #[inline(always)]
    pub fn is_ram0_1_0(&self) -> bool {
        **self == RAM0_1_A::RAM0_1_0
    }
    #[doc = "Checks if the value of the field is `RAM0_1_1`"]
    #[inline(always)]
    pub fn is_ram0_1_1(&self) -> bool {
        **self == RAM0_1_A::RAM0_1_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn ram0_1_0(self) -> &'a mut W {
        self.variant(RAM0_1_A::RAM0_1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn ram0_1_1(self) -> &'a mut W {
        self.variant(RAM0_1_A::RAM0_1_1)
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
    #[doc = "0: Disable."]
    FLASHREG_0 = 0,
    #[doc = "1: Enable."]
    FLASHREG_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASHREG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHREG_A {
        match self.bits {
            false => FLASHREG_A::FLASHREG_0,
            true => FLASHREG_A::FLASHREG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASHREG_0`"]
    #[inline(always)]
    pub fn is_flashreg_0(&self) -> bool {
        **self == FLASHREG_A::FLASHREG_0
    }
    #[doc = "Checks if the value of the field is `FLASHREG_1`"]
    #[inline(always)]
    pub fn is_flashreg_1(&self) -> bool {
        **self == FLASHREG_A::FLASHREG_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn flashreg_0(self) -> &'a mut W {
        self.variant(FLASHREG_A::FLASHREG_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn flashreg_1(self) -> &'a mut W {
        self.variant(FLASHREG_A::FLASHREG_1)
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
#[doc = "Enables clock for flash.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_A {
    #[doc = "0: Disable."]
    FLASH_0 = 0,
    #[doc = "1: Enable."]
    FLASH_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASH_A {
        match self.bits {
            false => FLASH_A::FLASH_0,
            true => FLASH_A::FLASH_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_0`"]
    #[inline(always)]
    pub fn is_flash_0(&self) -> bool {
        **self == FLASH_A::FLASH_0
    }
    #[doc = "Checks if the value of the field is `FLASH_1`"]
    #[inline(always)]
    pub fn is_flash_1(&self) -> bool {
        **self == FLASH_A::FLASH_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn flash_0(self) -> &'a mut W {
        self.variant(FLASH_A::FLASH_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn flash_1(self) -> &'a mut W {
        self.variant(FLASH_A::FLASH_1)
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
    #[doc = "0: Disable."]
    I2C0_0 = 0,
    #[doc = "1: Enable."]
    I2C0_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::I2C0_0,
            true => I2C0_A::I2C0_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C0_0`"]
    #[inline(always)]
    pub fn is_i2c0_0(&self) -> bool {
        **self == I2C0_A::I2C0_0
    }
    #[doc = "Checks if the value of the field is `I2C0_1`"]
    #[inline(always)]
    pub fn is_i2c0_1(&self) -> bool {
        **self == I2C0_A::I2C0_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn i2c0_0(self) -> &'a mut W {
        self.variant(I2C0_A::I2C0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn i2c0_1(self) -> &'a mut W {
        self.variant(I2C0_A::I2C0_1)
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
#[doc = "Enables clock for GPIO port registers and GPIO pin interrupt registers.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIO_A {
    #[doc = "0: Disable."]
    GPIO_0 = 0,
    #[doc = "1: Enable."]
    GPIO_1 = 1,
}
impl From<GPIO_A> for bool {
    #[inline(always)]
    fn from(variant: GPIO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPIO` reader - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
pub struct GPIO_R(crate::FieldReader<bool, GPIO_A>);
impl GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIO_A {
        match self.bits {
            false => GPIO_A::GPIO_0,
            true => GPIO_A::GPIO_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_0`"]
    #[inline(always)]
    pub fn is_gpio_0(&self) -> bool {
        **self == GPIO_A::GPIO_0
    }
    #[doc = "Checks if the value of the field is `GPIO_1`"]
    #[inline(always)]
    pub fn is_gpio_1(&self) -> bool {
        **self == GPIO_A::GPIO_1
    }
}
impl core::ops::Deref for GPIO_R {
    type Target = crate::FieldReader<bool, GPIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO` writer - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn gpio_0(self) -> &'a mut W {
        self.variant(GPIO_A::GPIO_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn gpio_1(self) -> &'a mut W {
        self.variant(GPIO_A::GPIO_1)
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
#[doc = "Enables clock for switch matrix.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWM_A {
    #[doc = "0: Disable."]
    SWM_0 = 0,
    #[doc = "1: Enable."]
    SWM_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SWM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWM_A {
        match self.bits {
            false => SWM_A::SWM_0,
            true => SWM_A::SWM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWM_0`"]
    #[inline(always)]
    pub fn is_swm_0(&self) -> bool {
        **self == SWM_A::SWM_0
    }
    #[doc = "Checks if the value of the field is `SWM_1`"]
    #[inline(always)]
    pub fn is_swm_1(&self) -> bool {
        **self == SWM_A::SWM_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn swm_0(self) -> &'a mut W {
        self.variant(SWM_A::SWM_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn swm_1(self) -> &'a mut W {
        self.variant(SWM_A::SWM_1)
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
    #[doc = "0: Disable."]
    SCT_0 = 0,
    #[doc = "1: Enable."]
    SCT_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_A {
        match self.bits {
            false => SCT_A::SCT_0,
            true => SCT_A::SCT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCT_0`"]
    #[inline(always)]
    pub fn is_sct_0(&self) -> bool {
        **self == SCT_A::SCT_0
    }
    #[doc = "Checks if the value of the field is `SCT_1`"]
    #[inline(always)]
    pub fn is_sct_1(&self) -> bool {
        **self == SCT_A::SCT_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn sct_0(self) -> &'a mut W {
        self.variant(SCT_A::SCT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn sct_1(self) -> &'a mut W {
        self.variant(SCT_A::SCT_1)
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
    #[doc = "0: Disable."]
    WKT_0 = 0,
    #[doc = "1: Enable."]
    WKT_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WKT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKT_A {
        match self.bits {
            false => WKT_A::WKT_0,
            true => WKT_A::WKT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WKT_0`"]
    #[inline(always)]
    pub fn is_wkt_0(&self) -> bool {
        **self == WKT_A::WKT_0
    }
    #[doc = "Checks if the value of the field is `WKT_1`"]
    #[inline(always)]
    pub fn is_wkt_1(&self) -> bool {
        **self == WKT_A::WKT_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn wkt_0(self) -> &'a mut W {
        self.variant(WKT_A::WKT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn wkt_1(self) -> &'a mut W {
        self.variant(WKT_A::WKT_1)
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
    #[doc = "0: Disable."]
    MRT_0 = 0,
    #[doc = "1: Enable."]
    MRT_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MRT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRT_A {
        match self.bits {
            false => MRT_A::MRT_0,
            true => MRT_A::MRT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MRT_0`"]
    #[inline(always)]
    pub fn is_mrt_0(&self) -> bool {
        **self == MRT_A::MRT_0
    }
    #[doc = "Checks if the value of the field is `MRT_1`"]
    #[inline(always)]
    pub fn is_mrt_1(&self) -> bool {
        **self == MRT_A::MRT_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn mrt_0(self) -> &'a mut W {
        self.variant(MRT_A::MRT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn mrt_1(self) -> &'a mut W {
        self.variant(MRT_A::MRT_1)
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
    #[doc = "0: Disable."]
    SPI0_0 = 0,
    #[doc = "1: Enable."]
    SPI0_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::SPI0_0,
            true => SPI0_A::SPI0_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI0_0`"]
    #[inline(always)]
    pub fn is_spi0_0(&self) -> bool {
        **self == SPI0_A::SPI0_0
    }
    #[doc = "Checks if the value of the field is `SPI0_1`"]
    #[inline(always)]
    pub fn is_spi0_1(&self) -> bool {
        **self == SPI0_A::SPI0_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn spi0_0(self) -> &'a mut W {
        self.variant(SPI0_A::SPI0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn spi0_1(self) -> &'a mut W {
        self.variant(SPI0_A::SPI0_1)
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
    #[doc = "0: Disable."]
    SPI1_0 = 0,
    #[doc = "1: Enable."]
    SPI1_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::SPI1_0,
            true => SPI1_A::SPI1_1,
        }
    }
    #[doc = "Checks if the value of the field is `SPI1_0`"]
    #[inline(always)]
    pub fn is_spi1_0(&self) -> bool {
        **self == SPI1_A::SPI1_0
    }
    #[doc = "Checks if the value of the field is `SPI1_1`"]
    #[inline(always)]
    pub fn is_spi1_1(&self) -> bool {
        **self == SPI1_A::SPI1_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn spi1_0(self) -> &'a mut W {
        self.variant(SPI1_A::SPI1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn spi1_1(self) -> &'a mut W {
        self.variant(SPI1_A::SPI1_1)
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
    #[doc = "0: Disable."]
    CRC_0 = 0,
    #[doc = "1: Enable."]
    CRC_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::CRC_0,
            true => CRC_A::CRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CRC_0`"]
    #[inline(always)]
    pub fn is_crc_0(&self) -> bool {
        **self == CRC_A::CRC_0
    }
    #[doc = "Checks if the value of the field is `CRC_1`"]
    #[inline(always)]
    pub fn is_crc_1(&self) -> bool {
        **self == CRC_A::CRC_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn crc_0(self) -> &'a mut W {
        self.variant(CRC_A::CRC_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn crc_1(self) -> &'a mut W {
        self.variant(CRC_A::CRC_1)
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
#[doc = "Enables clock for USART0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0_A {
    #[doc = "0: Disable."]
    UART0_0 = 0,
    #[doc = "1: Enable."]
    UART0_1 = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART0` reader - Enables clock for USART0."]
pub struct UART0_R(crate::FieldReader<bool, UART0_A>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::UART0_0,
            true => UART0_A::UART0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART0_0`"]
    #[inline(always)]
    pub fn is_uart0_0(&self) -> bool {
        **self == UART0_A::UART0_0
    }
    #[doc = "Checks if the value of the field is `UART0_1`"]
    #[inline(always)]
    pub fn is_uart0_1(&self) -> bool {
        **self == UART0_A::UART0_1
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, UART0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0` writer - Enables clock for USART0."]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart0_0(self) -> &'a mut W {
        self.variant(UART0_A::UART0_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart0_1(self) -> &'a mut W {
        self.variant(UART0_A::UART0_1)
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
#[doc = "Enables clock for USART1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART1_A {
    #[doc = "0: Disable."]
    UART1_0 = 0,
    #[doc = "1: Enable."]
    UART1_1 = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART1` reader - Enables clock for USART1."]
pub struct UART1_R(crate::FieldReader<bool, UART1_A>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::UART1_0,
            true => UART1_A::UART1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART1_0`"]
    #[inline(always)]
    pub fn is_uart1_0(&self) -> bool {
        **self == UART1_A::UART1_0
    }
    #[doc = "Checks if the value of the field is `UART1_1`"]
    #[inline(always)]
    pub fn is_uart1_1(&self) -> bool {
        **self == UART1_A::UART1_1
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, UART1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1` writer - Enables clock for USART1."]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart1_0(self) -> &'a mut W {
        self.variant(UART1_A::UART1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart1_1(self) -> &'a mut W {
        self.variant(UART1_A::UART1_1)
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
#[doc = "Enables clock for USART2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART2_A {
    #[doc = "0: Disable."]
    UART2_0 = 0,
    #[doc = "1: Enable."]
    UART2_1 = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - Enables clock for USART2."]
pub struct UART2_R(crate::FieldReader<bool, UART2_A>);
impl UART2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::UART2_0,
            true => UART2_A::UART2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UART2_0`"]
    #[inline(always)]
    pub fn is_uart2_0(&self) -> bool {
        **self == UART2_A::UART2_0
    }
    #[doc = "Checks if the value of the field is `UART2_1`"]
    #[inline(always)]
    pub fn is_uart2_1(&self) -> bool {
        **self == UART2_A::UART2_1
    }
}
impl core::ops::Deref for UART2_R {
    type Target = crate::FieldReader<bool, UART2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2` writer - Enables clock for USART2."]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn uart2_0(self) -> &'a mut W {
        self.variant(UART2_A::UART2_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn uart2_1(self) -> &'a mut W {
        self.variant(UART2_A::UART2_1)
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
    #[doc = "0: Disable."]
    WWDT_0 = 0,
    #[doc = "1: Enable."]
    WWDT_1 = 1,
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
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WWDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WWDT_A {
        match self.bits {
            false => WWDT_A::WWDT_0,
            true => WWDT_A::WWDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WWDT_0`"]
    #[inline(always)]
    pub fn is_wwdt_0(&self) -> bool {
        **self == WWDT_A::WWDT_0
    }
    #[doc = "Checks if the value of the field is `WWDT_1`"]
    #[inline(always)]
    pub fn is_wwdt_1(&self) -> bool {
        **self == WWDT_A::WWDT_1
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
    #[doc = "Disable."]
    #[inline(always)]
    pub fn wwdt_0(self) -> &'a mut W {
        self.variant(WWDT_A::WWDT_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn wwdt_1(self) -> &'a mut W {
        self.variant(WWDT_A::WWDT_1)
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
#[doc = "Enables clock for IOCON block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOCON_A {
    #[doc = "0: Disable."]
    IOCON_0 = 0,
    #[doc = "1: Enable."]
    IOCON_1 = 1,
}
impl From<IOCON_A> for bool {
    #[inline(always)]
    fn from(variant: IOCON_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOCON` reader - Enables clock for IOCON block."]
pub struct IOCON_R(crate::FieldReader<bool, IOCON_A>);
impl IOCON_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IOCON_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IOCON_A {
        match self.bits {
            false => IOCON_A::IOCON_0,
            true => IOCON_A::IOCON_1,
        }
    }
    #[doc = "Checks if the value of the field is `IOCON_0`"]
    #[inline(always)]
    pub fn is_iocon_0(&self) -> bool {
        **self == IOCON_A::IOCON_0
    }
    #[doc = "Checks if the value of the field is `IOCON_1`"]
    #[inline(always)]
    pub fn is_iocon_1(&self) -> bool {
        **self == IOCON_A::IOCON_1
    }
}
impl core::ops::Deref for IOCON_R {
    type Target = crate::FieldReader<bool, IOCON_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCON` writer - Enables clock for IOCON block."]
pub struct IOCON_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IOCON_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn iocon_0(self) -> &'a mut W {
        self.variant(IOCON_A::IOCON_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn iocon_1(self) -> &'a mut W {
        self.variant(IOCON_A::IOCON_1)
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
#[doc = "Enables clock to analog comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP_A {
    #[doc = "0: Disable."]
    ACMP_0 = 0,
    #[doc = "1: Enable."]
    ACMP_1 = 1,
}
impl From<ACMP_A> for bool {
    #[inline(always)]
    fn from(variant: ACMP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP` reader - Enables clock to analog comparator."]
pub struct ACMP_R(crate::FieldReader<bool, ACMP_A>);
impl ACMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACMP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMP_A {
        match self.bits {
            false => ACMP_A::ACMP_0,
            true => ACMP_A::ACMP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMP_0`"]
    #[inline(always)]
    pub fn is_acmp_0(&self) -> bool {
        **self == ACMP_A::ACMP_0
    }
    #[doc = "Checks if the value of the field is `ACMP_1`"]
    #[inline(always)]
    pub fn is_acmp_1(&self) -> bool {
        **self == ACMP_A::ACMP_1
    }
}
impl core::ops::Deref for ACMP_R {
    type Target = crate::FieldReader<bool, ACMP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACMP` writer - Enables clock to analog comparator."]
pub struct ACMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn acmp_0(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn acmp_1(self) -> &'a mut W {
        self.variant(ACMP_A::ACMP_1)
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
#[doc = "Enables clock to I2C1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1_A {
    #[doc = "0: Disable."]
    I2C1_0 = 0,
    #[doc = "1: Enable."]
    I2C1_1 = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - Enables clock to I2C1."]
pub struct I2C1_R(crate::FieldReader<bool, I2C1_A>);
impl I2C1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::I2C1_0,
            true => I2C1_A::I2C1_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C1_0`"]
    #[inline(always)]
    pub fn is_i2c1_0(&self) -> bool {
        **self == I2C1_A::I2C1_0
    }
    #[doc = "Checks if the value of the field is `I2C1_1`"]
    #[inline(always)]
    pub fn is_i2c1_1(&self) -> bool {
        **self == I2C1_A::I2C1_1
    }
}
impl core::ops::Deref for I2C1_R {
    type Target = crate::FieldReader<bool, I2C1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1` writer - Enables clock to I2C1."]
pub struct I2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn i2c1_0(self) -> &'a mut W {
        self.variant(I2C1_A::I2C1_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn i2c1_1(self) -> &'a mut W {
        self.variant(I2C1_A::I2C1_1)
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
#[doc = "Enables clock to I2C2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C2_A {
    #[doc = "0: Disable."]
    I2C2_0 = 0,
    #[doc = "1: Enable."]
    I2C2_1 = 1,
}
impl From<I2C2_A> for bool {
    #[inline(always)]
    fn from(variant: I2C2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C2` reader - Enables clock to I2C2."]
pub struct I2C2_R(crate::FieldReader<bool, I2C2_A>);
impl I2C2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C2_A {
        match self.bits {
            false => I2C2_A::I2C2_0,
            true => I2C2_A::I2C2_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C2_0`"]
    #[inline(always)]
    pub fn is_i2c2_0(&self) -> bool {
        **self == I2C2_A::I2C2_0
    }
    #[doc = "Checks if the value of the field is `I2C2_1`"]
    #[inline(always)]
    pub fn is_i2c2_1(&self) -> bool {
        **self == I2C2_A::I2C2_1
    }
}
impl core::ops::Deref for I2C2_R {
    type Target = crate::FieldReader<bool, I2C2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C2` writer - Enables clock to I2C2."]
pub struct I2C2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn i2c2_0(self) -> &'a mut W {
        self.variant(I2C2_A::I2C2_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn i2c2_1(self) -> &'a mut W {
        self.variant(I2C2_A::I2C2_1)
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
#[doc = "Enables clock to I2C3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C3_A {
    #[doc = "0: Disable."]
    I2C3_0 = 0,
    #[doc = "1: Enable."]
    I2C3_1 = 1,
}
impl From<I2C3_A> for bool {
    #[inline(always)]
    fn from(variant: I2C3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C3` reader - Enables clock to I2C3."]
pub struct I2C3_R(crate::FieldReader<bool, I2C3_A>);
impl I2C3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2C3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C3_A {
        match self.bits {
            false => I2C3_A::I2C3_0,
            true => I2C3_A::I2C3_1,
        }
    }
    #[doc = "Checks if the value of the field is `I2C3_0`"]
    #[inline(always)]
    pub fn is_i2c3_0(&self) -> bool {
        **self == I2C3_A::I2C3_0
    }
    #[doc = "Checks if the value of the field is `I2C3_1`"]
    #[inline(always)]
    pub fn is_i2c3_1(&self) -> bool {
        **self == I2C3_A::I2C3_1
    }
}
impl core::ops::Deref for I2C3_R {
    type Target = crate::FieldReader<bool, I2C3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C3` writer - Enables clock to I2C3."]
pub struct I2C3_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn i2c3_0(self) -> &'a mut W {
        self.variant(I2C3_A::I2C3_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn i2c3_1(self) -> &'a mut W {
        self.variant(I2C3_A::I2C3_1)
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
#[doc = "Enables clock to ADC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_A {
    #[doc = "0: Disable."]
    ADC_0 = 0,
    #[doc = "1: Enable."]
    ADC_1 = 1,
}
impl From<ADC_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC` reader - Enables clock to ADC."]
pub struct ADC_R(crate::FieldReader<bool, ADC_A>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_A {
        match self.bits {
            false => ADC_A::ADC_0,
            true => ADC_A::ADC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_0`"]
    #[inline(always)]
    pub fn is_adc_0(&self) -> bool {
        **self == ADC_A::ADC_0
    }
    #[doc = "Checks if the value of the field is `ADC_1`"]
    #[inline(always)]
    pub fn is_adc_1(&self) -> bool {
        **self == ADC_A::ADC_1
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC` writer - Enables clock to ADC."]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn adc_0(self) -> &'a mut W {
        self.variant(ADC_A::ADC_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn adc_1(self) -> &'a mut W {
        self.variant(ADC_A::ADC_1)
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
#[doc = "Enables clock to micro-trace buffer control registers.Turn on this clock when using the micro-trace buffer for debug purposes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTB_A {
    #[doc = "0: Disable."]
    MTB_0 = 0,
    #[doc = "1: Enable."]
    MTB_1 = 1,
}
impl From<MTB_A> for bool {
    #[inline(always)]
    fn from(variant: MTB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTB` reader - Enables clock to micro-trace buffer control registers.Turn on this clock when using the micro-trace buffer for debug purposes."]
pub struct MTB_R(crate::FieldReader<bool, MTB_A>);
impl MTB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MTB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTB_A {
        match self.bits {
            false => MTB_A::MTB_0,
            true => MTB_A::MTB_1,
        }
    }
    #[doc = "Checks if the value of the field is `MTB_0`"]
    #[inline(always)]
    pub fn is_mtb_0(&self) -> bool {
        **self == MTB_A::MTB_0
    }
    #[doc = "Checks if the value of the field is `MTB_1`"]
    #[inline(always)]
    pub fn is_mtb_1(&self) -> bool {
        **self == MTB_A::MTB_1
    }
}
impl core::ops::Deref for MTB_R {
    type Target = crate::FieldReader<bool, MTB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MTB` writer - Enables clock to micro-trace buffer control registers.Turn on this clock when using the micro-trace buffer for debug purposes."]
pub struct MTB_W<'a> {
    w: &'a mut W,
}
impl<'a> MTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MTB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn mtb_0(self) -> &'a mut W {
        self.variant(MTB_A::MTB_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn mtb_1(self) -> &'a mut W {
        self.variant(MTB_A::MTB_1)
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
#[doc = "Enables clock to DMA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA_A {
    #[doc = "0: Disable."]
    DMA_0 = 0,
    #[doc = "1: Enable."]
    DMA_1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - Enables clock to DMA."]
pub struct DMA_R(crate::FieldReader<bool, DMA_A>);
impl DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::DMA_0,
            true => DMA_A::DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_0`"]
    #[inline(always)]
    pub fn is_dma_0(&self) -> bool {
        **self == DMA_A::DMA_0
    }
    #[doc = "Checks if the value of the field is `DMA_1`"]
    #[inline(always)]
    pub fn is_dma_1(&self) -> bool {
        **self == DMA_A::DMA_1
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - Enables clock to DMA."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dma_0(self) -> &'a mut W {
        self.variant(DMA_A::DMA_0)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn dma_1(self) -> &'a mut W {
        self.variant(DMA_A::DMA_1)
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
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&self) -> FLASHREG_R {
        FLASHREG_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bit 6 - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[doc = "Bit 14 - Enables clock for USART0."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables clock for USART1."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables clock for USART2."]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&self) -> WWDT_R {
        WWDT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables clock for IOCON block."]
    #[inline(always)]
    pub fn iocon(&self) -> IOCON_R {
        IOCON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables clock to analog comparator."]
    #[inline(always)]
    pub fn acmp(&self) -> ACMP_R {
        ACMP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enables clock to I2C1."]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables clock to I2C2."]
    #[inline(always)]
    pub fn i2c2(&self) -> I2C2_R {
        I2C2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enables clock to I2C3."]
    #[inline(always)]
    pub fn i2c3(&self) -> I2C3_R {
        I2C3_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enables clock to ADC."]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers.Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline(always)]
    pub fn mtb(&self) -> MTB_R {
        MTB_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enables clock to DMA."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 29) & 0x01) != 0)
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
    #[doc = "Bit 3 - Enables clock for flash register interface."]
    #[inline(always)]
    pub fn flashreg(&mut self) -> FLASHREG_W {
        FLASHREG_W { w: self }
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
    #[doc = "Bit 6 - Enables clock for GPIO port registers and GPIO pin interrupt registers."]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
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
    #[doc = "Bit 14 - Enables clock for USART0."]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 15 - Enables clock for USART1."]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 16 - Enables clock for USART2."]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 17 - Enables clock for WWDT."]
    #[inline(always)]
    pub fn wwdt(&mut self) -> WWDT_W {
        WWDT_W { w: self }
    }
    #[doc = "Bit 18 - Enables clock for IOCON block."]
    #[inline(always)]
    pub fn iocon(&mut self) -> IOCON_W {
        IOCON_W { w: self }
    }
    #[doc = "Bit 19 - Enables clock to analog comparator."]
    #[inline(always)]
    pub fn acmp(&mut self) -> ACMP_W {
        ACMP_W { w: self }
    }
    #[doc = "Bit 21 - Enables clock to I2C1."]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2C1_W {
        I2C1_W { w: self }
    }
    #[doc = "Bit 22 - Enables clock to I2C2."]
    #[inline(always)]
    pub fn i2c2(&mut self) -> I2C2_W {
        I2C2_W { w: self }
    }
    #[doc = "Bit 23 - Enables clock to I2C3."]
    #[inline(always)]
    pub fn i2c3(&mut self) -> I2C3_W {
        I2C3_W { w: self }
    }
    #[doc = "Bit 24 - Enables clock to ADC."]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Bit 26 - Enables clock to micro-trace buffer control registers.Turn on this clock when using the micro-trace buffer for debug purposes."]
    #[inline(always)]
    pub fn mtb(&mut self) -> MTB_W {
        MTB_W { w: self }
    }
    #[doc = "Bit 29 - Enables clock to DMA."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
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
#[doc = "`reset()` method sets SYSAHBCLKCTRL to value 0xdf"]
impl crate::Resettable for SYSAHBCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xdf
    }
}
