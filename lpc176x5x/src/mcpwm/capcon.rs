#[doc = "Reader of register CAPCON"]
pub type R = crate::R<u32, super::CAPCON>;
#[doc = "Reader of field `CAP0MCI0_RE`"]
pub type CAP0MCI0_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP0MCI0_FE`"]
pub type CAP0MCI0_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP0MCI1_RE`"]
pub type CAP0MCI1_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP0MCI1_FE`"]
pub type CAP0MCI1_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP0MCI2_RE`"]
pub type CAP0MCI2_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP0MCI2_FE`"]
pub type CAP0MCI2_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI0_RE`"]
pub type CAP1MCI0_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI0_FE`"]
pub type CAP1MCI0_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI1_RE`"]
pub type CAP1MCI1_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI1_FE`"]
pub type CAP1MCI1_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI2_RE`"]
pub type CAP1MCI2_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP1MCI2_FE`"]
pub type CAP1MCI2_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI0_RE`"]
pub type CAP2MCI0_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI0_FE`"]
pub type CAP2MCI0_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI1_RE`"]
pub type CAP2MCI1_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI1_FE`"]
pub type CAP2MCI1_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI2_RE`"]
pub type CAP2MCI2_RE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CAP2MCI2_FE`"]
pub type CAP2MCI2_FE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RT0`"]
pub type RT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `RT1`"]
pub type RT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `RT2`"]
pub type RT2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_re(&self) -> CAP0MCI0_RE_R {
        CAP0MCI0_RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_fe(&self) -> CAP0MCI0_FE_R {
        CAP0MCI0_FE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_re(&self) -> CAP0MCI1_RE_R {
        CAP0MCI1_RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_fe(&self) -> CAP0MCI1_FE_R {
        CAP0MCI1_FE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_re(&self) -> CAP0MCI2_RE_R {
        CAP0MCI2_RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_fe(&self) -> CAP0MCI2_FE_R {
        CAP0MCI2_FE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_re(&self) -> CAP1MCI0_RE_R {
        CAP1MCI0_RE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_fe(&self) -> CAP1MCI0_FE_R {
        CAP1MCI0_FE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_re(&self) -> CAP1MCI1_RE_R {
        CAP1MCI1_RE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_fe(&self) -> CAP1MCI1_FE_R {
        CAP1MCI1_FE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_re(&self) -> CAP1MCI2_RE_R {
        CAP1MCI2_RE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_fe(&self) -> CAP1MCI2_FE_R {
        CAP1MCI2_FE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_re(&self) -> CAP2MCI0_RE_R {
        CAP2MCI0_RE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_fe(&self) -> CAP2MCI0_FE_R {
        CAP2MCI0_FE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_re(&self) -> CAP2MCI1_RE_R {
        CAP2MCI1_RE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_fe(&self) -> CAP2MCI1_FE_R {
        CAP2MCI1_FE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_re(&self) -> CAP2MCI2_RE_R {
        CAP2MCI2_RE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_fe(&self) -> CAP2MCI2_FE_R {
        CAP2MCI2_FE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - If this bit is 1, TC0 is reset by a channel 0 capture event."]
    #[inline(always)]
    pub fn rt0(&self) -> RT0_R {
        RT0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - If this bit is 1, TC1 is reset by a channel 1 capture event."]
    #[inline(always)]
    pub fn rt1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - If this bit is 1, TC2 is reset by a channel 2 capture event."]
    #[inline(always)]
    pub fn rt2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
