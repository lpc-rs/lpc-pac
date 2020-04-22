#[doc = "Reader of register EPBUFCFG"]
pub type R = crate::R<u32, super::EPBUFCFG>;
#[doc = "Writer for register EPBUFCFG"]
pub type W = crate::W<u32, super::EPBUFCFG>;
#[doc = "Register EPBUFCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::EPBUFCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUF_SB`"]
pub type BUF_SB_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUF_SB`"]
pub struct BUF_SB_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 2)) | (((value as u32) & 0x03ff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:11 - Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf_sb(&self) -> BUF_SB_R {
        BUF_SB_R::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn buf_sb(&mut self) -> BUF_SB_W {
        BUF_SB_W { w: self }
    }
}
