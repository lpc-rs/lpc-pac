#[doc = "Reader of register IER"]
pub type R = crate::R<u32, super::IER>;
#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRINTEN_A {
    #[doc = "0: Disable the RDA interrupt."]
    DISABLE_THE_RDA_INTE = 0,
    #[doc = "1: Enable the RDA interrupt."]
    ENABLE_THE_RDA_INTER = 1,
}
impl From<RBRINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RBRINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RBRINTEN`"]
pub type RBRINTEN_R = crate::R<bool, RBRINTEN_A>;
impl RBRINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRINTEN_A {
        match self.bits {
            false => RBRINTEN_A::DISABLE_THE_RDA_INTE,
            true => RBRINTEN_A::ENABLE_THE_RDA_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == RBRINTEN_A::DISABLE_THE_RDA_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == RBRINTEN_A::ENABLE_THE_RDA_INTER
    }
}
#[doc = "Write proxy for field `RBRINTEN`"]
pub struct RBRINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RDA interrupt."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut W {
        self.variant(RBRINTEN_A::DISABLE_THE_RDA_INTE)
    }
    #[doc = "Enable the RDA interrupt."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut W {
        self.variant(RBRINTEN_A::ENABLE_THE_RDA_INTER)
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
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREINTEN_A {
    #[doc = "0: Disable the THRE interrupt."]
    DISABLE_THE_THRE_INT = 0,
    #[doc = "1: Enable the THRE interrupt."]
    ENABLE_THE_THRE_INTE = 1,
}
impl From<THREINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: THREINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `THREINTEN`"]
pub type THREINTEN_R = crate::R<bool, THREINTEN_A>;
impl THREINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREINTEN_A {
        match self.bits {
            false => THREINTEN_A::DISABLE_THE_THRE_INT,
            true => THREINTEN_A::ENABLE_THE_THRE_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`"]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == THREINTEN_A::DISABLE_THE_THRE_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == THREINTEN_A::ENABLE_THE_THRE_INTE
    }
}
#[doc = "Write proxy for field `THREINTEN`"]
pub struct THREINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> THREINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the THRE interrupt."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut W {
        self.variant(THREINTEN_A::DISABLE_THE_THRE_INT)
    }
    #[doc = "Enable the THRE interrupt."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut W {
        self.variant(THREINTEN_A::ENABLE_THE_THRE_INTE)
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
#[doc = "Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RLSINTEN_A {
    #[doc = "0: Disable the RLS interrupt."]
    DISABLE_THE_RLS_INTE = 0,
    #[doc = "1: Enable the RLS interrupt."]
    ENABLE_THE_RLS_INTER = 1,
}
impl From<RLSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: RLSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RLSINTEN`"]
pub type RLSINTEN_R = crate::R<bool, RLSINTEN_A>;
impl RLSINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLSINTEN_A {
        match self.bits {
            false => RLSINTEN_A::DISABLE_THE_RLS_INTE,
            true => RLSINTEN_A::ENABLE_THE_RLS_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RLS_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rls_inte(&self) -> bool {
        *self == RLSINTEN_A::DISABLE_THE_RLS_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RLS_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rls_inter(&self) -> bool {
        *self == RLSINTEN_A::ENABLE_THE_RLS_INTER
    }
}
#[doc = "Write proxy for field `RLSINTEN`"]
pub struct RLSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RLSINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RLSINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RLS interrupt."]
    #[inline(always)]
    pub fn disable_the_rls_inte(self) -> &'a mut W {
        self.variant(RLSINTEN_A::DISABLE_THE_RLS_INTE)
    }
    #[doc = "Enable the RLS interrupt."]
    #[inline(always)]
    pub fn enable_the_rls_inter(self) -> &'a mut W {
        self.variant(RLSINTEN_A::ENABLE_THE_RLS_INTER)
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
#[doc = "Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSINTEN_A {
    #[doc = "0: Disable the MS interrupt."]
    DISABLE_THE_MS_INTER = 0,
    #[doc = "1: Enable the MS interrupt."]
    ENABLE_THE_MS_INTERR = 1,
}
impl From<MSINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MSINTEN`"]
pub type MSINTEN_R = crate::R<bool, MSINTEN_A>;
impl MSINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSINTEN_A {
        match self.bits {
            false => MSINTEN_A::DISABLE_THE_MS_INTER,
            true => MSINTEN_A::ENABLE_THE_MS_INTERR,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_MS_INTER`"]
    #[inline(always)]
    pub fn is_disable_the_ms_inter(&self) -> bool {
        *self == MSINTEN_A::DISABLE_THE_MS_INTER
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_MS_INTERR`"]
    #[inline(always)]
    pub fn is_enable_the_ms_interr(&self) -> bool {
        *self == MSINTEN_A::ENABLE_THE_MS_INTERR
    }
}
#[doc = "Write proxy for field `MSINTEN`"]
pub struct MSINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MSINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the MS interrupt."]
    #[inline(always)]
    pub fn disable_the_ms_inter(self) -> &'a mut W {
        self.variant(MSINTEN_A::DISABLE_THE_MS_INTER)
    }
    #[doc = "Enable the MS interrupt."]
    #[inline(always)]
    pub fn enable_the_ms_interr(self) -> &'a mut W {
        self.variant(MSINTEN_A::ENABLE_THE_MS_INTERR)
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
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTEN_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLED = 1,
}
impl From<ABEOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABEOINTEN`"]
pub type ABEOINTEN_R = crate::R<bool, ABEOINTEN_A>;
impl ABEOINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOINTEN_A {
        match self.bits {
            false => ABEOINTEN_A::DISABLED,
            true => ABEOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABEOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABEOINTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `ABEOINTEN`"]
pub struct ABEOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABEOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::DISABLED)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABEOINTEN_A::ENABLED)
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
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTEN_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLED = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLED = 1,
}
impl From<ABTOINTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOINTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABTOINTEN`"]
pub type ABTOINTEN_R = crate::R<bool, ABTOINTEN_A>;
impl ABTOINTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOINTEN_A {
        match self.bits {
            false => ABTOINTEN_A::DISABLED,
            true => ABTOINTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ABTOINTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ABTOINTEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `ABTOINTEN`"]
pub struct ABTOINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTOINTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOINTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::DISABLED)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ABTOINTEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&self) -> RBRINTEN_R {
        RBRINTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&self) -> THREINTEN_R {
        THREINTEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&self) -> RLSINTEN_R {
        RLSINTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&self) -> MSINTEN_R {
        MSINTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> ABEOINTEN_R {
        ABEOINTEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> ABTOINTEN_R {
        ABTOINTEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrinten(&mut self) -> RBRINTEN_W {
        RBRINTEN_W { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threinten(&mut self) -> THREINTEN_W {
        THREINTEN_W { w: self }
    }
    #[doc = "Bit 2 - Enables the Receive Line Status interrupt. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rlsinten(&mut self) -> RLSINTEN_W {
        RLSINTEN_W { w: self }
    }
    #[doc = "Bit 3 - Enables the Modem Status interrupt. The components of this interrupt can be read from the MSR."]
    #[inline(always)]
    pub fn msinten(&mut self) -> MSINTEN_W {
        MSINTEN_W { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> ABEOINTEN_W {
        ABEOINTEN_W { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> ABTOINTEN_W {
        ABTOINTEN_W { w: self }
    }
}
