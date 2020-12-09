#[doc = "Reader of register FPDSCR"]
pub type R = crate::R<u32, super::FPDSCR>;
#[doc = "Writer for register FPDSCR"]
pub type W = crate::W<u32, super::FPDSCR>;
#[doc = "Register FPDSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FPDSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Default value for FPSCR.RMode (Rounding Mode control field).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RMODE_A {
    #[doc = "0: Round to Nearest (RN) mode"]
    RMODE_0 = 0,
    #[doc = "1: Round towards Plus Infinity (RP) mode."]
    RMODE_1 = 1,
    #[doc = "2: Round towards Minus Infinity (RM) mode."]
    RMODE_2 = 2,
    #[doc = "3: Round towards Zero (RZ) mode."]
    RMODE_3 = 3,
}
impl From<RMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RMode`"]
pub type RMODE_R = crate::R<u8, RMODE_A>;
impl RMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMODE_A {
        match self.bits {
            0 => RMODE_A::RMODE_0,
            1 => RMODE_A::RMODE_1,
            2 => RMODE_A::RMODE_2,
            3 => RMODE_A::RMODE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RMODE_0`"]
    #[inline(always)]
    pub fn is_rmode_0(&self) -> bool {
        *self == RMODE_A::RMODE_0
    }
    #[doc = "Checks if the value of the field is `RMODE_1`"]
    #[inline(always)]
    pub fn is_rmode_1(&self) -> bool {
        *self == RMODE_A::RMODE_1
    }
    #[doc = "Checks if the value of the field is `RMODE_2`"]
    #[inline(always)]
    pub fn is_rmode_2(&self) -> bool {
        *self == RMODE_A::RMODE_2
    }
    #[doc = "Checks if the value of the field is `RMODE_3`"]
    #[inline(always)]
    pub fn is_rmode_3(&self) -> bool {
        *self == RMODE_A::RMODE_3
    }
}
#[doc = "Write proxy for field `RMode`"]
pub struct RMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Round to Nearest (RN) mode"]
    #[inline(always)]
    pub fn rmode_0(self) -> &'a mut W {
        self.variant(RMODE_A::RMODE_0)
    }
    #[doc = "Round towards Plus Infinity (RP) mode."]
    #[inline(always)]
    pub fn rmode_1(self) -> &'a mut W {
        self.variant(RMODE_A::RMODE_1)
    }
    #[doc = "Round towards Minus Infinity (RM) mode."]
    #[inline(always)]
    pub fn rmode_2(self) -> &'a mut W {
        self.variant(RMODE_A::RMODE_2)
    }
    #[doc = "Round towards Zero (RZ) mode."]
    #[inline(always)]
    pub fn rmode_3(self) -> &'a mut W {
        self.variant(RMODE_A::RMODE_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Default value for FPSCR.FZ (Flush-to-zero mode control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FZ_A {
    #[doc = "0: Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    FZ_0 = 0,
    #[doc = "1: Flush-to-zero mode enabled."]
    FZ_1 = 1,
}
impl From<FZ_A> for bool {
    #[inline(always)]
    fn from(variant: FZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FZ`"]
pub type FZ_R = crate::R<bool, FZ_A>;
impl FZ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FZ_A {
        match self.bits {
            false => FZ_A::FZ_0,
            true => FZ_A::FZ_1,
        }
    }
    #[doc = "Checks if the value of the field is `FZ_0`"]
    #[inline(always)]
    pub fn is_fz_0(&self) -> bool {
        *self == FZ_A::FZ_0
    }
    #[doc = "Checks if the value of the field is `FZ_1`"]
    #[inline(always)]
    pub fn is_fz_1(&self) -> bool {
        *self == FZ_A::FZ_1
    }
}
#[doc = "Write proxy for field `FZ`"]
pub struct FZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FZ_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Flush-to-zero mode disabled. Behavior of the floating-point system is fully compliant with the IEEE 754 standard."]
    #[inline(always)]
    pub fn fz_0(self) -> &'a mut W {
        self.variant(FZ_A::FZ_0)
    }
    #[doc = "Flush-to-zero mode enabled."]
    #[inline(always)]
    pub fn fz_1(self) -> &'a mut W {
        self.variant(FZ_A::FZ_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Default value for FPSCR.DN (Default NaN mode control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DN_A {
    #[doc = "0: NaN operands propagate through to the output of a floating-point operation."]
    DN_0 = 0,
    #[doc = "1: Any operation involving one or more NaNs returns the Default NaN."]
    DN_1 = 1,
}
impl From<DN_A> for bool {
    #[inline(always)]
    fn from(variant: DN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DN`"]
pub type DN_R = crate::R<bool, DN_A>;
impl DN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DN_A {
        match self.bits {
            false => DN_A::DN_0,
            true => DN_A::DN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DN_0`"]
    #[inline(always)]
    pub fn is_dn_0(&self) -> bool {
        *self == DN_A::DN_0
    }
    #[doc = "Checks if the value of the field is `DN_1`"]
    #[inline(always)]
    pub fn is_dn_1(&self) -> bool {
        *self == DN_A::DN_1
    }
}
#[doc = "Write proxy for field `DN`"]
pub struct DN_W<'a> {
    w: &'a mut W,
}
impl<'a> DN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NaN operands propagate through to the output of a floating-point operation."]
    #[inline(always)]
    pub fn dn_0(self) -> &'a mut W {
        self.variant(DN_A::DN_0)
    }
    #[doc = "Any operation involving one or more NaNs returns the Default NaN."]
    #[inline(always)]
    pub fn dn_1(self) -> &'a mut W {
        self.variant(DN_A::DN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Default value for FPSCR.AHP (Alternative half-precision control bit).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHP_A {
    #[doc = "0: IEEE half-precision format selected."]
    AHP_0 = 0,
    #[doc = "1: Alternative half-precision format selected."]
    AHP_1 = 1,
}
impl From<AHP_A> for bool {
    #[inline(always)]
    fn from(variant: AHP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHP`"]
pub type AHP_R = crate::R<bool, AHP_A>;
impl AHP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHP_A {
        match self.bits {
            false => AHP_A::AHP_0,
            true => AHP_A::AHP_1,
        }
    }
    #[doc = "Checks if the value of the field is `AHP_0`"]
    #[inline(always)]
    pub fn is_ahp_0(&self) -> bool {
        *self == AHP_A::AHP_0
    }
    #[doc = "Checks if the value of the field is `AHP_1`"]
    #[inline(always)]
    pub fn is_ahp_1(&self) -> bool {
        *self == AHP_A::AHP_1
    }
}
#[doc = "Write proxy for field `AHP`"]
pub struct AHP_W<'a> {
    w: &'a mut W,
}
impl<'a> AHP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "IEEE half-precision format selected."]
    #[inline(always)]
    pub fn ahp_0(self) -> &'a mut W {
        self.variant(AHP_A::AHP_0)
    }
    #[doc = "Alternative half-precision format selected."]
    #[inline(always)]
    pub fn ahp_1(self) -> &'a mut W {
        self.variant(AHP_A::AHP_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 22:23 - Default value for FPSCR.RMode (Rounding Mode control field)."]
    #[inline(always)]
    pub fn rmode(&mut self) -> RMODE_W {
        RMODE_W { w: self }
    }
    #[doc = "Bit 24 - Default value for FPSCR.FZ (Flush-to-zero mode control bit)."]
    #[inline(always)]
    pub fn fz(&mut self) -> FZ_W {
        FZ_W { w: self }
    }
    #[doc = "Bit 25 - Default value for FPSCR.DN (Default NaN mode control bit)."]
    #[inline(always)]
    pub fn dn(&mut self) -> DN_W {
        DN_W { w: self }
    }
    #[doc = "Bit 26 - Default value for FPSCR.AHP (Alternative half-precision control bit)."]
    #[inline(always)]
    pub fn ahp(&mut self) -> AHP_W {
        AHP_W { w: self }
    }
}
