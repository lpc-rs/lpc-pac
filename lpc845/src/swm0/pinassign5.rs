#[doc = "Reader of register PINASSIGN5"]
pub type R = crate::R<u32, super::PINASSIGN5>;
#[doc = "Writer for register PINASSIGN5"]
pub type W = crate::W<u32, super::PINASSIGN5>;
#[doc = "Register PINASSIGN5 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SPI0_SSEL2_IO`"]
pub type SPI0_SSEL2_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_SSEL2_IO`"]
pub struct SPI0_SSEL2_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL2_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SPI0_SSEL3_IO`"]
pub type SPI0_SSEL3_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_SSEL3_IO`"]
pub struct SPI0_SSEL3_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SSEL3_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI1_SCK_IO`"]
pub type SPI1_SCK_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_SCK_IO`"]
pub struct SPI1_SCK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SCK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI1_MOSI_IO`"]
pub type SPI1_MOSI_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_MOSI_IO`"]
pub struct SPI1_MOSI_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MOSI_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI0_SSEL2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel2_io(&self) -> SPI0_SSEL2_IO_R {
        SPI0_SSEL2_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI0_SSEL3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel3_io(&self) -> SPI0_SSEL3_IO_R {
        SPI0_SSEL3_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI1_SCK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_sck_io(&self) -> SPI1_SCK_IO_R {
        SPI1_SCK_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_mosi_io(&self) -> SPI1_MOSI_IO_R {
        SPI1_MOSI_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI0_SSEL2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel2_io(&mut self) -> SPI0_SSEL2_IO_W {
        SPI0_SSEL2_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - SPI0_SSEL3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi0_ssel3_io(&mut self) -> SPI0_SSEL3_IO_W {
        SPI0_SSEL3_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - SPI1_SCK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_sck_io(&mut self) -> SPI1_SCK_IO_W {
        SPI1_SCK_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SPI1_MOSI function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_mosi_io(&mut self) -> SPI1_MOSI_IO_W {
        SPI1_MOSI_IO_W { w: self }
    }
}
