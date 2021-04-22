#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PCON_SPEC>> for R {
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl core::convert::From<crate::W<PCON_SPEC>> for W {
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: Default. The part is in active or sleep mode."]
    DEFAULT = 0,
    #[doc = "1: Deep-sleep mode. ARM WFI will enter Deep-sleep mode."]
    DEEP_SLEEP_MODE = 1,
    #[doc = "2: Power-down mode. ARM WFI will enter Power-down mode."]
    POWER_DOWN_MODE = 2,
    #[doc = "3: Deep power-down mode. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    DEEP_POWER_DOWN_MODE = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PM` reader - Power mode"]
pub struct PM_R(crate::FieldReader<u8, PM_A>);
impl PM_R {
    pub(crate) fn new(bits: u8) -> Self {
        PM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PM_A> {
        match self.bits {
            0 => Some(PM_A::DEFAULT),
            1 => Some(PM_A::DEEP_SLEEP_MODE),
            2 => Some(PM_A::POWER_DOWN_MODE),
            3 => Some(PM_A::DEEP_POWER_DOWN_MODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        **self == PM_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEP_SLEEP_MODE`"]
    #[inline(always)]
    pub fn is_deep_sleep_mode(&self) -> bool {
        **self == PM_A::DEEP_SLEEP_MODE
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN_MODE`"]
    #[inline(always)]
    pub fn is_power_down_mode(&self) -> bool {
        **self == PM_A::POWER_DOWN_MODE
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN_MODE`"]
    #[inline(always)]
    pub fn is_deep_power_down_mode(&self) -> bool {
        **self == PM_A::DEEP_POWER_DOWN_MODE
    }
}
impl core::ops::Deref for PM_R {
    type Target = crate::FieldReader<u8, PM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM` writer - Power mode"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PM_A::DEFAULT)
    }
    #[doc = "Deep-sleep mode. ARM WFI will enter Deep-sleep mode."]
    #[inline(always)]
    pub fn deep_sleep_mode(self) -> &'a mut W {
        self.variant(PM_A::DEEP_SLEEP_MODE)
    }
    #[doc = "Power-down mode. ARM WFI will enter Power-down mode."]
    #[inline(always)]
    pub fn power_down_mode(self) -> &'a mut W {
        self.variant(PM_A::POWER_DOWN_MODE)
    }
    #[doc = "Deep power-down mode. ARM WFI will enter Deep-power down mode (ARM Cortex-M0+ core powered-down)."]
    #[inline(always)]
    pub fn deep_power_down_mode(self) -> &'a mut W {
        self.variant(PM_A::DEEP_POWER_DOWN_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `NODPD` reader - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
pub struct NODPD_R(crate::FieldReader<bool, bool>);
impl NODPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        NODPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NODPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NODPD` writer - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
pub struct NODPD_W<'a> {
    w: &'a mut W,
}
impl<'a> NODPD_W<'a> {
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
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAG_A {
    #[doc = "0: Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    ACTIVE_MODE = 0,
    #[doc = "1: Low power mode. Read: Sleep, Deep-sleep or Power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    LOW_POWER_MODE = 1,
}
impl From<SLEEPFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPFLAG` reader - Sleep mode flag"]
pub struct SLEEPFLAG_R(crate::FieldReader<bool, SLEEPFLAG_A>);
impl SLEEPFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEPFLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPFLAG_A {
        match self.bits {
            false => SLEEPFLAG_A::ACTIVE_MODE,
            true => SLEEPFLAG_A::LOW_POWER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_MODE`"]
    #[inline(always)]
    pub fn is_active_mode(&self) -> bool {
        **self == SLEEPFLAG_A::ACTIVE_MODE
    }
    #[doc = "Checks if the value of the field is `LOW_POWER_MODE`"]
    #[inline(always)]
    pub fn is_low_power_mode(&self) -> bool {
        **self == SLEEPFLAG_A::LOW_POWER_MODE
    }
}
impl core::ops::Deref for SLEEPFLAG_R {
    type Target = crate::FieldReader<bool, SLEEPFLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEPFLAG` writer - Sleep mode flag"]
pub struct SLEEPFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPFLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Active mode. Read: No power-down mode entered. Part is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn active_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::ACTIVE_MODE)
    }
    #[doc = "Low power mode. Read: Sleep, Deep-sleep or Power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn low_power_mode(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::LOW_POWER_MODE)
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
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAG_A {
    #[doc = "0: Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    NOT_DEEP_POWER_DOWN = 0,
    #[doc = "1: Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DEEP_POWER_DOWN = 1,
}
impl From<DPDFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DPDFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPDFLAG` reader - Deep power-down flag"]
pub struct DPDFLAG_R(crate::FieldReader<bool, DPDFLAG_A>);
impl DPDFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDFLAG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDFLAG_A {
        match self.bits {
            false => DPDFLAG_A::NOT_DEEP_POWER_DOWN,
            true => DPDFLAG_A::DEEP_POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_not_deep_power_down(&self) -> bool {
        **self == DPDFLAG_A::NOT_DEEP_POWER_DOWN
    }
    #[doc = "Checks if the value of the field is `DEEP_POWER_DOWN`"]
    #[inline(always)]
    pub fn is_deep_power_down(&self) -> bool {
        **self == DPDFLAG_A::DEEP_POWER_DOWN
    }
}
impl core::ops::Deref for DPDFLAG_R {
    type Target = crate::FieldReader<bool, DPDFLAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDFLAG` writer - Deep power-down flag"]
pub struct DPDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDFLAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not Deep power-down. Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn not_deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAG_A::NOT_DEEP_POWER_DOWN)
    }
    #[doc = "Deep power-down. Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn deep_power_down(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DEEP_POWER_DOWN)
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
impl R {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&self) -> NODPD_R {
        NODPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&mut self) -> NODPD_W {
        NODPD_W { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&mut self) -> SLEEPFLAG_W {
        SLEEPFLAG_W { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W {
        DPDFLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
