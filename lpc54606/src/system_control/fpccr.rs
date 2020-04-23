#[doc = "Reader of register FPCCR"]
pub type R = crate::R<u32, super::FPCCR>;
#[doc = "Writer for register FPCCR"]
pub type W = crate::W<u32, super::FPCCR>;
#[doc = "Register FPCCR `reset()`'s with value 0xc000_0000"]
impl crate::ResetValue for super::FPCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc000_0000
    }
}
#[doc = "Lazy state preservation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPACT_A {
    #[doc = "0: Lazy state preservation is not active."]
    LSPACT_0 = 0,
    #[doc = "1: Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    LSPACT_1 = 1,
}
impl From<LSPACT_A> for bool {
    #[inline(always)]
    fn from(variant: LSPACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSPACT`"]
pub type LSPACT_R = crate::R<bool, LSPACT_A>;
impl LSPACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPACT_A {
        match self.bits {
            false => LSPACT_A::LSPACT_0,
            true => LSPACT_A::LSPACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `LSPACT_0`"]
    #[inline(always)]
    pub fn is_lspact_0(&self) -> bool {
        *self == LSPACT_A::LSPACT_0
    }
    #[doc = "Checks if the value of the field is `LSPACT_1`"]
    #[inline(always)]
    pub fn is_lspact_1(&self) -> bool {
        *self == LSPACT_A::LSPACT_1
    }
}
#[doc = "Write proxy for field `LSPACT`"]
pub struct LSPACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPACT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Lazy state preservation is not active."]
    #[inline(always)]
    pub fn lspact_0(self) -> &'a mut W {
        self.variant(LSPACT_A::LSPACT_0)
    }
    #[doc = "Lazy state preservation is active. floating-point stack frame has been allocated but saving state to it has been deferred."]
    #[inline(always)]
    pub fn lspact_1(self) -> &'a mut W {
        self.variant(LSPACT_A::LSPACT_1)
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
#[doc = "Privilege level when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USER_A {
    #[doc = "0: Privilege level was not user when the floating-point stack frame was allocated."]
    USER_0 = 0,
    #[doc = "1: Privilege level was user when the floating-point stack frame was allocated."]
    USER_1 = 1,
}
impl From<USER_A> for bool {
    #[inline(always)]
    fn from(variant: USER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USER`"]
pub type USER_R = crate::R<bool, USER_A>;
impl USER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USER_A {
        match self.bits {
            false => USER_A::USER_0,
            true => USER_A::USER_1,
        }
    }
    #[doc = "Checks if the value of the field is `USER_0`"]
    #[inline(always)]
    pub fn is_user_0(&self) -> bool {
        *self == USER_A::USER_0
    }
    #[doc = "Checks if the value of the field is `USER_1`"]
    #[inline(always)]
    pub fn is_user_1(&self) -> bool {
        *self == USER_A::USER_1
    }
}
#[doc = "Write proxy for field `USER`"]
pub struct USER_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Privilege level was not user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user_0(self) -> &'a mut W {
        self.variant(USER_A::USER_0)
    }
    #[doc = "Privilege level was user when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user_1(self) -> &'a mut W {
        self.variant(USER_A::USER_1)
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
#[doc = "Mode when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREAD_A {
    #[doc = "0: Mode was not Thread Mode when the floating-point stack frame was allocated."]
    THREAD_0 = 0,
    #[doc = "1: Mode was Thread Mode when the floating-point stack frame was allocated."]
    THREAD_1 = 1,
}
impl From<THREAD_A> for bool {
    #[inline(always)]
    fn from(variant: THREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `THREAD`"]
pub type THREAD_R = crate::R<bool, THREAD_A>;
impl THREAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREAD_A {
        match self.bits {
            false => THREAD_A::THREAD_0,
            true => THREAD_A::THREAD_1,
        }
    }
    #[doc = "Checks if the value of the field is `THREAD_0`"]
    #[inline(always)]
    pub fn is_thread_0(&self) -> bool {
        *self == THREAD_A::THREAD_0
    }
    #[doc = "Checks if the value of the field is `THREAD_1`"]
    #[inline(always)]
    pub fn is_thread_1(&self) -> bool {
        *self == THREAD_A::THREAD_1
    }
}
#[doc = "Write proxy for field `THREAD`"]
pub struct THREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> THREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mode was not Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread_0(self) -> &'a mut W {
        self.variant(THREAD_A::THREAD_0)
    }
    #[doc = "Mode was Thread Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread_1(self) -> &'a mut W {
        self.variant(THREAD_A::THREAD_1)
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
#[doc = "Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFRDY_A {
    #[doc = "0: Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_0 = 0,
    #[doc = "1: Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    HFRDY_1 = 1,
}
impl From<HFRDY_A> for bool {
    #[inline(always)]
    fn from(variant: HFRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HFRDY`"]
pub type HFRDY_R = crate::R<bool, HFRDY_A>;
impl HFRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFRDY_A {
        match self.bits {
            false => HFRDY_A::HFRDY_0,
            true => HFRDY_A::HFRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `HFRDY_0`"]
    #[inline(always)]
    pub fn is_hfrdy_0(&self) -> bool {
        *self == HFRDY_A::HFRDY_0
    }
    #[doc = "Checks if the value of the field is `HFRDY_1`"]
    #[inline(always)]
    pub fn is_hfrdy_1(&self) -> bool {
        *self == HFRDY_A::HFRDY_1
    }
}
#[doc = "Write proxy for field `HFRDY`"]
pub struct HFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Priority did not permit setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy_0(self) -> &'a mut W {
        self.variant(HFRDY_A::HFRDY_0)
    }
    #[doc = "Priority permitted setting the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy_1(self) -> &'a mut W {
        self.variant(HFRDY_A::HFRDY_1)
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
#[doc = "Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMRDY_A {
    #[doc = "0: MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_0 = 0,
    #[doc = "1: MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    MMRDY_1 = 1,
}
impl From<MMRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MMRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MMRDY`"]
pub type MMRDY_R = crate::R<bool, MMRDY_A>;
impl MMRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMRDY_A {
        match self.bits {
            false => MMRDY_A::MMRDY_0,
            true => MMRDY_A::MMRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MMRDY_0`"]
    #[inline(always)]
    pub fn is_mmrdy_0(&self) -> bool {
        *self == MMRDY_A::MMRDY_0
    }
    #[doc = "Checks if the value of the field is `MMRDY_1`"]
    #[inline(always)]
    pub fn is_mmrdy_1(&self) -> bool {
        *self == MMRDY_A::MMRDY_1
    }
}
#[doc = "Write proxy for field `MMRDY`"]
pub struct MMRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MMRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "MemManage is disabled or priority did not permit setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy_0(self) -> &'a mut W {
        self.variant(MMRDY_A::MMRDY_0)
    }
    #[doc = "MemManage is enabled and priority permitted setting the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy_1(self) -> &'a mut W {
        self.variant(MMRDY_A::MMRDY_1)
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
#[doc = "Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BFRDY_A {
    #[doc = "0: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_0 = 0,
    #[doc = "1: BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    BFRDY_1 = 1,
}
impl From<BFRDY_A> for bool {
    #[inline(always)]
    fn from(variant: BFRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BFRDY`"]
pub type BFRDY_R = crate::R<bool, BFRDY_A>;
impl BFRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFRDY_A {
        match self.bits {
            false => BFRDY_A::BFRDY_0,
            true => BFRDY_A::BFRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFRDY_0`"]
    #[inline(always)]
    pub fn is_bfrdy_0(&self) -> bool {
        *self == BFRDY_A::BFRDY_0
    }
    #[doc = "Checks if the value of the field is `BFRDY_1`"]
    #[inline(always)]
    pub fn is_bfrdy_1(&self) -> bool {
        *self == BFRDY_A::BFRDY_1
    }
}
#[doc = "Write proxy for field `BFRDY`"]
pub struct BFRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BFRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BFRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy_0(self) -> &'a mut W {
        self.variant(BFRDY_A::BFRDY_0)
    }
    #[doc = "BusFault is disabled or priority did not permit setting the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy_1(self) -> &'a mut W {
        self.variant(BFRDY_A::BFRDY_1)
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
#[doc = "Permission to set the MON_PEND when the floating-point stack frame was allocated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MONRDY_A {
    #[doc = "0: DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_0 = 0,
    #[doc = "1: DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    MONRDY_1 = 1,
}
impl From<MONRDY_A> for bool {
    #[inline(always)]
    fn from(variant: MONRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MONRDY`"]
pub type MONRDY_R = crate::R<bool, MONRDY_A>;
impl MONRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONRDY_A {
        match self.bits {
            false => MONRDY_A::MONRDY_0,
            true => MONRDY_A::MONRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `MONRDY_0`"]
    #[inline(always)]
    pub fn is_monrdy_0(&self) -> bool {
        *self == MONRDY_A::MONRDY_0
    }
    #[doc = "Checks if the value of the field is `MONRDY_1`"]
    #[inline(always)]
    pub fn is_monrdy_1(&self) -> bool {
        *self == MONRDY_A::MONRDY_1
    }
}
#[doc = "Write proxy for field `MONRDY`"]
pub struct MONRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> MONRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MONRDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DebugMonitor is disabled or priority did not permit setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy_0(self) -> &'a mut W {
        self.variant(MONRDY_A::MONRDY_0)
    }
    #[doc = "DebugMonitor is enabled and priority permits setting MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy_1(self) -> &'a mut W {
        self.variant(MONRDY_A::MONRDY_1)
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
#[doc = "Lazy state preservation for floating-point context.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSPEN_A {
    #[doc = "0: Disable automatic lazy state preservation for floating-point context."]
    LSPEN_0 = 0,
    #[doc = "1: Enable automatic lazy state preservation for floating-point context."]
    LSPEN_1 = 1,
}
impl From<LSPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LSPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LSPEN`"]
pub type LSPEN_R = crate::R<bool, LSPEN_A>;
impl LSPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPEN_A {
        match self.bits {
            false => LSPEN_A::LSPEN_0,
            true => LSPEN_A::LSPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `LSPEN_0`"]
    #[inline(always)]
    pub fn is_lspen_0(&self) -> bool {
        *self == LSPEN_A::LSPEN_0
    }
    #[doc = "Checks if the value of the field is `LSPEN_1`"]
    #[inline(always)]
    pub fn is_lspen_1(&self) -> bool {
        *self == LSPEN_A::LSPEN_1
    }
}
#[doc = "Write proxy for field `LSPEN`"]
pub struct LSPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen_0(self) -> &'a mut W {
        self.variant(LSPEN_A::LSPEN_0)
    }
    #[doc = "Enable automatic lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen_1(self) -> &'a mut W {
        self.variant(LSPEN_A::LSPEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASPEN_A {
    #[doc = "0: Disable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_0 = 0,
    #[doc = "1: Enable CONTROL2 setting on execution of a floating-point instruction."]
    ASPEN_1 = 1,
}
impl From<ASPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ASPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASPEN`"]
pub type ASPEN_R = crate::R<bool, ASPEN_A>;
impl ASPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASPEN_A {
        match self.bits {
            false => ASPEN_A::ASPEN_0,
            true => ASPEN_A::ASPEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ASPEN_0`"]
    #[inline(always)]
    pub fn is_aspen_0(&self) -> bool {
        *self == ASPEN_A::ASPEN_0
    }
    #[doc = "Checks if the value of the field is `ASPEN_1`"]
    #[inline(always)]
    pub fn is_aspen_1(&self) -> bool {
        *self == ASPEN_A::ASPEN_1
    }
}
#[doc = "Write proxy for field `ASPEN`"]
pub struct ASPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn aspen_0(self) -> &'a mut W {
        self.variant(ASPEN_A::ASPEN_0)
    }
    #[doc = "Enable CONTROL2 setting on execution of a floating-point instruction."]
    #[inline(always)]
    pub fn aspen_1(self) -> &'a mut W {
        self.variant(ASPEN_A::ASPEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline(always)]
    pub fn lspact(&self) -> LSPACT_R {
        LSPACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&self) -> USER_R {
        USER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&self) -> THREAD_R {
        THREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&self) -> HFRDY_R {
        HFRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&self) -> MMRDY_R {
        MMRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&self) -> BFRDY_R {
        BFRDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&self) -> MONRDY_R {
        MONRDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&self) -> LSPEN_R {
        LSPEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline(always)]
    pub fn aspen(&self) -> ASPEN_R {
        ASPEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lazy state preservation."]
    #[inline(always)]
    pub fn lspact(&mut self) -> LSPACT_W {
        LSPACT_W { w: self }
    }
    #[doc = "Bit 1 - Privilege level when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn user(&mut self) -> USER_W {
        USER_W { w: self }
    }
    #[doc = "Bit 3 - Mode when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn thread(&mut self) -> THREAD_W {
        THREAD_W { w: self }
    }
    #[doc = "Bit 4 - Permission to set the HardFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn hfrdy(&mut self) -> HFRDY_W {
        HFRDY_W { w: self }
    }
    #[doc = "Bit 5 - Permission to set the MemManage handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn mmrdy(&mut self) -> MMRDY_W {
        MMRDY_W { w: self }
    }
    #[doc = "Bit 6 - Permission to set the BusFault handler to the pending state when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn bfrdy(&mut self) -> BFRDY_W {
        BFRDY_W { w: self }
    }
    #[doc = "Bit 8 - Permission to set the MON_PEND when the floating-point stack frame was allocated."]
    #[inline(always)]
    pub fn monrdy(&mut self) -> MONRDY_W {
        MONRDY_W { w: self }
    }
    #[doc = "Bit 30 - Lazy state preservation for floating-point context."]
    #[inline(always)]
    pub fn lspen(&mut self) -> LSPEN_W {
        LSPEN_W { w: self }
    }
    #[doc = "Bit 31 - Enables CONTROL2 setting on execution of a floating-point instruction. This results in automatic hardware state preservation and restoration, for floating-point context, on exception entry and exit."]
    #[inline(always)]
    pub fn aspen(&mut self) -> ASPEN_W {
        ASPEN_W { w: self }
    }
}
