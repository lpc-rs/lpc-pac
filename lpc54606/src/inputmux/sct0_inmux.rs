#[doc = "Reader of register SCT0_INMUX[%s]"]
pub type R = crate::R<u32, super::SCT0_INMUX>;
#[doc = "Writer for register SCT0_INMUX[%s]"]
pub type W = crate::W<u32, super::SCT0_INMUX>;
#[doc = "Register SCT0_INMUX[%s]
`reset()`'s with value 0x1f"]
impl crate::ResetValue for super::SCT0_INMUX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `INP_N`"]
pub type INP_N_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INP_N`"]
pub struct INP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> INP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&mut self) -> INP_N_W {
        INP_N_W { w: self }
    }
}
