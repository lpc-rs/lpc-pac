#[doc = "Reader of register CLAIMSET"]
pub type R = crate::R<u32, super::CLAIMSET>;
#[doc = "Writer for register CLAIMSET"]
pub type W = crate::W<u32, super::CLAIMSET>;
#[doc = "Register CLAIMSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CLAIMSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLAIMSET`"]
pub type CLAIMSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLAIMSET`"]
pub struct CLAIMSET_W<'a> {
    w: &'a mut W,
}
impl<'a> CLAIMSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
    #[inline(always)]
    pub fn claimset(&self) -> CLAIMSET_R {
        CLAIMSET_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - A bit programmable register bank which sets the Claim Tag Value. Write 1 to set the bit in the claim tag. A read will return a logic 1 for all implemented locations."]
    #[inline(always)]
    pub fn claimset(&mut self) -> CLAIMSET_W {
        CLAIMSET_W { w: self }
    }
}
