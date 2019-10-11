#[doc = "Reader of register SA0"]
pub type R = crate::R<u32, super::SA0>;
#[doc = "Writer for register SA0"]
pub type W = crate::W<u32, super::SA0>;
#[doc = "Register SA0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SA0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SADDR2`"]
pub type SADDR2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR2`"]
pub struct SADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `SADDR1`"]
pub type SADDR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SADDR1`"]
pub struct SADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&self) -> SADDR2_R {
        SADDR2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&self) -> SADDR1_R {
        SADDR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&mut self) -> SADDR2_W {
        SADDR2_W { w: self }
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&mut self) -> SADDR1_W {
        SADDR1_W { w: self }
    }
}
