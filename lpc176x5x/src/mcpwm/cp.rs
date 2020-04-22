#[doc = "Reader of register CP"]
pub type R = crate::R<u32, super::CP>;
#[doc = "Writer for register CP"]
pub type W = crate::W<u32, super::CP>;
#[doc = "Register CP `reset()`'s with value 0"]
impl crate::ResetValue for super::CP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Communication pattern output A, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA0_A {
    #[doc = "0: MCOA0 passive."]
    MCOA0_PASSIVE_ = 0,
    #[doc = "1: internal MCOA0."]
    INTERNAL_MCOA0_ = 1,
}
impl From<CCPA0_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPA0`"]
pub type CCPA0_R = crate::R<bool, CCPA0_A>;
impl CCPA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPA0_A {
        match self.bits {
            false => CCPA0_A::MCOA0_PASSIVE_,
            true => CCPA0_A::INTERNAL_MCOA0_,
        }
    }
    #[doc = "Checks if the value of the field is `MCOA0_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa0_passive_(&self) -> bool {
        *self == CCPA0_A::MCOA0_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `INTERNAL_MCOA0_`"]
    #[inline(always)]
    pub fn is_internal_mcoa0_(&self) -> bool {
        *self == CCPA0_A::INTERNAL_MCOA0_
    }
}
#[doc = "Write proxy for field `CCPA0`"]
pub struct CCPA0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn mcoa0_passive_(self) -> &'a mut W {
        self.variant(CCPA0_A::MCOA0_PASSIVE_)
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0_(self) -> &'a mut W {
        self.variant(CCPA0_A::INTERNAL_MCOA0_)
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
#[doc = "Communication pattern output B, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB0_A {
    #[doc = "0: MCOB0 passive."]
    MCOB0_PASSIVE_ = 0,
    #[doc = "1: MCOB0 tracks internal MCOA0."]
    MCOB0_TRACKS_INTERNA = 1,
}
impl From<CCPB0_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPB0`"]
pub type CCPB0_R = crate::R<bool, CCPB0_A>;
impl CCPB0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPB0_A {
        match self.bits {
            false => CCPB0_A::MCOB0_PASSIVE_,
            true => CCPB0_A::MCOB0_TRACKS_INTERNA,
        }
    }
    #[doc = "Checks if the value of the field is `MCOB0_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob0_passive_(&self) -> bool {
        *self == CCPB0_A::MCOB0_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB0_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob0_tracks_interna(&self) -> bool {
        *self == CCPB0_A::MCOB0_TRACKS_INTERNA
    }
}
#[doc = "Write proxy for field `CCPB0`"]
pub struct CCPB0_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn mcob0_passive_(self) -> &'a mut W {
        self.variant(CCPB0_A::MCOB0_PASSIVE_)
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob0_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB0_A::MCOB0_TRACKS_INTERNA)
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
#[doc = "Communication pattern output A, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA1_A {
    #[doc = "0: MCOA1 passive."]
    MCOA1_PASSIVE_ = 0,
    #[doc = "1: MCOA1 tracks internal MCOA0."]
    MCOA1_TRACKS_INTERNA = 1,
}
impl From<CCPA1_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPA1`"]
pub type CCPA1_R = crate::R<bool, CCPA1_A>;
impl CCPA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPA1_A {
        match self.bits {
            false => CCPA1_A::MCOA1_PASSIVE_,
            true => CCPA1_A::MCOA1_TRACKS_INTERNA,
        }
    }
    #[doc = "Checks if the value of the field is `MCOA1_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa1_passive_(&self) -> bool {
        *self == CCPA1_A::MCOA1_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOA1_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcoa1_tracks_interna(&self) -> bool {
        *self == CCPA1_A::MCOA1_TRACKS_INTERNA
    }
}
#[doc = "Write proxy for field `CCPA1`"]
pub struct CCPA1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn mcoa1_passive_(self) -> &'a mut W {
        self.variant(CCPA1_A::MCOA1_PASSIVE_)
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa1_tracks_interna(self) -> &'a mut W {
        self.variant(CCPA1_A::MCOA1_TRACKS_INTERNA)
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
#[doc = "Communication pattern output B, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB1_A {
    #[doc = "0: MCOB1 passive."]
    MCOB1_PASSIVE_ = 0,
    #[doc = "1: MCOB1 tracks internal MCOA0."]
    MCOB1_TRACKS_INTERNA = 1,
}
impl From<CCPB1_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPB1`"]
pub type CCPB1_R = crate::R<bool, CCPB1_A>;
impl CCPB1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPB1_A {
        match self.bits {
            false => CCPB1_A::MCOB1_PASSIVE_,
            true => CCPB1_A::MCOB1_TRACKS_INTERNA,
        }
    }
    #[doc = "Checks if the value of the field is `MCOB1_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob1_passive_(&self) -> bool {
        *self == CCPB1_A::MCOB1_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB1_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob1_tracks_interna(&self) -> bool {
        *self == CCPB1_A::MCOB1_TRACKS_INTERNA
    }
}
#[doc = "Write proxy for field `CCPB1`"]
pub struct CCPB1_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn mcob1_passive_(self) -> &'a mut W {
        self.variant(CCPB1_A::MCOB1_PASSIVE_)
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob1_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB1_A::MCOB1_TRACKS_INTERNA)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Communication pattern output A, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPA2_A {
    #[doc = "0: MCOA2 passive."]
    MCOA2_PASSIVE_ = 0,
    #[doc = "1: MCOA2 tracks internal MCOA0."]
    MCOA2_TRACKS_INTERNA = 1,
}
impl From<CCPA2_A> for bool {
    #[inline(always)]
    fn from(variant: CCPA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPA2`"]
pub type CCPA2_R = crate::R<bool, CCPA2_A>;
impl CCPA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPA2_A {
        match self.bits {
            false => CCPA2_A::MCOA2_PASSIVE_,
            true => CCPA2_A::MCOA2_TRACKS_INTERNA,
        }
    }
    #[doc = "Checks if the value of the field is `MCOA2_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcoa2_passive_(&self) -> bool {
        *self == CCPA2_A::MCOA2_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOA2_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcoa2_tracks_interna(&self) -> bool {
        *self == CCPA2_A::MCOA2_TRACKS_INTERNA
    }
}
#[doc = "Write proxy for field `CCPA2`"]
pub struct CCPA2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn mcoa2_passive_(self) -> &'a mut W {
        self.variant(CCPA2_A::MCOA2_PASSIVE_)
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa2_tracks_interna(self) -> &'a mut W {
        self.variant(CCPA2_A::MCOA2_TRACKS_INTERNA)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Communication pattern output B, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCPB2_A {
    #[doc = "0: MCOB2 passive."]
    MCOB2_PASSIVE_ = 0,
    #[doc = "1: MCOB2 tracks internal MCOA0."]
    MCOB2_TRACKS_INTERNA = 1,
}
impl From<CCPB2_A> for bool {
    #[inline(always)]
    fn from(variant: CCPB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCPB2`"]
pub type CCPB2_R = crate::R<bool, CCPB2_A>;
impl CCPB2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCPB2_A {
        match self.bits {
            false => CCPB2_A::MCOB2_PASSIVE_,
            true => CCPB2_A::MCOB2_TRACKS_INTERNA,
        }
    }
    #[doc = "Checks if the value of the field is `MCOB2_PASSIVE_`"]
    #[inline(always)]
    pub fn is_mcob2_passive_(&self) -> bool {
        *self == CCPB2_A::MCOB2_PASSIVE_
    }
    #[doc = "Checks if the value of the field is `MCOB2_TRACKS_INTERNA`"]
    #[inline(always)]
    pub fn is_mcob2_tracks_interna(&self) -> bool {
        *self == CCPB2_A::MCOB2_TRACKS_INTERNA
    }
}
#[doc = "Write proxy for field `CCPB2`"]
pub struct CCPB2_W<'a> {
    w: &'a mut W,
}
impl<'a> CCPB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCPB2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn mcob2_passive_(self) -> &'a mut W {
        self.variant(CCPB2_A::MCOB2_PASSIVE_)
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob2_tracks_interna(self) -> &'a mut W {
        self.variant(CCPB2_A::MCOB2_TRACKS_INTERNA)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&self) -> CCPA0_R {
        CCPA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&self) -> CCPB0_R {
        CCPB0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&self) -> CCPA1_R {
        CCPA1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&self) -> CCPB1_R {
        CCPB1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&self) -> CCPA2_R {
        CCPA2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&self) -> CCPB2_R {
        CCPB2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&mut self) -> CCPA0_W {
        CCPA0_W { w: self }
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&mut self) -> CCPB0_W {
        CCPB0_W { w: self }
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&mut self) -> CCPA1_W {
        CCPA1_W { w: self }
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&mut self) -> CCPB1_W {
        CCPB1_W { w: self }
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&mut self) -> CCPA2_W {
        CCPA2_W { w: self }
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&mut self) -> CCPB2_W {
        CCPB2_W { w: self }
    }
}
