#[doc = "Reader of register PINASSIGN0"]
pub type R = crate::R<u32, super::PINASSIGN0>;
#[doc = "Writer for register PINASSIGN0"]
pub type W = crate::W<u32, super::PINASSIGN0>;
#[doc = "Register PINASSIGN0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `U0_TXD_O`"]
pub type U0_TXD_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U0_TXD_O`"]
pub struct U0_TXD_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_TXD_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `U0_RXD_I`"]
pub type U0_RXD_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U0_RXD_I`"]
pub struct U0_RXD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_RXD_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `U0_RTS_O`"]
pub type U0_RTS_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U0_RTS_O`"]
pub struct U0_RTS_O_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_RTS_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `U0_CTS_I`"]
pub type U0_CTS_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `U0_CTS_I`"]
pub struct U0_CTS_I_W<'a> {
    w: &'a mut W,
}
impl<'a> U0_CTS_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35) ."]
    #[inline(always)]
    pub fn u0_txd_o(&self) -> U0_TXD_O_R {
        U0_TXD_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_rxd_i(&self) -> U0_RXD_I_R {
        U0_RXD_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_rts_o(&self) -> U0_RTS_O_R {
        U0_RTS_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_cts_i(&self) -> U0_CTS_I_R {
        U0_CTS_I_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - U0_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35) ."]
    #[inline(always)]
    pub fn u0_txd_o(&mut self) -> U0_TXD_O_W {
        U0_TXD_O_W { w: self }
    }
    #[doc = "Bits 8:15 - U0_RXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_rxd_i(&mut self) -> U0_RXD_I_W {
        U0_RXD_I_W { w: self }
    }
    #[doc = "Bits 16:23 - U0_RTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_rts_o(&mut self) -> U0_RTS_O_W {
        U0_RTS_O_W { w: self }
    }
    #[doc = "Bits 24:31 - U0_CTS function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn u0_cts_i(&mut self) -> U0_CTS_I_W {
        U0_CTS_I_W { w: self }
    }
}
