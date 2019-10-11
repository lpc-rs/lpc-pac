#[doc = "Reader of register PINASSIGN1"]
pub type R = crate::R<u32, super::PINASSIGN1>;
#[doc = "Writer for register PINASSIGN1"]
pub type W = crate::W<u32, super::PINASSIGN1>;
#[doc = "Register PINASSIGN1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `U0_SCLK_IO`"]
pub type U0_SCLK_IO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U0_SCLK_IO`"]
pub struct U0_SCLK_IO_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_SCLK_IO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `U1_TXD_O`"]
pub type U1_TXD_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1_TXD_O`"]
pub struct U1_TXD_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_TXD_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `U1_RXD_I`"]
pub type U1_RXD_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1_RXD_I`"]
pub struct U1_RXD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_RXD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `U1_RTS_O`"]
pub type U1_RTS_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U1_RTS_O`"]
pub struct U1_RTS_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U1_RTS_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u0_sclk_io(&self) -> U0_SCLK_IO_R {
        U0_SCLK_IO_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_txd_o(&self) -> U1_TXD_O_R {
        U1_TXD_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rxd_i(&self) -> U1_RXD_I_R {
        U1_RXD_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rts_o(&self) -> U1_RTS_O_R {
        U1_RTS_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U0_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u0_sclk_io(&mut self) -> U0_SCLK_IO_W {
        U0_SCLK_IO_W { w: self }
    }
    #[doc = "Bits 8:15 - U1_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_txd_o(&mut self) -> U1_TXD_O_W {
        U1_TXD_O_W { w: self }
    }
    #[doc = "Bits 16:23 - U1_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0(= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rxd_i(&mut self) -> U1_RXD_I_W {
        U1_RXD_I_W { w: self }
    }
    #[doc = "Bits 24:31 - U1_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn u1_rts_o(&mut self) -> U1_RTS_O_W {
        U1_RTS_O_W { w: self }
    }
}
