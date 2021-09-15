#[doc = "Register `FROCTRL` reader"]
pub struct R(crate::R<FROCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FROCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FROCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FROCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FROCTRL` writer"]
pub struct W(crate::W<FROCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FROCTRL_SPEC>;
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
impl From<crate::W<FROCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FROCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - This value is factory trimmed to account for bias and temperature compensation."]
pub struct TRIM_R(crate::FieldReader<u16, u16>);
impl TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIM` writer - This value is factory trimmed to account for bias and temperature compensation."]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `SEL` reader - Select the FRO HF output frequency."]
pub struct SEL_R(crate::FieldReader<bool, bool>);
impl SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL` writer - Select the FRO HF output frequency."]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
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
#[doc = "Field `FREQTRIM` reader - Frequency trim."]
pub struct FREQTRIM_R(crate::FieldReader<u8, u8>);
impl FREQTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FREQTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQTRIM` writer - Frequency trim."]
pub struct FREQTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `USBCLKADJ` reader - USB clock adjust mode."]
pub struct USBCLKADJ_R(crate::FieldReader<bool, bool>);
impl USBCLKADJ_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCLKADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCLKADJ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCLKADJ` writer - USB clock adjust mode."]
pub struct USBCLKADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKADJ_W<'a> {
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
#[doc = "Field `USBMODCHG` reader - USB Mode value Change flag."]
pub struct USBMODCHG_R(crate::FieldReader<bool, bool>);
impl USBMODCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBMODCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBMODCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBMODCHG` writer - USB Mode value Change flag."]
pub struct USBMODCHG_W<'a> {
    w: &'a mut W,
}
impl<'a> USBMODCHG_W<'a> {
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
#[doc = "Field `HSPDCLK` reader - High speed clock enable."]
pub struct HSPDCLK_R(crate::FieldReader<bool, bool>);
impl HSPDCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSPDCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSPDCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSPDCLK` writer - High speed clock enable."]
pub struct HSPDCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HSPDCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `WRTRIM` reader - Write Trim value."]
pub struct WRTRIM_R(crate::FieldReader<bool, bool>);
impl WRTRIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRTRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRTRIM` writer - Write Trim value."]
pub struct WRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&self) -> FREQTRIM_R {
        FREQTRIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&self) -> USBCLKADJ_R {
        USBCLKADJ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&self) -> USBMODCHG_R {
        USBMODCHG_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&self) -> HSPDCLK_R {
        HSPDCLK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&self) -> WRTRIM_R {
        WRTRIM_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - This value is factory trimmed to account for bias and temperature compensation."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 14 - Select the FRO HF output frequency."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Frequency trim."]
    #[inline(always)]
    pub fn freqtrim(&mut self) -> FREQTRIM_W {
        FREQTRIM_W { w: self }
    }
    #[doc = "Bit 24 - USB clock adjust mode."]
    #[inline(always)]
    pub fn usbclkadj(&mut self) -> USBCLKADJ_W {
        USBCLKADJ_W { w: self }
    }
    #[doc = "Bit 25 - USB Mode value Change flag."]
    #[inline(always)]
    pub fn usbmodchg(&mut self) -> USBMODCHG_W {
        USBMODCHG_W { w: self }
    }
    #[doc = "Bit 30 - High speed clock enable."]
    #[inline(always)]
    pub fn hspdclk(&mut self) -> HSPDCLK_W {
        HSPDCLK_W { w: self }
    }
    #[doc = "Bit 31 - Write Trim value."]
    #[inline(always)]
    pub fn wrtrim(&mut self) -> WRTRIM_W {
        WRTRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FRO oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [froctrl](index.html) module"]
pub struct FROCTRL_SPEC;
impl crate::RegisterSpec for FROCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [froctrl::R](R) reader structure"]
impl crate::Readable for FROCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [froctrl::W](W) writer structure"]
impl crate::Writable for FROCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FROCTRL to value 0x4000"]
impl crate::Resettable for FROCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
