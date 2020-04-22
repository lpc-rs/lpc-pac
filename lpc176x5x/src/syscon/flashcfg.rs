#[doc = "Reader of register FLASHCFG"]
pub type R = crate::R<u32, super::FLASHCFG>;
#[doc = "Writer for register FLASHCFG"]
pub type W = crate::W<u32, super::FLASHCFG>;
#[doc = "Register FLASHCFG `reset()`'s with value 0x303a"]
impl crate::ResetValue for super::FLASHCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x303a
    }
}
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    _1CLK = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    _2CLK = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    _3CLK = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    _4CLK = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    _5CLK = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    _6CLK = 5,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASHTIM`"]
pub type FLASHTIM_R = crate::R<u8, FLASHTIM_A>;
impl FLASHTIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLASHTIM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLASHTIM_A::_1CLK),
            1 => Val(FLASHTIM_A::_2CLK),
            2 => Val(FLASHTIM_A::_3CLK),
            3 => Val(FLASHTIM_A::_4CLK),
            4 => Val(FLASHTIM_A::_5CLK),
            5 => Val(FLASHTIM_A::_6CLK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1CLK`"]
    #[inline(always)]
    pub fn is_1clk(&self) -> bool {
        *self == FLASHTIM_A::_1CLK
    }
    #[doc = "Checks if the value of the field is `_2CLK`"]
    #[inline(always)]
    pub fn is_2clk(&self) -> bool {
        *self == FLASHTIM_A::_2CLK
    }
    #[doc = "Checks if the value of the field is `_3CLK`"]
    #[inline(always)]
    pub fn is_3clk(&self) -> bool {
        *self == FLASHTIM_A::_3CLK
    }
    #[doc = "Checks if the value of the field is `_4CLK`"]
    #[inline(always)]
    pub fn is_4clk(&self) -> bool {
        *self == FLASHTIM_A::_4CLK
    }
    #[doc = "Checks if the value of the field is `_5CLK`"]
    #[inline(always)]
    pub fn is_5clk(&self) -> bool {
        *self == FLASHTIM_A::_5CLK
    }
    #[doc = "Checks if the value of the field is `_6CLK`"]
    #[inline(always)]
    pub fn is_6clk(&self) -> bool {
        *self == FLASHTIM_A::_6CLK
    }
}
#[doc = "Write proxy for field `FLASHTIM`"]
pub struct FLASHTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHTIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHTIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock."]
    #[inline(always)]
    pub fn _1clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_1CLK)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock."]
    #[inline(always)]
    pub fn _2clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_2CLK)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock."]
    #[inline(always)]
    pub fn _3clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_3CLK)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock."]
    #[inline(always)]
    pub fn _4clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_4CLK)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock. Use for up to 120 Mhz for LPC1759 and LPC1769 only."]
    #[inline(always)]
    pub fn _5clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_5CLK)
    }
    #[doc = "Flash accesses use 6 CPU clocks. This safe setting will work under any conditions."]
    #[inline(always)]
    pub fn _6clk(self) -> &'a mut W {
        self.variant(FLASHTIM_A::_6CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. Other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FLASHTIM_W {
        FLASHTIM_W { w: self }
    }
}
