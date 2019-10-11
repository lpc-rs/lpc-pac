#[doc = "Reader of register PINASSIGN2"]
pub type R = crate::R<u32, super::PINASSIGN2>;
#[doc = "Writer for register PINASSIGN2"]
pub type W = crate::W<u32, super::PINASSIGN2>;
#[doc = "Register PINASSIGN2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `U1_CTS_I`"]
pub type U1_CTS_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1_CTS_I`"]
pub struct U1_CTS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_CTS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `U1_SCLK_IO`"]
pub type U1_SCLK_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1_SCLK_IO`"]
pub struct U1_SCLK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_SCLK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `U2_TXD_O`"]
pub type U2_TXD_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2_TXD_O`"]
pub struct U2_TXD_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_TXD_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `U2_RXD_I`"]
pub type U2_RXD_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U2_RXD_I`"]
pub struct U2_RXD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U2_RXD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u1_cts_i(&self) -> U1_CTS_I_R {
        U1_CTS_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u1_sclk_io(&self) -> U1_SCLK_IO_R {
        U1_SCLK_IO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u2_txd_o(&self) -> U2_TXD_O_R {
        U2_TXD_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u2_rxd_i(&self) -> U2_RXD_I_R {
        U2_RXD_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U1_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u1_cts_i(&mut self) -> U1_CTS_I_W {
        U1_CTS_I_W { w: self }
    }
    #[doc = "Bits 8:15 - U1_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u1_sclk_io(&mut self) -> U1_SCLK_IO_W {
        U1_SCLK_IO_W { w: self }
    }
    #[doc = "Bits 16:23 - U2_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u2_txd_o(&mut self) -> U2_TXD_O_W {
        U2_TXD_O_W { w: self }
    }
    #[doc = "Bits 24:31 - U2_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u2_rxd_i(&mut self) -> U2_RXD_I_W {
        U2_RXD_I_W { w: self }
    }
}
