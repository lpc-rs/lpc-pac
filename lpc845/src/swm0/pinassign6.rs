#[doc = "Reader of register PINASSIGN6"]
pub type R = crate::R<u32, super::PINASSIGN6>;
#[doc = "Writer for register PINASSIGN6"]
pub type W = crate::W<u32, super::PINASSIGN6>;
#[doc = "Register PINASSIGN6 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SPI1_MISO_IO`"]
pub type SPI1_MISO_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_MISO_IO`"]
pub struct SPI1_MISO_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_MISO_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SPI1_SSEL0_IO`"]
pub type SPI1_SSEL0_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_SSEL0_IO`"]
pub struct SPI1_SSEL0_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SSEL0_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPI1_SSEL1_IO`"]
pub type SPI1_SSEL1_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_SSEL1_IO`"]
pub struct SPI1_SSEL1_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_SSEL1_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCT0_GPIO_IN_A_I`"]
pub type SCT0_GPIO_IN_A_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT0_GPIO_IN_A_I`"]
pub struct SCT0_GPIO_IN_A_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_A_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_miso_io(&self) -> SPI1_MISO_IO_R {
        SPI1_MISO_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_ssel0_io(&self) -> SPI1_SSEL0_IO_R {
        SPI1_SSEL0_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_ssel1_io(&self) -> SPI1_SSEL1_IO_R {
        SPI1_SSEL1_IO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT0_GPIO_IN_A function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_a_i(&self) -> SCT0_GPIO_IN_A_I_R {
        SCT0_GPIO_IN_A_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPI1_MISO function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_miso_io(&mut self) -> SPI1_MISO_IO_W {
        SPI1_MISO_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - SPI1_SSEL0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_ssel0_io(&mut self) -> SPI1_SSEL0_IO_W {
        SPI1_SSEL0_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - SPI1_SSEL1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn spi1_ssel1_io(&mut self) -> SPI1_SSEL1_IO_W {
        SPI1_SSEL1_IO_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT0_GPIO_IN_A function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_a_i(&mut self) -> SCT0_GPIO_IN_A_I_W {
        SCT0_GPIO_IN_A_I_W { w: self }
    }
}
