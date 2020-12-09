#[doc = "Reader of register MAC_SYS_TIMESTMP_ADDEND"]
pub type R = crate::R<u32, super::MAC_SYS_TIMESTMP_ADDEND>;
#[doc = "Writer for register MAC_SYS_TIMESTMP_ADDEND"]
pub type W = crate::W<u32, super::MAC_SYS_TIMESTMP_ADDEND>;
#[doc = "Register MAC_SYS_TIMESTMP_ADDEND `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_SYS_TIMESTMP_ADDEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSAR`"]
pub type TSAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TSAR`"]
pub struct TSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Time stamp addend This register indicates the 32-bit time value to be added to the Accumulator register to achieve time synchronization."]
    #[inline(always)]
    pub fn tsar(&mut self) -> TSAR_W {
        TSAR_W { w: self }
    }
}
