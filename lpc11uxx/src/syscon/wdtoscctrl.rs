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
#[doc = "Select watchdog oscillator analog output frequency (Fclkana).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FREQSEL_A {
    #[doc = "1: 0.6 MHz"]
    _0_6_MHZ = 1,
    #[doc = "2: 1.05 MHz"]
    _1_05_MHZ = 2,
    #[doc = "3: 1.4 MHz"]
    _1_4_MHZ = 3,
    #[doc = "4: 1.75 MHz"]
    _1_75_MHZ = 4,
    #[doc = "5: 2.1 MHz"]
    _2_1_MHZ = 5,
    #[doc = "6: 2.4 MHz"]
    _2_4_MHZ = 6,
    #[doc = "7: 2.7 MHz"]
    _2_7_MHZ = 7,
    #[doc = "8: 3.0 MHz"]
    _3_0_MHZ = 8,
    #[doc = "9: 3.25 MHz"]
    _3_25_MHZ = 9,
    #[doc = "10: 3.5 MHz"]
    _3_5_MHZ = 10,
    #[doc = "11: 3.75 MHz"]
    _3_75_MHZ = 11,
    #[doc = "12: 4.0 MHz"]
    _4_0_MHZ = 12,
    #[doc = "13: 4.2 MHz"]
    _4_2_MHZ = 13,
    #[doc = "14: 4.4 MHz"]
    _4_4_MHZ = 14,
    #[doc = "15: 4.6 MHz"]
    _4_6_MHZ = 15,
}
impl From<FREQSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FREQSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQSEL`"]
pub type FREQSEL_R = crate::R<u8, FREQSEL_A>;
impl FREQSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FREQSEL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(FREQSEL_A::_0_6_MHZ),
            2 => Val(FREQSEL_A::_1_05_MHZ),
            3 => Val(FREQSEL_A::_1_4_MHZ),
            4 => Val(FREQSEL_A::_1_75_MHZ),
            5 => Val(FREQSEL_A::_2_1_MHZ),
            6 => Val(FREQSEL_A::_2_4_MHZ),
            7 => Val(FREQSEL_A::_2_7_MHZ),
            8 => Val(FREQSEL_A::_3_0_MHZ),
            9 => Val(FREQSEL_A::_3_25_MHZ),
            10 => Val(FREQSEL_A::_3_5_MHZ),
            11 => Val(FREQSEL_A::_3_75_MHZ),
            12 => Val(FREQSEL_A::_4_0_MHZ),
            13 => Val(FREQSEL_A::_4_2_MHZ),
            14 => Val(FREQSEL_A::_4_4_MHZ),
            15 => Val(FREQSEL_A::_4_6_MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0_6_MHZ`"]
    #[inline(always)]
    pub fn is_0_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_0_6_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_05_MHZ`"]
    #[inline(always)]
    pub fn is_1_05_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_05_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_4_MHZ`"]
    #[inline(always)]
    pub fn is_1_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_1_75_MHZ`"]
    #[inline(always)]
    pub fn is_1_75_mhz(&self) -> bool {
        *self == FREQSEL_A::_1_75_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_1_MHZ`"]
    #[inline(always)]
    pub fn is_2_1_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_1_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_4_MHZ`"]
    #[inline(always)]
    pub fn is_2_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_2_7_MHZ`"]
    #[inline(always)]
    pub fn is_2_7_mhz(&self) -> bool {
        *self == FREQSEL_A::_2_7_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_0_MHZ`"]
    #[inline(always)]
    pub fn is_3_0_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_25_MHZ`"]
    #[inline(always)]
    pub fn is_3_25_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_25_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_5_MHZ`"]
    #[inline(always)]
    pub fn is_3_5_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_5_MHZ
    }
    #[doc = "Checks if the value of the field is `_3_75_MHZ`"]
    #[inline(always)]
    pub fn is_3_75_mhz(&self) -> bool {
        *self == FREQSEL_A::_3_75_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_0_MHZ`"]
    #[inline(always)]
    pub fn is_4_0_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_0_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_2_MHZ`"]
    #[inline(always)]
    pub fn is_4_2_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_2_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_4_MHZ`"]
    #[inline(always)]
    pub fn is_4_4_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_4_MHZ
    }
    #[doc = "Checks if the value of the field is `_4_6_MHZ`"]
    #[inline(always)]
    pub fn is_4_6_mhz(&self) -> bool {
        *self == FREQSEL_A::_4_6_MHZ
    }
}
#[doc = "Write proxy for field `FREQSEL`"]
pub struct FREQSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0.6 MHz"]
    #[inline(always)]
    pub fn _0_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_0_6_MHZ)
    }
    #[doc = "1.05 MHz"]
    #[inline(always)]
    pub fn _1_05_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_05_MHZ)
    }
    #[doc = "1.4 MHz"]
    #[inline(always)]
    pub fn _1_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_4_MHZ)
    }
    #[doc = "1.75 MHz"]
    #[inline(always)]
    pub fn _1_75_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_1_75_MHZ)
    }
    #[doc = "2.1 MHz"]
    #[inline(always)]
    pub fn _2_1_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_1_MHZ)
    }
    #[doc = "2.4 MHz"]
    #[inline(always)]
    pub fn _2_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_4_MHZ)
    }
    #[doc = "2.7 MHz"]
    #[inline(always)]
    pub fn _2_7_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_2_7_MHZ)
    }
    #[doc = "3.0 MHz"]
    #[inline(always)]
    pub fn _3_0_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_0_MHZ)
    }
    #[doc = "3.25 MHz"]
    #[inline(always)]
    pub fn _3_25_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_25_MHZ)
    }
    #[doc = "3.5 MHz"]
    #[inline(always)]
    pub fn _3_5_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_5_MHZ)
    }
    #[doc = "3.75 MHz"]
    #[inline(always)]
    pub fn _3_75_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_3_75_MHZ)
    }
    #[doc = "4.0 MHz"]
    #[inline(always)]
    pub fn _4_0_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_0_MHZ)
    }
    #[doc = "4.2 MHz"]
    #[inline(always)]
    pub fn _4_2_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_2_MHZ)
    }
    #[doc = "4.4 MHz"]
    #[inline(always)]
    pub fn _4_4_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_4_MHZ)
    }
    #[doc = "4.6 MHz"]
    #[inline(always)]
    pub fn _4_6_mhz(self) -> &'a mut W {
        self.variant(FREQSEL_A::_4_6_MHZ)
    }
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
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
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
    #[doc = "Bits 5:8 - Select watchdog oscillator analog output frequency (Fclkana)."]
    #[inline(always)]
    pub fn freqsel(&mut self) -> FREQSEL_W {
        FREQSEL_W { w: self }
    }
}
