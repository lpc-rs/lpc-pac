#[doc = "Reader of register EXTPOLAR"]
pub type R = crate::R<u32, super::EXTPOLAR>;
#[doc = "Writer for register EXTPOLAR"]
pub type W = crate::W<u32, super::EXTPOLAR>;
#[doc = "Register EXTPOLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTPOLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External interrupt 0 EINT0 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR0_A {
    #[doc = "0: Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR0_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTPOLAR0`"]
pub type EXTPOLAR0_R = crate::R<bool, EXTPOLAR0_A>;
impl EXTPOLAR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR0_A {
        match self.bits {
            false => EXTPOLAR0_A::FALLING_EDGE,
            true => EXTPOLAR0_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR0_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR0_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `EXTPOLAR0`"]
pub struct EXTPOLAR0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge. EINT0 is low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR0_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT0 is high-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR0_A::RISING_EDGE)
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
#[doc = "External interrupt 1 EINT1 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR1_A {
    #[doc = "0: Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR1_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTPOLAR1`"]
pub type EXTPOLAR1_R = crate::R<bool, EXTPOLAR1_A>;
impl EXTPOLAR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR1_A {
        match self.bits {
            false => EXTPOLAR1_A::FALLING_EDGE,
            true => EXTPOLAR1_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR1_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR1_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `EXTPOLAR1`"]
pub struct EXTPOLAR1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge. EINT1 is low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR1_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT1 is high-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR1_A::RISING_EDGE)
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
#[doc = "External interrupt 2 EINT2 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR2_A {
    #[doc = "0: Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR2_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTPOLAR2`"]
pub type EXTPOLAR2_R = crate::R<bool, EXTPOLAR2_A>;
impl EXTPOLAR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR2_A {
        match self.bits {
            false => EXTPOLAR2_A::FALLING_EDGE,
            true => EXTPOLAR2_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR2_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR2_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `EXTPOLAR2`"]
pub struct EXTPOLAR2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge. EINT2 is low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR2_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT2 is high-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR2_A::RISING_EDGE)
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
#[doc = "External interrupt 3 EINT3 polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTPOLAR3_A {
    #[doc = "0: Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    RISING_EDGE = 1,
}
impl From<EXTPOLAR3_A> for bool {
    #[inline(always)]
    fn from(variant: EXTPOLAR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EXTPOLAR3`"]
pub type EXTPOLAR3_R = crate::R<bool, EXTPOLAR3_A>;
impl EXTPOLAR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTPOLAR3_A {
        match self.bits {
            false => EXTPOLAR3_A::FALLING_EDGE,
            true => EXTPOLAR3_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTPOLAR3_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTPOLAR3_A::RISING_EDGE
    }
}
#[doc = "Write proxy for field `EXTPOLAR3`"]
pub struct EXTPOLAR3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLAR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTPOLAR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Falling edge. EINT3 is low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR3_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. EINT3 is high-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTPOLAR3_A::RISING_EDGE)
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
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&self) -> EXTPOLAR0_R {
        EXTPOLAR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&self) -> EXTPOLAR1_R {
        EXTPOLAR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&self) -> EXTPOLAR2_R {
        EXTPOLAR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&self) -> EXTPOLAR3_R {
        EXTPOLAR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt 0 EINT0 polarity."]
    #[inline(always)]
    pub fn extpolar0(&mut self) -> EXTPOLAR0_W {
        EXTPOLAR0_W { w: self }
    }
    #[doc = "Bit 1 - External interrupt 1 EINT1 polarity."]
    #[inline(always)]
    pub fn extpolar1(&mut self) -> EXTPOLAR1_W {
        EXTPOLAR1_W { w: self }
    }
    #[doc = "Bit 2 - External interrupt 2 EINT2 polarity."]
    #[inline(always)]
    pub fn extpolar2(&mut self) -> EXTPOLAR2_W {
        EXTPOLAR2_W { w: self }
    }
    #[doc = "Bit 3 - External interrupt 3 EINT3 polarity."]
    #[inline(always)]
    pub fn extpolar3(&mut self) -> EXTPOLAR3_W {
        EXTPOLAR3_W { w: self }
    }
}
