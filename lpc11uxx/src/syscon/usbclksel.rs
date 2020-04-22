#[doc = "Reader of register USBCLKSEL"]
pub type R = crate::R<u32, super::USBCLKSEL>;
#[doc = "Writer for register USBCLKSEL"]
pub type W = crate::W<u32, super::USBCLKSEL>;
#[doc = "Register USBCLKSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB clock source. Values 0x2 and 0x3 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEL_A {
    #[doc = "0: USB PLL out"]
    USB_PLL_OUT = 0,
    #[doc = "1: Main clock"]
    MAIN_CLOCK = 1,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SEL_A::USB_PLL_OUT),
            1 => Val(SEL_A::MAIN_CLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_PLL_OUT`"]
    #[inline(always)]
    pub fn is_usb_pll_out(&self) -> bool {
        *self == SEL_A::USB_PLL_OUT
    }
    #[doc = "Checks if the value of the field is `MAIN_CLOCK`"]
    #[inline(always)]
    pub fn is_main_clock(&self) -> bool {
        *self == SEL_A::MAIN_CLOCK
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USB PLL out"]
    #[inline(always)]
    pub fn usb_pll_out(self) -> &'a mut W {
        self.variant(SEL_A::USB_PLL_OUT)
    }
    #[doc = "Main clock"]
    #[inline(always)]
    pub fn main_clock(self) -> &'a mut W {
        self.variant(SEL_A::MAIN_CLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USB clock source. Values 0x2 and 0x3 are reserved."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
