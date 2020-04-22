#[doc = "Reader of register WDTOSCCTRL"]
pub type R = crate::R<u32, super::WDTOSCCTRL>;
#[doc = "Writer for register WDTOSCCTRL"]
pub type W = crate::W<u32, super::WDTOSCCTRL>;
#[doc = "Register WDTOSCCTRL `reset()`'s with value 0xa0"]
impl crate::ResetValue for super::WDTOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xa0
    }
}
#[doc = "Reader of field `DIVSEL`"]
pub type DIVSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVSEL`"]
pub struct DIVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `FREQSEL`"]
pub type FREQSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQSEL`"]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | (((value as u32) & 0x1f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Divider select."]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Frequency select."]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Divider select."]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bits 5:9 - Frequency select."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
}
