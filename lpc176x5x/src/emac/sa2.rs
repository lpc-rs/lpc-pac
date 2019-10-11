#[doc = "Reader of register SA2"]
pub type R = crate::R<u32, super::SA2>;
#[doc = "Writer for register SA2"]
pub type W = crate::W<u32, super::SA2>;
#[doc = "Register SA2 `reset()`'s with value 0"]
impl crate::ResetValue for super::SA2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADDR6`"]
pub type SADDR6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR6`"]
pub struct SADDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SADDR5`"]
pub type SADDR5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR5`"]
pub struct SADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&self) -> SADDR6_R {
        SADDR6_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&self) -> SADDR5_R {
        SADDR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&mut self) -> SADDR6_W {
        SADDR6_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&mut self) -> SADDR5_W {
        SADDR5_W { w: self }
    }
}
