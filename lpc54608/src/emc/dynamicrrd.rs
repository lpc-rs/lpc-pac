#[doc = "Reader of register DYNAMICRRD"]
pub type R = crate::R<u32, super::DYNAMICRRD>;
#[doc = "Writer for register DYNAMICRRD"]
pub type W = crate::W<u32, super::DYNAMICRRD>;
#[doc = "Register DYNAMICRRD `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::DYNAMICRRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Reader of field `TRRD`"]
pub type TRRD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRRD`"]
pub struct TRRD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRRD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles."]
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W {
        TRRD_W { w: self }
    }
}
