#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MOD {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RMR {
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL_THE_CAN_CONTR,
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET_CAN_OPERATION,
}
impl RMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RMR::NORMAL_THE_CAN_CONTR => false,
            RMR::RESET_CAN_OPERATION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RMR {
        match value {
            false => RMR::NORMAL_THE_CAN_CONTR,
            true => RMR::RESET_CAN_OPERATION,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONTR`"]
    #[inline]
    pub fn is_normal_the_can_contr(&self) -> bool {
        *self == RMR::NORMAL_THE_CAN_CONTR
    }
    #[doc = "Checks if the value of the field is `RESET_CAN_OPERATION`"]
    #[inline]
    pub fn is_reset_can_operation(&self) -> bool {
        *self == RMR::RESET_CAN_OPERATION
    }
}
#[doc = "Possible values of the field `LOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOMR {
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL_THE_CAN_CONT,
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY_THE_CON,
}
impl LOMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LOMR::NORMAL_THE_CAN_CONT => false,
            LOMR::LISTEN_ONLY_THE_CON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOMR {
        match value {
            false => LOMR::NORMAL_THE_CAN_CONT,
            true => LOMR::LISTEN_ONLY_THE_CON,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_THE_CAN_CONT`"]
    #[inline]
    pub fn is_normal_the_can_cont(&self) -> bool {
        *self == LOMR::NORMAL_THE_CAN_CONT
    }
    #[doc = "Checks if the value of the field is `LISTEN_ONLY_THE_CON`"]
    #[inline]
    pub fn is_listen_only_the_con(&self) -> bool {
        *self == LOMR::LISTEN_ONLY_THE_CON
    }
}
#[doc = "Possible values of the field `STM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STMR {
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL_A_TRANSMITTE,
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST_THE_CONTR,
}
impl STMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            STMR::NORMAL_A_TRANSMITTE => false,
            STMR::SELF_TEST_THE_CONTR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STMR {
        match value {
            false => STMR::NORMAL_A_TRANSMITTE,
            true => STMR::SELF_TEST_THE_CONTR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_A_TRANSMITTE`"]
    #[inline]
    pub fn is_normal_a_transmitte(&self) -> bool {
        *self == STMR::NORMAL_A_TRANSMITTE
    }
    #[doc = "Checks if the value of the field is `SELF_TEST_THE_CONTR`"]
    #[inline]
    pub fn is_self_test_the_contr(&self) -> bool {
        *self == STMR::SELF_TEST_THE_CONTR
    }
}
#[doc = "Possible values of the field `TPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPMR {
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID_THE_TRANSMIT,
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY_THE_,
}
impl TPMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TPMR::CAN_ID_THE_TRANSMIT => false,
            TPMR::LOCAL_PRIORITY_THE_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPMR {
        match value {
            false => TPMR::CAN_ID_THE_TRANSMIT,
            true => TPMR::LOCAL_PRIORITY_THE_,
        }
    }
    #[doc = "Checks if the value of the field is `CAN_ID_THE_TRANSMIT`"]
    #[inline]
    pub fn is_can_id_the_transmit(&self) -> bool {
        *self == TPMR::CAN_ID_THE_TRANSMIT
    }
    #[doc = "Checks if the value of the field is `LOCAL_PRIORITY_THE_`"]
    #[inline]
    pub fn is_local_priority_the_(&self) -> bool {
        *self == TPMR::LOCAL_PRIORITY_THE_
    }
}
#[doc = "Possible values of the field `SM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMR {
    #[doc = "Wake-up. Normal operation."]
    WAKE_UP_NORMAL_OPER,
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP_THE_CAN_CONTR,
}
impl SMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            SMR::WAKE_UP_NORMAL_OPER => false,
            SMR::SLEEP_THE_CAN_CONTR => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMR {
        match value {
            false => SMR::WAKE_UP_NORMAL_OPER,
            true => SMR::SLEEP_THE_CAN_CONTR,
        }
    }
    #[doc = "Checks if the value of the field is `WAKE_UP_NORMAL_OPER`"]
    #[inline]
    pub fn is_wake_up_normal_oper(&self) -> bool {
        *self == SMR::WAKE_UP_NORMAL_OPER
    }
    #[doc = "Checks if the value of the field is `SLEEP_THE_CAN_CONTR`"]
    #[inline]
    pub fn is_sleep_the_can_contr(&self) -> bool {
        *self == SMR::SLEEP_THE_CAN_CONTR
    }
}
#[doc = "Possible values of the field `RPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPMR {
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE_RD_INPUT,
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE_RD_INPU,
}
impl RPMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            RPMR::LOW_ACTIVE_RD_INPUT => false,
            RPMR::HIGH_ACTIVE_RD_INPU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RPMR {
        match value {
            false => RPMR::LOW_ACTIVE_RD_INPUT,
            true => RPMR::HIGH_ACTIVE_RD_INPU,
        }
    }
    #[doc = "Checks if the value of the field is `LOW_ACTIVE_RD_INPUT`"]
    #[inline]
    pub fn is_low_active_rd_input(&self) -> bool {
        *self == RPMR::LOW_ACTIVE_RD_INPUT
    }
    #[doc = "Checks if the value of the field is `HIGH_ACTIVE_RD_INPU`"]
    #[inline]
    pub fn is_high_active_rd_inpu(&self) -> bool {
        *self == RPMR::HIGH_ACTIVE_RD_INPU
    }
}
#[doc = "Possible values of the field `TM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR {
    #[doc = "Disabled. Normal operation."]
    DISABLED_NORMAL_OPE,
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED_THE_TD_PIN_,
}
impl TMR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TMR::DISABLED_NORMAL_OPE => false,
            TMR::ENABLED_THE_TD_PIN_ => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMR {
        match value {
            false => TMR::DISABLED_NORMAL_OPE,
            true => TMR::ENABLED_THE_TD_PIN_,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED_NORMAL_OPE`"]
    #[inline]
    pub fn is_disabled_normal_ope(&self) -> bool {
        *self == TMR::DISABLED_NORMAL_OPE
    }
    #[doc = "Checks if the value of the field is `ENABLED_THE_TD_PIN_`"]
    #[inline]
    pub fn is_enabled_the_td_pin_(&self) -> bool {
        *self == TMR::ENABLED_THE_TD_PIN_
    }
}
#[doc = "Values that can be written to the field `RM`"]
pub enum RMW {
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NORMAL_THE_CAN_CONTR,
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    RESET_CAN_OPERATION,
}
impl RMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RMW::NORMAL_THE_CAN_CONTR => false,
            RMW::RESET_CAN_OPERATION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RMW<'a> {
    w: &'a mut W,
}
impl<'a> _RMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline]
    pub fn normal_the_can_contr(self) -> &'a mut W {
        self.variant(RMW::NORMAL_THE_CAN_CONTR)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline]
    pub fn reset_can_operation(self) -> &'a mut W {
        self.variant(RMW::RESET_CAN_OPERATION)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOM`"]
pub enum LOMW {
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NORMAL_THE_CAN_CONT,
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    LISTEN_ONLY_THE_CON,
}
impl LOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOMW::NORMAL_THE_CAN_CONT => false,
            LOMW::LISTEN_ONLY_THE_CON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOMW<'a> {
    w: &'a mut W,
}
impl<'a> _LOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline]
    pub fn normal_the_can_cont(self) -> &'a mut W {
        self.variant(LOMW::NORMAL_THE_CAN_CONT)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline]
    pub fn listen_only_the_con(self) -> &'a mut W {
        self.variant(LOMW::LISTEN_ONLY_THE_CON)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `STM`"]
pub enum STMW {
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    NORMAL_A_TRANSMITTE,
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SELF_TEST_THE_CONTR,
}
impl STMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STMW::NORMAL_A_TRANSMITTE => false,
            STMW::SELF_TEST_THE_CONTR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STMW<'a> {
    w: &'a mut W,
}
impl<'a> _STMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline]
    pub fn normal_a_transmitte(self) -> &'a mut W {
        self.variant(STMW::NORMAL_A_TRANSMITTE)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline]
    pub fn self_test_the_contr(self) -> &'a mut W {
        self.variant(STMW::SELF_TEST_THE_CONTR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TPM`"]
pub enum TPMW {
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CAN_ID_THE_TRANSMIT,
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LOCAL_PRIORITY_THE_,
}
impl TPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPMW::CAN_ID_THE_TRANSMIT => false,
            TPMW::LOCAL_PRIORITY_THE_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPMW<'a> {
    w: &'a mut W,
}
impl<'a> _TPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline]
    pub fn can_id_the_transmit(self) -> &'a mut W {
        self.variant(TPMW::CAN_ID_THE_TRANSMIT)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline]
    pub fn local_priority_the_(self) -> &'a mut W {
        self.variant(TPMW::LOCAL_PRIORITY_THE_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SM`"]
pub enum SMW {
    #[doc = "Wake-up. Normal operation."]
    WAKE_UP_NORMAL_OPER,
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SLEEP_THE_CAN_CONTR,
}
impl SMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMW::WAKE_UP_NORMAL_OPER => false,
            SMW::SLEEP_THE_CAN_CONTR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMW<'a> {
    w: &'a mut W,
}
impl<'a> _SMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline]
    pub fn wake_up_normal_oper(self) -> &'a mut W {
        self.variant(SMW::WAKE_UP_NORMAL_OPER)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline]
    pub fn sleep_the_can_contr(self) -> &'a mut W {
        self.variant(SMW::SLEEP_THE_CAN_CONTR)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RPM`"]
pub enum RPMW {
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    LOW_ACTIVE_RD_INPUT,
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HIGH_ACTIVE_RD_INPU,
}
impl RPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RPMW::LOW_ACTIVE_RD_INPUT => false,
            RPMW::HIGH_ACTIVE_RD_INPU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RPMW<'a> {
    w: &'a mut W,
}
impl<'a> _RPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RPMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline]
    pub fn low_active_rd_input(self) -> &'a mut W {
        self.variant(RPMW::LOW_ACTIVE_RD_INPUT)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline]
    pub fn high_active_rd_inpu(self) -> &'a mut W {
        self.variant(RPMW::HIGH_ACTIVE_RD_INPU)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TM`"]
pub enum TMW {
    #[doc = "Disabled. Normal operation."]
    DISABLED_NORMAL_OPE,
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    ENABLED_THE_TD_PIN_,
}
impl TMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMW::DISABLED_NORMAL_OPE => false,
            TMW::ENABLED_THE_TD_PIN_ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMW<'a> {
    w: &'a mut W,
}
impl<'a> _TMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline]
    pub fn disabled_normal_ope(self) -> &'a mut W {
        self.variant(TMW::DISABLED_NORMAL_OPE)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline]
    pub fn enabled_the_td_pin_(self) -> &'a mut W {
        self.variant(TMW::ENABLED_THE_TD_PIN_)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Reset Mode."]
    #[inline]
    pub fn rm(&self) -> RMR {
        RMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline]
    pub fn lom(&self) -> LOMR {
        LOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline]
    pub fn stm(&self) -> STMR {
        STMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline]
    pub fn tpm(&self) -> TPMR {
        TPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline]
    pub fn sm(&self) -> SMR {
        SMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline]
    pub fn rpm(&self) -> RPMR {
        RPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline]
    pub fn tm(&self) -> TMR {
        TMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Reset Mode."]
    #[inline]
    pub fn rm(&mut self) -> _RMW {
        _RMW { w: self }
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline]
    pub fn lom(&mut self) -> _LOMW {
        _LOMW { w: self }
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline]
    pub fn stm(&mut self) -> _STMW {
        _STMW { w: self }
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline]
    pub fn tpm(&mut self) -> _TPMW {
        _TPMW { w: self }
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline]
    pub fn sm(&mut self) -> _SMW {
        _SMW { w: self }
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline]
    pub fn rpm(&mut self) -> _RPMW {
        _RPMW { w: self }
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline]
    pub fn tm(&mut self) -> _TMW {
        _TMW { w: self }
    }
}
