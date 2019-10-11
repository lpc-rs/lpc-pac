#[doc = "Reader of register USBCLKCFG"]
pub type R = crate::R<u32, super::USBCLKCFG>;
#[doc = "Writer for register USBCLKCFG"]
pub type W = crate::W<u32, super::USBCLKCFG>;
#[doc = "Register USBCLKCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCLKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `USBSEL`"]
pub type USBSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USBSEL`"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the divide value for creating the USB clock from the PLL0 output. Only the values shown below can produce even number multiples of 48 MHz from the PLL0 output. Warning: Improper setting of this value will result in incorrect operation of the USB interface. 5 = PLL0 output is divided by 6. PLL0 output must be 288 MHz. 7 = PLL0 output is divided by 8. PLL0 output must be 384 MHz. 9 = PLL0 output is divided by 10. PLL0 output must be 480 MHz."]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
}
