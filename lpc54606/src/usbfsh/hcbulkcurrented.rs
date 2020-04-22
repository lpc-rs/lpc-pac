#[doc = "Reader of register HCBULKCURRENTED"]
pub type R = crate::R<u32, super::HCBULKCURRENTED>;
#[doc = "Writer for register HCBULKCURRENTED"]
pub type W = crate::W<u32, super::HCBULKCURRENTED>;
#[doc = "Register HCBULKCURRENTED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCBULKCURRENTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BCED`"]
pub type BCED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BCED`"]
pub struct BCED_W<'a> {
    w: &'a mut W,
}
impl<'a> BCED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub fn bced(&self) -> BCED_R {
        BCED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkCurrentED This is advanced to the next ED after the HC has served the current one."]
    #[inline(always)]
    pub fn bced(&mut self) -> BCED_W {
        BCED_W { w: self }
    }
}
