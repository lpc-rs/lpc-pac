#[doc = "Reader of register PINASSIGN14"]
pub type R = crate::R<u32, super::PINASSIGN14>;
#[doc = "Writer for register PINASSIGN14"]
pub type W = crate::W<u32, super::PINASSIGN14>;
#[doc = "Register PINASSIGN14 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PINASSIGN14 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `T0_MAT3`"]
pub type T0_MAT3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_MAT3`"]
pub struct T0_MAT3_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_MAT3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `T0_CAP0`"]
pub type T0_CAP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_CAP0`"]
pub struct T0_CAP0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `T0_CAP1`"]
pub type T0_CAP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_CAP1`"]
pub struct T0_CAP1_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `T0_CAP2`"]
pub type T0_CAP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0_CAP2`"]
pub struct T0_CAP2_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_CAP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat3(&self) -> T0_MAT3_R {
        T0_MAT3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap0(&self) -> T0_CAP0_R {
        T0_CAP0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap1(&self) -> T0_CAP1_R {
        T0_CAP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap2(&self) -> T0_CAP2_R {
        T0_CAP2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - T0_MAT3 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_mat3(&mut self) -> T0_MAT3_W {
        T0_MAT3_W { w: self }
    }
    #[doc = "Bits 8:15 - T0_CAP0 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap0(&mut self) -> T0_CAP0_W {
        T0_CAP0_W { w: self }
    }
    #[doc = "Bits 16:23 - T0_CAP1 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap1(&mut self) -> T0_CAP1_W {
        T0_CAP1_W { w: self }
    }
    #[doc = "Bits 24:31 - T0_CAP2 function assignment. The value is the pin number to be assigned to this function. The following pins are available: PIO0_0 (= 0) to PIO0_31 (= 0x1F) and from PIO1_0 (=0x20) to PIO1_21(=0x35)."]
    #[inline(always)]
    pub fn t0_cap2(&mut self) -> T0_CAP2_W {
        T0_CAP2_W { w: self }
    }
}
