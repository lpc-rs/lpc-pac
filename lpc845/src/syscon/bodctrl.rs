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
    #[doc = "1: Level 1"]
    LEVEL_1 = 1,
    #[doc = "2: Level 2"]
    LEVEL_2 = 2,
    #[doc = "3: Level 3"]
    LEVEL_3 = 3,
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
    pub fn variant(&self) -> crate::Variant<u8, BODRSTLEV_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BODRSTLEV_A::LEVEL_1),
            2 => Val(BODRSTLEV_A::LEVEL_2),
            3 => Val(BODRSTLEV_A::LEVEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_3
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
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_1)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_2)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTVAL_A {
    #[doc = "1: Level 1"]
    LEVEL_1 = 1,
    #[doc = "2: Level 2"]
    LEVEL_2 = 2,
    #[doc = "3: Level 3"]
    LEVEL_3 = 3,
}
impl From<BODINTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: BODINTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BODINTVAL`"]
pub type BODINTVAL_R = crate::R<u8, BODINTVAL_A>;
impl BODINTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BODINTVAL_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(BODINTVAL_A::LEVEL_1),
            2 => Val(BODINTVAL_A::LEVEL_2),
            3 => Val(BODINTVAL_A::LEVEL_3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LEVEL_2`"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_2
    }
    #[doc = "Checks if the value of the field is `LEVEL_3`"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_3
    }
}
#[doc = "Write proxy for field `BODINTVAL`"]
pub struct BODINTVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> BODINTVAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BODINTVAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_1)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_2)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&self) -> BODRSTLEV_R {
        BODRSTLEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&self) -> BODINTVAL_R {
        BODINTVAL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&self) -> BODRSTENA_R {
        BODRSTENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD reset level"]
    #[inline(always)]
    pub fn bodrstlev(&mut self) -> BODRSTLEV_W {
        BODRSTLEV_W { w: self }
    }
    #[doc = "Bits 2:3 - BOD interrupt level"]
    #[inline(always)]
    pub fn bodintval(&mut self) -> BODINTVAL_W {
        BODINTVAL_W { w: self }
    }
    #[doc = "Bit 4 - BOD reset enable"]
    #[inline(always)]
    pub fn bodrstena(&mut self) -> BODRSTENA_W {
        BODRSTENA_W { w: self }
    }
}
