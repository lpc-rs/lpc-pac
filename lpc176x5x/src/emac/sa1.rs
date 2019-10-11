#[doc = "Reader of register SA1"]
pub type R = crate::R<u32, super::SA1>;
#[doc = "Writer for register SA1"]
pub type W = crate::W<u32, super::SA1>;
#[doc = "Register SA1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SA1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADDR4`"]
pub type SADDR4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR4`"]
pub struct SADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SADDR3`"]
pub type SADDR3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR3`"]
pub struct SADDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&self) -> SADDR4_R {
        SADDR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&self) -> SADDR3_R {
        SADDR3_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&mut self) -> SADDR4_W {
        SADDR4_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&mut self) -> SADDR3_W {
        SADDR3_W { w: self }
    }
}
