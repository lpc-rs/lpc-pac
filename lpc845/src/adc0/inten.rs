#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sequence A interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQA_INTEN_A {
    #[doc = "0: Disabled. The sequence A interrupt/DMA trigger is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    ENABLED = 1,
}
impl From<SEQA_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQA_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQA_INTEN` reader - Sequence A interrupt enable."]
pub struct SEQA_INTEN_R(crate::FieldReader<bool, SEQA_INTEN_A>);
impl SEQA_INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQA_INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQA_INTEN_A {
        match self.bits {
            false => SEQA_INTEN_A::DISABLED,
            true => SEQA_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQA_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQA_INTEN_A::ENABLED
    }
}
impl core::ops::Deref for SEQA_INTEN_R {
    type Target = crate::FieldReader<bool, SEQA_INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQA_INTEN` writer - Sequence A interrupt enable."]
pub struct SEQA_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQA_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQA_INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The sequence A interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence A interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence A, or upon completion of the entire A sequence of conversions, depending on the MODE bit in the SEQA_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQA_INTEN_A::ENABLED)
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
#[doc = "Sequence B interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQB_INTEN_A {
    #[doc = "0: Disabled. The sequence B interrupt/DMA trigger is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    ENABLED = 1,
}
impl From<SEQB_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEQB_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEQB_INTEN` reader - Sequence B interrupt enable."]
pub struct SEQB_INTEN_R(crate::FieldReader<bool, SEQB_INTEN_A>);
impl SEQB_INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEQB_INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQB_INTEN_A {
        match self.bits {
            false => SEQB_INTEN_A::DISABLED,
            true => SEQB_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SEQB_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SEQB_INTEN_A::ENABLED
    }
}
impl core::ops::Deref for SEQB_INTEN_R {
    type Target = crate::FieldReader<bool, SEQB_INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEQB_INTEN` writer - Sequence B interrupt enable."]
pub struct SEQB_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQB_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQB_INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The sequence B interrupt/DMA trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The sequence B interrupt/DMA trigger is enabled and will be asserted either upon completion of each individual conversion performed as part of sequence B, or upon completion of the entire B sequence of conversions, depending on the MODE bit in the SEQB_CTRL register."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQB_INTEN_A::ENABLED)
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
#[doc = "Overrun interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVR_INTEN_A {
    #[doc = "0: Disabled. The overrun interrupt is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    ENABLED = 1,
}
impl From<OVR_INTEN_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_INTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR_INTEN` reader - Overrun interrupt enable."]
pub struct OVR_INTEN_R(crate::FieldReader<bool, OVR_INTEN_A>);
impl OVR_INTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVR_INTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_INTEN_A {
        match self.bits {
            false => OVR_INTEN_A::DISABLED,
            true => OVR_INTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVR_INTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVR_INTEN_A::ENABLED
    }
}
impl core::ops::Deref for OVR_INTEN_R {
    type Target = crate::FieldReader<bool, OVR_INTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVR_INTEN` writer - Overrun interrupt enable."]
pub struct OVR_INTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVR_INTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVR_INTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled. The overrun interrupt is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::DISABLED)
    }
    #[doc = "Enabled. The overrun interrupt is enabled. Detection of an overrun condition on any of the 12 channel data registers will cause an overrun interrupt/DMA trigger. In addition, if the MODE bit for a particular sequence is 0, then an overrun in the global data register for that sequence will also cause this interrupt/DMA trigger to be asserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVR_INTEN_A::ENABLED)
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
#[doc = "Threshold comparison interrupt enable for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCMPINTEN0_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Outside threshold."]
    OUTSIDE_THRESHOLD = 1,
    #[doc = "2: Crossing threshold."]
    CROSSING_THRESHOLD = 2,
}
impl From<ADCMPINTEN0_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCMPINTEN0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCMPINTEN0` reader - Threshold comparison interrupt enable for channel 0."]
pub struct ADCMPINTEN0_R(crate::FieldReader<u8, ADCMPINTEN0_A>);
impl ADCMPINTEN0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCMPINTEN0_A> {
        match self.bits {
            0 => Some(ADCMPINTEN0_A::DISABLED),
            1 => Some(ADCMPINTEN0_A::OUTSIDE_THRESHOLD),
            2 => Some(ADCMPINTEN0_A::CROSSING_THRESHOLD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ADCMPINTEN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTSIDE_THRESHOLD`"]
    #[inline(always)]
    pub fn is_outside_threshold(&self) -> bool {
        **self == ADCMPINTEN0_A::OUTSIDE_THRESHOLD
    }
    #[doc = "Checks if the value of the field is `CROSSING_THRESHOLD`"]
    #[inline(always)]
    pub fn is_crossing_threshold(&self) -> bool {
        **self == ADCMPINTEN0_A::CROSSING_THRESHOLD
    }
}
impl core::ops::Deref for ADCMPINTEN0_R {
    type Target = crate::FieldReader<u8, ADCMPINTEN0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN0` writer - Threshold comparison interrupt enable for channel 0."]
pub struct ADCMPINTEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCMPINTEN0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::DISABLED)
    }
    #[doc = "Outside threshold."]
    #[inline(always)]
    pub fn outside_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::OUTSIDE_THRESHOLD)
    }
    #[doc = "Crossing threshold."]
    #[inline(always)]
    pub fn crossing_threshold(self) -> &'a mut W {
        self.variant(ADCMPINTEN0_A::CROSSING_THRESHOLD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN1` reader - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN1_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN1_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN1` writer - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN2` reader - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN2_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN2_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN2` writer - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN3` reader - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN3_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN3_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN3` writer - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN4` reader - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN4_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN4_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN4` writer - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN5` reader - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN5_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN5_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN5` writer - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN6` reader - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN6_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN6_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN6` writer - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN7` reader - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN7_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN7_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN7` writer - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN8` reader - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN8_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN8_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN8` writer - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN9` reader - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN9_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN9_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN9` writer - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN10` reader - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN10_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN10_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN10` writer - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `ADCMPINTEN11` reader - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN11_R(crate::FieldReader<u8, u8>);
impl ADCMPINTEN11_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCMPINTEN11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCMPINTEN11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCMPINTEN11` writer - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
pub struct ADCMPINTEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCMPINTEN11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&self) -> SEQA_INTEN_R {
        SEQA_INTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&self) -> SEQB_INTEN_R {
        SEQB_INTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&self) -> OVR_INTEN_R {
        OVR_INTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&self) -> ADCMPINTEN0_R {
        ADCMPINTEN0_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&self) -> ADCMPINTEN1_R {
        ADCMPINTEN1_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&self) -> ADCMPINTEN2_R {
        ADCMPINTEN2_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&self) -> ADCMPINTEN3_R {
        ADCMPINTEN3_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&self) -> ADCMPINTEN4_R {
        ADCMPINTEN4_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&self) -> ADCMPINTEN5_R {
        ADCMPINTEN5_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&self) -> ADCMPINTEN6_R {
        ADCMPINTEN6_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&self) -> ADCMPINTEN7_R {
        ADCMPINTEN7_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&self) -> ADCMPINTEN8_R {
        ADCMPINTEN8_R::new(((self.bits >> 19) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&self) -> ADCMPINTEN9_R {
        ADCMPINTEN9_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&self) -> ADCMPINTEN10_R {
        ADCMPINTEN10_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&self) -> ADCMPINTEN11_R {
        ADCMPINTEN11_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sequence A interrupt enable."]
    #[inline(always)]
    pub fn seqa_inten(&mut self) -> SEQA_INTEN_W {
        SEQA_INTEN_W { w: self }
    }
    #[doc = "Bit 1 - Sequence B interrupt enable."]
    #[inline(always)]
    pub fn seqb_inten(&mut self) -> SEQB_INTEN_W {
        SEQB_INTEN_W { w: self }
    }
    #[doc = "Bit 2 - Overrun interrupt enable."]
    #[inline(always)]
    pub fn ovr_inten(&mut self) -> OVR_INTEN_W {
        OVR_INTEN_W { w: self }
    }
    #[doc = "Bits 3:4 - Threshold comparison interrupt enable for channel 0."]
    #[inline(always)]
    pub fn adcmpinten0(&mut self) -> ADCMPINTEN0_W {
        ADCMPINTEN0_W { w: self }
    }
    #[doc = "Bits 5:6 - Channel 1 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten1(&mut self) -> ADCMPINTEN1_W {
        ADCMPINTEN1_W { w: self }
    }
    #[doc = "Bits 7:8 - Channel 2 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten2(&mut self) -> ADCMPINTEN2_W {
        ADCMPINTEN2_W { w: self }
    }
    #[doc = "Bits 9:10 - Channel 3 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten3(&mut self) -> ADCMPINTEN3_W {
        ADCMPINTEN3_W { w: self }
    }
    #[doc = "Bits 11:12 - Channel 4 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten4(&mut self) -> ADCMPINTEN4_W {
        ADCMPINTEN4_W { w: self }
    }
    #[doc = "Bits 13:14 - Channel 5 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten5(&mut self) -> ADCMPINTEN5_W {
        ADCMPINTEN5_W { w: self }
    }
    #[doc = "Bits 15:16 - Channel 6 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten6(&mut self) -> ADCMPINTEN6_W {
        ADCMPINTEN6_W { w: self }
    }
    #[doc = "Bits 17:18 - Channel 7 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten7(&mut self) -> ADCMPINTEN7_W {
        ADCMPINTEN7_W { w: self }
    }
    #[doc = "Bits 19:20 - Channel 8 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten8(&mut self) -> ADCMPINTEN8_W {
        ADCMPINTEN8_W { w: self }
    }
    #[doc = "Bits 21:22 - Channel 9 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten9(&mut self) -> ADCMPINTEN9_W {
        ADCMPINTEN9_W { w: self }
    }
    #[doc = "Bits 23:24 - Channel 10 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten10(&mut self) -> ADCMPINTEN10_W {
        ADCMPINTEN10_W { w: self }
    }
    #[doc = "Bits 25:26 - Channel 21 threshold comparison interrupt enable. See description for channel 0."]
    #[inline(always)]
    pub fn adcmpinten11(&mut self) -> ADCMPINTEN11_W {
        ADCMPINTEN11_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Enable register. This register contains enable bits that enable the sequence-A, sequence-B, threshold compare and data overrun interrupts to be generated.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
