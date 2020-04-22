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
#[doc = "Reader of field `EP_LIST_PRG`"]
pub type EP_LIST_PRG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EP_LIST_PRG`"]
pub struct EP_LIST_PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_LIST_PRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | (((value as u32) & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EP_LIST_FIXED`"]
pub type EP_LIST_FIXED_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&self) -> EP_LIST_PRG_R {
        EP_LIST_PRG_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_fixed(&self) -> EP_LIST_FIXED_R {
        EP_LIST_FIXED_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn ep_list_prg(&mut self) -> EP_LIST_PRG_W {
        EP_LIST_PRG_W { w: self }
    }
}
