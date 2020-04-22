#[doc = "Reader of register MAT[%s]"]
pub type R = crate::R<u32, super::MAT>;
#[doc = "Writer for register MAT[%s]"]
pub type W = crate::W<u32, super::MAT>;
#[doc = "Register MAT[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::MAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCMAT`"]
pub type MCMAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MCMAT`"]
pub struct MCMAT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCMAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Match value."]
    #[inline(always)]
    pub fn mcmat(&self) -> MCMAT_R {
        MCMAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Match value."]
    #[inline(always)]
    pub fn mcmat(&mut self) -> MCMAT_W {
        MCMAT_W { w: self }
    }
}
