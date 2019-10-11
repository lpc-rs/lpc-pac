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
#[doc = "Reader of field `SCT0_GPIO_IN_B_I`"]
pub type SCT0_GPIO_IN_B_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT0_GPIO_IN_B_I`"]
pub struct SCT0_GPIO_IN_B_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_B_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SCT0_GPIO_IN_C_I`"]
pub type SCT0_GPIO_IN_C_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT0_GPIO_IN_C_I`"]
pub struct SCT0_GPIO_IN_C_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_C_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCT0_GPIO_IN_D_I`"]
pub type SCT0_GPIO_IN_D_I_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SCT0_GPIO_IN_D_I`"]
pub struct SCT0_GPIO_IN_D_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SCT0_GPIO_IN_D_I_W<'a> {
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
    #[doc = "Bits 0:7 - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_b_i(&self) -> SCT0_GPIO_IN_B_I_R {
        SCT0_GPIO_IN_B_I_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_c_i(&self) -> SCT0_GPIO_IN_C_I_R {
        SCT0_GPIO_IN_C_I_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_d_i(&self) -> SCT0_GPIO_IN_D_I_R {
        SCT0_GPIO_IN_D_I_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out0_o(&self) -> SCT_OUT0_O_R {
        SCT_OUT0_O_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCT0_GPIO_IN_B function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_b_i(&mut self) -> SCT0_GPIO_IN_B_I_W {
        SCT0_GPIO_IN_B_I_W { w: self }
    }
    #[doc = "Bits 8:15 - SCT0_GPIO_IN_C function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_c_i(&mut self) -> SCT0_GPIO_IN_C_I_W {
        SCT0_GPIO_IN_C_I_W { w: self }
    }
    #[doc = "Bits 16:23 - SCT0_GPIO_IN_D function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct0_gpio_in_d_i(&mut self) -> SCT0_GPIO_IN_D_I_W {
        SCT0_GPIO_IN_D_I_W { w: self }
    }
    #[doc = "Bits 24:31 - SCT_OUT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn sct_out0_o(&mut self) -> SCT_OUT0_O_W {
        SCT_OUT0_O_W { w: self }
    }
}
