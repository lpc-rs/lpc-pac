#[doc = "Reader of register MAC_SUB_SCND_INCR"]
pub type R = crate::R<u32, super::MAC_SUB_SCND_INCR>;
#[doc = "Writer for register MAC_SUB_SCND_INCR"]
pub type W = crate::W<u32, super::MAC_SUB_SCND_INCR>;
#[doc = "Register MAC_SUB_SCND_INCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MAC_SUB_SCND_INCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSINC`"]
pub type SSINC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SSINC`"]
pub struct SSINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SSINC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Sub-second increment value."]
    #[inline(always)]
    pub fn ssinc(&self) -> SSINC_R {
        SSINC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Sub-second increment value."]
    #[inline(always)]
    pub fn ssinc(&mut self) -> SSINC_W {
        SSINC_W { w: self }
    }
}
