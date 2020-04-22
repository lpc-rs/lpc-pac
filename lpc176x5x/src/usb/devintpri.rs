#[doc = "Writer for register DEVINTPRI"]
pub type W = crate::W<u32, super::DEVINTPRI>;
#[doc = "Register DEVINTPRI `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVINTPRI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Frame interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_AW {
    #[doc = "0: FRAME interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: FRAME interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<FRAME_AW> for bool {
    #[inline(always)]
    fn from(variant: FRAME_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FRAME`"]
pub struct FRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAME_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(FRAME_AW::LP)
    }
    #[doc = "FRAME interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(FRAME_AW::HP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Fast endpoint interrupt routing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP_FAST_AW {
    #[doc = "0: EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    LP = 0,
    #[doc = "1: EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    HP = 1,
}
impl From<EP_FAST_AW> for bool {
    #[inline(always)]
    fn from(variant: EP_FAST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `EP_FAST`"]
pub struct EP_FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_FAST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP_FAST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_LP."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(EP_FAST_AW::LP)
    }
    #[doc = "EP_FAST interrupt is routed to USB_INT_REQ_HP."]
    #[inline(always)]
    pub fn hp(self) -> &'a mut W {
        self.variant(EP_FAST_AW::HP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Frame interrupt routing"]
    #[inline(always)]
    pub fn frame(&mut self) -> FRAME_W {
        FRAME_W { w: self }
    }
    #[doc = "Bit 1 - Fast endpoint interrupt routing"]
    #[inline(always)]
    pub fn ep_fast(&mut self) -> EP_FAST_W {
        EP_FAST_W { w: self }
    }
}
