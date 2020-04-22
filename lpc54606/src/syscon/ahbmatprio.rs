#[doc = "Reader of register AHBMATPRIO"]
pub type R = crate::R<u32, super::AHBMATPRIO>;
#[doc = "Writer for register AHBMATPRIO"]
pub type W = crate::W<u32, super::AHBMATPRIO>;
#[doc = "Register AHBMATPRIO `reset()`'s with value 0"]
impl crate::ResetValue for super::AHBMATPRIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRI_ICODE`"]
pub type PRI_ICODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_ICODE`"]
pub struct PRI_ICODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ICODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PRI_DCODE`"]
pub type PRI_DCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_DCODE`"]
pub struct PRI_DCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PRI_SYS`"]
pub type PRI_SYS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SYS`"]
pub struct PRI_SYS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PRI_DMA`"]
pub type PRI_DMA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_DMA`"]
pub struct PRI_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_DMA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 6)) | (((value as u32) & 0x0f) << 6);
        self.w
    }
}
#[doc = "Reader of field `PRI_ETH`"]
pub type PRI_ETH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_ETH`"]
pub struct PRI_ETH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_ETH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PRI_LCD`"]
pub type PRI_LCD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_LCD`"]
pub struct PRI_LCD_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_LCD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `PRI_USB0`"]
pub type PRI_USB0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_USB0`"]
pub struct PRI_USB0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `PRI_USB1`"]
pub type PRI_USB1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_USB1`"]
pub struct PRI_USB1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_USB1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `PRI_SDIO`"]
pub type PRI_SDIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SDIO`"]
pub struct PRI_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SDIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `PRI_MCAN1`"]
pub type PRI_MCAN1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_MCAN1`"]
pub struct PRI_MCAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_MCAN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `PRI_MCAN2`"]
pub type PRI_MCAN2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_MCAN2`"]
pub struct PRI_MCAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_MCAN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRI_SHA`"]
pub type PRI_SHA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRI_SHA`"]
pub struct PRI_SHA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_SHA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
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
}
