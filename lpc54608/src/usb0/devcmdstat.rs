#[doc = "Register `DEVCMDSTAT` reader"]
pub struct R(crate::R<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCMDSTAT` writer"]
pub struct W(crate::W<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCMDSTAT_SPEC>;
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
impl From<crate::W<DEVCMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub struct DEV_ADDR_R(crate::FieldReader<u8, u8>);
impl DEV_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEV_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_ADDR` writer - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
pub struct DEV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `DEV_EN` reader - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub struct DEV_EN_R(crate::FieldReader<bool, bool>);
impl DEV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_EN` writer - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
pub struct DEV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_EN_W<'a> {
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
#[doc = "Field `SETUP` reader - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub struct SETUP_R(crate::FieldReader<bool, bool>);
impl SETUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP` writer - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Forces the NEEDCLK output to always be on:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_NEEDCLK_A {
    #[doc = "0: USB_NEEDCLK has normal function."]
    NORMAL = 0,
    #[doc = "1: USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    ALWAYS_ON = 1,
}
impl From<FORCE_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_NEEDCLK` reader - Forces the NEEDCLK output to always be on:"]
pub struct FORCE_NEEDCLK_R(crate::FieldReader<bool, FORCE_NEEDCLK_A>);
impl FORCE_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_NEEDCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_NEEDCLK_A {
        match self.bits {
            false => FORCE_NEEDCLK_A::NORMAL,
            true => FORCE_NEEDCLK_A::ALWAYS_ON,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == FORCE_NEEDCLK_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ON`"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        **self == FORCE_NEEDCLK_A::ALWAYS_ON
    }
}
impl core::ops::Deref for FORCE_NEEDCLK_R {
    type Target = crate::FieldReader<bool, FORCE_NEEDCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_NEEDCLK` writer - Forces the NEEDCLK output to always be on:"]
pub struct FORCE_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_NEEDCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_NEEDCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "USB_NEEDCLK has normal function."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FORCE_NEEDCLK_A::NORMAL)
    }
    #[doc = "USB_NEEDCLK always 1. Clock will not be stopped in case of suspend."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut W {
        self.variant(FORCE_NEEDCLK_A::ALWAYS_ON)
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
#[doc = "LPM Supported:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPM_SUP_A {
    #[doc = "0: LPM not supported."]
    NO = 0,
    #[doc = "1: LPM supported."]
    YES = 1,
}
impl From<LPM_SUP_A> for bool {
    #[inline(always)]
    fn from(variant: LPM_SUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPM_SUP` reader - LPM Supported:"]
pub struct LPM_SUP_R(crate::FieldReader<bool, LPM_SUP_A>);
impl LPM_SUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_SUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPM_SUP_A {
        match self.bits {
            false => LPM_SUP_A::NO,
            true => LPM_SUP_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        **self == LPM_SUP_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        **self == LPM_SUP_A::YES
    }
}
impl core::ops::Deref for LPM_SUP_R {
    type Target = crate::FieldReader<bool, LPM_SUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_SUP` writer - LPM Supported:"]
pub struct LPM_SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_SUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPM_SUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPM not supported."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(LPM_SUP_A::NO)
    }
    #[doc = "LPM supported."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(LPM_SUP_A::YES)
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
#[doc = "Interrupt on NAK for interrupt and bulk OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    DISABLED = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 1,
}
impl From<INTONNAK_AO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP"]
pub struct INTONNAK_AO_R(crate::FieldReader<bool, INTONNAK_AO_A>);
impl INTONNAK_AO_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_AO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AO_A {
        match self.bits {
            false => INTONNAK_AO_A::DISABLED,
            true => INTONNAK_AO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTONNAK_AO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTONNAK_AO_A::ENABLED
    }
}
impl core::ops::Deref for INTONNAK_AO_R {
    type Target = crate::FieldReader<bool, INTONNAK_AO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP"]
pub struct INTONNAK_AO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_AO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_AO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::DISABLED)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTONNAK_AO_A::ENABLED)
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
#[doc = "Interrupt on NAK for interrupt and bulk IN EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_AI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    DISABLED = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 1,
}
impl From<INTONNAK_AI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_AI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP"]
pub struct INTONNAK_AI_R(crate::FieldReader<bool, INTONNAK_AI_A>);
impl INTONNAK_AI_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_AI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_AI_A {
        match self.bits {
            false => INTONNAK_AI_A::DISABLED,
            true => INTONNAK_AI_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTONNAK_AI_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTONNAK_AI_A::ENABLED
    }
}
impl core::ops::Deref for INTONNAK_AI_R {
    type Target = crate::FieldReader<bool, INTONNAK_AI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP"]
pub struct INTONNAK_AI_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_AI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_AI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::DISABLED)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTONNAK_AI_A::ENABLED)
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
#[doc = "Interrupt on NAK for control OUT EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_CO_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    DISABLED = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 1,
}
impl From<INTONNAK_CO_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP"]
pub struct INTONNAK_CO_R(crate::FieldReader<bool, INTONNAK_CO_A>);
impl INTONNAK_CO_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_CO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CO_A {
        match self.bits {
            false => INTONNAK_CO_A::DISABLED,
            true => INTONNAK_CO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTONNAK_CO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTONNAK_CO_A::ENABLED
    }
}
impl core::ops::Deref for INTONNAK_CO_R {
    type Target = crate::FieldReader<bool, INTONNAK_CO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP"]
pub struct INTONNAK_CO_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_CO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_CO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::DISABLED)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTONNAK_CO_A::ENABLED)
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
#[doc = "Interrupt on NAK for control IN EP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTONNAK_CI_A {
    #[doc = "0: Only acknowledged packets generate an interrupt"]
    DISABLED = 0,
    #[doc = "1: Both acknowledged and NAKed packets generate interrupts."]
    ENABLED = 1,
}
impl From<INTONNAK_CI_A> for bool {
    #[inline(always)]
    fn from(variant: INTONNAK_CI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP"]
pub struct INTONNAK_CI_R(crate::FieldReader<bool, INTONNAK_CI_A>);
impl INTONNAK_CI_R {
    pub(crate) fn new(bits: bool) -> Self {
        INTONNAK_CI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTONNAK_CI_A {
        match self.bits {
            false => INTONNAK_CI_A::DISABLED,
            true => INTONNAK_CI_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == INTONNAK_CI_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == INTONNAK_CI_A::ENABLED
    }
}
impl core::ops::Deref for INTONNAK_CI_R {
    type Target = crate::FieldReader<bool, INTONNAK_CI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP"]
pub struct INTONNAK_CI_W<'a> {
    w: &'a mut W,
}
impl<'a> INTONNAK_CI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTONNAK_CI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only acknowledged packets generate an interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::DISABLED)
    }
    #[doc = "Both acknowledged and NAKed packets generate interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INTONNAK_CI_A::ENABLED)
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
#[doc = "Field `DCON` reader - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
pub struct DCON_R(crate::FieldReader<bool, bool>);
impl DCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCON` writer - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
pub struct DCON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCON_W<'a> {
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
#[doc = "Field `DSUS` reader - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub struct DSUS_R(crate::FieldReader<bool, bool>);
impl DSUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSUS` writer - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
pub struct DSUS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSUS_W<'a> {
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
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub struct LPM_SUS_R(crate::FieldReader<bool, bool>);
impl LPM_SUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_SUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_SUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
pub struct LPM_SUS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_SUS_W<'a> {
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
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
pub struct LPM_REWP_R(crate::FieldReader<bool, bool>);
impl LPM_REWP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_REWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_REWP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCON_C` reader - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub struct DCON_C_R(crate::FieldReader<bool, bool>);
impl DCON_C_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCON_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCON_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCON_C` writer - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
pub struct DCON_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DCON_C_W<'a> {
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
#[doc = "Field `DSUS_C` reader - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub struct DSUS_C_R(crate::FieldReader<bool, bool>);
impl DSUS_C_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSUS_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSUS_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSUS_C` writer - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
pub struct DSUS_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DSUS_C_W<'a> {
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
#[doc = "Field `DRES_C` reader - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub struct DRES_C_R(crate::FieldReader<bool, bool>);
impl DRES_C_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRES_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRES_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRES_C` writer - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
pub struct DRES_C_W<'a> {
    w: &'a mut W,
}
impl<'a> DRES_C_W<'a> {
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
#[doc = "Field `VBUSDEBOUNCED` reader - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
pub struct VBUSDEBOUNCED_R(crate::FieldReader<bool, bool>);
impl VBUSDEBOUNCED_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSDEBOUNCED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSDEBOUNCED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:"]
    #[inline(always)]
    pub fn force_needclk(&self) -> FORCE_NEEDCLK_R {
        FORCE_NEEDCLK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host. HW sets this bit to one when the bRemoteWake bit in the LPM extended token is set to 1. HW will reset this bit to 0 when it receives the host initiated LPM resume, when a remote wake-up is sent by the device or when a USB bus reset is received. Software can use this bit to check if the remote wake-up feature is enabled by the host for the LPM transaction."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if Vbus is detected or not. The bit raises immediately when Vbus becomes high. It drops to zero if Vbus is low for at least 3 ms. If this bit is high and the DCon bit is set, the HW will enable the pull-up resistor to signal a connect."]
    #[inline(always)]
    pub fn vbusdebounced(&self) -> VBUSDEBOUNCED_R {
        VBUSDEBOUNCED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address. After bus reset, the address is reset to 0x00. If the enable bit is set, the device will respond on packets for function address DEV_ADDR. When receiving a SetAddress Control Request from the USB host, software must program the new address before completing the status phase of the SetAddress Control Request."]
    #[inline(always)]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W {
        DEV_ADDR_W { w: self }
    }
    #[doc = "Bit 7 - USB device enable. If this bit is set, the HW will start responding on packets for function address DEV_ADDR."]
    #[inline(always)]
    pub fn dev_en(&mut self) -> DEV_EN_W {
        DEV_EN_W { w: self }
    }
    #[doc = "Bit 8 - SETUP token received. If a SETUP token is received and acknowledged by the device, this bit is set. As long as this bit is set all received IN and OUT tokens will be NAKed by HW. SW must clear this bit by writing a one. If this bit is zero, HW will handle the tokens to the CTRL EP0 as indicated by the CTRL EP0 IN and OUT data information programmed by SW."]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:"]
    #[inline(always)]
    pub fn force_needclk(&mut self) -> FORCE_NEEDCLK_W {
        FORCE_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 11 - LPM Supported:"]
    #[inline(always)]
    pub fn lpm_sup(&mut self) -> LPM_SUP_W {
        LPM_SUP_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP"]
    #[inline(always)]
    pub fn intonnak_ao(&mut self) -> INTONNAK_AO_W {
        INTONNAK_AO_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP"]
    #[inline(always)]
    pub fn intonnak_ai(&mut self) -> INTONNAK_AI_W {
        INTONNAK_AI_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP"]
    #[inline(always)]
    pub fn intonnak_co(&mut self) -> INTONNAK_CO_W {
        INTONNAK_CO_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP"]
    #[inline(always)]
    pub fn intonnak_ci(&mut self) -> INTONNAK_CI_W {
        INTONNAK_CI_W { w: self }
    }
    #[doc = "Bit 16 - Device status - connect. The connect bit must be set by SW to indicate that the device must signal a connect. The pull-up resistor on USB_DP will be enabled when this bit is set and the VBUSDEBOUNCED bit is one."]
    #[inline(always)]
    pub fn dcon(&mut self) -> DCON_W {
        DCON_W { w: self }
    }
    #[doc = "Bit 17 - Device status - suspend. The suspend bit indicates the current suspend state. It is set to 1 when the device hasn't seen any activity on its upstream port for more than 3 milliseconds. It is reset to 0 on any activity. When the device is suspended (Suspend bit DSUS = 1) and the software writes a 0 to it, the device will generate a remote wake-up. This will only happen when the device is connected (Connect bit = 1). When the device is not connected or not suspended, a writing a 0 has no effect. Writing a 1 never has an effect."]
    #[inline(always)]
    pub fn dsus(&mut self) -> DSUS_W {
        DSUS_W { w: self }
    }
    #[doc = "Bit 19 - Device status - LPM Suspend. This bit represents the current LPM suspend state. It is set to 1 by HW when the device has acknowledged the LPM request from the USB host and the Token Retry Time of 10 ms has elapsed. When the device is in the LPM suspended state (LPM suspend bit = 1) and the software writes a zero to this bit, the device will generate a remote walk-up. Software can only write a zero to this bit when the LPM_REWP bit is set to 1. HW resets this bit when it receives a host initiated resume. HW only updates the LPM_SUS bit when the LPM_SUPP bit is equal to one."]
    #[inline(always)]
    pub fn lpm_sus(&mut self) -> LPM_SUS_W {
        LPM_SUS_W { w: self }
    }
    #[doc = "Bit 24 - Device status - connect change. The Connect Change bit is set when the device's pull-up resistor is disconnected because VBus disappeared. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dcon_c(&mut self) -> DCON_C_W {
        DCON_C_W { w: self }
    }
    #[doc = "Bit 25 - Device status - suspend change. The suspend change bit is set to 1 when the suspend bit toggles. The suspend bit can toggle because: - The device goes in the suspended state - The device is disconnected - The device receives resume signaling on its upstream port. The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dsus_c(&mut self) -> DSUS_C_W {
        DSUS_C_W { w: self }
    }
    #[doc = "Bit 26 - Device status - reset change. This bit is set when the device received a bus reset. On a bus reset the device will automatically go to the default state (unconfigured and responding to address 0). The bit is reset by writing a one to it."]
    #[inline(always)]
    pub fn dres_c(&mut self) -> DRES_C_W {
        DRES_C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Command/Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcmdstat](index.html) module"]
pub struct DEVCMDSTAT_SPEC;
impl crate::RegisterSpec for DEVCMDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcmdstat::R](R) reader structure"]
impl crate::Readable for DEVCMDSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcmdstat::W](W) writer structure"]
impl crate::Writable for DEVCMDSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DEVCMDSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
