#[doc = "Register `USBCMD` reader"]
pub struct R(crate::R<USBCMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<USBCMD_SPEC>> for R {
    fn from(reader: crate::R<USBCMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCMD` writer"]
pub struct W(crate::W<USBCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCMD_SPEC>;
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
impl core::convert::From<crate::W<USBCMD_SPEC>> for W {
    fn from(writer: crate::W<USBCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RS` reader - Run/Stop: 1b = Run."]
pub struct RS_R(crate::FieldReader<bool, bool>);
impl RS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RS` writer - Run/Stop: 1b = Run."]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
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
#[doc = "Field `HCRESET` reader - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub struct HCRESET_R(crate::FieldReader<bool, bool>);
impl HCRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCRESET` writer - Host Controller Reset: This control bit is used by the software to reset the host controller."]
pub struct HCRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> HCRESET_W<'a> {
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
#[doc = "Field `FLS` reader - Frame List Size: This field specifies the size of the frame list."]
pub struct FLS_R(crate::FieldReader<u8, u8>);
impl FLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLS` writer - Frame List Size: This field specifies the size of the frame list."]
pub struct FLS_W<'a> {
    w: &'a mut W,
}
impl<'a> FLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `LHCR` reader - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub struct LHCR_R(crate::FieldReader<bool, bool>);
impl LHCR_R {
    pub(crate) fn new(bits: bool) -> Self {
        LHCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LHCR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LHCR` writer - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
pub struct LHCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LHCR_W<'a> {
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
#[doc = "Field `ATL_EN` reader - ATL List enabled."]
pub struct ATL_EN_R(crate::FieldReader<bool, bool>);
impl ATL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ATL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATL_EN` writer - ATL List enabled."]
pub struct ATL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_EN_W<'a> {
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
#[doc = "Field `ISO_EN` reader - ISO List enabled."]
pub struct ISO_EN_R(crate::FieldReader<bool, bool>);
impl ISO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISO_EN` writer - ISO List enabled."]
pub struct ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_EN_W<'a> {
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
#[doc = "Field `INT_EN` reader - INT List enabled."]
pub struct INT_EN_R(crate::FieldReader<bool, bool>);
impl INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_EN` writer - INT List enabled."]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
#[doc = "Field `HIRD` reader - Host-Initiated Resume Duration."]
pub struct HIRD_R(crate::FieldReader<u8, u8>);
impl HIRD_R {
    pub(crate) fn new(bits: u8) -> Self {
        HIRD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIRD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIRD` writer - Host-Initiated Resume Duration."]
pub struct HIRD_W<'a> {
    w: &'a mut W,
}
impl<'a> HIRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `LPM_RWU` reader - bRemoteWake field."]
pub struct LPM_RWU_R(crate::FieldReader<bool, bool>);
impl LPM_RWU_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_RWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_RWU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_RWU` writer - bRemoteWake field."]
pub struct LPM_RWU_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_RWU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&self) -> HCRESET_R {
        HCRESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&self) -> FLS_R {
        FLS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&self) -> LHCR_R {
        LHCR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&self) -> ATL_EN_R {
        ATL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&self) -> ISO_EN_R {
        ISO_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&self) -> HIRD_R {
        HIRD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&self) -> LPM_RWU_R {
        LPM_RWU_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop: 1b = Run."]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
    #[doc = "Bit 1 - Host Controller Reset: This control bit is used by the software to reset the host controller."]
    #[inline(always)]
    pub fn hcreset(&mut self) -> HCRESET_W {
        HCRESET_W { w: self }
    }
    #[doc = "Bits 2:3 - Frame List Size: This field specifies the size of the frame list."]
    #[inline(always)]
    pub fn fls(&mut self) -> FLS_W {
        FLS_W { w: self }
    }
    #[doc = "Bit 7 - Light Host Controller Reset: This bit allows the driver software to reset the host controller without affecting the state of the ports."]
    #[inline(always)]
    pub fn lhcr(&mut self) -> LHCR_W {
        LHCR_W { w: self }
    }
    #[doc = "Bit 8 - ATL List enabled."]
    #[inline(always)]
    pub fn atl_en(&mut self) -> ATL_EN_W {
        ATL_EN_W { w: self }
    }
    #[doc = "Bit 9 - ISO List enabled."]
    #[inline(always)]
    pub fn iso_en(&mut self) -> ISO_EN_W {
        ISO_EN_W { w: self }
    }
    #[doc = "Bit 10 - INT List enabled."]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bits 24:27 - Host-Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&mut self) -> HIRD_W {
        HIRD_W { w: self }
    }
    #[doc = "Bit 28 - bRemoteWake field."]
    #[inline(always)]
    pub fn lpm_rwu(&mut self) -> LPM_RWU_W {
        LPM_RWU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcmd](index.html) module"]
pub struct USBCMD_SPEC;
impl crate::RegisterSpec for USBCMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcmd::R](R) reader structure"]
impl crate::Readable for USBCMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcmd::W](W) writer structure"]
impl crate::Writable for USBCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCMD to value 0"]
impl crate::Resettable for USBCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
