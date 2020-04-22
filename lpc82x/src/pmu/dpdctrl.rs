#[doc = "Reader of register DPDCTRL"]
pub type R = crate::R<u32, super::DPDCTRL>;
#[doc = "Writer for register DPDCTRL"]
pub type W = crate::W<u32, super::DPDCTRL>;
#[doc = "Register DPDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DPDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "WAKEUP pin hysteresis enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPHYS_A {
    #[doc = "0: Disabled. Hysteresis for WAKEUP pin disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Hysteresis for WAKEUP pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPHYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUPHYS`"]
pub type WAKEUPHYS_R = crate::R<bool, WAKEUPHYS_A>;
impl WAKEUPHYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPHYS_A {
        match self.bits {
            false => WAKEUPHYS_A::DISABLED,
            true => WAKEUPHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPHYS_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKEUPHYS`"]
pub struct WAKEUPHYS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPHYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPHYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Hysteresis for WAKEUP pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::DISABLED)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPHYS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEPAD_DISABLE_A {
    #[doc = "0: Enabled. The wake-up function is enabled on pin PIO0_4."]
    ENABLED = 0,
    #[doc = "1: Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    DISABLED = 1,
}
impl From<WAKEPAD_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEPAD_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEPAD_DISABLE`"]
pub type WAKEPAD_DISABLE_R = crate::R<bool, WAKEPAD_DISABLE_A>;
impl WAKEPAD_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEPAD_DISABLE_A {
        match self.bits {
            false => WAKEPAD_DISABLE_A::ENABLED,
            true => WAKEPAD_DISABLE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEPAD_DISABLE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEPAD_DISABLE_A::DISABLED
    }
}
#[doc = "Write proxy for field `WAKEPAD_DISABLE`"]
pub struct WAKEPAD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEPAD_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEPAD_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enabled. The wake-up function is enabled on pin PIO0_4."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLE_A::ENABLED)
    }
    #[doc = "Disabled. Setting this bit disables the wake-up function on pin PIO0_4."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEPAD_DISABLE_A::DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<LPOSCEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPOSCEN`"]
pub type LPOSCEN_R = crate::R<bool, LPOSCEN_A>;
impl LPOSCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSCEN_A {
        match self.bits {
            false => LPOSCEN_A::DISABLED,
            true => LPOSCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `LPOSCEN`"]
pub struct LPOSCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOSCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOSCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSCDPDEN_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<LPOSCDPDEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPOSCDPDEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPOSCDPDEN`"]
pub type LPOSCDPDEN_R = crate::R<bool, LPOSCDPDEN_A>;
impl LPOSCDPDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOSCDPDEN_A {
        match self.bits {
            false => LPOSCDPDEN_A::DISABLED,
            true => LPOSCDPDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LPOSCDPDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LPOSCDPDEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `LPOSCDPDEN`"]
pub struct LPOSCDPDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOSCDPDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOSCDPDEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LPOSCDPDEN_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LPOSCDPDEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "External clock input for the self wake-up timer WKTCLKIN hysteresis enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKEUPCLKHYS_A {
    #[doc = "0: Disabled. Hysteresis for WAKEUP clock pin disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Hysteresis for WAKEUP clock pin enabled."]
    ENABLED = 1,
}
impl From<WAKEUPCLKHYS_A> for bool {
    #[inline(always)]
    fn from(variant: WAKEUPCLKHYS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKEUPCLKHYS`"]
pub type WAKEUPCLKHYS_R = crate::R<bool, WAKEUPCLKHYS_A>;
impl WAKEUPCLKHYS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKEUPCLKHYS_A {
        match self.bits {
            false => WAKEUPCLKHYS_A::DISABLED,
            true => WAKEUPCLKHYS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKEUPCLKHYS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKEUPCLKHYS_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKEUPCLKHYS`"]
pub struct WAKEUPCLKHYS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPCLKHYS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKEUPCLKHYS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Hysteresis for WAKEUP clock pin disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKEUPCLKHYS_A::DISABLED)
    }
    #[doc = "Enabled. Hysteresis for WAKEUP clock pin enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKEUPCLKHYS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Disable the external clock input for the self wake-up timer. Setting this bit enables the self wake-up timer clock pin WKTCLKLIN. To minimize power consumption, especially in deep power-down mode, disable this clock input when not using the external clock option for the self wake-up timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKECLKPAD_DISABLE_A {
    #[doc = "0: Disabled. Setting this bit disables external clock input on pin PIO0_28."]
    DISABLED = 0,
    #[doc = "1: Enabled. The external clock input for the self wake-up timer is enabled on pin PIO0_28."]
    ENABLED = 1,
}
impl From<WAKECLKPAD_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKECLKPAD_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKECLKPAD_DISABLE`"]
pub type WAKECLKPAD_DISABLE_R = crate::R<bool, WAKECLKPAD_DISABLE_A>;
impl WAKECLKPAD_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKECLKPAD_DISABLE_A {
        match self.bits {
            false => WAKECLKPAD_DISABLE_A::DISABLED,
            true => WAKECLKPAD_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WAKECLKPAD_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WAKECLKPAD_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `WAKECLKPAD_DISABLE`"]
pub struct WAKECLKPAD_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKECLKPAD_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKECLKPAD_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Setting this bit disables external clock input on pin PIO0_28."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WAKECLKPAD_DISABLE_A::DISABLED)
    }
    #[doc = "Enabled. The external clock input for the self wake-up timer is enabled on pin PIO0_28."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WAKECLKPAD_DISABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `GPDATA`"]
pub type GPDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPDATA`"]
pub struct GPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&self) -> WAKEUPHYS_R {
        WAKEUPHYS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&self) -> WAKEPAD_DISABLE_R {
        WAKEPAD_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline(always)]
    pub fn lposcen(&self) -> LPOSCEN_R {
        LPOSCEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline(always)]
    pub fn lposcdpden(&self) -> LPOSCDPDEN_R {
        LPOSCDPDEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External clock input for the self wake-up timer WKTCLKIN hysteresis enable."]
    #[inline(always)]
    pub fn wakeupclkhys(&self) -> WAKEUPCLKHYS_R {
        WAKEUPCLKHYS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Disable the external clock input for the self wake-up timer. Setting this bit enables the self wake-up timer clock pin WKTCLKLIN. To minimize power consumption, especially in deep power-down mode, disable this clock input when not using the external clock option for the self wake-up timer."]
    #[inline(always)]
    pub fn wakeclkpad_disable(&self) -> WAKECLKPAD_DISABLE_R {
        WAKECLKPAD_DISABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - WAKEUP pin hysteresis enable"]
    #[inline(always)]
    pub fn wakeuphys(&mut self) -> WAKEUPHYS_W {
        WAKEUPHYS_W { w: self }
    }
    #[doc = "Bit 1 - WAKEUP pin disable. Setting this bit disables the wake-up pin, so it can be used for other purposes. Remark: Never set this bit if you intend to use a pin to wake up the part from Deep power-down mode. You can only disable the wake-up pin if the self wake-up timer is enabled and configured. Remark: Setting this bit is not necessary if Deep power-down mode is not used."]
    #[inline(always)]
    pub fn wakepad_disable(&mut self) -> WAKEPAD_DISABLE_W {
        WAKEPAD_DISABLE_W { w: self }
    }
    #[doc = "Bit 2 - Enable the low-power oscillator for use with the 10 kHz self wake-up timer clock. You must set this bit if the CLKSEL bit in the self wake-up timer CTRL bit is set. Do not enable the low-power oscillator if the self wake-up timer is clocked by the divided IRC or the external clock input."]
    #[inline(always)]
    pub fn lposcen(&mut self) -> LPOSCEN_W {
        LPOSCEN_W { w: self }
    }
    #[doc = "Bit 3 - causes the low-power oscillator to remain running during Deep power-down mode provided that bit 2 in this register is set as well. You must set this bit for the self wake-up timer to be able to wake up the part from Deep power-down mode. Remark: Do not set this bit unless you use the self wake-up timer with the low-power oscillator clock source to wake up from Deep power-down mode."]
    #[inline(always)]
    pub fn lposcdpden(&mut self) -> LPOSCDPDEN_W {
        LPOSCDPDEN_W { w: self }
    }
    #[doc = "Bit 4 - External clock input for the self wake-up timer WKTCLKIN hysteresis enable."]
    #[inline(always)]
    pub fn wakeupclkhys(&mut self) -> WAKEUPCLKHYS_W {
        WAKEUPCLKHYS_W { w: self }
    }
    #[doc = "Bit 5 - Disable the external clock input for the self wake-up timer. Setting this bit enables the self wake-up timer clock pin WKTCLKLIN. To minimize power consumption, especially in deep power-down mode, disable this clock input when not using the external clock option for the self wake-up timer."]
    #[inline(always)]
    pub fn wakeclkpad_disable(&mut self) -> WAKECLKPAD_DISABLE_W {
        WAKECLKPAD_DISABLE_W { w: self }
    }
    #[doc = "Bits 6:31 - Data retained during Deep power-down mode."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W {
        GPDATA_W { w: self }
    }
}
