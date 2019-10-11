#[doc = "Reader of register DIV"]
pub type R = crate::R<u32, super::DIV>;
#[doc = "Writer for register DIV"]
pub type W = crate::W<u32, super::DIV>;
#[doc = "Register DIV `reset()`'s with value 0"]
impl crate::ResetValue for super::DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVVAL`"]
pub type DIVVAL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVVAL`"]
pub struct DIVVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub fn divval(&self) -> DIVVAL_R {
        DIVVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub fn divval(&mut self) -> DIVVAL_W {
        DIVVAL_W { w: self }
    }
}
