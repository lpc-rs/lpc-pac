#[doc = "Reader of register HCBULKHEADED"]
pub type R = crate::R<u32, super::HCBULKHEADED>;
#[doc = "Writer for register HCBULKHEADED"]
pub type W = crate::W<u32, super::HCBULKHEADED>;
#[doc = "Register HCBULKHEADED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCBULKHEADED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BHED`"]
pub type BHED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BHED`"]
pub struct BHED_W<'a> {
    w: &'a mut W,
}
impl<'a> BHED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&self) -> BHED_R {
        BHED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - BulkHeadED HC traverses the bulk list starting with the HcBulkHeadED pointer."]
    #[inline(always)]
    pub fn bhed(&mut self) -> BHED_W {
        BHED_W { w: self }
    }
}
