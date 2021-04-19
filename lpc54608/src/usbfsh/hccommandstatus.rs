#[doc = "Register `HCCOMMANDSTATUS` reader"]
pub struct R(crate::R<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCCOMMANDSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HCCOMMANDSTATUS_SPEC>> for R {
    fn from(reader: crate::R<HCCOMMANDSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCCOMMANDSTATUS` writer"]
pub struct W(crate::W<HCCOMMANDSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCCOMMANDSTATUS_SPEC>;
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
impl core::convert::From<crate::W<HCCOMMANDSTATUS_SPEC>> for W {
    fn from(writer: crate::W<HCCOMMANDSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HCR` reader - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub struct HCR_R(crate::FieldReader<bool, bool>);
impl HCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCR` writer - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
pub struct HCR_W<'a> {
    w: &'a mut W,
}
impl<'a> HCR_W<'a> {
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
#[doc = "Field `CLF` reader - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub struct CLF_R(crate::FieldReader<bool, bool>);
impl CLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLF` writer - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
pub struct CLF_W<'a> {
    w: &'a mut W,
}
impl<'a> CLF_W<'a> {
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
#[doc = "Field `BLF` reader - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub struct BLF_R(crate::FieldReader<bool, bool>);
impl BLF_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLF` writer - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
pub struct BLF_W<'a> {
    w: &'a mut W,
}
impl<'a> BLF_W<'a> {
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
#[doc = "Field `OCR` reader - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub struct OCR_R(crate::FieldReader<bool, bool>);
impl OCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCR` writer - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
pub struct OCR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCR_W<'a> {
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
#[doc = "Field `SOC` reader - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub struct SOC_R(crate::FieldReader<u8, u8>);
impl SOC_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOC` writer - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
pub struct SOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&self) -> HCR_R {
        HCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&self) -> CLF_R {
        CLF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&self) -> BLF_R {
        BLF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&self) -> OCR_R {
        OCR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&self) -> SOC_R {
        SOC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HostControllerReset This bit is set by HCD to initiate a software reset of HC."]
    #[inline(always)]
    pub fn hcr(&mut self) -> HCR_W {
        HCR_W { w: self }
    }
    #[doc = "Bit 1 - ControlListFilled This bit is used to indicate whether there are any TDs on the Control list."]
    #[inline(always)]
    pub fn clf(&mut self) -> CLF_W {
        CLF_W { w: self }
    }
    #[doc = "Bit 2 - BulkListFilled This bit is used to indicate whether there are any TDs on the Bulk list."]
    #[inline(always)]
    pub fn blf(&mut self) -> BLF_W {
        BLF_W { w: self }
    }
    #[doc = "Bit 3 - OwnershipChangeRequest This bit is set by an OS HCD to request a change of control of the HC."]
    #[inline(always)]
    pub fn ocr(&mut self) -> OCR_W {
        OCR_W { w: self }
    }
    #[doc = "Bits 6:7 - SchedulingOverrunCount These bits are incremented on each scheduling overrun error."]
    #[inline(always)]
    pub fn soc(&mut self) -> SOC_W {
        SOC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hccommandstatus](index.html) module"]
pub struct HCCOMMANDSTATUS_SPEC;
impl crate::RegisterSpec for HCCOMMANDSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hccommandstatus::R](R) reader structure"]
impl crate::Readable for HCCOMMANDSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hccommandstatus::W](W) writer structure"]
impl crate::Writable for HCCOMMANDSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCCOMMANDSTATUS to value 0"]
impl crate::Resettable for HCCOMMANDSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
