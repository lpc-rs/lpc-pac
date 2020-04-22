#[doc = "Reader of register PCON"]
pub type R = crate::R<u32, super::PCON>;
#[doc = "Writer for register PCON"]
pub type W = crate::W<u32, super::PCON>;
#[doc = "Register PCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PM_A {
    #[doc = "0: Default. The part is in active or sleep mode."]
    DEFAULT = 0,
    #[doc = "1: ARM WFI will enter Deep-sleep mode."]
    DEEPSLEEP = 1,
    #[doc = "2: ARM WFI will enter Power-down mode."]
    POWERDOWN = 2,
    #[doc = "3: ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    DEEPPOWERDOWN = 3,
}
impl From<PM_A> for u8 {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PM`"]
pub type PM_R = crate::R<u8, PM_A>;
impl PM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PM_A::DEFAULT),
            1 => Val(PM_A::DEEPSLEEP),
            2 => Val(PM_A::POWERDOWN),
            3 => Val(PM_A::DEEPPOWERDOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == PM_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `DEEPSLEEP`"]
    #[inline(always)]
    pub fn is_deepsleep(&self) -> bool {
        *self == PM_A::DEEPSLEEP
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == PM_A::POWERDOWN
    }
    #[doc = "Checks if the value of the field is `DEEPPOWERDOWN`"]
    #[inline(always)]
    pub fn is_deeppowerdown(&self) -> bool {
        *self == PM_A::DEEPPOWERDOWN
    }
}
#[doc = "Write proxy for field `PM`"]
pub struct PM_W<'a> {
    w: &'a mut W,
}
impl<'a> PM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Default. The part is in active or sleep mode."]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(PM_A::DEFAULT)
    }
    #[doc = "ARM WFI will enter Deep-sleep mode."]
    #[inline(always)]
    pub fn deepsleep(self) -> &'a mut W {
        self.variant(PM_A::DEEPSLEEP)
    }
    #[doc = "ARM WFI will enter Power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(PM_A::POWERDOWN)
    }
    #[doc = "ARM WFI will enter Deep-power down mode (ARM Cortex-M0 core powered-down)."]
    #[inline(always)]
    pub fn deeppowerdown(self) -> &'a mut W {
        self.variant(PM_A::DEEPPOWERDOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `NODPD`"]
pub type NODPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NODPD`"]
pub struct NODPD_W<'a> {
    w: &'a mut W,
}
impl<'a> NODPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Sleep mode flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEPFLAG_A {
    #[doc = "0: Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    NOPOWERDOWN = 0,
    #[doc = "1: Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    POWERDOWN = 1,
}
impl From<SLEEPFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SLEEPFLAG`"]
pub type SLEEPFLAG_R = crate::R<bool, SLEEPFLAG_A>;
impl SLEEPFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPFLAG_A {
        match self.bits {
            false => SLEEPFLAG_A::NOPOWERDOWN,
            true => SLEEPFLAG_A::POWERDOWN,
        }
    }
    #[doc = "Checks if the value of the field is `NOPOWERDOWN`"]
    #[inline(always)]
    pub fn is_nopowerdown(&self) -> bool {
        *self == SLEEPFLAG_A::NOPOWERDOWN
    }
    #[doc = "Checks if the value of the field is `POWERDOWN`"]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == SLEEPFLAG_A::POWERDOWN
    }
}
#[doc = "Write proxy for field `SLEEPFLAG`"]
pub struct SLEEPFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEPFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEPFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: No power-down mode entered. LPC11U1x is in Active mode. Write: No effect."]
    #[inline(always)]
    pub fn nopowerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::NOPOWERDOWN)
    }
    #[doc = "Read: Sleep/Deep-sleep or Deep power-down mode entered. Write: Writing a 1 clears the SLEEPFLAG bit to 0."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut W {
        self.variant(SLEEPFLAG_A::POWERDOWN)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Deep power-down flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPDFLAG_A {
    #[doc = "0: Read: Deep power-down mode  not entered. Write: No effect."]
    DPNOTENTERED = 0,
    #[doc = "1: Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    DPENTERED = 1,
}
impl From<DPDFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: DPDFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DPDFLAG`"]
pub type DPDFLAG_R = crate::R<bool, DPDFLAG_A>;
impl DPDFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPDFLAG_A {
        match self.bits {
            false => DPDFLAG_A::DPNOTENTERED,
            true => DPDFLAG_A::DPENTERED,
        }
    }
    #[doc = "Checks if the value of the field is `DPNOTENTERED`"]
    #[inline(always)]
    pub fn is_dpnotentered(&self) -> bool {
        *self == DPDFLAG_A::DPNOTENTERED
    }
    #[doc = "Checks if the value of the field is `DPENTERED`"]
    #[inline(always)]
    pub fn is_dpentered(&self) -> bool {
        *self == DPDFLAG_A::DPENTERED
    }
}
#[doc = "Write proxy for field `DPDFLAG`"]
pub struct DPDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DPDFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: Deep power-down mode not entered. Write: No effect."]
    #[inline(always)]
    pub fn dpnotentered(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DPNOTENTERED)
    }
    #[doc = "Read: Deep power-down mode entered. Write: Clear the Deep power-down flag."]
    #[inline(always)]
    pub fn dpentered(self) -> &'a mut W {
        self.variant(DPDFLAG_A::DPENTERED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&self) -> NODPD_R {
        NODPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&self) -> SLEEPFLAG_R {
        SLEEPFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Power mode"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W {
        PM_W { w: self }
    }
    #[doc = "Bit 3 - A 1 in this bit prevents entry to Deep power-down mode when 0x3 is written to the PM field above, the SLEEPDEEP bit is set, and a WFI is executed. Execution continues after the WFI if this bit is 1. This bit is cleared only by power-on reset, so writing a one to this bit locks the part in a mode in which Deep power-down mode is blocked."]
    #[inline(always)]
    pub fn nodpd(&mut self) -> NODPD_W {
        NODPD_W { w: self }
    }
    #[doc = "Bit 8 - Sleep mode flag"]
    #[inline(always)]
    pub fn sleepflag(&mut self) -> SLEEPFLAG_W {
        SLEEPFLAG_W { w: self }
    }
    #[doc = "Bit 11 - Deep power-down flag"]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W {
        DPDFLAG_W { w: self }
    }
}
