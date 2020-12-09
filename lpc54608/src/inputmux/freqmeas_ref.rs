#[doc = "Reader of register FREQMEAS_REF"]
pub type R = crate::R<u32, super::FREQMEAS_REF>;
#[doc = "Writer for register FREQMEAS_REF"]
pub type W = crate::W<u32, super::FREQMEAS_REF>;
#[doc = "Register FREQMEAS_REF `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::FREQMEAS_REF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `CLKIN`"]
pub type CLKIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKIN`"]
pub struct CLKIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    pub fn clkin(&self) -> CLKIN_R {
        CLKIN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Clock source number (decimal value) for frequency measure function target clock: 0 = CLK_IN 1 = FRO 12 MHz oscillator 2 = Watchdog oscillator 3 = 32 kHz RTC oscillator 4 = Main clock (see Section 4.5.23) 5 = PIO0_4 6 = PIO0_20 7 = PIO0_24 8 = PIO1_4"]
    #[inline(always)]
    pub fn clkin(&mut self) -> CLKIN_W {
        CLKIN_W { w: self }
    }
}
