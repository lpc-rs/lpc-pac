#[doc = "Reader of register GPREG[%s]"]
pub type R = crate::R<u32, super::GPREG>;
#[doc = "Writer for register GPREG[%s]"]
pub type W = crate::W<u32, super::GPREG>;
#[doc = "Register GPREG[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::GPREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPDATA`"]
pub type GPDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `GPDATA`"]
pub struct GPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub fn gpdata(&self) -> GPDATA_R {
        GPDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data retained during Deep power-down mode or loss of main power as long as VBAT is supplied."]
    #[inline(always)]
    pub fn gpdata(&mut self) -> GPDATA_W {
        GPDATA_W { w: self }
    }
}
