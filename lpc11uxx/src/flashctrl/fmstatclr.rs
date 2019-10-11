#[doc = "Writer for register FMSTATCLR"]
pub type W = crate::W<u32, super::FMSTATCLR>;
#[doc = "Register FMSTATCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::FMSTATCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SIG_DONE_CLR`"]
pub struct SIG_DONE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_DONE_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
    #[inline(always)]
    pub fn sig_done_clr(&mut self) -> SIG_DONE_CLR_W {
        SIG_DONE_CLR_W { w: self }
    }
}
