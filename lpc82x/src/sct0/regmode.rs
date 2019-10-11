#[doc = "Reader of register REGMODE"]
pub type R = crate::R<u32, super::REGMODE>;
#[doc = "Writer for register REGMODE"]
pub type W = crate::W<u32, super::REGMODE>;
#[doc = "Register REGMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::REGMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGMOD_L`"]
pub type REGMOD_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGMOD_L`"]
pub struct REGMOD_L_W<'a> {
    w: &'a mut W,
}
impl<'a> REGMOD_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `REGMOD_H`"]
pub type REGMOD_H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGMOD_H`"]
pub struct REGMOD_H_W<'a> {
    w: &'a mut W,
}
impl<'a> REGMOD_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&self) -> REGMOD_L_R {
        REGMOD_L_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&self) -> REGMOD_H_R {
        REGMOD_H_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Each bit controls one match/capture register (register 0 = bit 0, register 1 = bit 1, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match register. 1 = register operates as capture register."]
    #[inline(always)]
    pub fn regmod_l(&mut self) -> REGMOD_L_W {
        REGMOD_L_W { w: self }
    }
    #[doc = "Bits 16:23 - Each bit controls one match/capture register (register 0 = bit 16, register 1 = bit 17, etc.). The number of bits = number of match/captures in this SCT. 0 = register operates as match registers. 1 = register operates as capture registers."]
    #[inline(always)]
    pub fn regmod_h(&mut self) -> REGMOD_H_W {
        REGMOD_H_W { w: self }
    }
}
