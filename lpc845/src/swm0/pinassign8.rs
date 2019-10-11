#[doc = "Reader of register PINASSIGN8"]
pub type R = crate::R<u32, super::PINASSIGN8>;
#[doc = "Writer for register PINASSIGN8"]
pub type W = crate::W<u32, super::PINASSIGN8>;
#[doc = "Register PINASSIGN8 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN8 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SCT_OUT1_O`"]
pub type SCT_OUT1_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT1_O`"]
pub struct SCT_OUT1_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT1_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SCT_OUT2_O`"]
pub type SCT_OUT2_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT2_O`"]
pub struct SCT_OUT2_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT2_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCT_OUT3_O`"]
pub type SCT_OUT3_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT3_O`"]
pub struct SCT_OUT3_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT3_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCT_OUT4_O`"]
pub type SCT_OUT4_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT4_O`"]
pub struct SCT_OUT4_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT4_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out1_o(&self) -> SCT_OUT1_O_R {
        SCT_OUT1_O_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out2_o(&self) -> SCT_OUT2_O_R {
        SCT_OUT2_O_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out3_o(&self) -> SCT_OUT3_O_R {
        SCT_OUT3_O_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out4_o(&self) -> SCT_OUT4_O_R {
        SCT_OUT4_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT_OUT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out1_o(&mut self) -> SCT_OUT1_O_W {
        SCT_OUT1_O_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT_OUT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out2_o(&mut self) -> SCT_OUT2_O_W {
        SCT_OUT2_O_W { w: self }
    }
    #[doc = "Bits 16:23 - SCT_OUT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out3_o(&mut self) -> SCT_OUT3_O_W {
        SCT_OUT3_O_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_OUT4 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out4_o(&mut self) -> SCT_OUT4_O_W {
        SCT_OUT4_O_W { w: self }
    }
}
