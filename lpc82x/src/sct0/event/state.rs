#[doc = "Reader of register STATE"]
pub type R = crate::R<u32, super::STATE>;
#[doc = "Writer for register STATE"]
pub type W = crate::W<u32, super::STATE>;
#[doc = "Register STATE `reset()`'s with value 0"]
impl crate::ResetValue for super::STATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STATEMSKn`"]
pub type STATEMSKN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATEMSKn`"]
pub struct STATEMSKN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATEMSKN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&self) -> STATEMSKN_R {
        STATEMSKN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - If bit m is one, event n happens in state m of the counter selected by the HEVENT bit (n = event number, m = state number; state 0 = bit 0, state 1= bit 1, etc.). The number of bits = number of states in this SCT."]
    #[inline(always)]
    pub fn statemskn(&mut self) -> STATEMSKN_W {
        STATEMSKN_W { w: self }
    }
}
