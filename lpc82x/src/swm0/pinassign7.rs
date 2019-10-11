#[doc = "Reader of register PINASSIGN7"]
pub type R = crate::R<u32, super::PINASSIGN7>;
#[doc = "Writer for register PINASSIGN7"]
pub type W = crate::W<u32, super::PINASSIGN7>;
#[doc = "Register PINASSIGN7 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN7 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `SCT_PIN1_I`"]
pub type SCT_PIN1_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_PIN1_I`"]
pub struct SCT_PIN1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_PIN1_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SCT_PIN2_I`"]
pub type SCT_PIN2_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_PIN2_I`"]
pub struct SCT_PIN2_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_PIN2_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCT_PIN3_I`"]
pub type SCT_PIN3_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_PIN3_I`"]
pub struct SCT_PIN3_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_PIN3_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SCT_OUT0_O`"]
pub type SCT_OUT0_O_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT_OUT0_O`"]
pub struct SCT_OUT0_O_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT_OUT0_O_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SCT_PIN1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin1_i(&self) -> SCT_PIN1_I_R {
        SCT_PIN1_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT_PIN2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin2_i(&self) -> SCT_PIN2_I_R {
        SCT_PIN2_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCT_PIN3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin3_i(&self) -> SCT_PIN3_I_R {
        SCT_PIN3_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out0_o(&self) -> SCT_OUT0_O_R {
        SCT_OUT0_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT_PIN1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin1_i(&mut self) -> SCT_PIN1_I_W {
        SCT_PIN1_I_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT_PIN2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin2_i(&mut self) -> SCT_PIN2_I_W {
        SCT_PIN2_I_W { w: self }
    }
    #[doc = "Bits 16:23 - SCT_PIN3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_pin3_i(&mut self) -> SCT_PIN3_I_W {
        SCT_PIN3_I_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_28 (= 0x1C)."]
    #[inline(always)]
    pub fn sct_out0_o(&mut self) -> SCT_OUT0_O_W {
        SCT_OUT0_O_W { w: self }
    }
}
