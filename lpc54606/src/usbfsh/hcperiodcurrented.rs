#[doc = "Reader of register HCPERIODCURRENTED"]
pub type R = crate::R<u32, super::HCPERIODCURRENTED>;
#[doc = "Writer for register HCPERIODCURRENTED"]
pub type W = crate::W<u32, super::HCPERIODCURRENTED>;
#[doc = "Register HCPERIODCURRENTED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCPERIODCURRENTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCED`"]
pub type PCED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PCED`"]
pub struct PCED_W<'a> {
    w: &'a mut W,
}
impl<'a> PCED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&self) -> PCED_R {
        PCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - The content of this register is updated by HC after a periodic ED is processed."]
    #[inline(always)]
    pub fn pced(&mut self) -> PCED_W {
        PCED_W { w: self }
    }
}
