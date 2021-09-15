#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THCMP0` reader - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
pub struct THCMP0_R(crate::FieldReader<bool, bool>);
impl THCMP0_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP0` writer - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
pub struct THCMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP0_W<'a> {
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
#[doc = "Field `THCMP1` reader - Threshold comparison event on Channel 1. See description for channel 0."]
pub struct THCMP1_R(crate::FieldReader<bool, bool>);
impl THCMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP1` writer - Threshold comparison event on Channel 1. See description for channel 0."]
pub struct THCMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP1_W<'a> {
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
#[doc = "Field `THCMP2` reader - Threshold comparison event on Channel 2. See description for channel 0."]
pub struct THCMP2_R(crate::FieldReader<bool, bool>);
impl THCMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP2` writer - Threshold comparison event on Channel 2. See description for channel 0."]
pub struct THCMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP2_W<'a> {
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
#[doc = "Field `THCMP3` reader - Threshold comparison event on Channel 3. See description for channel 0."]
pub struct THCMP3_R(crate::FieldReader<bool, bool>);
impl THCMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP3` writer - Threshold comparison event on Channel 3. See description for channel 0."]
pub struct THCMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP3_W<'a> {
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
#[doc = "Field `THCMP4` reader - Threshold comparison event on Channel 4. See description for channel 0."]
pub struct THCMP4_R(crate::FieldReader<bool, bool>);
impl THCMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP4` writer - Threshold comparison event on Channel 4. See description for channel 0."]
pub struct THCMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP4_W<'a> {
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
#[doc = "Field `THCMP5` reader - Threshold comparison event on Channel 5. See description for channel 0."]
pub struct THCMP5_R(crate::FieldReader<bool, bool>);
impl THCMP5_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP5` writer - Threshold comparison event on Channel 5. See description for channel 0."]
pub struct THCMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP5_W<'a> {
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
#[doc = "Field `THCMP6` reader - Threshold comparison event on Channel 6. See description for channel 0."]
pub struct THCMP6_R(crate::FieldReader<bool, bool>);
impl THCMP6_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP6` writer - Threshold comparison event on Channel 6. See description for channel 0."]
pub struct THCMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP6_W<'a> {
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
#[doc = "Field `THCMP7` reader - Threshold comparison event on Channel 7. See description for channel 0."]
pub struct THCMP7_R(crate::FieldReader<bool, bool>);
impl THCMP7_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP7` writer - Threshold comparison event on Channel 7. See description for channel 0."]
pub struct THCMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP7_W<'a> {
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
#[doc = "Field `THCMP8` reader - Threshold comparison event on Channel 8. See description for channel 0."]
pub struct THCMP8_R(crate::FieldReader<bool, bool>);
impl THCMP8_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP8` writer - Threshold comparison event on Channel 8. See description for channel 0."]
pub struct THCMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP8_W<'a> {
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
#[doc = "Field `THCMP9` reader - Threshold comparison event on Channel 9. See description for channel 0."]
pub struct THCMP9_R(crate::FieldReader<bool, bool>);
impl THCMP9_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP9` writer - Threshold comparison event on Channel 9. See description for channel 0."]
pub struct THCMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP9_W<'a> {
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
#[doc = "Field `THCMP10` reader - Threshold comparison event on Channel 10. See description for channel 0."]
pub struct THCMP10_R(crate::FieldReader<bool, bool>);
impl THCMP10_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP10` writer - Threshold comparison event on Channel 10. See description for channel 0."]
pub struct THCMP10_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP10_W<'a> {
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
#[doc = "Field `THCMP11` reader - Threshold comparison event on Channel 11. See description for channel 0."]
pub struct THCMP11_R(crate::FieldReader<bool, bool>);
impl THCMP11_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP11` writer - Threshold comparison event on Channel 11. See description for channel 0."]
pub struct THCMP11_W<'a> {
    w: &'a mut W,
}
impl<'a> THCMP11_W<'a> {
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
#[doc = "Field `OVERRUN0` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 0"]
pub struct OVERRUN0_R(crate::FieldReader<bool, bool>);
impl OVERRUN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN1` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 1"]
pub struct OVERRUN1_R(crate::FieldReader<bool, bool>);
impl OVERRUN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN2` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 2"]
pub struct OVERRUN2_R(crate::FieldReader<bool, bool>);
impl OVERRUN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN3` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 3"]
pub struct OVERRUN3_R(crate::FieldReader<bool, bool>);
impl OVERRUN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN4` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 4"]
pub struct OVERRUN4_R(crate::FieldReader<bool, bool>);
impl OVERRUN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN5` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 5"]
pub struct OVERRUN5_R(crate::FieldReader<bool, bool>);
impl OVERRUN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN6` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 6"]
pub struct OVERRUN6_R(crate::FieldReader<bool, bool>);
impl OVERRUN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN7` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 7"]
pub struct OVERRUN7_R(crate::FieldReader<bool, bool>);
impl OVERRUN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN8` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 8"]
pub struct OVERRUN8_R(crate::FieldReader<bool, bool>);
impl OVERRUN8_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN9` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 9"]
pub struct OVERRUN9_R(crate::FieldReader<bool, bool>);
impl OVERRUN9_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN10` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 10"]
pub struct OVERRUN10_R(crate::FieldReader<bool, bool>);
impl OVERRUN10_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVERRUN11` reader - Mirrors the OVERRRUN status flag from the result register for ADC channel 11"]
pub struct OVERRUN11_R(crate::FieldReader<bool, bool>);
impl OVERRUN11_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVERRUN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERRUN11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQA_OVR` reader - Mirrors the global OVERRUN status flag in the SEQA_GDAT register"]
pub struct SEQA_OVR_R(crate::FieldReader<bool, bool>);
impl SEQA_OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQA_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQA_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQB_OVR` reader - Mirrors the global OVERRUN status flag in the SEQB_GDAT register"]
pub struct SEQB_OVR_R(crate::FieldReader<bool, bool>);
impl SEQB_OVR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQB_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQB_OVR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQA_INT` reader - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQA_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQA_GDAT), which is set at the end of every ADC conversion performed as part of sequence A. It will be cleared automatically when the SEQA_GDAT register is read. If the MODE bit in the SEQA_CTRL register is 1, this flag will be set upon completion of an entire A sequence. In this case it must be cleared by writing a 1 to this SEQA_INT bit. This interrupt must be enabled in the INTEN register."]
pub struct SEQA_INT_R(crate::FieldReader<bool, bool>);
impl SEQA_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQA_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQA_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQB_INT` reader - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQB_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQB_GDAT), which is set at the end of every ADC conversion performed as part of sequence B. It will be cleared automatically when the SEQB_GDAT register is read. If the MODE bit in the SEQB_CTRL register is 1, this flag will be set upon completion of an entire B sequence. In this case it must be cleared by writing a 1 to this SEQB_INT bit. This interrupt must be enabled in the INTEN register."]
pub struct SEQB_INT_R(crate::FieldReader<bool, bool>);
impl SEQB_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQB_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEQB_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THCMP_INT` reader - Threshold Comparison Interrupt. This bit will be set if any of the THCMP flags in the lower bits of this register are set to 1 (due to an enabled out-of-range or threshold-crossing event on any channel). Each type of threshold comparison interrupt on each channel must be individually enabled in the INTEN register to cause this interrupt. This bit will be cleared when all of the individual threshold flags are cleared via writing 1s to those bits."]
pub struct THCMP_INT_R(crate::FieldReader<bool, bool>);
impl THCMP_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        THCMP_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THCMP_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_INT` reader - Overrun Interrupt flag. Any overrun bit in any of the individual channel data registers will cause this interrupt. In addition, if the MODE bit in either of the SEQn_CTRL registers is 0 then the OVERRUN bit in the corresponding SEQn_GDAT register will also cause this interrupt. This interrupt must be enabled in the INTEN register. This bit will be cleared when all of the individual overrun bits have been cleared via reading the corresponding data registers."]
pub struct OVR_INT_R(crate::FieldReader<bool, bool>);
impl OVR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
    #[inline(always)]
    pub fn thcmp0(&self) -> THCMP0_R {
        THCMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Threshold comparison event on Channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp1(&self) -> THCMP1_R {
        THCMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Threshold comparison event on Channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp2(&self) -> THCMP2_R {
        THCMP2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Threshold comparison event on Channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp3(&self) -> THCMP3_R {
        THCMP3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Threshold comparison event on Channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp4(&self) -> THCMP4_R {
        THCMP4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Threshold comparison event on Channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp5(&self) -> THCMP5_R {
        THCMP5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Threshold comparison event on Channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp6(&self) -> THCMP6_R {
        THCMP6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Threshold comparison event on Channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp7(&self) -> THCMP7_R {
        THCMP7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Threshold comparison event on Channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp8(&self) -> THCMP8_R {
        THCMP8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Threshold comparison event on Channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp9(&self) -> THCMP9_R {
        THCMP9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Threshold comparison event on Channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp10(&self) -> THCMP10_R {
        THCMP10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Threshold comparison event on Channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp11(&self) -> THCMP11_R {
        THCMP11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mirrors the OVERRRUN status flag from the result register for ADC channel 0"]
    #[inline(always)]
    pub fn overrun0(&self) -> OVERRUN0_R {
        OVERRUN0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mirrors the OVERRRUN status flag from the result register for ADC channel 1"]
    #[inline(always)]
    pub fn overrun1(&self) -> OVERRUN1_R {
        OVERRUN1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mirrors the OVERRRUN status flag from the result register for ADC channel 2"]
    #[inline(always)]
    pub fn overrun2(&self) -> OVERRUN2_R {
        OVERRUN2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mirrors the OVERRRUN status flag from the result register for ADC channel 3"]
    #[inline(always)]
    pub fn overrun3(&self) -> OVERRUN3_R {
        OVERRUN3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mirrors the OVERRRUN status flag from the result register for ADC channel 4"]
    #[inline(always)]
    pub fn overrun4(&self) -> OVERRUN4_R {
        OVERRUN4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mirrors the OVERRRUN status flag from the result register for ADC channel 5"]
    #[inline(always)]
    pub fn overrun5(&self) -> OVERRUN5_R {
        OVERRUN5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mirrors the OVERRRUN status flag from the result register for ADC channel 6"]
    #[inline(always)]
    pub fn overrun6(&self) -> OVERRUN6_R {
        OVERRUN6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mirrors the OVERRRUN status flag from the result register for ADC channel 7"]
    #[inline(always)]
    pub fn overrun7(&self) -> OVERRUN7_R {
        OVERRUN7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mirrors the OVERRRUN status flag from the result register for ADC channel 8"]
    #[inline(always)]
    pub fn overrun8(&self) -> OVERRUN8_R {
        OVERRUN8_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mirrors the OVERRRUN status flag from the result register for ADC channel 9"]
    #[inline(always)]
    pub fn overrun9(&self) -> OVERRUN9_R {
        OVERRUN9_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mirrors the OVERRRUN status flag from the result register for ADC channel 10"]
    #[inline(always)]
    pub fn overrun10(&self) -> OVERRUN10_R {
        OVERRUN10_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mirrors the OVERRRUN status flag from the result register for ADC channel 11"]
    #[inline(always)]
    pub fn overrun11(&self) -> OVERRUN11_R {
        OVERRUN11_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mirrors the global OVERRUN status flag in the SEQA_GDAT register"]
    #[inline(always)]
    pub fn seqa_ovr(&self) -> SEQA_OVR_R {
        SEQA_OVR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Mirrors the global OVERRUN status flag in the SEQB_GDAT register"]
    #[inline(always)]
    pub fn seqb_ovr(&self) -> SEQB_OVR_R {
        SEQB_OVR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQA_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQA_GDAT), which is set at the end of every ADC conversion performed as part of sequence A. It will be cleared automatically when the SEQA_GDAT register is read. If the MODE bit in the SEQA_CTRL register is 1, this flag will be set upon completion of an entire A sequence. In this case it must be cleared by writing a 1 to this SEQA_INT bit. This interrupt must be enabled in the INTEN register."]
    #[inline(always)]
    pub fn seqa_int(&self) -> SEQA_INT_R {
        SEQA_INT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Sequence A interrupt/DMA trigger. If the MODE bit in the SEQB_CTRL register is 0, this flag will mirror the DATAVALID bit in the sequence A global data register (SEQB_GDAT), which is set at the end of every ADC conversion performed as part of sequence B. It will be cleared automatically when the SEQB_GDAT register is read. If the MODE bit in the SEQB_CTRL register is 1, this flag will be set upon completion of an entire B sequence. In this case it must be cleared by writing a 1 to this SEQB_INT bit. This interrupt must be enabled in the INTEN register."]
    #[inline(always)]
    pub fn seqb_int(&self) -> SEQB_INT_R {
        SEQB_INT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Threshold Comparison Interrupt. This bit will be set if any of the THCMP flags in the lower bits of this register are set to 1 (due to an enabled out-of-range or threshold-crossing event on any channel). Each type of threshold comparison interrupt on each channel must be individually enabled in the INTEN register to cause this interrupt. This bit will be cleared when all of the individual threshold flags are cleared via writing 1s to those bits."]
    #[inline(always)]
    pub fn thcmp_int(&self) -> THCMP_INT_R {
        THCMP_INT_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Overrun Interrupt flag. Any overrun bit in any of the individual channel data registers will cause this interrupt. In addition, if the MODE bit in either of the SEQn_CTRL registers is 0 then the OVERRUN bit in the corresponding SEQn_GDAT register will also cause this interrupt. This interrupt must be enabled in the INTEN register. This bit will be cleared when all of the individual overrun bits have been cleared via reading the corresponding data registers."]
    #[inline(always)]
    pub fn ovr_int(&self) -> OVR_INT_R {
        OVR_INT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Threshold comparison event on Channel 0. Set to 1 upon either an out-of-range result or a threshold-crossing result if enabled to do so in the INTEN register. This bit is cleared by writing a 1."]
    #[inline(always)]
    pub fn thcmp0(&mut self) -> THCMP0_W {
        THCMP0_W { w: self }
    }
    #[doc = "Bit 1 - Threshold comparison event on Channel 1. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp1(&mut self) -> THCMP1_W {
        THCMP1_W { w: self }
    }
    #[doc = "Bit 2 - Threshold comparison event on Channel 2. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp2(&mut self) -> THCMP2_W {
        THCMP2_W { w: self }
    }
    #[doc = "Bit 3 - Threshold comparison event on Channel 3. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp3(&mut self) -> THCMP3_W {
        THCMP3_W { w: self }
    }
    #[doc = "Bit 4 - Threshold comparison event on Channel 4. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp4(&mut self) -> THCMP4_W {
        THCMP4_W { w: self }
    }
    #[doc = "Bit 5 - Threshold comparison event on Channel 5. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp5(&mut self) -> THCMP5_W {
        THCMP5_W { w: self }
    }
    #[doc = "Bit 6 - Threshold comparison event on Channel 6. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp6(&mut self) -> THCMP6_W {
        THCMP6_W { w: self }
    }
    #[doc = "Bit 7 - Threshold comparison event on Channel 7. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp7(&mut self) -> THCMP7_W {
        THCMP7_W { w: self }
    }
    #[doc = "Bit 8 - Threshold comparison event on Channel 8. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp8(&mut self) -> THCMP8_W {
        THCMP8_W { w: self }
    }
    #[doc = "Bit 9 - Threshold comparison event on Channel 9. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp9(&mut self) -> THCMP9_W {
        THCMP9_W { w: self }
    }
    #[doc = "Bit 10 - Threshold comparison event on Channel 10. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp10(&mut self) -> THCMP10_W {
        THCMP10_W { w: self }
    }
    #[doc = "Bit 11 - Threshold comparison event on Channel 11. See description for channel 0."]
    #[inline(always)]
    pub fn thcmp11(&mut self) -> THCMP11_W {
        THCMP11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Flags register. Contains the four interrupt/DMA trigger flags and the individual component overrun and threshold-compare flags. (The overrun bits replicate information stored in the result registers).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
