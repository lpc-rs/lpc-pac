#[doc = "Reader of register CRSR_CLIP"]
pub type R = crate::R<u32, super::CRSR_CLIP>;
#[doc = "Writer for register CRSR_CLIP"]
pub type W = crate::W<u32, super::CRSR_CLIP>;
#[doc = "Register CRSR_CLIP `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_CLIP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSRCLIPX`"]
pub type CRSRCLIPX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRSRCLIPX`"]
pub struct CRSRCLIPX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRCLIPX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `CRSRCLIPY`"]
pub type CRSRCLIPY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CRSRCLIPY`"]
pub struct CRSRCLIPY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRCLIPY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Cursor clip position for X direction."]
    #[inline(always)]
    pub fn crsrclipx(&self) -> CRSRCLIPX_R {
        CRSRCLIPX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction."]
    #[inline(always)]
    pub fn crsrclipy(&self) -> CRSRCLIPY_R {
        CRSRCLIPY_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cursor clip position for X direction."]
    #[inline(always)]
    pub fn crsrclipx(&mut self) -> CRSRCLIPX_W {
        CRSRCLIPX_W { w: self }
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction."]
    #[inline(always)]
    pub fn crsrclipy(&mut self) -> CRSRCLIPY_W {
        CRSRCLIPY_W { w: self }
    }
}
