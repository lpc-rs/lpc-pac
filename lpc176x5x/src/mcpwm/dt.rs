#[doc = "Reader of register DT"]
pub type R = crate::R<u32, super::DT>;
#[doc = "Writer for register DT"]
pub type W = crate::W<u32, super::DT>;
#[doc = "Register DT `reset()`'s with value 0x3fff_ffff"]
impl crate::ResetValue for super::DT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3fff_ffff
    }
}
#[doc = "Reader of field `DT0`"]
pub type DT0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT0`"]
pub struct DT0_W<'a> {
    w: &'a mut W,
}
impl<'a> DT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DT1`"]
pub type DT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT1`"]
pub struct DT1_W<'a> {
    w: &'a mut W,
}
impl<'a> DT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `DT2`"]
pub type DT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DT2`"]
pub struct DT2_W<'a> {
    w: &'a mut W,
}
impl<'a> DT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | (((value as u32) & 0x03ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&self) -> DT2_R {
        DT2_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W {
        DT0_W { w: self }
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W {
        DT1_W { w: self }
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&mut self) -> DT2_W {
        DT2_W { w: self }
    }
}
