#[doc = "Reader of register DYNAMICRASCAS"]
pub type R = crate::R<u32, super::DYNAMICRASCAS>;
#[doc = "Writer for register DYNAMICRASCAS"]
pub type W = crate::W<u32, super::DYNAMICRASCAS>;
#[doc = "Register DYNAMICRASCAS `reset()`'s with value 0x0303"]
impl crate::ResetValue for super::DYNAMICRASCAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0303
    }
}
#[doc = "Reader of field `RAS`"]
pub type RAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RAS`"]
pub struct RAS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CAS`"]
pub type CAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAS`"]
pub struct CAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&mut self) -> RAS_W {
        RAS_W { w: self }
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W {
        CAS_W { w: self }
    }
}
