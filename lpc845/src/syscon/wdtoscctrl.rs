#[doc = "Reader of register WDTOSCCTRL"]
pub type R = crate::R<u32, super::WDTOSCCTRL>;
#[doc = "Writer for register WDTOSCCTRL"]
pub type W = crate::W<u32, super::WDTOSCCTRL>;
#[doc = "Register WDTOSCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::WDTOSCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !(0x0f << 5)) | (((value as u32) & 0x0f) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    pub fn divsel(&self) -> DIVSEL_R {
        DIVSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - Frequency select. Selects the frequency of the oscillator. 0x00 = invalid setting when watchdog oscillator is running 0x1 = 0.6 MHz 0x2 = 1.05 MHz 0x3 = 1.4 MHz 0x4 = 1.75 MHz 0x5 = 2.1 MHz 0x6 = 2.4 MHz 0x7 = 2.7 MHz 0x8 = 3.0 MHz 0x9 = 3.25 MHz 0xA = 3.5 MHz 0xB = 3.75 MHz 0xC = 4.0 MHz 0xD = 4.2 MHz 0xE = 4.4 MHz 0xF = 4.6 MHz"]
    #[inline(always)]
    pub fn freqsel(&self) -> FREQSEL_R {
        FREQSEL_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select divider for Fclkana. wdt_osc_clk = Fclkana/ (2 x (1 + DIVSEL)) 00000: 2 x (1 + DIVSEL) = 2 00001: 2 x (1 + DIVSEL) = 4 to 11111: 2 x (1 + DIVSEL) = 64"]
    #[inline(always)]
    pub fn divsel(&mut self) -> DIVSEL_W {
        DIVSEL_W { w: self }
    }
    #[doc = "Bits 5:8 - Frequency select. Selects the frequency of the oscillator. 0x00 = invalid setting when watchdog oscillator is running 0x1 = 0.6 MHz 0x2 = 1.05 MHz 0x3 = 1.4 MHz 0x4 = 1.75 MHz 0x5 = 2.1 MHz 0x6 = 2.4 MHz 0x7 = 2.7 MHz 0x8 = 3.0 MHz 0x9 = 3.25 MHz 0xA = 3.5 MHz 0xB = 3.75 MHz 0xC = 4.0 MHz 0xD = 4.2 MHz 0xE = 4.4 MHz 0xF = 4.6 MHz"]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
}
