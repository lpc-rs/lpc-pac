#[doc = "Reader of register TCBCNT"]
pub type R = crate::R<u32, super::TCBCNT>;
#[doc = "Writer for register TCBCNT"]
pub type W = crate::W<u32, super::TCBCNT>;
#[doc = "Register TCBCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::TCBCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TRANS_CARD_BYTE_COUNT`"]
pub type TRANS_CARD_BYTE_COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TRANS_CARD_BYTE_COUNT`"]
pub struct TRANS_CARD_BYTE_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_CARD_BYTE_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&self) -> TRANS_CARD_BYTE_COUNT_R {
        TRANS_CARD_BYTE_COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transferred by CIU unit to card."]
    #[inline(always)]
    pub fn trans_card_byte_count(&mut self) -> TRANS_CARD_BYTE_COUNT_W {
        TRANS_CARD_BYTE_COUNT_W { w: self }
    }
}
