#[doc = "Reader of register FLADJ_FRINDEX"]
pub type R = crate::R<u32, super::FLADJ_FRINDEX>;
#[doc = "Writer for register FLADJ_FRINDEX"]
pub type W = crate::W<u32, super::FLADJ_FRINDEX>;
#[doc = "Register FLADJ_FRINDEX `reset()`'s with value 0x20"]
impl crate::ResetValue for super::FLADJ_FRINDEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `FLADJ`"]
pub type FLADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FLADJ`"]
pub struct FLADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> FLADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `FRINDEX`"]
pub type FRINDEX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRINDEX`"]
pub struct FRINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&self) -> FLADJ_R {
        FLADJ_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timing Value."]
    #[inline(always)]
    pub fn fladj(&mut self) -> FLADJ_W {
        FLADJ_W { w: self }
    }
    #[doc = "Bits 16:29 - Frame Index: Bits 29 to16 in this register are used for the frame number field in the SOF packet."]
    #[inline(always)]
    pub fn frindex(&mut self) -> FRINDEX_W {
        FRINDEX_W { w: self }
    }
}
