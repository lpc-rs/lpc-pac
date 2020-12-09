#[doc = "Reader of register EPLISTSTART"]
pub type R = crate::R<u32, super::EPLISTSTART>;
#[doc = "Writer for register EPLISTSTART"]
pub type W = crate::W<u32, super::EPLISTSTART>;
#[doc = "Register EPLISTSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::EPLISTSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EP_LIST`"]
pub type EP_LIST_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `EP_LIST`"]
pub struct EP_LIST_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_LIST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&self) -> EP_LIST_R {
        EP_LIST_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 8:31 - Start address of the USB EP Command/Status List."]
    #[inline(always)]
    pub fn ep_list(&mut self) -> EP_LIST_W {
        EP_LIST_W { w: self }
    }
}
