#[doc = "Reader of register BODCTRL"]
pub type R = crate::R<u32, super::BODCTRL>;
#[doc = "Writer for register BODCTRL"]
pub type W = crate::W<u32, super::BODCTRL>;
#[doc = "Register BODCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BODCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "BOD reset level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODRSTLEV_A {
    #[doc = "0: Level 0: 1.5 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 1.85 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.0 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 2.3 V"]
    LEVEL3 = 3,
}
impl From<BODRSTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODRSTLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BODRSTLEV`"]
pub type BODRSTLEV_R = crate::R<u8, BODRSTLEV_A>;
impl BODRSTLEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTLEV_A {
        match self.bits {
            0 => BODRSTLEV_A::LEVEL0,
            1 => BODRSTLEV_A::LEVEL1,
            2 => BODRSTLEV_A::LEVEL2,
            3 => BODRSTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL3
    }
}
#[doc = "Write proxy for field `BODRSTLEV`"]
pub struct BODRSTLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTLEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level 0: 1.5 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 1.85 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.0 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 2.3 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE = 0,
    #[doc = "1: Enable reset function."]
    ENABLE = 1,
}
impl From<BODRSTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODRSTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BODRSTENA`"]
pub type BODRSTENA_R = crate::R<bool, BODRSTENA_A>;
impl BODRSTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODRSTENA_A {
        match self.bits {
            false => BODRSTENA_A::DISABLE,
            true => BODRSTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODRSTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODRSTENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `BODRSTENA`"]
pub struct BODRSTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODRSTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable reset function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE)
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
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTLEV_A {
    #[doc = "0: Level 0: 2.05 V"]
    LEVEL0 = 0,
    #[doc = "1: Level 1: 2.45 V"]
    LEVEL1 = 1,
    #[doc = "2: Level 2: 2.75 V"]
    LEVEL2 = 2,
    #[doc = "3: Level 3: 3.05 V"]
    LEVEL3 = 3,
}
impl From<BODINTLEV_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTLEV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BODINTLEV`"]
pub type BODINTLEV_R = crate::R<u8, BODINTLEV_A>;
impl BODINTLEV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTLEV_A {
        match self.bits {
            0 => BODINTLEV_A::LEVEL0,
            1 => BODINTLEV_A::LEVEL1,
            2 => BODINTLEV_A::LEVEL2,
            3 => BODINTLEV_A::LEVEL3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL0`"]
    #[inline(always)]
    pub fn is_level0(&self) -> bool {
        *self == BODINTLEV_A::LEVEL0
    }
    #[doc = "Checks if the value of the field is `LEVEL1`"]
    #[inline(always)]
    pub fn is_level1(&self) -> bool {
        *self == BODINTLEV_A::LEVEL1
    }
    #[doc = "Checks if the value of the field is `LEVEL2`"]
    #[inline(always)]
    pub fn is_level2(&self) -> bool {
        *self == BODINTLEV_A::LEVEL2
    }
    #[doc = "Checks if the value of the field is `LEVEL3`"]
    #[inline(always)]
    pub fn is_level3(&self) -> bool {
        *self == BODINTLEV_A::LEVEL3
    }
}
#[doc = "Write proxy for field `BODINTLEV`"]
pub struct BODINTLEV_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTLEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTLEV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level 0: 2.05 V"]
    #[inline(always)]
    pub fn level0(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL0)
    }
    #[doc = "Level 1: 2.45 V"]
    #[inline(always)]
    pub fn level1(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL1)
    }
    #[doc = "Level 2: 2.75 V"]
    #[inline(always)]
    pub fn level2(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL2)
    }
    #[doc = "Level 3: 3.05 V"]
    #[inline(always)]
    pub fn level3(self) -> &'a mut W {
        self.variant(BODINTLEV_A::LEVEL3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "BOD interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODINTENA_A {
    #[doc = "0: Disable interrupt function."]
    DISABLE = 0,
    #[doc = "1: Enable interrupt function."]
    ENABLE = 1,
}
impl From<BODINTENA_A> for bool {
    #[inline(always)]
    fn from(variant: BODINTENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BODINTENA`"]
pub type BODINTENA_R = crate::R<bool, BODINTENA_A>;
impl BODINTENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BODINTENA_A {
        match self.bits {
            false => BODINTENA_A::DISABLE,
            true => BODINTENA_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BODINTENA_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BODINTENA_A::ENABLE
    }
}
#[doc = "Write proxy for field `BODINTENA`"]
pub struct BODINTENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BODINTENA_A::DISABLE)
    }
    #[doc = "Enable interrupt function."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BODINTENA_A::ENABLE)
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
#[doc = "Reader of field `BODRSTSTAT`"]
pub type BODRSTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODRSTSTAT`"]
pub struct BODRSTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRSTSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BODINTSTAT`"]
pub type BODINTSTAT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BODINTSTAT`"]
pub struct BODINTSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTSTAT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&self) -> BODINTLEV_R {
        BODINTLEV_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&self) -> BODINTENA_R {
        BODINTENA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&self) -> BODRSTSTAT_R {
        BODRSTSTAT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&self) -> BODINTSTAT_R {
        BODINTSTAT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W {
        BODRSTLEV_W { w: self }
    }
    #[doc = "Bit 2 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W {
        BODRSTENA_W { w: self }
    }
    #[doc = "Bits 3:4 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintlev(&mut self) -> BODINTLEV_W {
        BODINTLEV_W { w: self }
    }
    #[doc = "Bit 5 - BOD interrupt enable"]
    #[inline(always)]
    pub fn bodintena(&mut self) -> BODINTENA_W {
        BODINTENA_W { w: self }
    }
    #[doc = "Bit 6 - BOD reset status. When 1, a BOD reset has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodrststat(&mut self) -> BODRSTSTAT_W {
        BODRSTSTAT_W { w: self }
    }
    #[doc = "Bit 7 - BOD interrupt status. When 1, a BOD interrupt has occurred. Cleared by writing 1 to this bit."]
    #[inline(always)]
    pub fn bodintstat(&mut self) -> BODINTSTAT_W {
        BODINTSTAT_W { w: self }
    }
}
