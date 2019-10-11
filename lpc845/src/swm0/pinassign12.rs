#[doc = "Reader of register PINASSIGN12"]
pub type R = crate::R<u32, super::PINASSIGN12>;
#[doc = "Writer for register PINASSIGN12"]
pub type W = crate::W<u32, super::PINASSIGN12>;
#[doc = "Register PINASSIGN12 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN12 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `UART3_RXD`"]
pub type UART3_RXD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART3_RXD`"]
pub struct UART3_RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_RXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `UART3_SCLK`"]
pub type UART3_SCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART3_SCLK`"]
pub struct UART3_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `UART4_TXD`"]
pub type UART4_TXD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART4_TXD`"]
pub struct UART4_TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART4_RXD`"]
pub type UART4_RXD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART4_RXD`"]
pub struct UART4_RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_RXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_rxd(&self) -> UART3_RXD_R {
        UART3_RXD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_sclk(&self) -> UART3_SCLK_R {
        UART3_SCLK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_txd(&self) -> UART4_TXD_R {
        UART4_TXD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_rxd(&self) -> UART4_RXD_R {
        UART4_RXD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART3_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_rxd(&mut self) -> UART3_RXD_W {
        UART3_RXD_W { w: self }
    }
    #[doc = "Bits 8:15 - UART3_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_sclk(&mut self) -> UART3_SCLK_W {
        UART3_SCLK_W { w: self }
    }
    #[doc = "Bits 16:23 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_txd(&mut self) -> UART4_TXD_W {
        UART4_TXD_W { w: self }
    }
    #[doc = "Bits 24:31 - UART4_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_rxd(&mut self) -> UART4_RXD_W {
        UART4_RXD_W { w: self }
    }
}
