#[doc = "Register `PRESETCTRL1` reader"]
pub struct R(crate::R<PRESETCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRESETCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRESETCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRESETCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRESETCTRL1` writer"]
pub struct W(crate::W<PRESETCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRESETCTRL1_SPEC>;
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
impl From<crate::W<PRESETCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRESETCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRT_RST` reader - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct MRT_RST_R(crate::FieldReader<bool, bool>);
impl MRT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRT_RST` writer - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct MRT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MRT_RST_W<'a> {
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
#[doc = "Field `SCT0_RST` reader - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct SCT0_RST_R(crate::FieldReader<bool, bool>);
impl SCT0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCT0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCT0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCT0_RST` writer - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct SCT0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_RST_W<'a> {
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
#[doc = "Field `MCAN0_RST` reader - 0 = Clear reset to this function."]
pub struct MCAN0_RST_R(crate::FieldReader<bool, bool>);
impl MCAN0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCAN0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCAN0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCAN0_RST` writer - 0 = Clear reset to this function."]
pub struct MCAN0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN0_RST_W<'a> {
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
#[doc = "Field `MCAN1_RST` reader - 0 = Clear reset to this function."]
pub struct MCAN1_RST_R(crate::FieldReader<bool, bool>);
impl MCAN1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCAN1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCAN1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCAN1_RST` writer - 0 = Clear reset to this function."]
pub struct MCAN1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> MCAN1_RST_W<'a> {
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
#[doc = "Field `UTICK_RST` reader - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct UTICK_RST_R(crate::FieldReader<bool, bool>);
impl UTICK_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTICK_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTICK_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UTICK_RST` writer - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct UTICK_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> UTICK_RST_W<'a> {
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
#[doc = "Field `FC0_RST` reader - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC0_RST_R(crate::FieldReader<bool, bool>);
impl FC0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0_RST` writer - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_RST_W<'a> {
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
#[doc = "Field `FC1_RST` reader - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC1_RST_R(crate::FieldReader<bool, bool>);
impl FC1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC1_RST` writer - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_RST_W<'a> {
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
#[doc = "Field `FC2_RST` reader - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC2_RST_R(crate::FieldReader<bool, bool>);
impl FC2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC2_RST` writer - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_RST_W<'a> {
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
#[doc = "Field `FC3_RST` reader - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC3_RST_R(crate::FieldReader<bool, bool>);
impl FC3_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC3_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC3_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC3_RST` writer - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC3_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC3_RST_W<'a> {
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
#[doc = "Field `FC4_RST` reader - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC4_RST_R(crate::FieldReader<bool, bool>);
impl FC4_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC4_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC4_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC4_RST` writer - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC4_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC4_RST_W<'a> {
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
#[doc = "Field `FC5_RST` reader - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC5_RST_R(crate::FieldReader<bool, bool>);
impl FC5_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC5_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC5_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC5_RST` writer - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC5_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC5_RST_W<'a> {
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
#[doc = "Field `FC6_RST` reader - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC6_RST_R(crate::FieldReader<bool, bool>);
impl FC6_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC6_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC6_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC6_RST` writer - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC6_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC6_RST_W<'a> {
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
#[doc = "Field `FC7_RST` reader - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC7_RST_R(crate::FieldReader<bool, bool>);
impl FC7_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC7_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC7_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC7_RST` writer - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct FC7_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> FC7_RST_W<'a> {
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
#[doc = "Field `DMIC_RST` reader - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct DMIC_RST_R(crate::FieldReader<bool, bool>);
impl DMIC_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMIC_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMIC_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMIC_RST` writer - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct DMIC_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMIC_RST_W<'a> {
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
#[doc = "Field `CTIMER2_RST` reader - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
pub struct CTIMER2_RST_R(crate::FieldReader<bool, bool>);
impl CTIMER2_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER2_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER2_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER2_RST` writer - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
pub struct CTIMER2_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER2_RST_W<'a> {
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
#[doc = "Field `USB0D_RST` reader - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct USB0D_RST_R(crate::FieldReader<bool, bool>);
impl USB0D_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0D_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0D_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0D_RST` writer - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct USB0D_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0D_RST_W<'a> {
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
#[doc = "Field `CTIMER0_RST` reader - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER0_RST_R(crate::FieldReader<bool, bool>);
impl CTIMER0_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER0_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER0_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER0_RST` writer - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER0_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER0_RST_W<'a> {
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
#[doc = "Field `CTIMER1_RST` reader - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER1_RST_R(crate::FieldReader<bool, bool>);
impl CTIMER1_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER1_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER1_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER1_RST` writer - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
pub struct CTIMER1_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER1_RST_W<'a> {
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
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&self) -> MRT_RST_R {
        MRT_RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&self) -> SCT0_RST_R {
        SCT0_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&self) -> MCAN0_RST_R {
        MCAN0_RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&self) -> MCAN1_RST_R {
        MCAN1_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&self) -> UTICK_RST_R {
        UTICK_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&self) -> FC0_RST_R {
        FC0_RST_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&self) -> FC1_RST_R {
        FC1_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&self) -> FC2_RST_R {
        FC2_RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&self) -> FC3_RST_R {
        FC3_RST_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&self) -> FC4_RST_R {
        FC4_RST_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&self) -> FC5_RST_R {
        FC5_RST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&self) -> FC6_RST_R {
        FC6_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&self) -> FC7_RST_R {
        FC7_RST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&self) -> DMIC_RST_R {
        DMIC_RST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&self) -> CTIMER2_RST_R {
        CTIMER2_RST_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&self) -> USB0D_RST_R {
        USB0D_RST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&self) -> CTIMER0_RST_R {
        CTIMER0_RST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&self) -> CTIMER1_RST_R {
        CTIMER1_RST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Multi-rate timer (MRT) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn mrt_rst(&mut self) -> MRT_RST_W {
        MRT_RST_W { w: self }
    }
    #[doc = "Bit 2 - State configurable timer 0 (SCT0) reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn sct0_rst(&mut self) -> SCT0_RST_W {
        SCT0_RST_W { w: self }
    }
    #[doc = "Bit 7 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan0_rst(&mut self) -> MCAN0_RST_W {
        MCAN0_RST_W { w: self }
    }
    #[doc = "Bit 8 - 0 = Clear reset to this function."]
    #[inline(always)]
    pub fn mcan1_rst(&mut self) -> MCAN1_RST_W {
        MCAN1_RST_W { w: self }
    }
    #[doc = "Bit 10 - Micro-tick Timer reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn utick_rst(&mut self) -> UTICK_RST_W {
        UTICK_RST_W { w: self }
    }
    #[doc = "Bit 11 - Flexcomm 0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc0_rst(&mut self) -> FC0_RST_W {
        FC0_RST_W { w: self }
    }
    #[doc = "Bit 12 - Flexcomm 1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc1_rst(&mut self) -> FC1_RST_W {
        FC1_RST_W { w: self }
    }
    #[doc = "Bit 13 - Flexcomm 2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc2_rst(&mut self) -> FC2_RST_W {
        FC2_RST_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm 3 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc3_rst(&mut self) -> FC3_RST_W {
        FC3_RST_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm 4 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc4_rst(&mut self) -> FC4_RST_W {
        FC4_RST_W { w: self }
    }
    #[doc = "Bit 16 - Flexcomm 5 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc5_rst(&mut self) -> FC5_RST_W {
        FC5_RST_W { w: self }
    }
    #[doc = "Bit 17 - Flexcomm 6 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc6_rst(&mut self) -> FC6_RST_W {
        FC6_RST_W { w: self }
    }
    #[doc = "Bit 18 - Flexcomm 7 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn fc7_rst(&mut self) -> FC7_RST_W {
        FC7_RST_W { w: self }
    }
    #[doc = "Bit 19 - Digital microphone interface reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn dmic_rst(&mut self) -> DMIC_RST_W {
        DMIC_RST_W { w: self }
    }
    #[doc = "Bit 22 - CTIMER2 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function"]
    #[inline(always)]
    pub fn ctimer2_rst(&mut self) -> CTIMER2_RST_W {
        CTIMER2_RST_W { w: self }
    }
    #[doc = "Bit 25 - USB0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn usb0d_rst(&mut self) -> USB0D_RST_W {
        USB0D_RST_W { w: self }
    }
    #[doc = "Bit 26 - CTIMER0 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer0_rst(&mut self) -> CTIMER0_RST_W {
        CTIMER0_RST_W { w: self }
    }
    #[doc = "Bit 27 - CTIMER1 reset control. 0 = Clear reset to this function. 1 = Assert reset to this function."]
    #[inline(always)]
    pub fn ctimer1_rst(&mut self) -> CTIMER1_RST_W {
        CTIMER1_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [presetctrl1](index.html) module"]
pub struct PRESETCTRL1_SPEC;
impl crate::RegisterSpec for PRESETCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [presetctrl1::R](R) reader structure"]
impl crate::Readable for PRESETCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [presetctrl1::W](W) writer structure"]
impl crate::Writable for PRESETCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRESETCTRL1 to value 0"]
impl crate::Resettable for PRESETCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
