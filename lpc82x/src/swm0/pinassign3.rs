#[doc = "Reader of register PINASSIGN3"]
pub type R = crate::R<u32, super::PINASSIGN3>;
#[doc = "Writer for register PINASSIGN3"]
pub type W = crate::W<u32, super::PINASSIGN3>;
#[doc = "Register PINASSIGN3 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `U2_RTS_O`"]
pub type U2_RTS_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2_RTS_O`"]
pub struct U2_RTS_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_RTS_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `U2_CTS_I`"]
pub type U2_CTS_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2_CTS_I`"]
pub struct U2_CTS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_CTS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `U2_SCLK_IO`"]
pub type U2_SCLK_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2_SCLK_IO`"]
pub struct U2_SCLK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_SCLK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SPI0_SCK_IO`"]
pub type SPI0_SCK_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI0_SCK_IO`"]
pub struct SPI0_SCK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_SCK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U2_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_rts_o(&self) -> U2_RTS_O_R {
        U2_RTS_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U2_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_cts_i(&self) -> U2_CTS_I_R {
        U2_CTS_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U2_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_sclk_io(&self) -> U2_SCLK_IO_R {
        U2_SCLK_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPI0_SCK function assignment. The value is the pin number to be assigned to this function. The following pins are available:PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_sck_io(&self) -> SPI0_SCK_IO_R {
        SPI0_SCK_IO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U2_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_rts_o(&mut self) -> U2_RTS_O_W {
        U2_RTS_O_W { w: self }
    }
    #[doc = "Bits 8:15 - U2_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_cts_i(&mut self) -> U2_CTS_I_W {
        U2_CTS_I_W { w: self }
    }
    #[doc = "Bits 16:23 - U2_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u2_sclk_io(&mut self) -> U2_SCLK_IO_W {
        U2_SCLK_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SPI0_SCK function assignment. The value is the pin number to be assigned to this function. The following pins are available:PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn spi0_sck_io(&mut self) -> SPI0_SCK_IO_W {
        SPI0_SCK_IO_W { w: self }
    }
}
