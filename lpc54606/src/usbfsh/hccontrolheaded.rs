#[doc = "Reader of register HCCONTROLHEADED"]
pub type R = crate::R<u32, super::HCCONTROLHEADED>;
#[doc = "Writer for register HCCONTROLHEADED"]
pub type W = crate::W<u32, super::HCCONTROLHEADED>;
#[doc = "Register HCCONTROLHEADED `reset()`'s with value 0"]
impl crate::ResetValue for super::HCCONTROLHEADED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHED`"]
pub type CHED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHED`"]
pub struct CHED_W<'a> {
    w: &'a mut W,
}
impl<'a> CHED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub fn ched(&self) -> CHED_R {
        CHED_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - HC traverses the Control list starting with the HcControlHeadED pointer."]
    #[inline(always)]
    pub fn ched(&mut self) -> CHED_W {
        CHED_W { w: self }
    }
}
