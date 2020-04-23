#[doc = "Reader of register LAST_PTD_INUSE"]
pub type R = crate::R<u32, super::LAST_PTD_INUSE>;
#[doc = "Writer for register LAST_PTD_INUSE"]
pub type W = crate::W<u32, super::LAST_PTD_INUSE>;
#[doc = "Register LAST_PTD_INUSE `reset()`'s with value 0"]
impl crate::ResetValue for super::LAST_PTD_INUSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ATL_LAST`"]
pub type ATL_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ATL_LAST`"]
pub struct ATL_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ATL_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `ISO_LAST`"]
pub type ISO_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ISO_LAST`"]
pub struct ISO_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ISO_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `INT_LAST`"]
pub type INT_LAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INT_LAST`"]
pub struct INT_LAST_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_LAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&self) -> ATL_LAST_R {
        ATL_LAST_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&self) -> ISO_LAST_R {
        ISO_LAST_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&self) -> INT_LAST_R {
        INT_LAST_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - If hardware has reached this PTD and the J bit is not set, it will go to PTD0 as the next PTD to be processed."]
    #[inline(always)]
    pub fn atl_last(&mut self) -> ATL_LAST_W {
        ATL_LAST_W { w: self }
    }
    #[doc = "Bits 8:12 - This indicates the last PTD in the ISO list."]
    #[inline(always)]
    pub fn iso_last(&mut self) -> ISO_LAST_W {
        ISO_LAST_W { w: self }
    }
    #[doc = "Bits 16:20 - This indicates the last PTD in the INT list."]
    #[inline(always)]
    pub fn int_last(&mut self) -> INT_LAST_W {
        INT_LAST_W { w: self }
    }
}
