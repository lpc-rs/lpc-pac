#[doc = "Reader of register CRSR_PAL0"]
pub type R = crate::R<u32, super::CRSR_PAL0>;
#[doc = "Writer for register CRSR_PAL0"]
pub type W = crate::W<u32, super::CRSR_PAL0>;
#[doc = "Register CRSR_PAL0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_PAL0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RED`"]
pub type RED_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RED`"]
pub struct RED_W<'a> {
    w: &'a mut W,
}
impl<'a> RED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `GREEN`"]
pub type GREEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GREEN`"]
pub struct GREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BLUE`"]
pub type BLUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BLUE`"]
pub struct BLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Red color component."]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green color component."]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Blue color component."]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Red color component."]
    #[inline(always)]
    pub fn red(&mut self) -> RED_W {
        RED_W { w: self }
    }
    #[doc = "Bits 8:15 - Green color component."]
    #[inline(always)]
    pub fn green(&mut self) -> GREEN_W {
        GREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - Blue color component."]
    #[inline(always)]
    pub fn blue(&mut self) -> BLUE_W {
        BLUE_W { w: self }
    }
}
