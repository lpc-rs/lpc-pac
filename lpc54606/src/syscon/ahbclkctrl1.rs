#[doc = "Register `AHBCLKCTRL1` reader"]
pub struct R(crate::R<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRL1` writer"]
pub struct W(crate::W<AHBCLKCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRL1_SPEC>;
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
impl From<crate::W<AHBCLKCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT` reader - Enables the clock for the Multi-Rate Timer."]
pub struct MRT_R(crate::FieldReader<bool, bool>);
impl MRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT` writer - Enables the clock for the Multi-Rate Timer."]
pub struct MRT_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_W<'a> {
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
#[doc = "Field `RIT` reader - Enables the clock for the Repetitive Interrupt Timer."]
pub struct RIT_R(crate::FieldReader<bool, bool>);
impl RIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIT` writer - Enables the clock for the Repetitive Interrupt Timer."]
pub struct RIT_W<'a> {
    w: &'a mut W,
}
impl<'a> RIT_W<'a> {
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
#[doc = "Field `SCT0` reader - Enables the clock for SCT0."]
pub struct SCT0_R(crate::FieldReader<bool, bool>);
impl SCT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT0` writer - Enables the clock for SCT0."]
pub struct SCT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_W<'a> {
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
#[doc = "Field `MCAN0` reader - Enables the clock for MCAN0."]
pub struct MCAN0_R(crate::FieldReader<bool, bool>);
impl MCAN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCAN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCAN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCAN0` writer - Enables the clock for MCAN0."]
pub struct MCAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN0_W<'a> {
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
#[doc = "Field `MCAN1` reader - Enables the clock for MCAN1."]
pub struct MCAN1_R(crate::FieldReader<bool, bool>);
impl MCAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCAN1` writer - Enables the clock for MCAN1."]
pub struct MCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN1_W<'a> {
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
#[doc = "Field `UTICK` reader - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
pub struct UTICK_R(crate::FieldReader<bool, bool>);
impl UTICK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTICK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTICK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK` writer - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
pub struct UTICK_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_W<'a> {
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
#[doc = "Field `FLEXCOMM0` reader - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM0_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM0` writer - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM0_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM0_W<'a> {
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
#[doc = "Field `FLEXCOMM1` reader - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM1_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM1` writer - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM1_W<'a> {
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
#[doc = "Field `FLEXCOMM2` reader - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM2_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM2_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM2` writer - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM2_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM2_W<'a> {
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
#[doc = "Field `FLEXCOMM3` reader - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM3_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM3_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM3` writer - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM3_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM3_W<'a> {
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
#[doc = "Field `FLEXCOMM4` reader - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM4_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM4_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM4` writer - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM4_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM4_W<'a> {
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
#[doc = "Field `FLEXCOMM5` reader - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM5_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM5` writer - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM5_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM5_W<'a> {
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
#[doc = "Field `FLEXCOMM6` reader - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM6_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM6_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM6` writer - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM6_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM6_W<'a> {
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
#[doc = "Field `FLEXCOMM7` reader - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM7_R(crate::FieldReader<bool, bool>);
impl FLEXCOMM7_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLEXCOMM7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLEXCOMM7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLEXCOMM7` writer - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
pub struct FLEXCOMM7_W<'a> {
    w: &'a mut W,
}
impl<'a> FLEXCOMM7_W<'a> {
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
#[doc = "Field `DMIC` reader - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
pub struct DMIC_R(crate::FieldReader<bool, bool>);
impl DMIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIC` writer - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
pub struct DMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_W<'a> {
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
#[doc = "Field `CTIMER2` reader - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
pub struct CTIMER2_R(crate::FieldReader<bool, bool>);
impl CTIMER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER2` writer - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
pub struct CTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_W<'a> {
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
#[doc = "Field `USB0D` reader - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
pub struct USB0D_R(crate::FieldReader<bool, bool>);
impl USB0D_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0D` writer - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
pub struct USB0D_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0D_W<'a> {
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
#[doc = "Field `CTIMER0` reader - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
pub struct CTIMER0_R(crate::FieldReader<bool, bool>);
impl CTIMER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER0` writer - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
pub struct CTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_W<'a> {
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
#[doc = "Field `CTIMER1` reader - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
pub struct CTIMER1_R(crate::FieldReader<bool, bool>);
impl CTIMER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER1` writer - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
pub struct CTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_W<'a> {
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
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&self) -> RIT_R {
        RIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&self) -> MCAN0_R {
        MCAN0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&self) -> MCAN1_R {
        MCAN1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&self) -> CTIMER2_R {
        CTIMER2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&self) -> USB0D_R {
        USB0D_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the clock for the Multi-Rate Timer."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 1 - Enables the clock for the Repetitive Interrupt Timer."]
    #[inline(always)]
    pub fn rit(&mut self) -> RIT_W {
        RIT_W { w: self }
    }
    #[doc = "Bit 2 - Enables the clock for SCT0."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 7 - Enables the clock for MCAN0."]
    #[inline(always)]
    pub fn mcan0(&mut self) -> MCAN0_W {
        MCAN0_W { w: self }
    }
    #[doc = "Bit 8 - Enables the clock for MCAN1."]
    #[inline(always)]
    pub fn mcan1(&mut self) -> MCAN1_W {
        MCAN1_W { w: self }
    }
    #[doc = "Bit 10 - Enables the clock for the Micro-tick Timer. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W {
        UTICK_W { w: self }
    }
    #[doc = "Bit 11 - Enables the clock for Flexcomm 0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W {
        FLEXCOMM0_W { w: self }
    }
    #[doc = "Bit 12 - Enables the clock for Flexcomm 1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W {
        FLEXCOMM1_W { w: self }
    }
    #[doc = "Bit 13 - Enables the clock for Flexcomm 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W {
        FLEXCOMM2_W { w: self }
    }
    #[doc = "Bit 14 - Enables the clock for Flexcomm 3. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W {
        FLEXCOMM3_W { w: self }
    }
    #[doc = "Bit 15 - Enables the clock for Flexcomm 4. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W {
        FLEXCOMM4_W { w: self }
    }
    #[doc = "Bit 16 - Enables the clock for Flexcomm 5. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W {
        FLEXCOMM5_W { w: self }
    }
    #[doc = "Bit 17 - Enables the clock for Flexcomm 6. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W {
        FLEXCOMM6_W { w: self }
    }
    #[doc = "Bit 18 - Enables the clock for Flexcomm 7. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W {
        FLEXCOMM7_W { w: self }
    }
    #[doc = "Bit 19 - Enables the clock for the digital microphone interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W {
        DMIC_W { w: self }
    }
    #[doc = "Bit 22 - Enables the clock for CTIMER 2. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer2(&mut self) -> CTIMER2_W {
        CTIMER2_W { w: self }
    }
    #[doc = "Bit 25 - Enables the clock for the USB0 device interface. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn usb0d(&mut self) -> USB0D_W {
        USB0D_W { w: self }
    }
    #[doc = "Bit 26 - Enables the clock for timer CTIMER0. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W {
        CTIMER0_W { w: self }
    }
    #[doc = "Bit 27 - Enables the clock for timer CTIMER1. 0 = Disable; 1 = Enable."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W {
        CTIMER1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Clock control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrl1](index.html) module"]
pub struct AHBCLKCTRL1_SPEC;
impl crate::RegisterSpec for AHBCLKCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrl1::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrl1::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRL1 to value 0"]
impl crate::Resettable for AHBCLKCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
