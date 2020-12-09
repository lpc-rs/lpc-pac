#[doc = "Reader of register HCCONTROLCURRENTED"]
pub type R = crate::R<u32, super::HCCONTROLCURRENTED>;
#[doc = "Writer for register HCCONTROLCURRENTED"]
pub type W = crate::W<u32, super::HCCONTROLCURRENTED>;
#[doc = "Register HCCONTROLCURRENTED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCCONTROLCURRENTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCED`"]
pub type CCED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CCED`"]
pub struct CCED_W<'a> {
    w: &'a mut W,
}
impl<'a> CCED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&self) -> CCED_R {
        CCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - ControlCurrentED."]
    #[inline(always)]
    pub fn cced(&mut self) -> CCED_W {
        CCED_W { w: self }
    }
}
