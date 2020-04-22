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
    #[doc = "0: Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    LEVEL_0_THE_RESET_A = 0,
    #[doc = "1: Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    LEVEL_1_THE_RESET_A = 1,
    #[doc = "2: Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    LEVEL_2_THE_RESET_A = 2,
    #[doc = "3: Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    LEVEL_3_THE_RESET_A = 3,
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
            0 => BODRSTLEV_A::LEVEL_0_THE_RESET_A,
            1 => BODRSTLEV_A::LEVEL_1_THE_RESET_A,
            2 => BODRSTLEV_A::LEVEL_2_THE_RESET_A,
            3 => BODRSTLEV_A::LEVEL_3_THE_RESET_A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_THE_RESET_A`"]
    #[inline(always)]
    pub fn is_level_0_the_reset_a(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_0_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_1_THE_RESET_A`"]
    #[inline(always)]
    pub fn is_level_1_the_reset_a(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_1_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_THE_RESET_A`"]
    #[inline(always)]
    pub fn is_level_2_the_reset_a(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_2_THE_RESET_A
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_THE_RESET_A`"]
    #[inline(always)]
    pub fn is_level_3_the_reset_a(&self) -> bool {
        *self == BODRSTLEV_A::LEVEL_3_THE_RESET_A
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
    #[doc = "Level 0: The reset assertion threshold voltage is 1.46 V; the reset de-assertion threshold voltage is 1.63 V."]
    #[inline(always)]
    pub fn level_0_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_0_THE_RESET_A)
    }
    #[doc = "Level 1: The reset assertion threshold voltage is 2.06 V; the reset de-assertion threshold voltage is 2.15 V."]
    #[inline(always)]
    pub fn level_1_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_1_THE_RESET_A)
    }
    #[doc = "Level 2: The reset assertion threshold voltage is 2.35 V; the reset de-assertion threshold voltage is 2.43 V."]
    #[inline(always)]
    pub fn level_2_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_2_THE_RESET_A)
    }
    #[doc = "Level 3: The reset assertion threshold voltage is 2.63 V; the reset de-assertion threshold voltage is 2.71 V."]
    #[inline(always)]
    pub fn level_3_the_reset_a(self) -> &'a mut W {
        self.variant(BODRSTLEV_A::LEVEL_3_THE_RESET_A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "BOD interrupt level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BODINTVAL_A {
    #[doc = "0: Level 0: Reserved."]
    LEVEL_0_RESERVED = 0,
    #[doc = "1: Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    LEVEL_1THE_INTERRUP = 1,
    #[doc = "2: Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    LEVEL_2_THE_INTERRU = 2,
    #[doc = "3: Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    LEVEL_3_THE_INTERRU = 3,
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
    pub fn variant(&self) -> BODINTVAL_A {
        match self.bits {
            0 => BODINTVAL_A::LEVEL_0_RESERVED,
            1 => BODINTVAL_A::LEVEL_1THE_INTERRUP,
            2 => BODINTVAL_A::LEVEL_2_THE_INTERRU,
            3 => BODINTVAL_A::LEVEL_3_THE_INTERRU,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_0_RESERVED`"]
    #[inline(always)]
    pub fn is_level_0_reserved(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_0_RESERVED
    }
    #[doc = "Checks if the value of the field is `LEVEL_1THE_INTERRUP`"]
    #[inline(always)]
    pub fn is_level_1the_interrup(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_1THE_INTERRUP
    }
    #[doc = "Checks if the value of the field is `LEVEL_2_THE_INTERRU`"]
    #[inline(always)]
    pub fn is_level_2_the_interru(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_2_THE_INTERRU
    }
    #[doc = "Checks if the value of the field is `LEVEL_3_THE_INTERRU`"]
    #[inline(always)]
    pub fn is_level_3_the_interru(&self) -> bool {
        *self == BODINTVAL_A::LEVEL_3_THE_INTERRU
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Level 0: Reserved."]
    #[inline(always)]
    pub fn level_0_reserved(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_0_RESERVED)
    }
    #[doc = "Level 1:The interrupt assertion threshold voltage is 2.22 V; the interrupt de-assertion threshold voltage is 2.35 V."]
    #[inline(always)]
    pub fn level_1the_interrup(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_1THE_INTERRUP)
    }
    #[doc = "Level 2: The interrupt assertion threshold voltage is 2.52 V; the interrupt de-assertion threshold voltage is 2.66 V."]
    #[inline(always)]
    pub fn level_2_the_interru(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_2_THE_INTERRU)
    }
    #[doc = "Level 3: The interrupt assertion threshold voltage is 2.80 V; the interrupt de-assertion threshold voltage is 2.90 V."]
    #[inline(always)]
    pub fn level_3_the_interru(self) -> &'a mut W {
        self.variant(BODINTVAL_A::LEVEL_3_THE_INTERRU)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "BOD reset enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BODRSTENA_A {
    #[doc = "0: Disable reset function."]
    DISABLE_RESET_FUNCTI = 0,
    #[doc = "1: Enable reset function."]
    ENABLE_RESET_FUNCTIO = 1,
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
            false => BODRSTENA_A::DISABLE_RESET_FUNCTI,
            true => BODRSTENA_A::ENABLE_RESET_FUNCTIO,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_RESET_FUNCTI`"]
    #[inline(always)]
    pub fn is_disable_reset_functi(&self) -> bool {
        *self == BODRSTENA_A::DISABLE_RESET_FUNCTI
    }
    #[doc = "Checks if the value of the field is `ENABLE_RESET_FUNCTIO`"]
    #[inline(always)]
    pub fn is_enable_reset_functio(&self) -> bool {
        *self == BODRSTENA_A::ENABLE_RESET_FUNCTIO
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
    pub fn disable_reset_functi(self) -> &'a mut W {
        self.variant(BODRSTENA_A::DISABLE_RESET_FUNCTI)
    }
    #[doc = "Enable reset function."]
    #[inline(always)]
    pub fn enable_reset_functio(self) -> &'a mut W {
        self.variant(BODRSTENA_A::ENABLE_RESET_FUNCTIO)
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
