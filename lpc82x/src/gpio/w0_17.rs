#[doc = "Reader of register W0_17"]
pub type R = crate::R<u32, super::W0_17>;
#[doc = "Writer for register W0_17"]
pub type W = crate::W<u32, super::W0_17>;
#[doc = "Register W0_17 `reset()`'s with value 0"]
impl crate::ResetValue for super::W0_17 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PWORD`"]
pub type PWORD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PWORD`"]
pub struct PWORD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&self) -> PWORD_R {
        PWORD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn pword(&mut self) -> PWORD_W {
        PWORD_W { w: self }
    }
}
