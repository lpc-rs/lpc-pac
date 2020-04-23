#[doc = "Reader of register STAT"]
pub type R = crate::R<u32, super::STAT>;
#[doc = "Writer for register STAT"]
pub type W = crate::W<u32, super::STAT>;
#[doc = "Register STAT `reset()`'s with value 0"]
impl crate::ResetValue for super::STAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Monitors the interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTFLAG_A {
    #[doc = "0: No pending interrupt. Writing a zero is equivalent to no operation."]
    NO_PENDING_INTERRUPT = 0,
    #[doc = "1: Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    PENDING_INTERRUPT = 1,
}
impl From<INTFLAG_A> for bool {
    #[inline(always)]
    fn from(variant: INTFLAG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INTFLAG`"]
pub type INTFLAG_R = crate::R<bool, INTFLAG_A>;
impl INTFLAG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTFLAG_A {
        match self.bits {
            false => INTFLAG_A::NO_PENDING_INTERRUPT,
            true => INTFLAG_A::PENDING_INTERRUPT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_no_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::NO_PENDING_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PENDING_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pending_interrupt(&self) -> bool {
        *self == INTFLAG_A::PENDING_INTERRUPT
    }
}
#[doc = "Write proxy for field `INTFLAG`"]
pub struct INTFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> INTFLAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTFLAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No pending interrupt. Writing a zero is equivalent to no operation."]
    #[inline(always)]
    pub fn no_pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::NO_PENDING_INTERRUPT)
    }
    #[doc = "Pending interrupt. The interrupt is pending because TIMERn has reached the end of the time interval. If the INTEN bit in the CONTROLn is also set to 1, the interrupt for timer channel n and the global interrupt are raised. Writing a 1 to this bit clears the interrupt request."]
    #[inline(always)]
    pub fn pending_interrupt(self) -> &'a mut W {
        self.variant(INTFLAG_A::PENDING_INTERRUPT)
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
#[doc = "Indicates the state of TIMERn. This bit is read-only.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUN_A {
    #[doc = "0: Idle state. TIMERn is stopped."]
    IDLE_STATE = 0,
    #[doc = "1: Running. TIMERn is running."]
    RUNNING = 1,
}
impl From<RUN_A> for bool {
    #[inline(always)]
    fn from(variant: RUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RUN`"]
pub type RUN_R = crate::R<bool, RUN_A>;
impl RUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUN_A {
        match self.bits {
            false => RUN_A::IDLE_STATE,
            true => RUN_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_STATE`"]
    #[inline(always)]
    pub fn is_idle_state(&self) -> bool {
        *self == RUN_A::IDLE_STATE
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUN_A::RUNNING
    }
}
#[doc = "Write proxy for field `RUN`"]
pub struct RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle state. TIMERn is stopped."]
    #[inline(always)]
    pub fn idle_state(self) -> &'a mut W {
        self.variant(RUN_A::IDLE_STATE)
    }
    #[doc = "Running. TIMERn is running."]
    #[inline(always)]
    pub fn running(self) -> &'a mut W {
        self.variant(RUN_A::RUNNING)
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
#[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INUSE_A {
    #[doc = "0: This channel is not in use."]
    NO = 0,
    #[doc = "1: This channel is in use."]
    YES = 1,
}
impl From<INUSE_A> for bool {
    #[inline(always)]
    fn from(variant: INUSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INUSE`"]
pub type INUSE_R = crate::R<bool, INUSE_A>;
impl INUSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INUSE_A {
        match self.bits {
            false => INUSE_A::NO,
            true => INUSE_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == INUSE_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == INUSE_A::YES
    }
}
#[doc = "Write proxy for field `INUSE`"]
pub struct INUSE_W<'a> {
    w: &'a mut W,
}
impl<'a> INUSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INUSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "This channel is not in use."]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(INUSE_A::NO)
    }
    #[doc = "This channel is in use."]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(INUSE_A::YES)
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
impl R {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&self) -> INTFLAG_R {
        INTFLAG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn inuse(&self) -> INUSE_R {
        INUSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitors the interrupt flag."]
    #[inline(always)]
    pub fn intflag(&mut self) -> INTFLAG_W {
        INTFLAG_W { w: self }
    }
    #[doc = "Bit 1 - Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W {
        RUN_W { w: self }
    }
    #[doc = "Bit 2 - Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn inuse(&mut self) -> INUSE_W {
        INUSE_W { w: self }
    }
}
