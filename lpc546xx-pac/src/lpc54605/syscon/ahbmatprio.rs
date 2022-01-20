#[doc = "Register `AHBMATPRIO` reader"]
pub struct R(crate::R<AHBMATPRIO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBMATPRIO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBMATPRIO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBMATPRIO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBMATPRIO` writer"]
pub struct W(crate::W<AHBMATPRIO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBMATPRIO_SPEC>;
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
impl From<crate::W<AHBMATPRIO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBMATPRIO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_ICODE` reader - I-Code bus priority."]
pub struct PRI_ICODE_R(crate::FieldReader<u8, u8>);
impl PRI_ICODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_ICODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_ICODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_ICODE` writer - I-Code bus priority."]
pub struct PRI_ICODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ICODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PRI_DCODE` reader - D-Code bus priority."]
pub struct PRI_DCODE_R(crate::FieldReader<u8, u8>);
impl PRI_DCODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_DCODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_DCODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_DCODE` writer - D-Code bus priority."]
pub struct PRI_DCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PRI_SYS` reader - System bus priority."]
pub struct PRI_SYS_R(crate::FieldReader<u8, u8>);
impl PRI_SYS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SYS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SYS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SYS` writer - System bus priority."]
pub struct PRI_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PRI_DMA` reader - DMA controller priority."]
pub struct PRI_DMA_R(crate::FieldReader<u8, u8>);
impl PRI_DMA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_DMA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_DMA` writer - DMA controller priority."]
pub struct PRI_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | ((value as u32 & 0x0f) << 6);
        self.w
    }
}
#[doc = "Field `PRI_ETH` reader - Ethernet DMA priority."]
pub struct PRI_ETH_R(crate::FieldReader<u8, u8>);
impl PRI_ETH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_ETH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_ETH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_ETH` writer - Ethernet DMA priority."]
pub struct PRI_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `PRI_LCD` reader - LCD DMA priority."]
pub struct PRI_LCD_R(crate::FieldReader<u8, u8>);
impl PRI_LCD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_LCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_LCD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_LCD` writer - LCD DMA priority."]
pub struct PRI_LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_LCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `PRI_USB0` reader - USB0 DMA priority."]
pub struct PRI_USB0_R(crate::FieldReader<u8, u8>);
impl PRI_USB0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_USB0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_USB0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_USB0` writer - USB0 DMA priority."]
pub struct PRI_USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `PRI_USB1` reader - USB1 DMA priority."]
pub struct PRI_USB1_R(crate::FieldReader<u8, u8>);
impl PRI_USB1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_USB1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_USB1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_USB1` writer - USB1 DMA priority."]
pub struct PRI_USB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PRI_SDIO` reader - SDIO priority."]
pub struct PRI_SDIO_R(crate::FieldReader<u8, u8>);
impl PRI_SDIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SDIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SDIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SDIO` writer - SDIO priority."]
pub struct PRI_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `PRI_MCAN1` reader - MCAN1 priority."]
pub struct PRI_MCAN1_R(crate::FieldReader<u8, u8>);
impl PRI_MCAN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_MCAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_MCAN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_MCAN1` writer - MCAN1 priority."]
pub struct PRI_MCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_MCAN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `PRI_MCAN2` reader - MCAN2 priority."]
pub struct PRI_MCAN2_R(crate::FieldReader<u8, u8>);
impl PRI_MCAN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_MCAN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_MCAN2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_MCAN2` writer - MCAN2 priority."]
pub struct PRI_MCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_MCAN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PRI_SHA` reader - SHA priority."]
pub struct PRI_SHA_R(crate::FieldReader<u8, u8>);
impl PRI_SHA_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRI_SHA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_SHA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_SHA` writer - SHA priority."]
pub struct PRI_SHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I-Code bus priority."]
    #[inline(always)]
    pub fn pri_icode(&self) -> PRI_ICODE_R {
        PRI_ICODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&self) -> PRI_DCODE_R {
        PRI_DCODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&self) -> PRI_SYS_R {
        PRI_SYS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:9 - DMA controller priority."]
    #[inline(always)]
    pub fn pri_dma(&self) -> PRI_DMA_R {
        PRI_DMA_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&self) -> PRI_ETH_R {
        PRI_ETH_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&self) -> PRI_LCD_R {
        PRI_LCD_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - USB0 DMA priority."]
    #[inline(always)]
    pub fn pri_usb0(&self) -> PRI_USB0_R {
        PRI_USB0_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - USB1 DMA priority."]
    #[inline(always)]
    pub fn pri_usb1(&self) -> PRI_USB1_R {
        PRI_USB1_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - SDIO priority."]
    #[inline(always)]
    pub fn pri_sdio(&self) -> PRI_SDIO_R {
        PRI_SDIO_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - MCAN1 priority."]
    #[inline(always)]
    pub fn pri_mcan1(&self) -> PRI_MCAN1_R {
        PRI_MCAN1_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - MCAN2 priority."]
    #[inline(always)]
    pub fn pri_mcan2(&self) -> PRI_MCAN2_R {
        PRI_MCAN2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - SHA priority."]
    #[inline(always)]
    pub fn pri_sha(&self) -> PRI_SHA_R {
        PRI_SHA_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - I-Code bus priority."]
    #[inline(always)]
    pub fn pri_icode(&mut self) -> PRI_ICODE_W {
        PRI_ICODE_W { w: self }
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&mut self) -> PRI_DCODE_W {
        PRI_DCODE_W { w: self }
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&mut self) -> PRI_SYS_W {
        PRI_SYS_W { w: self }
    }
    #[doc = "Bits 6:9 - DMA controller priority."]
    #[inline(always)]
    pub fn pri_dma(&mut self) -> PRI_DMA_W {
        PRI_DMA_W { w: self }
    }
    #[doc = "Bits 10:11 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&mut self) -> PRI_ETH_W {
        PRI_ETH_W { w: self }
    }
    #[doc = "Bits 12:13 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&mut self) -> PRI_LCD_W {
        PRI_LCD_W { w: self }
    }
    #[doc = "Bits 14:15 - USB0 DMA priority."]
    #[inline(always)]
    pub fn pri_usb0(&mut self) -> PRI_USB0_W {
        PRI_USB0_W { w: self }
    }
    #[doc = "Bits 16:17 - USB1 DMA priority."]
    #[inline(always)]
    pub fn pri_usb1(&mut self) -> PRI_USB1_W {
        PRI_USB1_W { w: self }
    }
    #[doc = "Bits 18:19 - SDIO priority."]
    #[inline(always)]
    pub fn pri_sdio(&mut self) -> PRI_SDIO_W {
        PRI_SDIO_W { w: self }
    }
    #[doc = "Bits 20:21 - MCAN1 priority."]
    #[inline(always)]
    pub fn pri_mcan1(&mut self) -> PRI_MCAN1_W {
        PRI_MCAN1_W { w: self }
    }
    #[doc = "Bits 22:23 - MCAN2 priority."]
    #[inline(always)]
    pub fn pri_mcan2(&mut self) -> PRI_MCAN2_W {
        PRI_MCAN2_W { w: self }
    }
    #[doc = "Bits 24:25 - SHA priority."]
    #[inline(always)]
    pub fn pri_sha(&mut self) -> PRI_SHA_W {
        PRI_SHA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB multilayer matrix priority control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbmatprio](index.html) module"]
pub struct AHBMATPRIO_SPEC;
impl crate::RegisterSpec for AHBMATPRIO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbmatprio::R](R) reader structure"]
impl crate::Readable for AHBMATPRIO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbmatprio::W](W) writer structure"]
impl crate::Writable for AHBMATPRIO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBMATPRIO to value 0"]
impl crate::Resettable for AHBMATPRIO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
