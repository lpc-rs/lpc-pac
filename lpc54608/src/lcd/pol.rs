#[doc = "Register `POL` reader"]
pub struct R(crate::R<POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<POL_SPEC>> for R {
    fn from(reader: crate::R<POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POL` writer"]
pub struct W(crate::W<POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POL_SPEC>;
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
impl core::convert::From<crate::W<POL_SPEC>> for W {
    fn from(writer: crate::W<POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCD_LO` reader - Lower five bits of panel clock divisor."]
pub struct PCD_LO_R(crate::FieldReader<u8, u8>);
impl PCD_LO_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCD_LO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCD_LO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCD_LO` writer - Lower five bits of panel clock divisor."]
pub struct PCD_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> PCD_LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ACB` reader - AC bias pin frequency."]
pub struct ACB_R(crate::FieldReader<u8, u8>);
impl ACB_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACB` writer - AC bias pin frequency."]
pub struct ACB_W<'a> {
    w: &'a mut W,
}
impl<'a> ACB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `IVS` reader - Invert vertical synchronization."]
pub struct IVS_R(crate::FieldReader<bool, bool>);
impl IVS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IVS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IVS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IVS` writer - Invert vertical synchronization."]
pub struct IVS_W<'a> {
    w: &'a mut W,
}
impl<'a> IVS_W<'a> {
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
#[doc = "Field `IHS` reader - Invert horizontal synchronization."]
pub struct IHS_R(crate::FieldReader<bool, bool>);
impl IHS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IHS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IHS` writer - Invert horizontal synchronization."]
pub struct IHS_W<'a> {
    w: &'a mut W,
}
impl<'a> IHS_W<'a> {
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
#[doc = "Field `IPC` reader - Invert panel clock."]
pub struct IPC_R(crate::FieldReader<bool, bool>);
impl IPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPC` writer - Invert panel clock."]
pub struct IPC_W<'a> {
    w: &'a mut W,
}
impl<'a> IPC_W<'a> {
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
#[doc = "Field `IOE` reader - Invert output enable."]
pub struct IOE_R(crate::FieldReader<bool, bool>);
impl IOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOE` writer - Invert output enable."]
pub struct IOE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOE_W<'a> {
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
#[doc = "Field `CPL` reader - Clocks per line."]
pub struct CPL_R(crate::FieldReader<u16, u16>);
impl CPL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPL` writer - Clocks per line."]
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `BCD` reader - Bypass panel clock divider."]
pub struct BCD_R(crate::FieldReader<bool, bool>);
impl BCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCD` writer - Bypass panel clock divider."]
pub struct BCD_W<'a> {
    w: &'a mut W,
}
impl<'a> BCD_W<'a> {
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
#[doc = "Field `PCD_HI` reader - Upper five bits of panel clock divisor."]
pub struct PCD_HI_R(crate::FieldReader<u8, u8>);
impl PCD_HI_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCD_HI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCD_HI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCD_HI` writer - Upper five bits of panel clock divisor."]
pub struct PCD_HI_W<'a> {
    w: &'a mut W,
}
impl<'a> PCD_HI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | ((value as u32 & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&self) -> PCD_LO_R {
        PCD_LO_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&self) -> ACB_R {
        ACB_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&self) -> IVS_R {
        IVS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&self) -> IHS_R {
        IHS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&self) -> IPC_R {
        IPC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&self) -> IOE_R {
        IOE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&self) -> PCD_HI_R {
        PCD_HI_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_lo(&mut self) -> PCD_LO_W {
        PCD_LO_W { w: self }
    }
    #[doc = "Bits 6:10 - AC bias pin frequency."]
    #[inline(always)]
    pub fn acb(&mut self) -> ACB_W {
        ACB_W { w: self }
    }
    #[doc = "Bit 11 - Invert vertical synchronization."]
    #[inline(always)]
    pub fn ivs(&mut self) -> IVS_W {
        IVS_W { w: self }
    }
    #[doc = "Bit 12 - Invert horizontal synchronization."]
    #[inline(always)]
    pub fn ihs(&mut self) -> IHS_W {
        IHS_W { w: self }
    }
    #[doc = "Bit 13 - Invert panel clock."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IPC_W {
        IPC_W { w: self }
    }
    #[doc = "Bit 14 - Invert output enable."]
    #[inline(always)]
    pub fn ioe(&mut self) -> IOE_W {
        IOE_W { w: self }
    }
    #[doc = "Bits 16:25 - Clocks per line."]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    #[doc = "Bit 26 - Bypass panel clock divider."]
    #[inline(always)]
    pub fn bcd(&mut self) -> BCD_W {
        BCD_W { w: self }
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor."]
    #[inline(always)]
    pub fn pcd_hi(&mut self) -> PCD_HI_W {
        PCD_HI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Signal Polarity Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](index.html) module"]
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pol::R](R) reader structure"]
impl crate::Readable for POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pol::W](W) writer structure"]
impl crate::Writable for POL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for POL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
