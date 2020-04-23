#[doc = "Reader of register COMPVAL_H"]
pub type R = crate::R<u32, super::COMPVAL_H>;
#[doc = "Writer for register COMPVAL_H"]
pub type W = crate::W<u32, super::COMPVAL_H>;
#[doc = "Register COMPVAL_H `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::COMPVAL_H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `RICOMP`"]
pub type RICOMP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RICOMP`"]
pub struct RICOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RICOMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Compare value MSB register."]
    #[inline(always)]
    pub fn ricomp(&self) -> RICOMP_R {
        RICOMP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare value MSB register."]
    #[inline(always)]
    pub fn ricomp(&mut self) -> RICOMP_W {
        RICOMP_W { w: self }
    }
}
