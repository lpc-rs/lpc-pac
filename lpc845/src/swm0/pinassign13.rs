#[doc = "Reader of register PINASSIGN13"]
pub type R = crate::R<u32, super::PINASSIGN13>;
#[doc = "Writer for register PINASSIGN13"]
pub type W = crate::W<u32, super::PINASSIGN13>;
#[doc = "Register PINASSIGN13 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `UART4_SCLK`"]
pub type UART4_SCLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UART4_SCLK`"]
pub struct UART4_SCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4_SCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `T0_MAT0`"]
pub type T0_MAT0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_MAT0`"]
pub struct T0_MAT0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `T0_MAT1`"]
pub type T0_MAT1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_MAT1`"]
pub struct T0_MAT1_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `T0_MAT2`"]
pub type T0_MAT2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_MAT2`"]
pub struct T0_MAT2_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_sclk(&self) -> UART4_SCLK_R {
        UART4_SCLK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat0(&self) -> T0_MAT0_R {
        T0_MAT0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat1(&self) -> T0_MAT1_R {
        T0_MAT1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat2(&self) -> T0_MAT2_R {
        T0_MAT2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART4_SCLK function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn uart4_sclk(&mut self) -> UART4_SCLK_W {
        UART4_SCLK_W { w: self }
    }
    #[doc = "Bits 8:15 - T0_MAT0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat0(&mut self) -> T0_MAT0_W {
        T0_MAT0_W { w: self }
    }
    #[doc = "Bits 16:23 - T0_MAT1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat1(&mut self) -> T0_MAT1_W {
        T0_MAT1_W { w: self }
    }
    #[doc = "Bits 24:31 - T0_MAT2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat2(&mut self) -> T0_MAT2_W {
        T0_MAT2_W { w: self }
    }
}
