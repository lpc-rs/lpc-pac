#[doc = "Reader of register DIVHFCLK"]
pub type R = crate::R<u32, super::DIVHFCLK>;
#[doc = "Writer for register DIVHFCLK"]
pub type W = crate::W<u32, super::DIVHFCLK>;
#[doc = "Register DIVHFCLK `reset()`'s with value 0"]
impl crate::ResetValue for super::DIVHFCLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDMDIV`"]
pub type PDMDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PDMDIV`"]
pub struct PDMDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
    #[inline(always)]
    pub fn pdmdiv(&self) -> PDMDIV_R {
        PDMDIV_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDM clock divider value. 0 = divide by 1 1 = divide by 2 2 = divide by 3 3 = divide by 4 4 = divide by 6 5 = divide by 8 6 = divide by 12 7 = divide by 16 8 = divide by 24 9 = divide by 32 10 = divide by 48 11 = divide by 64 12 = divide by 96 13 = divide by 128 others = reserved."]
    #[inline(always)]
    pub fn pdmdiv(&mut self) -> PDMDIV_W {
        PDMDIV_W { w: self }
    }
}
