#[doc = "Reader of register HCDONEHEAD"]
pub type R = crate::R<u32, super::HCDONEHEAD>;
#[doc = "Writer for register HCDONEHEAD"]
pub type W = crate::W<u32, super::HCDONEHEAD>;
#[doc = "Register HCDONEHEAD `reset()`'s with value 0"]
impl crate::ResetValue for super::HCDONEHEAD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DH`"]
pub type DH_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DH`"]
pub struct DH_W<'a> {
    w: &'a mut W,
}
impl<'a> DH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff_ffff << 4)) | (((value as u32) & 0x0fff_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:31 - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new(((self.bits >> 4) & 0x0fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:31 - DoneHead When a TD is completed, HC writes the content of HcDoneHead to the NextTD field of the TD."]
    #[inline(always)]
    pub fn dh(&mut self) -> DH_W {
        DH_W { w: self }
    }
}
