#[doc = "Register `HCFMINTERVAL` reader"]
pub struct R(crate::R<HCFMINTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMINTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFMINTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFMINTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFMINTERVAL` writer"]
pub struct W(crate::W<HCFMINTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFMINTERVAL_SPEC>;
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
impl From<crate::W<HCFMINTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFMINTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI` reader - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub struct FI_R(crate::FieldReader<u16, u16>);
impl FI_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FI` writer - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub struct FI_W<'a> {
    w: &'a mut W,
}
impl<'a> FI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `FSMPS` reader - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub struct FSMPS_R(crate::FieldReader<u16, u16>);
impl FSMPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FSMPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSMPS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSMPS` writer - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub struct FSMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSMPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff << 16)) | ((value as u32 & 0x7fff) << 16);
        self.w
    }
}
#[doc = "Field `FIT` reader - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub struct FIT_R(crate::FieldReader<bool, bool>);
impl FIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIT` writer - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub struct FIT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIT_W<'a> {
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
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&self) -> FSMPS_R {
        FSMPS_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&self) -> FIT_R {
        FIT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&mut self) -> FI_W {
        FI_W { w: self }
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&mut self) -> FSMPS_W {
        FSMPS_W { w: self }
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&mut self) -> FIT_W {
        FIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfminterval](index.html) module"]
pub struct HCFMINTERVAL_SPEC;
impl crate::RegisterSpec for HCFMINTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfminterval::R](R) reader structure"]
impl crate::Readable for HCFMINTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfminterval::W](W) writer structure"]
impl crate::Writable for HCFMINTERVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCFMINTERVAL to value 0x2edf"]
impl crate::Resettable for HCFMINTERVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2edf
    }
}
