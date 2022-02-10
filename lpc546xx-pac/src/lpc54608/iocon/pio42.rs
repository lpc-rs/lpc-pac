///Register `PIO42` reader
pub struct R(crate::R<PIO42_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO42_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO42_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO42_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PIO42` writer
pub struct W(crate::W<PIO42_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO42_SPEC>;
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
impl From<crate::W<PIO42_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO42_SPEC>) -> Self {
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
///Selects function mode (on-chip pull-up/pull-down resistor control).
///
///Value on reset: 2
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    ///0: Inactive. Inactive (no pull-down/pull-up resistor enabled).
    INACTIVE = 0,
    ///1: Pull-down. Pull-down resistor enabled.
    PULL_DOWN = 1,
    ///2: Pull-up. Pull-up resistor enabled.
    PULL_UP = 2,
    ///3: Repeater. Repeater mode.
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
///Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control).
pub struct MODE_R(crate::FieldReader<u8, MODE_A>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `INACTIVE`
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        **self == MODE_A::INACTIVE
    }
    ///Checks if the value of the field is `PULL_DOWN`
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        **self == MODE_A::PULL_DOWN
    }
    ///Checks if the value of the field is `PULL_UP`
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        **self == MODE_A::PULL_UP
    }
    ///Checks if the value of the field is `REPEATER`
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        **self == MODE_A::REPEATER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control).
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    ///Inactive. Inactive (no pull-down/pull-up resistor enabled).
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE)
    }
    ///Pull-down. Pull-down resistor enabled.
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    ///Pull-up. Pull-up resistor enabled.
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    ///Repeater. Repeater mode.
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
    ///Writes raw bits to the field
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
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
///Driver slew rate.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEW_A {
    ///0: Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously.
    STANDARD = 0,
    ///1: Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details.
    FAST = 1,
}
impl From<SLEW_A> for bool {
    #[inline(always)]
    fn from(variant: SLEW_A) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEW` reader - Driver slew rate.
pub struct SLEW_R(crate::FieldReader<bool, SLEW_A>);
impl SLEW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLEW_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SLEW_A {
        match self.bits {
            false => SLEW_A::STANDARD,
            true => SLEW_A::FAST,
        }
    }
    ///Checks if the value of the field is `STANDARD`
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        **self == SLEW_A::STANDARD
    }
    ///Checks if the value of the field is `FAST`
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        **self == SLEW_A::FAST
    }
}
impl core::ops::Deref for SLEW_R {
    type Target = crate::FieldReader<bool, SLEW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `SLEW` writer - Driver slew rate.
pub struct SLEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEW_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: SLEW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously.
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEW_A::STANDARD)
    }
    ///Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details.
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEW_A::FAST)
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
///Controls open-drain mode.
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OD_A {
    ///0: Normal. Normal push-pull output
    NORMAL = 0,
    ///1: Open-drain. Simulated open-drain output (high drive disabled).
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
///Field `OD` reader - Controls open-drain mode.
pub struct OD_R(crate::FieldReader<bool, OD_A>);
impl OD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OD_R(crate::FieldReader::new(bits))
    }
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::NORMAL,
            true => OD_A::OPEN_DRAIN,
        }
    }
    ///Checks if the value of the field is `NORMAL`
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == OD_A::NORMAL
    }
    ///Checks if the value of the field is `OPEN_DRAIN`
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        **self == OD_A::OPEN_DRAIN
    }
}
impl core::ops::Deref for OD_R {
    type Target = crate::FieldReader<bool, OD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
///Field `OD` writer - Controls open-drain mode.
pub struct OD_W<'a> {
    w: &'a mut W,
}
impl<'a> OD_W<'a> {
    ///Writes `variant` to the field
    #[inline(always)]
    pub fn variant(self, variant: OD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    ///Normal. Normal push-pull output
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OD_A::NORMAL)
    }
    ///Open-drain. Simulated open-drain output (high drive disabled).
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
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
    ///Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control).
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
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
    ///Bit 10 - Driver slew rate.
    #[inline(always)]
    pub fn slew(&self) -> SLEW_R {
        SLEW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    ///Bit 11 - Controls open-drain mode.
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    ///Bits 0:3 - Selects pin function.
    #[inline(always)]
    pub fn func(&mut self) -> FUNC_W {
        FUNC_W { w: self }
    }
    ///Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control).
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
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
    ///Bit 10 - Driver slew rate.
    #[inline(always)]
    pub fn slew(&mut self) -> SLEW_W {
        SLEW_W { w: self }
    }
    ///Bit 11 - Controls open-drain mode.
    #[inline(always)]
    pub fn od(&mut self) -> OD_W {
        OD_W { w: self }
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Digital I/O control for port 4 pins PIO4_2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pio42](index.html) module
pub struct PIO42_SPEC;
impl crate::RegisterSpec for PIO42_SPEC {
    type Ux = u32;
}
///`read()` method returns [pio42::R](R) reader structure
impl crate::Readable for PIO42_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pio42::W](W) writer structure
impl crate::Writable for PIO42_SPEC {
    type Writer = W;
}
///`reset()` method sets PIO42 to value 0x0320
impl crate::Resettable for PIO42_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0320
    }
}
