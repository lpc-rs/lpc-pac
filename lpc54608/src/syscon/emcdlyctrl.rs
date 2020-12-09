#[doc = "Reader of register EMCDLYCTRL"]
pub type R = crate::R<u32, super::EMCDLYCTRL>;
#[doc = "Writer for register EMCDLYCTRL"]
pub type W = crate::W<u32, super::EMCDLYCTRL>;
#[doc = "Register EMCDLYCTRL `reset()`'s with value 0x0210"]
impl crate::ResetValue for super::EMCDLYCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0210
    }
}
#[doc = "Reader of field `CMD_DELAY`"]
pub type CMD_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMD_DELAY`"]
pub struct CMD_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `FBCLK_DELAY`"]
pub type FBCLK_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FBCLK_DELAY`"]
pub struct FBCLK_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FBCLK_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode."]
    #[inline(always)]
    pub fn cmd_delay(&self) -> CMD_DELAY_R {
        CMD_DELAY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling."]
    #[inline(always)]
    pub fn fbclk_delay(&self) -> FBCLK_DELAY_R {
        FBCLK_DELAY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Programmable delay value for EMC outputs in command delayed mode."]
    #[inline(always)]
    pub fn cmd_delay(&mut self) -> CMD_DELAY_W {
        CMD_DELAY_W { w: self }
    }
    #[doc = "Bits 8:12 - Programmable delay value for the feedback clock that controls input data sampling."]
    #[inline(always)]
    pub fn fbclk_delay(&mut self) -> FBCLK_DELAY_W {
        FBCLK_DELAY_W { w: self }
    }
}
