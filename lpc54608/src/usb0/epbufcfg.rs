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
pub type BUF_SB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BUF_SB`"]
pub struct BUF_SB_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_SB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 2)) | (((value as u32) & 0xff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&self) -> BUF_SB_R {
        BUF_SB_R::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:9 - Buffer usage: This register has one bit per physical endpoint. 0: Single-buffer. 1: Double-buffer. If the bit is set to single-buffer (0), it will not toggle the corresponding EPINUSE bit when it clears the active bit. If the bit is set to double-buffer (1), HW will toggle the EPINUSE bit when it clears the Active bit for the buffer."]
    #[inline(always)]
    pub fn buf_sb(&mut self) -> BUF_SB_W {
        BUF_SB_W { w: self }
    }
}
