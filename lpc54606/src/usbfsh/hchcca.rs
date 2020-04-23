#[doc = "Reader of register HCHCCA"]
pub type R = crate::R<u32, super::HCHCCA>;
#[doc = "Writer for register HCHCCA"]
pub type W = crate::W<u32, super::HCHCCA>;
#[doc = "Register HCHCCA `reset()`'s with value 0"]
impl crate::ResetValue for super::HCHCCA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HCCA`"]
pub type HCCA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HCCA`"]
pub struct HCCA_W<'a> {
    w: &'a mut W,
}
impl<'a> HCCA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub fn hcca(&self) -> HCCA_R {
        HCCA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Base address of the Host Controller Communication Area."]
    #[inline(always)]
    pub fn hcca(&mut self) -> HCCA_W {
        HCCA_W { w: self }
    }
}
