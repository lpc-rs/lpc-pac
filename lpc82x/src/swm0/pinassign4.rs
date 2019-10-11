#[doc = "Reader of register PINASSIGN4"]
pub type R = crate::R<u32, super::PINASSIGN4>;
#[doc = "Writer for register PINASSIGN4"]
pub type W = crate::W<u32, super::PINASSIGN4>;
#[doc = "Register PINASSIGN4 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SPI0_MOSI_IO`"]
pub type SPI0_MOSI_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_MOSI_IO`"]
pub struct SPI0_MOSI_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MOSI_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SPI0_MISO_IO`"]
pub type SPI0_MISO_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_MISO_IO`"]
pub struct SPI0_MISO_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_MISO_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI0_SSEL0_IO`"]
pub type SPI0_SSEL0_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_SSEL0_IO`"]
pub struct SPI0_SSEL0_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL0_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI0_SSEL1_IO`"]
pub type SPI0_SSEL1_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_SSEL1_IO`"]
pub struct SPI0_SSEL1_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL1_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_mosi_io(&self) -> SPI0_MOSI_IO_R {
        SPI0_MOSI_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available:PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_miso_io(&self) -> SPI0_MISO_IO_R {
        SPI0_MISO_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_ssel0_io(&self) -> SPI0_SSEL0_IO_R {
        SPI0_SSEL0_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_ssel1_io(&self) -> SPI0_SSEL1_IO_R {
        SPI0_SSEL1_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI0_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_mosi_io(&mut self) -> SPI0_MOSI_IO_W {
        SPI0_MOSI_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - SPI0_MISIO function assignment. The value is the pin number to be assigned to this function. The following pins are available:PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_miso_io(&mut self) -> SPI0_MISO_IO_W {
        SPI0_MISO_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - SPI0_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_ssel0_io(&mut self) -> SPI0_SSEL0_IO_W {
        SPI0_SSEL0_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SPI0_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_ssel1_io(&mut self) -> SPI0_SSEL1_IO_W {
        SPI0_SSEL1_IO_W { w: self }
    }
}
