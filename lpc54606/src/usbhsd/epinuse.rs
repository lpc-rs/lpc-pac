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
pub type BUF_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUF`"]
pub struct BUF_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf(&self) -> BUF_R {
        BUF_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf(&mut self) -> BUF_W {
        BUF_W { w: self }
    }
}
