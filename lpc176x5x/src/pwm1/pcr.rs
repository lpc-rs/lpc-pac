#[doc = "Reader of register PCR"]
pub type R = crate::R<u32, super::PCR>;
#[doc = "Writer for register PCR"]
pub type W = crate::W<u32, super::PCR>;
#[doc = "Register PCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PWM\\[2\\] output single/double edge mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL2_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL,
}
impl From<PWMSEL2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL2_A) -> Self {
        match variant {
            PWMSEL2_A::SINGLE_EDGE_CONTROLL => false,
            PWMSEL2_A::DOUBLE_EDGE_CONTROLL => true,
        }
    }
}
#[doc = "Reader of field `PWMSEL2`"]
pub type PWMSEL2_R = crate::R<bool, PWMSEL2_A>;
impl PWMSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL2_A {
        match self.bits {
            false => PWMSEL2_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL2_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == PWMSEL2_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == PWMSEL2_A::DOUBLE_EDGE_CONTROLL
    }
}
#[doc = "Write proxy for field `PWMSEL2`"]
pub struct PWMSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL2_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL2_A::DOUBLE_EDGE_CONTROLL)
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
#[doc = "PWM\\[3\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL3_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL,
}
impl From<PWMSEL3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL3_A) -> Self {
        match variant {
            PWMSEL3_A::SINGLE_EDGE_CONTROLL => false,
            PWMSEL3_A::DOUBLE_EDGE_CONTROLL => true,
        }
    }
}
#[doc = "Reader of field `PWMSEL3`"]
pub type PWMSEL3_R = crate::R<bool, PWMSEL3_A>;
impl PWMSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL3_A {
        match self.bits {
            false => PWMSEL3_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL3_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == PWMSEL3_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == PWMSEL3_A::DOUBLE_EDGE_CONTROLL
    }
}
#[doc = "Write proxy for field `PWMSEL3`"]
pub struct PWMSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL3_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL3_A::DOUBLE_EDGE_CONTROLL)
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
#[doc = "PWM\\[4\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL4_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL,
}
impl From<PWMSEL4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL4_A) -> Self {
        match variant {
            PWMSEL4_A::SINGLE_EDGE_CONTROLL => false,
            PWMSEL4_A::DOUBLE_EDGE_CONTROLL => true,
        }
    }
}
#[doc = "Reader of field `PWMSEL4`"]
pub type PWMSEL4_R = crate::R<bool, PWMSEL4_A>;
impl PWMSEL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL4_A {
        match self.bits {
            false => PWMSEL4_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL4_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == PWMSEL4_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == PWMSEL4_A::DOUBLE_EDGE_CONTROLL
    }
}
#[doc = "Write proxy for field `PWMSEL4`"]
pub struct PWMSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL4_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL4_A::DOUBLE_EDGE_CONTROLL)
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
#[doc = "PWM\\[5\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL5_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL,
}
impl From<PWMSEL5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL5_A) -> Self {
        match variant {
            PWMSEL5_A::SINGLE_EDGE_CONTROLL => false,
            PWMSEL5_A::DOUBLE_EDGE_CONTROLL => true,
        }
    }
}
#[doc = "Reader of field `PWMSEL5`"]
pub type PWMSEL5_R = crate::R<bool, PWMSEL5_A>;
impl PWMSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL5_A {
        match self.bits {
            false => PWMSEL5_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL5_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == PWMSEL5_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == PWMSEL5_A::DOUBLE_EDGE_CONTROLL
    }
}
#[doc = "Write proxy for field `PWMSEL5`"]
pub struct PWMSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL5_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL5_A::DOUBLE_EDGE_CONTROLL)
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
#[doc = "PWM\\[6\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMSEL6_A {
    #[doc = "0: Single edge controlled mode is selected."]
    SINGLE_EDGE_CONTROLL,
    #[doc = "1: Double edge controlled mode is selected."]
    DOUBLE_EDGE_CONTROLL,
}
impl From<PWMSEL6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSEL6_A) -> Self {
        match variant {
            PWMSEL6_A::SINGLE_EDGE_CONTROLL => false,
            PWMSEL6_A::DOUBLE_EDGE_CONTROLL => true,
        }
    }
}
#[doc = "Reader of field `PWMSEL6`"]
pub type PWMSEL6_R = crate::R<bool, PWMSEL6_A>;
impl PWMSEL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSEL6_A {
        match self.bits {
            false => PWMSEL6_A::SINGLE_EDGE_CONTROLL,
            true => PWMSEL6_A::DOUBLE_EDGE_CONTROLL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == PWMSEL6_A::SINGLE_EDGE_CONTROLL
    }
    #[doc = "Checks if the value of the field is `DOUBLE_EDGE_CONTROLL`"]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == PWMSEL6_A::DOUBLE_EDGE_CONTROLL
    }
}
#[doc = "Write proxy for field `PWMSEL6`"]
pub struct PWMSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMSEL6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL6_A::SINGLE_EDGE_CONTROLL)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut W {
        self.variant(PWMSEL6_A::DOUBLE_EDGE_CONTROLL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "PWM\\[1\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA1_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA1_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA1_A) -> Self {
        match variant {
            PWMENA1_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA1_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA1`"]
pub type PWMENA1_R = crate::R<bool, PWMENA1_A>;
impl PWMENA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA1_A {
        match self.bits {
            false => PWMENA1_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA1_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA1_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA1_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA1`"]
pub struct PWMENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA1_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA1_A::THE_PWM_OUTPUT_IS_EN)
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
#[doc = "PWM\\[2\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA2_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA2_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA2_A) -> Self {
        match variant {
            PWMENA2_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA2_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA2`"]
pub type PWMENA2_R = crate::R<bool, PWMENA2_A>;
impl PWMENA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA2_A {
        match self.bits {
            false => PWMENA2_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA2_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA2_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA2_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA2`"]
pub struct PWMENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA2_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA2_A::THE_PWM_OUTPUT_IS_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "PWM\\[3\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA3_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA3_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA3_A) -> Self {
        match variant {
            PWMENA3_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA3_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA3`"]
pub type PWMENA3_R = crate::R<bool, PWMENA3_A>;
impl PWMENA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA3_A {
        match self.bits {
            false => PWMENA3_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA3_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA3_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA3_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA3`"]
pub struct PWMENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA3_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA3_A::THE_PWM_OUTPUT_IS_EN)
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
#[doc = "PWM\\[4\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA4_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA4_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA4_A) -> Self {
        match variant {
            PWMENA4_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA4_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA4`"]
pub type PWMENA4_R = crate::R<bool, PWMENA4_A>;
impl PWMENA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA4_A {
        match self.bits {
            false => PWMENA4_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA4_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA4_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA4_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA4`"]
pub struct PWMENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA4_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA4_A::THE_PWM_OUTPUT_IS_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "PWM\\[5\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA5_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA5_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA5_A) -> Self {
        match variant {
            PWMENA5_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA5_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA5`"]
pub type PWMENA5_R = crate::R<bool, PWMENA5_A>;
impl PWMENA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA5_A {
        match self.bits {
            false => PWMENA5_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA5_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA5_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA5_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA5`"]
pub struct PWMENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA5_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA5_A::THE_PWM_OUTPUT_IS_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "PWM\\[6\\] output enable control. See PWMENA1 for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMENA6_A {
    #[doc = "0: The PWM output is disabled."]
    THE_PWM_OUTPUT_IS_DI,
    #[doc = "1: The PWM output is enabled."]
    THE_PWM_OUTPUT_IS_EN,
}
impl From<PWMENA6_A> for bool {
    #[inline(always)]
    fn from(variant: PWMENA6_A) -> Self {
        match variant {
            PWMENA6_A::THE_PWM_OUTPUT_IS_DI => false,
            PWMENA6_A::THE_PWM_OUTPUT_IS_EN => true,
        }
    }
}
#[doc = "Reader of field `PWMENA6`"]
pub type PWMENA6_R = crate::R<bool, PWMENA6_A>;
impl PWMENA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMENA6_A {
        match self.bits {
            false => PWMENA6_A::THE_PWM_OUTPUT_IS_DI,
            true => PWMENA6_A::THE_PWM_OUTPUT_IS_EN,
        }
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_DI`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == PWMENA6_A::THE_PWM_OUTPUT_IS_DI
    }
    #[doc = "Checks if the value of the field is `THE_PWM_OUTPUT_IS_EN`"]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == PWMENA6_A::THE_PWM_OUTPUT_IS_EN
    }
}
#[doc = "Write proxy for field `PWMENA6`"]
pub struct PWMENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMENA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMENA6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut W {
        self.variant(PWMENA6_A::THE_PWM_OUTPUT_IS_DI)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut W {
        self.variant(PWMENA6_A::THE_PWM_OUTPUT_IS_EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&self) -> PWMSEL2_R {
        PWMSEL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&self) -> PWMSEL3_R {
        PWMSEL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&self) -> PWMSEL4_R {
        PWMSEL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&self) -> PWMSEL5_R {
        PWMSEL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&self) -> PWMSEL6_R {
        PWMSEL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&self) -> PWMENA1_R {
        PWMENA1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&self) -> PWMENA2_R {
        PWMENA2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&self) -> PWMENA3_R {
        PWMENA3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&self) -> PWMENA4_R {
        PWMENA4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&self) -> PWMENA5_R {
        PWMENA5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&self) -> PWMENA6_R {
        PWMENA6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&mut self) -> PWMSEL2_W {
        PWMSEL2_W { w: self }
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&mut self) -> PWMSEL3_W {
        PWMSEL3_W { w: self }
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&mut self) -> PWMSEL4_W {
        PWMSEL4_W { w: self }
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&mut self) -> PWMSEL5_W {
        PWMSEL5_W { w: self }
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&mut self) -> PWMSEL6_W {
        PWMSEL6_W { w: self }
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&mut self) -> PWMENA1_W {
        PWMENA1_W { w: self }
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&mut self) -> PWMENA2_W {
        PWMENA2_W { w: self }
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&mut self) -> PWMENA3_W {
        PWMENA3_W { w: self }
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&mut self) -> PWMENA4_W {
        PWMENA4_W { w: self }
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&mut self) -> PWMENA5_W {
        PWMENA5_W { w: self }
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&mut self) -> PWMENA6_W {
        PWMENA6_W { w: self }
    }
}
