#[doc = "Reader of register EPINUSE"]
pub type R = crate::R<u32, super::EPINUSE>;
#[doc = "Writer for register EPINUSE"]
pub type W = crate::W<u32, super::EPINUSE>;
#[doc = "Register EPINUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::EPINUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF`"]
pub type BUF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUF`"]
pub struct BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer in use: This register has one bit per physical endpoint. 0: HW is accessing buffer 0. 1: HW is accessing buffer 1."]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W {
        BUF_W { w: self }
    }
}
