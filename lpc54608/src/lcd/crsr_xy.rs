#[doc = "Reader of register CRSR_XY"]
pub type R = crate::R<u32, super::CRSR_XY>;
#[doc = "Writer for register CRSR_XY"]
pub type W = crate::W<u32, super::CRSR_XY>;
#[doc = "Register CRSR_XY `reset()`'s with value 0"]
impl crate::ResetValue for super::CRSR_XY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CRSRX`"]
pub type CRSRX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRSRX`"]
pub struct CRSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `CRSRY`"]
pub type CRSRY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CRSRY`"]
pub struct CRSRY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsrx(&self) -> CRSRX_R {
        CRSRX_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsry(&self) -> CRSRY_R {
        CRSRY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsrx(&mut self) -> CRSRX_W {
        CRSRX_W { w: self }
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels."]
    #[inline(always)]
    pub fn crsry(&mut self) -> CRSRY_W {
        CRSRY_W { w: self }
    }
}
