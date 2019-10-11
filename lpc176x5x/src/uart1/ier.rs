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
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRIE_A {
    #[doc = "0: Disable the RDA interrupts."]
    DISABLE_THE_RDA_INTE,
    #[doc = "1: Enable the RDA interrupts."]
    ENABLE_THE_RDA_INTER,
}
impl From<RBRIE_A> for bool {
    #[inline(always)]
    fn from(variant: RBRIE_A) -> Self {
        match variant {
            RBRIE_A::DISABLE_THE_RDA_INTE => false,
            RBRIE_A::ENABLE_THE_RDA_INTER => true,
        }
    }
}
#[doc = "Reader of field `RBRIE`"]
pub type RBRIE_R = crate::R<bool, RBRIE_A>;
impl RBRIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBRIE_A {
        match self.bits {
            false => RBRIE_A::DISABLE_THE_RDA_INTE,
            true => RBRIE_A::ENABLE_THE_RDA_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == RBRIE_A::DISABLE_THE_RDA_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == RBRIE_A::ENABLE_THE_RDA_INTER
    }
}
#[doc = "Write proxy for field `RBRIE`"]
pub struct RBRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RBRIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn disable_the_rda_inte(self) -> &'a mut W {
        self.variant(RBRIE_A::DISABLE_THE_RDA_INTE)
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn enable_the_rda_inter(self) -> &'a mut W {
        self.variant(RBRIE_A::ENABLE_THE_RDA_INTER)
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
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIE_A {
    #[doc = "0: Disable the THRE interrupts."]
    DISABLE_THE_THRE_INT,
    #[doc = "1: Enable the THRE interrupts."]
    ENABLE_THE_THRE_INTE,
}
impl From<THREIE_A> for bool {
    #[inline(always)]
    fn from(variant: THREIE_A) -> Self {
        match variant {
            THREIE_A::DISABLE_THE_THRE_INT => false,
            THREIE_A::ENABLE_THE_THRE_INTE => true,
        }
    }
}
#[doc = "Reader of field `THREIE`"]
pub type THREIE_R = crate::R<bool, THREIE_A>;
impl THREIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THREIE_A {
        match self.bits {
            false => THREIE_A::DISABLE_THE_THRE_INT,
            true => THREIE_A::ENABLE_THE_THRE_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`"]
    #[inline(always)]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == THREIE_A::DISABLE_THE_THRE_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`"]
    #[inline(always)]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == THREIE_A::ENABLE_THE_THRE_INTE
    }
}
#[doc = "Write proxy for field `THREIE`"]
pub struct THREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THREIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn disable_the_thre_int(self) -> &'a mut W {
        self.variant(THREIE_A::DISABLE_THE_THRE_INT)
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn enable_the_thre_inte(self) -> &'a mut W {
        self.variant(THREIE_A::ENABLE_THE_THRE_INTE)
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
#[doc = "RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIE_A {
    #[doc = "0: Disable the RX line status interrupts."]
    DISABLE_THE_RX_LINE_,
    #[doc = "1: Enable the RX line status interrupts."]
    ENABLE_THE_RX_LINE_S,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        match variant {
            RXIE_A::DISABLE_THE_RX_LINE_ => false,
            RXIE_A::ENABLE_THE_RX_LINE_S => true,
        }
    }
}
#[doc = "Reader of field `RXIE`"]
pub type RXIE_R = crate::R<bool, RXIE_A>;
impl RXIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::DISABLE_THE_RX_LINE_,
            true => RXIE_A::ENABLE_THE_RX_LINE_S,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RX_LINE_`"]
    #[inline(always)]
    pub fn is_disable_the_rx_line_(&self) -> bool {
        *self == RXIE_A::DISABLE_THE_RX_LINE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RX_LINE_S`"]
    #[inline(always)]
    pub fn is_enable_the_rx_line_s(&self) -> bool {
        *self == RXIE_A::ENABLE_THE_RX_LINE_S
    }
}
#[doc = "Write proxy for field `RXIE`"]
pub struct RXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disable_the_rx_line_(self) -> &'a mut W {
        self.variant(RXIE_A::DISABLE_THE_RX_LINE_)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enable_the_rx_line_s(self) -> &'a mut W {
        self.variant(RXIE_A::ENABLE_THE_RX_LINE_S)
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
#[doc = "Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSIE_A {
    #[doc = "0: Disable the modem interrupt."]
    DISABLE_THE_MODEM_IN,
    #[doc = "1: Enable the modem interrupt."]
    ENABLE_THE_MODEM_INT,
}
impl From<MSIE_A> for bool {
    #[inline(always)]
    fn from(variant: MSIE_A) -> Self {
        match variant {
            MSIE_A::DISABLE_THE_MODEM_IN => false,
            MSIE_A::ENABLE_THE_MODEM_INT => true,
        }
    }
}
#[doc = "Reader of field `MSIE`"]
pub type MSIE_R = crate::R<bool, MSIE_A>;
impl MSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSIE_A {
        match self.bits {
            false => MSIE_A::DISABLE_THE_MODEM_IN,
            true => MSIE_A::ENABLE_THE_MODEM_INT,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_MODEM_IN`"]
    #[inline(always)]
    pub fn is_disable_the_modem_in(&self) -> bool {
        *self == MSIE_A::DISABLE_THE_MODEM_IN
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_MODEM_INT`"]
    #[inline(always)]
    pub fn is_enable_the_modem_int(&self) -> bool {
        *self == MSIE_A::ENABLE_THE_MODEM_INT
    }
}
#[doc = "Write proxy for field `MSIE`"]
pub struct MSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the modem interrupt."]
    #[inline(always)]
    pub fn disable_the_modem_in(self) -> &'a mut W {
        self.variant(MSIE_A::DISABLE_THE_MODEM_IN)
    }
    #[doc = "Enable the modem interrupt."]
    #[inline(always)]
    pub fn enable_the_modem_int(self) -> &'a mut W {
        self.variant(MSIE_A::ENABLE_THE_MODEM_INT)
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
#[doc = "CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    #[doc = "0: Disable the CTS interrupt."]
    DISABLE_THE_CTS_INTE,
    #[doc = "1: Enable the CTS interrupt."]
    ENABLE_THE_CTS_INTER,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        match variant {
            CTSIE_A::DISABLE_THE_CTS_INTE => false,
            CTSIE_A::ENABLE_THE_CTS_INTER => true,
        }
    }
}
#[doc = "Reader of field `CTSIE`"]
pub type CTSIE_R = crate::R<bool, CTSIE_A>;
impl CTSIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLE_THE_CTS_INTE,
            true => CTSIE_A::ENABLE_THE_CTS_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_CTS_INTE`"]
    #[inline(always)]
    pub fn is_disable_the_cts_inte(&self) -> bool {
        *self == CTSIE_A::DISABLE_THE_CTS_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_CTS_INTER`"]
    #[inline(always)]
    pub fn is_enable_the_cts_inter(&self) -> bool {
        *self == CTSIE_A::ENABLE_THE_CTS_INTER
    }
}
#[doc = "Write proxy for field `CTSIE`"]
pub struct CTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the CTS interrupt."]
    #[inline(always)]
    pub fn disable_the_cts_inte(self) -> &'a mut W {
        self.variant(CTSIE_A::DISABLE_THE_CTS_INTE)
    }
    #[doc = "Enable the CTS interrupt."]
    #[inline(always)]
    pub fn enable_the_cts_inter(self) -> &'a mut W {
        self.variant(CTSIE_A::ENABLE_THE_CTS_INTER)
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
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOIE_A {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO_,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B,
}
impl From<ABEOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABEOIE_A) -> Self {
        match variant {
            ABEOIE_A::DISABLE_END_OF_AUTO_ => false,
            ABEOIE_A::ENABLE_END_OF_AUTO_B => true,
        }
    }
}
#[doc = "Reader of field `ABEOIE`"]
pub type ABEOIE_R = crate::R<bool, ABEOIE_A>;
impl ABEOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABEOIE_A {
        match self.bits {
            false => ABEOIE_A::DISABLE_END_OF_AUTO_,
            true => ABEOIE_A::ENABLE_END_OF_AUTO_B,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_END_OF_AUTO_`"]
    #[inline(always)]
    pub fn is_disable_end_of_auto_(&self) -> bool {
        *self == ABEOIE_A::DISABLE_END_OF_AUTO_
    }
    #[doc = "Checks if the value of the field is `ENABLE_END_OF_AUTO_B`"]
    #[inline(always)]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == ABEOIE_A::ENABLE_END_OF_AUTO_B
    }
}
#[doc = "Write proxy for field `ABEOIE`"]
pub struct ABEOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABEOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABEOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disable_end_of_auto_(self) -> &'a mut W {
        self.variant(ABEOIE_A::DISABLE_END_OF_AUTO_)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enable_end_of_auto_b(self) -> &'a mut W {
        self.variant(ABEOIE_A::ENABLE_END_OF_AUTO_B)
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
pub enum ABTOIE_A {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    DISABLE_AUTO_BAUD_TI,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    ENABLE_AUTO_BAUD_TIM,
}
impl From<ABTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: ABTOIE_A) -> Self {
        match variant {
            ABTOIE_A::DISABLE_AUTO_BAUD_TI => false,
            ABTOIE_A::ENABLE_AUTO_BAUD_TIM => true,
        }
    }
}
#[doc = "Reader of field `ABTOIE`"]
pub type ABTOIE_R = crate::R<bool, ABTOIE_A>;
impl ABTOIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABTOIE_A {
        match self.bits {
            false => ABTOIE_A::DISABLE_AUTO_BAUD_TI,
            true => ABTOIE_A::ENABLE_AUTO_BAUD_TIM,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_BAUD_TI`"]
    #[inline(always)]
    pub fn is_disable_auto_baud_ti(&self) -> bool {
        *self == ABTOIE_A::DISABLE_AUTO_BAUD_TI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_BAUD_TIM`"]
    #[inline(always)]
    pub fn is_enable_auto_baud_tim(&self) -> bool {
        *self == ABTOIE_A::ENABLE_AUTO_BAUD_TIM
    }
}
#[doc = "Write proxy for field `ABTOIE`"]
pub struct ABTOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ABTOIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABTOIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disable_auto_baud_ti(self) -> &'a mut W {
        self.variant(ABTOIE_A::DISABLE_AUTO_BAUD_TI)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enable_auto_baud_tim(self) -> &'a mut W {
        self.variant(ABTOIE_A::ENABLE_AUTO_BAUD_TIM)
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RBRIE_R {
        RBRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> THREIE_R {
        THREIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    pub fn msie(&self) -> MSIE_R {
        MSIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeoie(&self) -> ABEOIE_R {
        ABEOIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtoie(&self) -> ABTOIE_R {
        ABTOIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UART1. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RBRIE_W {
        RBRIE_W { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UART1. The status of this interrupt can be read from LSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&mut self) -> THREIE_W {
        THREIE_W { w: self }
    }
    #[doc = "Bit 2 - RX Line Interrupt Enable. Enables the UART1 RX line status interrupts. The status of this interrupt can be read from LSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W { w: self }
    }
    #[doc = "Bit 3 - Modem Status Interrupt Enable. Enables the modem interrupt. The status of this interrupt can be read from MSR\\[3:0\\]."]
    #[inline(always)]
    pub fn msie(&mut self) -> MSIE_W {
        MSIE_W { w: self }
    }
    #[doc = "Bit 7 - CTS Interrupt Enable. If auto-cts mode is enabled this bit enables/disables the modem status interrupt generation on a CTS1 signal transition. If auto-cts mode is disabled a CTS1 transition will generate an interrupt if Modem Status Interrupt Enable (IER\\[3\\]) is set. In normal operation a CTS1 signal transition will generate a Modem Status Interrupt unless the interrupt has been disabled by clearing the IER\\[3\\] bit in the IER register. In auto-cts mode a transition on the CTS1 bit will trigger an interrupt only if both the IER\\[3\\] and IER\\[7\\] bits are set."]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W {
        CTSIE_W { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeoie(&mut self) -> ABEOIE_W {
        ABEOIE_W { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtoie(&mut self) -> ABTOIE_W {
        ABTOIE_W { w: self }
    }
}
