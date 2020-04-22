#[doc = "Reader of register ETHSBDCTRL"]
pub type R = crate::R<u32, super::ETHSBDCTRL>;
#[doc = "Writer for register ETHSBDCTRL"]
pub type W = crate::W<u32, super::ETHSBDCTRL>;
#[doc = "Register ETHSBDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ETHSBDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBD_CTRL`"]
pub type SBD_CTRL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SBD_CTRL`"]
pub struct SBD_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SBD_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sideband Flow Control."]
    #[inline(always)]
    pub fn sbd_ctrl(&self) -> SBD_CTRL_R {
        SBD_CTRL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sideband Flow Control."]
    #[inline(always)]
    pub fn sbd_ctrl(&mut self) -> SBD_CTRL_W {
        SBD_CTRL_W { w: self }
    }
}
