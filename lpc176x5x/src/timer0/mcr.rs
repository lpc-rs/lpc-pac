#[doc = "Reader of register MCR"]
pub type R = crate::R<u32, super::MCR>;
#[doc = "Writer for register MCR"]
pub type W = crate::W<u32, super::MCR>;
#[doc = "Register MCR `reset()`'s with value 0"]
impl crate::ResetValue for super::MCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Interrupt on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0I_A {
    #[doc = "1: Interrupt is generated when MR0 matches the value in the TC."]
    INTERRUPT_IS_GENERAT = 1,
    #[doc = "0: Interrupt is disabled"]
    INTERRUPT_IS_DISABLE = 0,
}
impl From<MR0I_A> for bool {
    #[inline(always)]
    fn from(variant: MR0I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR0I`"]
pub type MR0I_R = crate::R<bool, MR0I_A>;
impl MR0I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0I_A {
        match self.bits {
            true => MR0I_A::INTERRUPT_IS_GENERAT,
            false => MR0I_A::INTERRUPT_IS_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`"]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == MR0I_A::INTERRUPT_IS_GENERAT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == MR0I_A::INTERRUPT_IS_DISABLE
    }
}
#[doc = "Write proxy for field `MR0I`"]
pub struct MR0I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut W {
        self.variant(MR0I_A::INTERRUPT_IS_GENERAT)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut W {
        self.variant(MR0I_A::INTERRUPT_IS_DISABLE)
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
#[doc = "Reset on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0R_A {
    #[doc = "1: TC will be reset if MR0 matches it."]
    TC_WILL_BE_RESET_IF_ = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR0R_A> for bool {
    #[inline(always)]
    fn from(variant: MR0R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR0R`"]
pub type MR0R_R = crate::R<bool, MR0R_A>;
impl MR0R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0R_A {
        match self.bits {
            true => MR0R_A::TC_WILL_BE_RESET_IF_,
            false => MR0R_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`"]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == MR0R_A::TC_WILL_BE_RESET_IF_
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR0R_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR0R`"]
pub struct MR0R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut W {
        self.variant(MR0R_A::TC_WILL_BE_RESET_IF_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR0R_A::FEATURE_DISABLED_)
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
#[doc = "Stop on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR0S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    TC_AND_PC_WILL_BE_ST = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR0S_A> for bool {
    #[inline(always)]
    fn from(variant: MR0S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR0S`"]
pub type MR0S_R = crate::R<bool, MR0S_A>;
impl MR0S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR0S_A {
        match self.bits {
            true => MR0S_A::TC_AND_PC_WILL_BE_ST,
            false => MR0S_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == MR0S_A::TC_AND_PC_WILL_BE_ST
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR0S_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR0S`"]
pub struct MR0S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR0S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
        self.variant(MR0S_A::TC_AND_PC_WILL_BE_ST)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR0S_A::FEATURE_DISABLED_)
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
#[doc = "Interrupt on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1I_A {
    #[doc = "1: Interrupt is generated when MR1 matches the value in the TC."]
    INTERRUPT_IS_GENERAT = 1,
    #[doc = "0: Interrupt is disabled."]
    INTERRUPT_IS_DISABLE = 0,
}
impl From<MR1I_A> for bool {
    #[inline(always)]
    fn from(variant: MR1I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR1I`"]
pub type MR1I_R = crate::R<bool, MR1I_A>;
impl MR1I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1I_A {
        match self.bits {
            true => MR1I_A::INTERRUPT_IS_GENERAT,
            false => MR1I_A::INTERRUPT_IS_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`"]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == MR1I_A::INTERRUPT_IS_GENERAT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == MR1I_A::INTERRUPT_IS_DISABLE
    }
}
#[doc = "Write proxy for field `MR1I`"]
pub struct MR1I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut W {
        self.variant(MR1I_A::INTERRUPT_IS_GENERAT)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut W {
        self.variant(MR1I_A::INTERRUPT_IS_DISABLE)
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
#[doc = "Reset on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1R_A {
    #[doc = "1: TC will be reset if MR1 matches it."]
    TC_WILL_BE_RESET_IF_ = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR1R_A> for bool {
    #[inline(always)]
    fn from(variant: MR1R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR1R`"]
pub type MR1R_R = crate::R<bool, MR1R_A>;
impl MR1R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1R_A {
        match self.bits {
            true => MR1R_A::TC_WILL_BE_RESET_IF_,
            false => MR1R_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`"]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == MR1R_A::TC_WILL_BE_RESET_IF_
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR1R_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR1R`"]
pub struct MR1R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut W {
        self.variant(MR1R_A::TC_WILL_BE_RESET_IF_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR1R_A::FEATURE_DISABLED_)
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
#[doc = "Stop on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR1S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    TC_AND_PC_WILL_BE_ST = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR1S_A> for bool {
    #[inline(always)]
    fn from(variant: MR1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR1S`"]
pub type MR1S_R = crate::R<bool, MR1S_A>;
impl MR1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR1S_A {
        match self.bits {
            true => MR1S_A::TC_AND_PC_WILL_BE_ST,
            false => MR1S_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == MR1S_A::TC_AND_PC_WILL_BE_ST
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR1S_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR1S`"]
pub struct MR1S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR1S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
        self.variant(MR1S_A::TC_AND_PC_WILL_BE_ST)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR1S_A::FEATURE_DISABLED_)
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
#[doc = "Interrupt on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2I_A {
    #[doc = "1: Interrupt is generated when MR2 matches the value in the TC."]
    INTERRUPT_IS_GENERAT = 1,
    #[doc = "0: Interrupt is disabled"]
    INTERRUPT_IS_DISABLE = 0,
}
impl From<MR2I_A> for bool {
    #[inline(always)]
    fn from(variant: MR2I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR2I`"]
pub type MR2I_R = crate::R<bool, MR2I_A>;
impl MR2I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2I_A {
        match self.bits {
            true => MR2I_A::INTERRUPT_IS_GENERAT,
            false => MR2I_A::INTERRUPT_IS_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`"]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == MR2I_A::INTERRUPT_IS_GENERAT
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_DISABLE`"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == MR2I_A::INTERRUPT_IS_DISABLE
    }
}
#[doc = "Write proxy for field `MR2I`"]
pub struct MR2I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut W {
        self.variant(MR2I_A::INTERRUPT_IS_GENERAT)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut W {
        self.variant(MR2I_A::INTERRUPT_IS_DISABLE)
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
#[doc = "Reset on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2R_A {
    #[doc = "1: TC will be reset if MR2 matches it."]
    TC_WILL_BE_RESET_IF_ = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR2R_A> for bool {
    #[inline(always)]
    fn from(variant: MR2R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR2R`"]
pub type MR2R_R = crate::R<bool, MR2R_A>;
impl MR2R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2R_A {
        match self.bits {
            true => MR2R_A::TC_WILL_BE_RESET_IF_,
            false => MR2R_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`"]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == MR2R_A::TC_WILL_BE_RESET_IF_
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR2R_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR2R`"]
pub struct MR2R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut W {
        self.variant(MR2R_A::TC_WILL_BE_RESET_IF_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR2R_A::FEATURE_DISABLED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Stop on MR2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR2S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC"]
    TC_AND_PC_WILL_BE_ST = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR2S_A> for bool {
    #[inline(always)]
    fn from(variant: MR2S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR2S`"]
pub type MR2S_R = crate::R<bool, MR2S_A>;
impl MR2S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR2S_A {
        match self.bits {
            true => MR2S_A::TC_AND_PC_WILL_BE_ST,
            false => MR2S_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == MR2S_A::TC_AND_PC_WILL_BE_ST
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR2S_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR2S`"]
pub struct MR2S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR2S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
        self.variant(MR2S_A::TC_AND_PC_WILL_BE_ST)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR2S_A::FEATURE_DISABLED_)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Interrupt on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3I_A {
    #[doc = "1: Interrupt is generated when MR3 matches the value in the TC."]
    INTERRUPT_IS_GENERAT = 1,
    #[doc = "0: This interrupt is disabled"]
    THIS_INTERRUPT_IS_DI = 0,
}
impl From<MR3I_A> for bool {
    #[inline(always)]
    fn from(variant: MR3I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR3I`"]
pub type MR3I_R = crate::R<bool, MR3I_A>;
impl MR3I_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3I_A {
        match self.bits {
            true => MR3I_A::INTERRUPT_IS_GENERAT,
            false => MR3I_A::THIS_INTERRUPT_IS_DI,
        }
    }
    #[doc = "Checks if the value of the field is `INTERRUPT_IS_GENERAT`"]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == MR3I_A::INTERRUPT_IS_GENERAT
    }
    #[doc = "Checks if the value of the field is `THIS_INTERRUPT_IS_DI`"]
    #[inline(always)]
    pub fn is_this_interrupt_is_di(&self) -> bool {
        *self == MR3I_A::THIS_INTERRUPT_IS_DI
    }
}
#[doc = "Write proxy for field `MR3I`"]
pub struct MR3I_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3I_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut W {
        self.variant(MR3I_A::INTERRUPT_IS_GENERAT)
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn this_interrupt_is_di(self) -> &'a mut W {
        self.variant(MR3I_A::THIS_INTERRUPT_IS_DI)
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
#[doc = "Reset on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3R_A {
    #[doc = "1: TC will be reset if MR3 matches it."]
    TC_WILL_BE_RESET_IF_ = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR3R_A> for bool {
    #[inline(always)]
    fn from(variant: MR3R_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR3R`"]
pub type MR3R_R = crate::R<bool, MR3R_A>;
impl MR3R_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3R_A {
        match self.bits {
            true => MR3R_A::TC_WILL_BE_RESET_IF_,
            false => MR3R_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_WILL_BE_RESET_IF_`"]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == MR3R_A::TC_WILL_BE_RESET_IF_
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR3R_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR3R`"]
pub struct MR3R_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3R_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3R_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut W {
        self.variant(MR3R_A::TC_WILL_BE_RESET_IF_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR3R_A::FEATURE_DISABLED_)
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
#[doc = "Stop on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR3S_A {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    TC_AND_PC_WILL_BE_ST = 1,
    #[doc = "0: Feature disabled."]
    FEATURE_DISABLED_ = 0,
}
impl From<MR3S_A> for bool {
    #[inline(always)]
    fn from(variant: MR3S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MR3S`"]
pub type MR3S_R = crate::R<bool, MR3S_A>;
impl MR3S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MR3S_A {
        match self.bits {
            true => MR3S_A::TC_AND_PC_WILL_BE_ST,
            false => MR3S_A::FEATURE_DISABLED_,
        }
    }
    #[doc = "Checks if the value of the field is `TC_AND_PC_WILL_BE_ST`"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == MR3S_A::TC_AND_PC_WILL_BE_ST
    }
    #[doc = "Checks if the value of the field is `FEATURE_DISABLED_`"]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == MR3S_A::FEATURE_DISABLED_
    }
}
#[doc = "Write proxy for field `MR3S`"]
pub struct MR3S_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MR3S_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\]
will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut W {
        self.variant(MR3S_A::TC_AND_PC_WILL_BE_ST)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut W {
        self.variant(MR3S_A::FEATURE_DISABLED_)
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
impl R {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&self) -> MR0I_R {
        MR0I_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&self) -> MR0R_R {
        MR0R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&self) -> MR0S_R {
        MR0S_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&self) -> MR1I_R {
        MR1I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&self) -> MR1R_R {
        MR1R_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&self) -> MR1S_R {
        MR1S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&self) -> MR2I_R {
        MR2I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&self) -> MR2R_R {
        MR2R_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&self) -> MR2S_R {
        MR2S_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&self) -> MR3I_R {
        MR3I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&self) -> MR3R_R {
        MR3R_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&self) -> MR3S_R {
        MR3S_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&mut self) -> MR0I_W {
        MR0I_W { w: self }
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&mut self) -> MR0R_W {
        MR0R_W { w: self }
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&mut self) -> MR0S_W {
        MR0S_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&mut self) -> MR1I_W {
        MR1I_W { w: self }
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&mut self) -> MR1R_W {
        MR1R_W { w: self }
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&mut self) -> MR1S_W {
        MR1S_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&mut self) -> MR2I_W {
        MR2I_W { w: self }
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&mut self) -> MR2R_W {
        MR2R_W { w: self }
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> MR2S_W {
        MR2S_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&mut self) -> MR3I_W {
        MR3I_W { w: self }
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&mut self) -> MR3R_W {
        MR3R_W { w: self }
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&mut self) -> MR3S_W {
        MR3S_W { w: self }
    }
}
