#[doc = "Reader of register PINASSIGN11"]
pub type R = crate::R<u32, super::PINASSIGN11>;
#[doc = "Writer for register PINASSIGN11"]
pub type W = crate::W<u32, super::PINASSIGN11>;
#[doc = "Register PINASSIGN11 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN11 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `COMP0_OUT_O`"]
pub type COMP0_OUT_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COMP0_OUT_O`"]
pub struct COMP0_OUT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_OUT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CLKOUT_O`"]
pub type CLKOUT_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKOUT_O`"]
pub struct CLKOUT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `GPIO_INT_BMAT_O`"]
pub type GPIO_INT_BMAT_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_INT_BMAT_O`"]
pub struct GPIO_INT_BMAT_O_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_INT_BMAT_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `UART3_TXD`"]
pub type UART3_TXD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART3_TXD`"]
pub struct UART3_TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - COMP0_OUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn comp0_out_o(&self) -> COMP0_OUT_O_R {
        COMP0_OUT_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn clkout_o(&self) -> CLKOUT_O_R {
        CLKOUT_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&self) -> GPIO_INT_BMAT_O_R {
        GPIO_INT_BMAT_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - UART3_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_txd(&self) -> UART3_TXD_R {
        UART3_TXD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - COMP0_OUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn comp0_out_o(&mut self) -> COMP0_OUT_O_W {
        COMP0_OUT_O_W { w: self }
    }
    #[doc = "Bits 8:15 - CLKOUT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn clkout_o(&mut self) -> CLKOUT_O_W {
        CLKOUT_O_W { w: self }
    }
    #[doc = "Bits 16:23 - GPIO_INT_BMAT function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn gpio_int_bmat_o(&mut self) -> GPIO_INT_BMAT_O_W {
        GPIO_INT_BMAT_O_W { w: self }
    }
    #[doc = "Bits 24:31 - UART3_TXD function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart3_txd(&mut self) -> UART3_TXD_W {
        UART3_TXD_W { w: self }
    }
}
