#[doc = "Reader of register CLAIMCLR"]
pub type R = crate::R<u32, super::CLAIMCLR>;
#[doc = "Writer for register CLAIMCLR"]
pub type W = crate::W<u32, super::CLAIMCLR>;
#[doc = "Register CLAIMCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::CLAIMCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLAIMCLR`"]
pub type CLAIMCLR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLAIMCLR`"]
pub struct CLAIMCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - A bit programmable register bank that is zero at reset. Write 1 to clear the bit in the claim tag. On reads, returns the current setting of the claim tag."]
    #[inline(always)]
    pub fn claimclr(&self) -> CLAIMCLR_R {
        CLAIMCLR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A bit programmable register bank that is zero at reset. Write 1 to clear the bit in the claim tag. On reads, returns the current setting of the claim tag."]
    #[inline(always)]
    pub fn claimclr(&mut self) -> CLAIMCLR_W {
        CLAIMCLR_W { w: self }
    }
}
