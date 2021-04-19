#[doc = "Register `STARTER0` reader"]
pub struct R(crate::R<STARTER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<STARTER0_SPEC>> for R {
    fn from(reader: crate::R<STARTER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTER0` writer"]
pub struct W(crate::W<STARTER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTER0_SPEC>;
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
impl core::convert::From<crate::W<STARTER0_SPEC>> for W {
    fn from(writer: crate::W<STARTER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_BOD` reader - WWDT and BOD interrupt wake-up."]
pub struct WDT_BOD_R(crate::FieldReader<bool, bool>);
impl WDT_BOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_BOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_BOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_BOD` writer - WWDT and BOD interrupt wake-up."]
pub struct WDT_BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_BOD_W<'a> {
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
#[doc = "Field `DMA` reader - DMA wake-up."]
pub struct DMA_R(crate::FieldReader<bool, bool>);
impl DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA` writer - DMA wake-up."]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
#[doc = "Field `GINT0` reader - Group interrupt 0 wake-up."]
pub struct GINT0_R(crate::FieldReader<bool, bool>);
impl GINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT0` writer - Group interrupt 0 wake-up."]
pub struct GINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT0_W<'a> {
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
#[doc = "Field `GINT1` reader - Group interrupt 1 wake-up."]
pub struct GINT1_R(crate::FieldReader<bool, bool>);
impl GINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GINT1` writer - Group interrupt 1 wake-up."]
pub struct GINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> GINT1_W<'a> {
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
#[doc = "Field `PIN_INT0` reader - GPIO pin interrupt 0 wake-up."]
pub struct PIN_INT0_R(crate::FieldReader<bool, bool>);
impl PIN_INT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_INT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT0` writer - GPIO pin interrupt 0 wake-up."]
pub struct PIN_INT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT0_W<'a> {
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
#[doc = "Field `PIN_INT1` reader - GPIO pin interrupt 1 wake-up."]
pub struct PIN_INT1_R(crate::FieldReader<bool, bool>);
impl PIN_INT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_INT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT1` writer - GPIO pin interrupt 1 wake-up."]
pub struct PIN_INT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT1_W<'a> {
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
#[doc = "Field `PIN_INT2` reader - GPIO pin interrupt 2 wake-up."]
pub struct PIN_INT2_R(crate::FieldReader<bool, bool>);
impl PIN_INT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_INT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT2` writer - GPIO pin interrupt 2 wake-up."]
pub struct PIN_INT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT2_W<'a> {
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
#[doc = "Field `PIN_INT3` reader - GPIO pin interrupt 3 wake-up."]
pub struct PIN_INT3_R(crate::FieldReader<bool, bool>);
impl PIN_INT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_INT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN_INT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_INT3` writer - GPIO pin interrupt 3 wake-up."]
pub struct PIN_INT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_INT3_W<'a> {
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
#[doc = "Field `UTICK` reader - Micro-tick Timer wake-up."]
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
#[doc = "Field `UTICK` writer - Micro-tick Timer wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `MRT` reader - Multi-Rate Timer wake-up."]
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
#[doc = "Field `MRT` writer - Multi-Rate Timer wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CTIMER0` reader - Standard counter/timer CTIMER0 wake-up."]
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
#[doc = "Field `CTIMER0` writer - Standard counter/timer CTIMER0 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CTIMER1` reader - Standard counter/timer CTIMER1 wake-up."]
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
#[doc = "Field `CTIMER1` writer - Standard counter/timer CTIMER1 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SCT0` reader - SCT0 wake-up."]
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
#[doc = "Field `SCT0` writer - SCT0 wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CTIMER3` reader - Standard counter/timer CTIMER3 wake-up."]
pub struct CTIMER3_R(crate::FieldReader<bool, bool>);
impl CTIMER3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTIMER3` writer - Standard counter/timer CTIMER3 wake-up."]
pub struct CTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIMER3_W<'a> {
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
#[doc = "Field `FLEXCOMM0` reader - Flexcomm0 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM0` writer - Flexcomm0 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `FLEXCOMM1` reader - Flexcomm1 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM1` writer - Flexcomm1 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `FLEXCOMM2` reader - Flexcomm2 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM2` writer - Flexcomm2 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FLEXCOMM3` reader - Flexcomm3 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM3` writer - Flexcomm3 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `FLEXCOMM4` reader - Flexcomm4 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM4` writer - Flexcomm4 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `FLEXCOMM5` reader - Flexcomm5 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM5` writer - Flexcomm5 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `FLEXCOMM6` reader - Flexcomm6 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM6` writer - Flexcomm6 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `FLEXCOMM7` reader - Flexcomm7 peripheral interrupt wake-up."]
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
#[doc = "Field `FLEXCOMM7` writer - Flexcomm7 peripheral interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ADC0_SEQA` reader - ADC0 sequence A interrupt wake-up."]
pub struct ADC0_SEQA_R(crate::FieldReader<bool, bool>);
impl ADC0_SEQA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_SEQA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0_SEQA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_SEQA` writer - ADC0 sequence A interrupt wake-up."]
pub struct ADC0_SEQA_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_SEQA_W<'a> {
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
#[doc = "Field `ADC0_SEQB` reader - ADC0 sequence B interrupt wake-up."]
pub struct ADC0_SEQB_R(crate::FieldReader<bool, bool>);
impl ADC0_SEQB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_SEQB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0_SEQB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_SEQB` writer - ADC0 sequence B interrupt wake-up."]
pub struct ADC0_SEQB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_SEQB_W<'a> {
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
#[doc = "Field `ADC0_THCMP` reader - ADC0 threshold and error interrupt wake-up."]
pub struct ADC0_THCMP_R(crate::FieldReader<bool, bool>);
impl ADC0_THCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC0_THCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC0_THCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC0_THCMP` writer - ADC0 threshold and error interrupt wake-up."]
pub struct ADC0_THCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_THCMP_W<'a> {
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
#[doc = "Field `DMIC` reader - Digital microphone interrupt wake-up."]
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
#[doc = "Field `DMIC` writer - Digital microphone interrupt wake-up."]
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `HWVAD` reader - Hardware voice activity detect interrupt wake-up."]
pub struct HWVAD_R(crate::FieldReader<bool, bool>);
impl HWVAD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWVAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWVAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWVAD` writer - Hardware voice activity detect interrupt wake-up."]
pub struct HWVAD_W<'a> {
    w: &'a mut W,
}
impl<'a> HWVAD_W<'a> {
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
#[doc = "Field `USB0_NEEDCLK` reader - USB activity interrupt wake-up."]
pub struct USB0_NEEDCLK_R(crate::FieldReader<bool, bool>);
impl USB0_NEEDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_NEEDCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0_NEEDCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0_NEEDCLK` writer - USB activity interrupt wake-up."]
pub struct USB0_NEEDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_NEEDCLK_W<'a> {
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
#[doc = "Field `USB0` reader - USB function interrupt wake-up."]
pub struct USB0_R(crate::FieldReader<bool, bool>);
impl USB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB0` writer - USB function interrupt wake-up."]
pub struct USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> USB0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `RTC` reader - RTC interrupt alarm and wake-up timer."]
pub struct RTC_R(crate::FieldReader<bool, bool>);
impl RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC` writer - RTC interrupt alarm and wake-up timer."]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&self) -> WDT_BOD_R {
        WDT_BOD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&self) -> GINT0_R {
        GINT0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&self) -> GINT1_R {
        GINT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&self) -> PIN_INT0_R {
        PIN_INT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&self) -> PIN_INT1_R {
        PIN_INT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&self) -> PIN_INT2_R {
        PIN_INT2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&self) -> PIN_INT3_R {
        PIN_INT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&self) -> UTICK_R {
        UTICK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&self) -> MRT_R {
        MRT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&self) -> CTIMER0_R {
        CTIMER0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&self) -> CTIMER1_R {
        CTIMER1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&self) -> SCT0_R {
        SCT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&self) -> CTIMER3_R {
        CTIMER3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&self) -> FLEXCOMM0_R {
        FLEXCOMM0_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&self) -> FLEXCOMM1_R {
        FLEXCOMM1_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&self) -> FLEXCOMM2_R {
        FLEXCOMM2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&self) -> FLEXCOMM3_R {
        FLEXCOMM3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&self) -> FLEXCOMM4_R {
        FLEXCOMM4_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&self) -> FLEXCOMM5_R {
        FLEXCOMM5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&self) -> FLEXCOMM6_R {
        FLEXCOMM6_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&self) -> FLEXCOMM7_R {
        FLEXCOMM7_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&self) -> ADC0_SEQA_R {
        ADC0_SEQA_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&self) -> ADC0_SEQB_R {
        ADC0_SEQB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&self) -> ADC0_THCMP_R {
        ADC0_THCMP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&self) -> DMIC_R {
        DMIC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&self) -> HWVAD_R {
        HWVAD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&self) -> USB0_NEEDCLK_R {
        USB0_NEEDCLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&self) -> USB0_R {
        USB0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDT and BOD interrupt wake-up."]
    #[inline(always)]
    pub fn wdt_bod(&mut self) -> WDT_BOD_W {
        WDT_BOD_W { w: self }
    }
    #[doc = "Bit 1 - DMA wake-up."]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bit 2 - Group interrupt 0 wake-up."]
    #[inline(always)]
    pub fn gint0(&mut self) -> GINT0_W {
        GINT0_W { w: self }
    }
    #[doc = "Bit 3 - Group interrupt 1 wake-up."]
    #[inline(always)]
    pub fn gint1(&mut self) -> GINT1_W {
        GINT1_W { w: self }
    }
    #[doc = "Bit 4 - GPIO pin interrupt 0 wake-up."]
    #[inline(always)]
    pub fn pin_int0(&mut self) -> PIN_INT0_W {
        PIN_INT0_W { w: self }
    }
    #[doc = "Bit 5 - GPIO pin interrupt 1 wake-up."]
    #[inline(always)]
    pub fn pin_int1(&mut self) -> PIN_INT1_W {
        PIN_INT1_W { w: self }
    }
    #[doc = "Bit 6 - GPIO pin interrupt 2 wake-up."]
    #[inline(always)]
    pub fn pin_int2(&mut self) -> PIN_INT2_W {
        PIN_INT2_W { w: self }
    }
    #[doc = "Bit 7 - GPIO pin interrupt 3 wake-up."]
    #[inline(always)]
    pub fn pin_int3(&mut self) -> PIN_INT3_W {
        PIN_INT3_W { w: self }
    }
    #[doc = "Bit 8 - Micro-tick Timer wake-up."]
    #[inline(always)]
    pub fn utick(&mut self) -> UTICK_W {
        UTICK_W { w: self }
    }
    #[doc = "Bit 9 - Multi-Rate Timer wake-up."]
    #[inline(always)]
    pub fn mrt(&mut self) -> MRT_W {
        MRT_W { w: self }
    }
    #[doc = "Bit 10 - Standard counter/timer CTIMER0 wake-up."]
    #[inline(always)]
    pub fn ctimer0(&mut self) -> CTIMER0_W {
        CTIMER0_W { w: self }
    }
    #[doc = "Bit 11 - Standard counter/timer CTIMER1 wake-up."]
    #[inline(always)]
    pub fn ctimer1(&mut self) -> CTIMER1_W {
        CTIMER1_W { w: self }
    }
    #[doc = "Bit 12 - SCT0 wake-up."]
    #[inline(always)]
    pub fn sct0(&mut self) -> SCT0_W {
        SCT0_W { w: self }
    }
    #[doc = "Bit 13 - Standard counter/timer CTIMER3 wake-up."]
    #[inline(always)]
    pub fn ctimer3(&mut self) -> CTIMER3_W {
        CTIMER3_W { w: self }
    }
    #[doc = "Bit 14 - Flexcomm0 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm0(&mut self) -> FLEXCOMM0_W {
        FLEXCOMM0_W { w: self }
    }
    #[doc = "Bit 15 - Flexcomm1 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm1(&mut self) -> FLEXCOMM1_W {
        FLEXCOMM1_W { w: self }
    }
    #[doc = "Bit 16 - Flexcomm2 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm2(&mut self) -> FLEXCOMM2_W {
        FLEXCOMM2_W { w: self }
    }
    #[doc = "Bit 17 - Flexcomm3 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm3(&mut self) -> FLEXCOMM3_W {
        FLEXCOMM3_W { w: self }
    }
    #[doc = "Bit 18 - Flexcomm4 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm4(&mut self) -> FLEXCOMM4_W {
        FLEXCOMM4_W { w: self }
    }
    #[doc = "Bit 19 - Flexcomm5 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm5(&mut self) -> FLEXCOMM5_W {
        FLEXCOMM5_W { w: self }
    }
    #[doc = "Bit 20 - Flexcomm6 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm6(&mut self) -> FLEXCOMM6_W {
        FLEXCOMM6_W { w: self }
    }
    #[doc = "Bit 21 - Flexcomm7 peripheral interrupt wake-up."]
    #[inline(always)]
    pub fn flexcomm7(&mut self) -> FLEXCOMM7_W {
        FLEXCOMM7_W { w: self }
    }
    #[doc = "Bit 22 - ADC0 sequence A interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqa(&mut self) -> ADC0_SEQA_W {
        ADC0_SEQA_W { w: self }
    }
    #[doc = "Bit 23 - ADC0 sequence B interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_seqb(&mut self) -> ADC0_SEQB_W {
        ADC0_SEQB_W { w: self }
    }
    #[doc = "Bit 24 - ADC0 threshold and error interrupt wake-up."]
    #[inline(always)]
    pub fn adc0_thcmp(&mut self) -> ADC0_THCMP_W {
        ADC0_THCMP_W { w: self }
    }
    #[doc = "Bit 25 - Digital microphone interrupt wake-up."]
    #[inline(always)]
    pub fn dmic(&mut self) -> DMIC_W {
        DMIC_W { w: self }
    }
    #[doc = "Bit 26 - Hardware voice activity detect interrupt wake-up."]
    #[inline(always)]
    pub fn hwvad(&mut self) -> HWVAD_W {
        HWVAD_W { w: self }
    }
    #[doc = "Bit 27 - USB activity interrupt wake-up."]
    #[inline(always)]
    pub fn usb0_needclk(&mut self) -> USB0_NEEDCLK_W {
        USB0_NEEDCLK_W { w: self }
    }
    #[doc = "Bit 28 - USB function interrupt wake-up."]
    #[inline(always)]
    pub fn usb0(&mut self) -> USB0_W {
        USB0_W { w: self }
    }
    #[doc = "Bit 29 - RTC interrupt alarm and wake-up timer."]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start logic 0 wake-up enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [starter0](index.html) module"]
pub struct STARTER0_SPEC;
impl crate::RegisterSpec for STARTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [starter0::R](R) reader structure"]
impl crate::Readable for STARTER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [starter0::W](W) writer structure"]
impl crate::Writable for STARTER0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STARTER0 to value 0"]
impl crate::Resettable for STARTER0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
