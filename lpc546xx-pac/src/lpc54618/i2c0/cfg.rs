#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTEN_A {
    #[doc = "0: Disabled. The I2C Master function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C Master function is enabled."]
    ENABLED = 1,
}
impl From<MSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTEN` reader - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub struct MSTEN_R(crate::FieldReader<bool, MSTEN_A>);
impl MSTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTEN_A {
        match self.bits {
            false => MSTEN_A::DISABLED,
            true => MSTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MSTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MSTEN_A::ENABLED
    }
}
impl core::ops::Deref for MSTEN_R {
    type Target = crate::FieldReader<bool, MSTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTEN` writer - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub struct MSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTEN_A::ENABLED)
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
#[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVEN_A {
    #[doc = "0: Disabled. The I2C slave function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C slave function is enabled."]
    ENABLED = 1,
}
impl From<SLVEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVEN` reader - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub struct SLVEN_R(crate::FieldReader<bool, SLVEN_A>);
impl SLVEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLVEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVEN_A {
        match self.bits {
            false => SLVEN_A::DISABLED,
            true => SLVEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SLVEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SLVEN_A::ENABLED
    }
}
impl core::ops::Deref for SLVEN_R {
    type Target = crate::FieldReader<bool, SLVEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLVEN` writer - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub struct SLVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLVEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVEN_A::ENABLED)
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
#[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONEN_A {
    #[doc = "0: Disabled. The I2C Monitor function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C Monitor function is enabled."]
    ENABLED = 1,
}
impl From<MONEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONEN` reader - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub struct MONEN_R(crate::FieldReader<bool, MONEN_A>);
impl MONEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONEN_A {
        match self.bits {
            false => MONEN_A::DISABLED,
            true => MONEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MONEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MONEN_A::ENABLED
    }
}
impl core::ops::Deref for MONEN_R {
    type Target = crate::FieldReader<bool, MONEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONEN` writer - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub struct MONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MONEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONEN_A::ENABLED)
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
#[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUTEN_A {
    #[doc = "0: Disabled. Time-out function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    ENABLED = 1,
}
impl From<TIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUTEN` reader - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub struct TIMEOUTEN_R(crate::FieldReader<bool, TIMEOUTEN_A>);
impl TIMEOUTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTEN_A {
        match self.bits {
            false => TIMEOUTEN_A::DISABLED,
            true => TIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIMEOUTEN_A::ENABLED
    }
}
impl core::ops::Deref for TIMEOUTEN_R {
    type Target = crate::FieldReader<bool, TIMEOUTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUTEN` writer - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub struct TIMEOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMEOUTEN_A::ENABLED)
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
#[doc = "Monitor function Clock Stretching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONCLKSTR_A {
    #[doc = "0: Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    ENABLED = 1,
}
impl From<MONCLKSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MONCLKSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MONCLKSTR` reader - Monitor function Clock Stretching."]
pub struct MONCLKSTR_R(crate::FieldReader<bool, MONCLKSTR_A>);
impl MONCLKSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MONCLKSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONCLKSTR_A {
        match self.bits {
            false => MONCLKSTR_A::DISABLED,
            true => MONCLKSTR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == MONCLKSTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == MONCLKSTR_A::ENABLED
    }
}
impl core::ops::Deref for MONCLKSTR_R {
    type Target = crate::FieldReader<bool, MONCLKSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MONCLKSTR` writer - Monitor function Clock Stretching."]
pub struct MONCLKSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MONCLKSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONCLKSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONCLKSTR_A::DISABLED)
    }
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONCLKSTR_A::ENABLED)
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
#[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSCAPABLE_A {
    #[doc = "0: Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FAST_MODE_PLUS = 0,
    #[doc = "1: High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED = 1,
}
impl From<HSCAPABLE_A> for bool {
    #[inline(always)]
    fn from(variant: HSCAPABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCAPABLE` reader - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub struct HSCAPABLE_R(crate::FieldReader<bool, HSCAPABLE_A>);
impl HSCAPABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HSCAPABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSCAPABLE_A {
        match self.bits {
            false => HSCAPABLE_A::FAST_MODE_PLUS,
            true => HSCAPABLE_A::HIGH_SPEED,
        }
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        **self == HSCAPABLE_A::FAST_MODE_PLUS
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        **self == HSCAPABLE_A::HIGH_SPEED
    }
}
impl core::ops::Deref for HSCAPABLE_R {
    type Target = crate::FieldReader<bool, HSCAPABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSCAPABLE` writer - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub struct HSCAPABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSCAPABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSCAPABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(HSCAPABLE_A::FAST_MODE_PLUS)
    }
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(HSCAPABLE_A::HIGH_SPEED)
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
impl R {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&self) -> TIMEOUTEN_R {
        TIMEOUTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&self) -> MONCLKSTR_R {
        MONCLKSTR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&self) -> HSCAPABLE_R {
        HSCAPABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&mut self) -> MSTEN_W {
        MSTEN_W { w: self }
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W {
        SLVEN_W { w: self }
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&mut self) -> MONEN_W {
        MONEN_W { w: self }
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&mut self) -> TIMEOUTEN_W {
        TIMEOUTEN_W { w: self }
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&mut self) -> MONCLKSTR_W {
        MONCLKSTR_W { w: self }
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&mut self) -> HSCAPABLE_W {
        HSCAPABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for shared functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
