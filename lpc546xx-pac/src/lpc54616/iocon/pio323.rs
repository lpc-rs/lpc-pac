///Register `PIO323` reader
pub struct R(crate::R<PIO323_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO323_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO323_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO323_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PIO323` writer
pub struct W(crate::W<PIO323_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO323_SPEC>;
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
impl From<crate::W<PIO323_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO323_SPEC>) -> Self {
        W(writer)
    }
}
///Selects pin function.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FUNC_A {
    ///0: Alternative connection 0.
    ALT0 = 0,
    ///1: Alternative connection 1.
    ALT1 = 1,
    ///2: Alternative connection 2.
    ALT2 = 2,
    ///3: Alternative connection 3.
    ALT3 = 3,
    ///4: Alternative connection 4.
    ALT4 = 4,
    ///5: Alternative connection 5.
    ALT5 = 5,
    ///6: Alternative connection 6.
    ALT6 = 6,
    ///7: Alternative connection 7.
    ALT7 = 7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
///Field `FUNC` reader - Selects pin function.
pub struct FUNC_R(crate::FieldReader<u8, FUNC_A>);
impl FUNC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FUNC_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::ALT0),
            1 => Some(FUNC_A::ALT1),
            2 => Some(FUNC_A::ALT2),
            3 => Some(FUNC_A::ALT3),
            4 => Some(FUNC_A::ALT4),
            5 => Some(FUNC_A::ALT5),
            6 => Some(FUNC_A::ALT6),
            7 => Some(FUNC_A::ALT7),
            _ => None,
        }
    }
    ///Checks if the value of the field is `ALT0`
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        **self == FUNC_A::ALT0
    }
    ///Checks if the value of the field is `ALT1`
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        **self == FUNC_A::ALT1
    }
    ///Checks if the value of the field is `ALT2`
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        **self == FUNC_A::ALT2
    }
    ///Checks if the value of the field is `ALT3`
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        **self == FUNC_A::ALT3
    }
    ///Checks if the value of the field is `ALT4`
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        **self == FUNC_A::ALT4
    }
    ///Checks if the value of the field is `ALT5`
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        **self == FUNC_A::ALT5
    }
    ///Checks if the value of the field is `ALT6`
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        **self == FUNC_A::ALT6
    }
    ///Checks if the value of the field is `ALT7`
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        **self == FUNC_A::ALT7
    }
}
impl core::ops::Deref for FUNC_R {
    type Target = crate::FieldReader<u8, FUNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FUNC` writer - Selects pin function.
pub struct FUNC_W<'a> {
    w: &'a mut W,
}
impl<'a> FUNC_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FUNC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    ///Alternative connection 0.
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNC_A::ALT0)
    }
    ///Alternative connection 1.
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNC_A::ALT1)
    }
    ///Alternative connection 2.
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNC_A::ALT2)
    }
    ///Alternative connection 3.
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNC_A::ALT3)
    }
    ///Alternative connection 4.
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNC_A::ALT4)
    }
    ///Alternative connection 5.
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNC_A::ALT5)
    }
    ///Alternative connection 6.
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNC_A::ALT6)
    }
    ///Alternative connection 7.
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNC_A::ALT7)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
///Controls slew rate of I2C pad.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CSLEW_A {
    ///0: I2C mode.
    I2C_MODE = 0,
    ///1: GPIO mode.
    GPIO_MODE = 1,
}
impl From<I2CSLEW_A> for bool {
    #[inline(always)]
    fn from(variant: I2CSLEW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2CSLEW` reader - Controls slew rate of I2C pad.
pub struct I2CSLEW_R(crate::FieldReader<bool, I2CSLEW_A>);
impl I2CSLEW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CSLEW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2CSLEW_A {
        match self.bits {
            false => I2CSLEW_A::I2C_MODE,
            true => I2CSLEW_A::GPIO_MODE,
        }
    }
    ///Checks if the value of the field is `I2C_MODE`
    #[inline(always)]
    pub fn is_i2c_mode(&self) -> bool {
        **self == I2CSLEW_A::I2C_MODE
    }
    ///Checks if the value of the field is `GPIO_MODE`
    #[inline(always)]
    pub fn is_gpio_mode(&self) -> bool {
        **self == I2CSLEW_A::GPIO_MODE
    }
}
impl core::ops::Deref for I2CSLEW_R {
    type Target = crate::FieldReader<bool, I2CSLEW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2CSLEW` writer - Controls slew rate of I2C pad.
pub struct I2CSLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CSLEW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2CSLEW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///I2C mode.
    #[inline(always)]
    pub fn i2c_mode(self) -> &'a mut W {
        self.variant(I2CSLEW_A::I2C_MODE)
    }
    ///GPIO mode.
    #[inline(always)]
    pub fn gpio_mode(self) -> &'a mut W {
        self.variant(I2CSLEW_A::GPIO_MODE)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
///Input polarity.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVERT_A {
    ///0: Disabled. Input function is not inverted.
    DISABLED = 0,
    ///1: Enabled. Input is function inverted.
    ENABLED = 1,
}
impl From<INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_A) -> Self {
        variant as u8 != 0
    }
}
///Field `INVERT` reader - Input polarity.
pub struct INVERT_R(crate::FieldReader<bool, INVERT_A>);
impl INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVERT_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> INVERT_A {
        match self.bits {
            false => INVERT_A::DISABLED,
            true => INVERT_A::ENABLED,
        }
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INVERT_A::DISABLED
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INVERT_A::ENABLED
    }
}
impl core::ops::Deref for INVERT_R {
    type Target = crate::FieldReader<bool, INVERT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `INVERT` writer - Input polarity.
pub struct INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> INVERT_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: INVERT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Disabled. Input function is not inverted.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERT_A::DISABLED)
    }
    ///Enabled. Input is function inverted.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERT_A::ENABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
///Select Analog/Digital mode.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIGIMODE_A {
    ///0: Analog mode.
    ANALOG = 0,
    ///1: Digital mode.
    DIGITAL = 1,
}
impl From<DIGIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIGIMODE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `DIGIMODE` reader - Select Analog/Digital mode.
pub struct DIGIMODE_R(crate::FieldReader<bool, DIGIMODE_A>);
impl DIGIMODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIGIMODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DIGIMODE_A {
        match self.bits {
            false => DIGIMODE_A::ANALOG,
            true => DIGIMODE_A::DIGITAL,
        }
    }
    ///Checks if the value of the field is `ANALOG`
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        **self == DIGIMODE_A::ANALOG
    }
    ///Checks if the value of the field is `DIGITAL`
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        **self == DIGIMODE_A::DIGITAL
    }
}
impl core::ops::Deref for DIGIMODE_R {
    type Target = crate::FieldReader<bool, DIGIMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `DIGIMODE` writer - Select Analog/Digital mode.
pub struct DIGIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DIGIMODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: DIGIMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Analog mode.
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODE_A::ANALOG)
    }
    ///Digital mode.
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODE_A::DIGITAL)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
///Controls input glitch filter.
///
///Value on reset: 1
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTEROFF_A {
    ///0: Filter enabled. Noise pulses below approximately 10 ns are filtered out.
    ENABLED = 0,
    ///1: Filter disabled. No input filtering is done.
    DISABLED = 1,
}
impl From<FILTEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEROFF_A) -> Self {
        variant as u8 != 0
    }
}
///Field `FILTEROFF` reader - Controls input glitch filter.
pub struct FILTEROFF_R(crate::FieldReader<bool, FILTEROFF_A>);
impl FILTEROFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FILTEROFF_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FILTEROFF_A {
        match self.bits {
            false => FILTEROFF_A::ENABLED,
            true => FILTEROFF_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == FILTEROFF_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == FILTEROFF_A::DISABLED
    }
}
impl core::ops::Deref for FILTEROFF_R {
    type Target = crate::FieldReader<bool, FILTEROFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `FILTEROFF` writer - Controls input glitch filter.
pub struct FILTEROFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEROFF_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: FILTEROFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Filter enabled. Noise pulses below approximately 10 ns are filtered out.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::ENABLED)
    }
    ///Filter disabled. No input filtering is done.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FILTEROFF_A::DISABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
///Controls the current sink capability of the pin.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CDRIVE_A {
    ///0: Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C.
    LOW = 0,
    ///1: High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details.
    HIGH = 1,
}
impl From<I2CDRIVE_A> for bool {
    #[inline(always)]
    fn from(variant: I2CDRIVE_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2CDRIVE` reader - Controls the current sink capability of the pin.
pub struct I2CDRIVE_R(crate::FieldReader<bool, I2CDRIVE_A>);
impl I2CDRIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CDRIVE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2CDRIVE_A {
        match self.bits {
            false => I2CDRIVE_A::LOW,
            true => I2CDRIVE_A::HIGH,
        }
    }
    ///Checks if the value of the field is `LOW`
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == I2CDRIVE_A::LOW
    }
    ///Checks if the value of the field is `HIGH`
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == I2CDRIVE_A::HIGH
    }
}
impl core::ops::Deref for I2CDRIVE_R {
    type Target = crate::FieldReader<bool, I2CDRIVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2CDRIVE` writer - Controls the current sink capability of the pin.
pub struct I2CDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDRIVE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2CDRIVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Low drive. Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C.
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(I2CDRIVE_A::LOW)
    }
    ///High drive. Output drive sink is 20 mA. This is needed for Fast Mode Plus I 2C. Refer to the appropriate specific device data sheet for details.
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(I2CDRIVE_A::HIGH)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
///Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CFILTER_A {
    ///0: Enabled. I2C 50 ns glitch filter enabled.
    ENABLED = 0,
    ///1: Disabled. I2C 50 ns glitch filter disabled.
    DISABLED = 1,
}
impl From<I2CFILTER_A> for bool {
    #[inline(always)]
    fn from(variant: I2CFILTER_A) -> Self {
        variant as u8 != 0
    }
}
///Field `I2CFILTER` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.
pub struct I2CFILTER_R(crate::FieldReader<bool, I2CFILTER_A>);
impl I2CFILTER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CFILTER_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> I2CFILTER_A {
        match self.bits {
            false => I2CFILTER_A::ENABLED,
            true => I2CFILTER_A::DISABLED,
        }
    }
    ///Checks if the value of the field is `ENABLED`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == I2CFILTER_A::ENABLED
    }
    ///Checks if the value of the field is `DISABLED`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == I2CFILTER_A::DISABLED
    }
}
impl core::ops::Deref for I2CFILTER_R {
    type Target = crate::FieldReader<bool, I2CFILTER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `I2CFILTER` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.
pub struct I2CFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CFILTER_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: I2CFILTER_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Enabled. I2C 50 ns glitch filter enabled.
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2CFILTER_A::ENABLED)
    }
    ///Disabled. I2C 50 ns glitch filter disabled.
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2CFILTER_A::DISABLED)
    }
    ///Sets the field bit
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    ///Clears the field bit
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl R {
    ///Bits 0:3 - Selects pin function.
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 6 - Controls slew rate of I2C pad.
    #[inline(always)]
    pub fn i2cslew(&self) -> I2CSLEW_R {
        I2CSLEW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    ///Bit 7 - Input polarity.
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    ///Bit 8 - Select Analog/Digital mode.
    #[inline(always)]
    pub fn digimode(&self) -> DIGIMODE_R {
        DIGIMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    ///Bit 9 - Controls input glitch filter.
    #[inline(always)]
    pub fn filteroff(&self) -> FILTEROFF_R {
        FILTEROFF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    ///Bit 10 - Controls the current sink capability of the pin.
    #[inline(always)]
    pub fn i2cdrive(&self) -> I2CDRIVE_R {
        I2CDRIVE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.
    #[inline(always)]
    pub fn i2cfilter(&self) -> I2CFILTER_R {
        I2CFILTER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Selects pin function.
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    ///Bit 6 - Controls slew rate of I2C pad.
    #[inline(always)]
    pub fn i2cslew(&mut self) -> I2CSLEW_W {
        I2CSLEW_W { w: self }
    }
    ///Bit 7 - Input polarity.
    #[inline(always)]
    pub fn invert(&mut self) -> INVERT_W {
        INVERT_W { w: self }
    }
    ///Bit 8 - Select Analog/Digital mode.
    #[inline(always)]
    pub fn digimode(&mut self) -> DIGIMODE_W {
        DIGIMODE_W { w: self }
    }
    ///Bit 9 - Controls input glitch filter.
    #[inline(always)]
    pub fn filteroff(&mut self) -> FILTEROFF_W {
        FILTEROFF_W { w: self }
    }
    ///Bit 10 - Controls the current sink capability of the pin.
    #[inline(always)]
    pub fn i2cdrive(&mut self) -> I2CDRIVE_W {
        I2CDRIVE_W { w: self }
    }
    ///Bit 11 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.
    #[inline(always)]
    pub fn i2cfilter(&mut self) -> I2CFILTER_W {
        I2CFILTER_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Digital I/O control for port 3 pins PIO3_23
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pio323](index.html) module
pub struct PIO323_SPEC;
impl crate::RegisterSpec for PIO323_SPEC {
    type Ux = u32;
}
///`read()` method returns [pio323::R](R) reader structure
impl crate::Readable for PIO323_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pio323::W](W) writer structure
impl crate::Writable for PIO323_SPEC {
    type Writer = W;
}
///`reset()` method sets PIO323 to value 0x0340
impl crate::Resettable for PIO323_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0340
    }
}
