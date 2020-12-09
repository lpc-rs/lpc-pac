#[doc = "Reader of register MAC_VERSION"]
pub type R = crate::R<u32, super::MAC_VERSION>;
#[doc = "Writer for register MAC_VERSION"]
pub type W = crate::W<u32, super::MAC_VERSION>;
#[doc = "Register MAC_VERSION `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_VERSION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SNPVER`"]
pub type SNPVER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SNPVER`"]
pub struct SNPVER_W<'a> {
    w: &'a mut W,
}
impl<'a> SNPVER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `USERVER`"]
pub type USERVER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USERVER`"]
pub struct USERVER_W<'a> {
    w: &'a mut W,
}
impl<'a> USERVER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NXP defined version."]
    #[inline(always)]
    pub fn snpver(&self) -> SNPVER_R {
        SNPVER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User defined version."]
    #[inline(always)]
    pub fn userver(&self) -> USERVER_R {
        USERVER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NXP defined version."]
    #[inline(always)]
    pub fn snpver(&mut self) -> SNPVER_W {
        SNPVER_W { w: self }
    }
    #[doc = "Bits 8:15 - User defined version."]
    #[inline(always)]
    pub fn userver(&mut self) -> USERVER_W {
        USERVER_W { w: self }
    }
}
