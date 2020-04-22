#[doc = "Reader of register MOD"]
pub type R = crate::R<u32, super::MOD>;
#[doc = "Writer for register MOD"]
pub type W = crate::W<u32, super::MOD>;
#[doc = "Register MOD `reset()`'s with value 0"]
impl crate::ResetValue for super::MOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reset Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RM_A {
    #[doc = "0: Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL_THE_CAN_CONTR = 0,
    #[doc = "1: Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET_CAN_OPERATION = 1,
}
impl From<RM_A> for bool {
    #[inline(always)]
    fn from(variant: RM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RM`"]
pub type RM_R = crate::R<bool, RM_A>;
impl RM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RM_A {
        match self.bits {
            false => RM_A::NORMAL_THE_CAN_CONTR,
            true => RM_A::RESET_CAN_OPERATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONTR`"]
    #[inline(always)]
    pub fn is_normal_the_can_contr(&self) -> bool {
        *self == RM_A::NORMAL_THE_CAN_CONTR
    }
    #[doc = "Checks if the value of the field is `RESET_CAN_OPERATION`"]
    #[inline(always)]
    pub fn is_reset_can_operation(&self) -> bool {
        *self == RM_A::RESET_CAN_OPERATION
    }
}
#[doc = "Write proxy for field `RM`"]
pub struct RM_W<'a> {
    w: &'a mut W,
}
impl<'a> RM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn normal_the_can_contr(self) -> &'a mut W {
        self.variant(RM_A::NORMAL_THE_CAN_CONTR)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn reset_can_operation(self) -> &'a mut W {
        self.variant(RM_A::RESET_CAN_OPERATION)
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
#[doc = "Listen Only Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOM_A {
    #[doc = "0: Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL_THE_CAN_CONT = 0,
    #[doc = "1: Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY_THE_CON = 1,
}
impl From<LOM_A> for bool {
    #[inline(always)]
    fn from(variant: LOM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOM`"]
pub type LOM_R = crate::R<bool, LOM_A>;
impl LOM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOM_A {
        match self.bits {
            false => LOM_A::NORMAL_THE_CAN_CONT,
            true => LOM_A::LISTEN_ONLY_THE_CON,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONT`"]
    #[inline(always)]
    pub fn is_normal_the_can_cont(&self) -> bool {
        *self == LOM_A::NORMAL_THE_CAN_CONT
    }
    #[doc = "Checks if the value of the field is `LISTEN_ONLY_THE_CON`"]
    #[inline(always)]
    pub fn is_listen_only_the_con(&self) -> bool {
        *self == LOM_A::LISTEN_ONLY_THE_CON
    }
}
#[doc = "Write proxy for field `LOM`"]
pub struct LOM_W<'a> {
    w: &'a mut W,
}
impl<'a> LOM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn normal_the_can_cont(self) -> &'a mut W {
        self.variant(LOM_A::NORMAL_THE_CAN_CONT)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn listen_only_the_con(self) -> &'a mut W {
        self.variant(LOM_A::LISTEN_ONLY_THE_CON)
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
#[doc = "Self Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STM_A {
    #[doc = "0: Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL_A_TRANSMITTE = 0,
    #[doc = "1: Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST_THE_CONTR = 1,
}
impl From<STM_A> for bool {
    #[inline(always)]
    fn from(variant: STM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STM`"]
pub type STM_R = crate::R<bool, STM_A>;
impl STM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STM_A {
        match self.bits {
            false => STM_A::NORMAL_A_TRANSMITTE,
            true => STM_A::SELF_TEST_THE_CONTR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_A_TRANSMITTE`"]
    #[inline(always)]
    pub fn is_normal_a_transmitte(&self) -> bool {
        *self == STM_A::NORMAL_A_TRANSMITTE
    }
    #[doc = "Checks if the value of the field is `SELF_TEST_THE_CONTR`"]
    #[inline(always)]
    pub fn is_self_test_the_contr(&self) -> bool {
        *self == STM_A::SELF_TEST_THE_CONTR
    }
}
#[doc = "Write proxy for field `STM`"]
pub struct STM_W<'a> {
    w: &'a mut W,
}
impl<'a> STM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn normal_a_transmitte(self) -> &'a mut W {
        self.variant(STM_A::NORMAL_A_TRANSMITTE)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn self_test_the_contr(self) -> &'a mut W {
        self.variant(STM_A::SELF_TEST_THE_CONTR)
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
#[doc = "Transmit Priority Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPM_A {
    #[doc = "0: CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID_THE_TRANSMIT = 0,
    #[doc = "1: Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY_THE_ = 1,
}
impl From<TPM_A> for bool {
    #[inline(always)]
    fn from(variant: TPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TPM`"]
pub type TPM_R = crate::R<bool, TPM_A>;
impl TPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPM_A {
        match self.bits {
            false => TPM_A::CAN_ID_THE_TRANSMIT,
            true => TPM_A::LOCAL_PRIORITY_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `CAN_ID_THE_TRANSMIT`"]
    #[inline(always)]
    pub fn is_can_id_the_transmit(&self) -> bool {
        *self == TPM_A::CAN_ID_THE_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `LOCAL_PRIORITY_THE_`"]
    #[inline(always)]
    pub fn is_local_priority_the_(&self) -> bool {
        *self == TPM_A::LOCAL_PRIORITY_THE_
    }
}
#[doc = "Write proxy for field `TPM`"]
pub struct TPM_W<'a> {
    w: &'a mut W,
}
impl<'a> TPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn can_id_the_transmit(self) -> &'a mut W {
        self.variant(TPM_A::CAN_ID_THE_TRANSMIT)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn local_priority_the_(self) -> &'a mut W {
        self.variant(TPM_A::LOCAL_PRIORITY_THE_)
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
#[doc = "Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SM_A {
    #[doc = "0: Wake-up. Normal operation."]
    WAKE_UP_NORMAL_OPER = 0,
    #[doc = "1: Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP_THE_CAN_CONTR = 1,
}
impl From<SM_A> for bool {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SM`"]
pub type SM_R = crate::R<bool, SM_A>;
impl SM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            false => SM_A::WAKE_UP_NORMAL_OPER,
            true => SM_A::SLEEP_THE_CAN_CONTR,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_UP_NORMAL_OPER`"]
    #[inline(always)]
    pub fn is_wake_up_normal_oper(&self) -> bool {
        *self == SM_A::WAKE_UP_NORMAL_OPER
    }
    #[doc = "Checks if the value of the field is `SLEEP_THE_CAN_CONTR`"]
    #[inline(always)]
    pub fn is_sleep_the_can_contr(&self) -> bool {
        *self == SM_A::SLEEP_THE_CAN_CONTR
    }
}
#[doc = "Write proxy for field `SM`"]
pub struct SM_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn wake_up_normal_oper(self) -> &'a mut W {
        self.variant(SM_A::WAKE_UP_NORMAL_OPER)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn sleep_the_can_contr(self) -> &'a mut W {
        self.variant(SM_A::SLEEP_THE_CAN_CONTR)
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
#[doc = "Receive Polarity Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPM_A {
    #[doc = "0: Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE_RD_INPUT = 0,
    #[doc = "1: High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE_RD_INPU = 1,
}
impl From<RPM_A> for bool {
    #[inline(always)]
    fn from(variant: RPM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RPM`"]
pub type RPM_R = crate::R<bool, RPM_A>;
impl RPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPM_A {
        match self.bits {
            false => RPM_A::LOW_ACTIVE_RD_INPUT,
            true => RPM_A::HIGH_ACTIVE_RD_INPU,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_RD_INPUT`"]
    #[inline(always)]
    pub fn is_low_active_rd_input(&self) -> bool {
        *self == RPM_A::LOW_ACTIVE_RD_INPUT
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_RD_INPU`"]
    #[inline(always)]
    pub fn is_high_active_rd_inpu(&self) -> bool {
        *self == RPM_A::HIGH_ACTIVE_RD_INPU
    }
}
#[doc = "Write proxy for field `RPM`"]
pub struct RPM_W<'a> {
    w: &'a mut W,
}
impl<'a> RPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RPM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn low_active_rd_input(self) -> &'a mut W {
        self.variant(RPM_A::LOW_ACTIVE_RD_INPUT)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn high_active_rd_inpu(self) -> &'a mut W {
        self.variant(RPM_A::HIGH_ACTIVE_RD_INPU)
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
#[doc = "Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TM_A {
    #[doc = "0: Disabled. Normal operation."]
    DISABLED_NORMAL_OPE = 0,
    #[doc = "1: Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED_THE_TD_PIN_ = 1,
}
impl From<TM_A> for bool {
    #[inline(always)]
    fn from(variant: TM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TM`"]
pub type TM_R = crate::R<bool, TM_A>;
impl TM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TM_A {
        match self.bits {
            false => TM_A::DISABLED_NORMAL_OPE,
            true => TM_A::ENABLED_THE_TD_PIN_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_NORMAL_OPE`"]
    #[inline(always)]
    pub fn is_disabled_normal_ope(&self) -> bool {
        *self == TM_A::DISABLED_NORMAL_OPE
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_TD_PIN_`"]
    #[inline(always)]
    pub fn is_enabled_the_td_pin_(&self) -> bool {
        *self == TM_A::ENABLED_THE_TD_PIN_
    }
}
#[doc = "Write proxy for field `TM`"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn disabled_normal_ope(self) -> &'a mut W {
        self.variant(TM_A::DISABLED_NORMAL_OPE)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn enabled_the_td_pin_(self) -> &'a mut W {
        self.variant(TM_A::ENABLED_THE_TD_PIN_)
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
impl R {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&self) -> RM_R {
        RM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&self) -> LOM_R {
        LOM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&self) -> STM_R {
        STM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&self) -> TPM_R {
        TPM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&self) -> RPM_R {
        RPM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&mut self) -> RM_W {
        RM_W { w: self }
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&mut self) -> LOM_W {
        LOM_W { w: self }
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&mut self) -> STM_W {
        STM_W { w: self }
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&mut self) -> TPM_W {
        TPM_W { w: self }
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W {
        SM_W { w: self }
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&mut self) -> RPM_W {
        RPM_W { w: self }
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
}
