#[doc = "Reader of register EXTMODE"]
pub type R = crate::R<u32, super::EXTMODE>;
#[doc = "Writer for register EXTMODE"]
pub type W = crate::W<u32, super::EXTMODE>;
#[doc = "Register EXTMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External interrupt 0 EINT0 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE0_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT0."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT0 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTMODE0`"]
pub type EXTMODE0_R = crate::R<bool, EXTMODE0_A>;
impl EXTMODE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE0_A {
        match self.bits {
            false => EXTMODE0_A::LEVEL_SENSITIVE,
            true => EXTMODE0_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE0_A::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE0_A::EDGE_SENSITIVE
    }
}
#[doc = "Write proxy for field `EXTMODE0`"]
pub struct EXTMODE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT0."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT0 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE0_A::EDGE_SENSITIVE)
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
#[doc = "External interrupt 1 EINT1 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE1_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT1."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT1 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTMODE1`"]
pub type EXTMODE1_R = crate::R<bool, EXTMODE1_A>;
impl EXTMODE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE1_A {
        match self.bits {
            false => EXTMODE1_A::LEVEL_SENSITIVE,
            true => EXTMODE1_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE1_A::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE1_A::EDGE_SENSITIVE
    }
}
#[doc = "Write proxy for field `EXTMODE1`"]
pub struct EXTMODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT1."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT1 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE1_A::EDGE_SENSITIVE)
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
#[doc = "External interrupt 2 EINT2 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE2_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT2."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT2 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTMODE2`"]
pub type EXTMODE2_R = crate::R<bool, EXTMODE2_A>;
impl EXTMODE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE2_A {
        match self.bits {
            false => EXTMODE2_A::LEVEL_SENSITIVE,
            true => EXTMODE2_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE2_A::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE2_A::EDGE_SENSITIVE
    }
}
#[doc = "Write proxy for field `EXTMODE2`"]
pub struct EXTMODE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT2."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT2 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE2_A::EDGE_SENSITIVE)
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
#[doc = "External interrupt 3 EINT3 mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTMODE3_A {
    #[doc = "0: Level-sensitive. Level-sensitivity is selected for EINT3."]
    LEVEL_SENSITIVE = 0,
    #[doc = "1: Edge-sensitive. EINT3 is edge sensitive."]
    EDGE_SENSITIVE = 1,
}
impl From<EXTMODE3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTMODE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTMODE3`"]
pub type EXTMODE3_R = crate::R<bool, EXTMODE3_A>;
impl EXTMODE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTMODE3_A {
        match self.bits {
            false => EXTMODE3_A::LEVEL_SENSITIVE,
            true => EXTMODE3_A::EDGE_SENSITIVE,
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_level_sensitive(&self) -> bool {
        *self == EXTMODE3_A::LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `EDGE_SENSITIVE`"]
    #[inline(always)]
    pub fn is_edge_sensitive(&self) -> bool {
        *self == EXTMODE3_A::EDGE_SENSITIVE
    }
}
#[doc = "Write proxy for field `EXTMODE3`"]
pub struct EXTMODE3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTMODE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTMODE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Level-sensitive. Level-sensitivity is selected for EINT3."]
    #[inline(always)]
    pub fn level_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3_A::LEVEL_SENSITIVE)
    }
    #[doc = "Edge-sensitive. EINT3 is edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive(self) -> &'a mut W {
        self.variant(EXTMODE3_A::EDGE_SENSITIVE)
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
impl R {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&self) -> EXTMODE0_R {
        EXTMODE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&self) -> EXTMODE1_R {
        EXTMODE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&self) -> EXTMODE2_R {
        EXTMODE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&self) -> EXTMODE3_R {
        EXTMODE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 mode."]
    #[inline(always)]
    pub fn extmode0(&mut self) -> EXTMODE0_W {
        EXTMODE0_W { w: self }
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 mode."]
    #[inline(always)]
    pub fn extmode1(&mut self) -> EXTMODE1_W {
        EXTMODE1_W { w: self }
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 mode."]
    #[inline(always)]
    pub fn extmode2(&mut self) -> EXTMODE2_W {
        EXTMODE2_W { w: self }
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 mode."]
    #[inline(always)]
    pub fn extmode3(&mut self) -> EXTMODE3_W {
        EXTMODE3_W { w: self }
    }
}
