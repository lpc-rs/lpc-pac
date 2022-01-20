#[doc = "Register `HCINTERRUPTENABLE` reader"]
pub struct R(crate::R<HCINTERRUPTENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCINTERRUPTENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCINTERRUPTENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCINTERRUPTENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCINTERRUPTENABLE` writer"]
pub struct W(crate::W<HCINTERRUPTENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCINTERRUPTENABLE_SPEC>;
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
impl From<crate::W<HCINTERRUPTENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCINTERRUPTENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SO` reader - Scheduling Overrun interrupt."]
pub struct SO_R(crate::FieldReader<bool, bool>);
impl SO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SO` writer - Scheduling Overrun interrupt."]
pub struct SO_W<'a> {
    w: &'a mut W,
}
impl<'a> SO_W<'a> {
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
#[doc = "Field `WDH` reader - HcDoneHead Writeback interrupt."]
pub struct WDH_R(crate::FieldReader<bool, bool>);
impl WDH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDH` writer - HcDoneHead Writeback interrupt."]
pub struct WDH_W<'a> {
    w: &'a mut W,
}
impl<'a> WDH_W<'a> {
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
#[doc = "Field `SF` reader - Start of Frame interrupt."]
pub struct SF_R(crate::FieldReader<bool, bool>);
impl SF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SF` writer - Start of Frame interrupt."]
pub struct SF_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_W<'a> {
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
#[doc = "Field `RD` reader - Resume Detect interrupt."]
pub struct RD_R(crate::FieldReader<bool, bool>);
impl RD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD` writer - Resume Detect interrupt."]
pub struct RD_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_W<'a> {
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
#[doc = "Field `UE` reader - Unrecoverable Error interrupt."]
pub struct UE_R(crate::FieldReader<bool, bool>);
impl UE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UE` writer - Unrecoverable Error interrupt."]
pub struct UE_W<'a> {
    w: &'a mut W,
}
impl<'a> UE_W<'a> {
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
#[doc = "Field `FNO` reader - Frame Number Overflow interrupt."]
pub struct FNO_R(crate::FieldReader<bool, bool>);
impl FNO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FNO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNO` writer - Frame Number Overflow interrupt."]
pub struct FNO_W<'a> {
    w: &'a mut W,
}
impl<'a> FNO_W<'a> {
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
#[doc = "Field `RHSC` reader - Root Hub Status Change interrupt."]
pub struct RHSC_R(crate::FieldReader<bool, bool>);
impl RHSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RHSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RHSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RHSC` writer - Root Hub Status Change interrupt."]
pub struct RHSC_W<'a> {
    w: &'a mut W,
}
impl<'a> RHSC_W<'a> {
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
#[doc = "Field `OC` reader - Ownership Change interrupt."]
pub struct OC_R(crate::FieldReader<bool, bool>);
impl OC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OC` writer - Ownership Change interrupt."]
pub struct OC_W<'a> {
    w: &'a mut W,
}
impl<'a> OC_W<'a> {
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
#[doc = "Field `MIE` reader - Master Interrupt Enable."]
pub struct MIE_R(crate::FieldReader<bool, bool>);
impl MIE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MIE` writer - Master Interrupt Enable."]
pub struct MIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MIE_W<'a> {
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
    #[doc = "Bit 0 - Scheduling Overrun interrupt."]
    #[inline(always)]
    pub fn so(&self) -> SO_R {
        SO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub fn wdh(&self) -> WDH_R {
        WDH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame interrupt."]
    #[inline(always)]
    pub fn sf(&self) -> SF_R {
        SF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Resume Detect interrupt."]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Unrecoverable Error interrupt."]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Frame Number Overflow interrupt."]
    #[inline(always)]
    pub fn fno(&self) -> FNO_R {
        FNO_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Root Hub Status Change interrupt."]
    #[inline(always)]
    pub fn rhsc(&self) -> RHSC_R {
        RHSC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ownership Change interrupt."]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Master Interrupt Enable."]
    #[inline(always)]
    pub fn mie(&self) -> MIE_R {
        MIE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Scheduling Overrun interrupt."]
    #[inline(always)]
    pub fn so(&mut self) -> SO_W {
        SO_W { w: self }
    }
    #[doc = "Bit 1 - HcDoneHead Writeback interrupt."]
    #[inline(always)]
    pub fn wdh(&mut self) -> WDH_W {
        WDH_W { w: self }
    }
    #[doc = "Bit 2 - Start of Frame interrupt."]
    #[inline(always)]
    pub fn sf(&mut self) -> SF_W {
        SF_W { w: self }
    }
    #[doc = "Bit 3 - Resume Detect interrupt."]
    #[inline(always)]
    pub fn rd(&mut self) -> RD_W {
        RD_W { w: self }
    }
    #[doc = "Bit 4 - Unrecoverable Error interrupt."]
    #[inline(always)]
    pub fn ue(&mut self) -> UE_W {
        UE_W { w: self }
    }
    #[doc = "Bit 5 - Frame Number Overflow interrupt."]
    #[inline(always)]
    pub fn fno(&mut self) -> FNO_W {
        FNO_W { w: self }
    }
    #[doc = "Bit 6 - Root Hub Status Change interrupt."]
    #[inline(always)]
    pub fn rhsc(&mut self) -> RHSC_W {
        RHSC_W { w: self }
    }
    #[doc = "Bit 30 - Ownership Change interrupt."]
    #[inline(always)]
    pub fn oc(&mut self) -> OC_W {
        OC_W { w: self }
    }
    #[doc = "Bit 31 - Master Interrupt Enable."]
    #[inline(always)]
    pub fn mie(&mut self) -> MIE_W {
        MIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcinterruptenable](index.html) module"]
pub struct HCINTERRUPTENABLE_SPEC;
impl crate::RegisterSpec for HCINTERRUPTENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcinterruptenable::R](R) reader structure"]
impl crate::Readable for HCINTERRUPTENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcinterruptenable::W](W) writer structure"]
impl crate::Writable for HCINTERRUPTENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCINTERRUPTENABLE to value 0"]
impl crate::Resettable for HCINTERRUPTENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
