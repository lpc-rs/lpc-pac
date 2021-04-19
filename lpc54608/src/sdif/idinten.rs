#[doc = "Register `IDINTEN` reader"]
pub struct R(crate::R<IDINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDINTEN_SPEC>> for R {
    fn from(reader: crate::R<IDINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDINTEN` writer"]
pub struct W(crate::W<IDINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDINTEN_SPEC>;
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
impl core::convert::From<crate::W<IDINTEN_SPEC>> for W {
    fn from(writer: crate::W<IDINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI` reader - Transmit Interrupt Enable."]
pub struct TI_R(crate::FieldReader<bool, bool>);
impl TI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TI` writer - Transmit Interrupt Enable."]
pub struct TI_W<'a> {
    w: &'a mut W,
}
impl<'a> TI_W<'a> {
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
#[doc = "Field `RI` reader - Receive Interrupt Enable."]
pub struct RI_R(crate::FieldReader<bool, bool>);
impl RI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RI` writer - Receive Interrupt Enable."]
pub struct RI_W<'a> {
    w: &'a mut W,
}
impl<'a> RI_W<'a> {
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
#[doc = "Field `FBE` reader - Fatal Bus Error Enable."]
pub struct FBE_R(crate::FieldReader<bool, bool>);
impl FBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FBE` writer - Fatal Bus Error Enable."]
pub struct FBE_W<'a> {
    w: &'a mut W,
}
impl<'a> FBE_W<'a> {
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
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt."]
pub struct DU_R(crate::FieldReader<bool, bool>);
impl DU_R {
    pub(crate) fn new(bits: bool) -> Self {
        DU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt."]
pub struct DU_W<'a> {
    w: &'a mut W,
}
impl<'a> DU_W<'a> {
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
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable."]
pub struct CES_R(crate::FieldReader<bool, bool>);
impl CES_R {
    pub(crate) fn new(bits: bool) -> Self {
        CES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CES_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable."]
pub struct CES_W<'a> {
    w: &'a mut W,
}
impl<'a> CES_W<'a> {
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
#[doc = "Field `NIS` reader - Normal Interrupt Summary Enable."]
pub struct NIS_R(crate::FieldReader<bool, bool>);
impl NIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NIS` writer - Normal Interrupt Summary Enable."]
pub struct NIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NIS_W<'a> {
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
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Enable."]
pub struct AIS_R(crate::FieldReader<bool, bool>);
impl AIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Enable."]
pub struct AIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&self) -> TI_R {
        TI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&self) -> FBE_R {
        FBE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&self) -> NIS_R {
        NIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&self) -> AIS_R {
        AIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&mut self) -> TI_W {
        TI_W { w: self }
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&mut self) -> RI_W {
        RI_W { w: self }
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FBE_W {
        FBE_W { w: self }
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W {
        DU_W { w: self }
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&mut self) -> CES_W {
        CES_W { w: self }
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&mut self) -> NIS_W {
        NIS_W { w: self }
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&mut self) -> AIS_W {
        AIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal DMAC Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idinten](index.html) module"]
pub struct IDINTEN_SPEC;
impl crate::RegisterSpec for IDINTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idinten::R](R) reader structure"]
impl crate::Readable for IDINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idinten::W](W) writer structure"]
impl crate::Writable for IDINTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IDINTEN to value 0"]
impl crate::Resettable for IDINTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
