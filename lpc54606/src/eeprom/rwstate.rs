#[doc = "Reader of register RWSTATE"]
pub type R = crate::R<u32, super::RWSTATE>;
#[doc = "Writer for register RWSTATE"]
pub type W = crate::W<u32, super::RWSTATE>;
#[doc = "Register RWSTATE `reset()`'s with value 0x0e07"]
impl crate::ResetValue for super::RWSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0e07
    }
}
#[doc = "Reader of field `RPHASE2`"]
pub type RPHASE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPHASE2`"]
pub struct RPHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `RPHASE1`"]
pub type RPHASE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPHASE1`"]
pub struct RPHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> RPHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Wait states 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase2(&self) -> RPHASE2_R {
        RPHASE2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase1(&self) -> RPHASE1_R {
        RPHASE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase2(&mut self) -> RPHASE2_W {
        RPHASE2_W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn rphase1(&mut self) -> RPHASE1_W {
        RPHASE1_W { w: self }
    }
}
