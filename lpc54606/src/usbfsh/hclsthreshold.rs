#[doc = "Reader of register HCLSTHRESHOLD"]
pub type R = crate::R<u32, super::HCLSTHRESHOLD>;
#[doc = "Writer for register HCLSTHRESHOLD"]
pub type W = crate::W<u32, super::HCLSTHRESHOLD>;
#[doc = "Register HCLSTHRESHOLD `reset()`'s with value 0x0628"]
impl crate::ResetValue for super::HCLSTHRESHOLD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0628
    }
}
#[doc = "Reader of field `LST`"]
pub type LST_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LST`"]
pub struct LST_W<'a> {
    w: &'a mut W,
}
impl<'a> LST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&self) -> LST_R {
        LST_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - LSThreshold This field contains a value which is compared to the FrameRemaining field prior to initiating a Low Speed transaction."]
    #[inline(always)]
    pub fn lst(&mut self) -> LST_W {
        LST_W { w: self }
    }
}
