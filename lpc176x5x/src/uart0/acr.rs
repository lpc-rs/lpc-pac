#[doc = "Reader of register ACR"]
pub type R = crate::R<u32, super::ACR>;
#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Register ACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start bit. This bit is automatically cleared after auto-baud completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum START_A {
    #[doc = "0: Auto-baud stop (auto-baud is not running)."]
    AUTO_BAUD_STOP_AUTO = 0,
    #[doc = "1: Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    AUTO_BAUD_START_AUT = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, START_A>;
impl START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::AUTO_BAUD_STOP_AUTO,
            true => START_A::AUTO_BAUD_START_AUT,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_BAUD_STOP_AUTO`"]
    #[inline(always)]
    pub fn is_auto_baud_stop_auto(&self) -> bool {
        *self == START_A::AUTO_BAUD_STOP_AUTO
    }
    #[doc = "Checks if the value of the field is `AUTO_BAUD_START_AUT`"]
    #[inline(always)]
    pub fn is_auto_baud_start_aut(&self) -> bool {
        *self == START_A::AUTO_BAUD_START_AUT
    }
}
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn auto_baud_stop_auto(self) -> &'a mut W {
        self.variant(START_A::AUTO_BAUD_STOP_AUTO)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn auto_baud_start_aut(self) -> &'a mut W {
        self.variant(START_A::AUTO_BAUD_START_AUT)
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
#[doc = "Auto-baud mode select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Mode 0."]
    MODE_0_ = 0,
    #[doc = "1: Mode 1."]
    MODE_1_ = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0_,
            true => MODE_A::MODE_1_,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0_`"]
    #[inline(always)]
    pub fn is_mode_0_(&self) -> bool {
        *self == MODE_A::MODE_0_
    }
    #[doc = "Checks if the value of the field is `MODE_1_`"]
    #[inline(always)]
    pub fn is_mode_1_(&self) -> bool {
        *self == MODE_A::MODE_1_
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn mode_0_(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0_)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode_1_(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1_)
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
#[doc = "Restart bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTORESTART_A {
    #[doc = "0: No restart."]
    NO_RESTART_ = 0,
    #[doc = "1: Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    RESTART_IN_CASE_OF_T = 1,
}
impl From<AUTORESTART_A> for bool {
    #[inline(always)]
    fn from(variant: AUTORESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTORESTART`"]
pub type AUTORESTART_R = crate::R<bool, AUTORESTART_A>;
impl AUTORESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTORESTART_A {
        match self.bits {
            false => AUTORESTART_A::NO_RESTART_,
            true => AUTORESTART_A::RESTART_IN_CASE_OF_T,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESTART_`"]
    #[inline(always)]
    pub fn is_no_restart_(&self) -> bool {
        *self == AUTORESTART_A::NO_RESTART_
    }
    #[doc = "Checks if the value of the field is `RESTART_IN_CASE_OF_T`"]
    #[inline(always)]
    pub fn is_restart_in_case_of_t(&self) -> bool {
        *self == AUTORESTART_A::RESTART_IN_CASE_OF_T
    }
}
#[doc = "Write proxy for field `AUTORESTART`"]
pub struct AUTORESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTORESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTORESTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No restart."]
    #[inline(always)]
    pub fn no_restart_(self) -> &'a mut W {
        self.variant(AUTORESTART_A::NO_RESTART_)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline(always)]
    pub fn restart_in_case_of_t(self) -> &'a mut W {
        self.variant(AUTORESTART_A::RESTART_IN_CASE_OF_T)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTCLR_A {
    #[doc = "0: No impact."]
    NO_IMPACT_ = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND = 1,
}
impl From<ABEOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABEOINTCLR`"]
pub type ABEOINTCLR_R = crate::R<bool, ABEOINTCLR_A>;
impl ABEOINTCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTCLR_A {
        match self.bits {
            false => ABEOINTCLR_A::NO_IMPACT_,
            true => ABEOINTCLR_A::CLEAR_THE_CORRESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT_`"]
    #[inline(always)]
    pub fn is_no_impact_(&self) -> bool {
        *self == ABEOINTCLR_A::NO_IMPACT_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == ABEOINTCLR_A::CLEAR_THE_CORRESPOND
    }
}
#[doc = "Write proxy for field `ABEOINTCLR`"]
pub struct ABEOINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABEOINTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_impact_(self) -> &'a mut W {
        self.variant(ABEOINTCLR_A::NO_IMPACT_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(ABEOINTCLR_A::CLEAR_THE_CORRESPOND)
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
#[doc = "Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTCLR_A {
    #[doc = "0: No impact."]
    NO_IMPACT_ = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    CLEAR_THE_CORRESPOND = 1,
}
impl From<ABTOINTCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABTOINTCLR`"]
pub type ABTOINTCLR_R = crate::R<bool, ABTOINTCLR_A>;
impl ABTOINTCLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTCLR_A {
        match self.bits {
            false => ABTOINTCLR_A::NO_IMPACT_,
            true => ABTOINTCLR_A::CLEAR_THE_CORRESPOND,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMPACT_`"]
    #[inline(always)]
    pub fn is_no_impact_(&self) -> bool {
        *self == ABTOINTCLR_A::NO_IMPACT_
    }
    #[doc = "Checks if the value of the field is `CLEAR_THE_CORRESPOND`"]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == ABTOINTCLR_A::CLEAR_THE_CORRESPOND
    }
}
#[doc = "Write proxy for field `ABTOINTCLR`"]
pub struct ABTOINTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTOINTCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTCLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_impact_(self) -> &'a mut W {
        self.variant(ABTOINTCLR_A::NO_IMPACT_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut W {
        self.variant(ABTOINTCLR_A::CLEAR_THE_CORRESPOND)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    pub fn autorestart(&self) -> AUTORESTART_R {
        AUTORESTART_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abeointclr(&self) -> ABEOINTCLR_R {
        ABEOINTCLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abtointclr(&self) -> ABTOINTCLR_R {
        ABTOINTCLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    pub fn autorestart(&mut self) -> AUTORESTART_W {
        AUTORESTART_W { w: self }
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abeointclr(&mut self) -> ABEOINTCLR_W {
        ABEOINTCLR_W { w: self }
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abtointclr(&mut self) -> ABTOINTCLR_W {
        ABTOINTCLR_W { w: self }
    }
}
