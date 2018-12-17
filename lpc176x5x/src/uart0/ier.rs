#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IER {
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
#[doc = "Possible values of the field `RBRIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RBRIER {
    #[doc = "Disable the RDA interrupts."]
    DISABLE_THE_RDA_INTE,
    #[doc = "Enable the RDA interrupts."]
    ENABLE_THE_RDA_INTER,
}
impl RBRIER {
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
            RBRIER::DISABLE_THE_RDA_INTE => false,
            RBRIER::ENABLE_THE_RDA_INTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RBRIER {
        match value {
            false => RBRIER::DISABLE_THE_RDA_INTE,
            true => RBRIER::ENABLE_THE_RDA_INTER,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RDA_INTE`"]
    #[inline]
    pub fn is_disable_the_rda_inte(&self) -> bool {
        *self == RBRIER::DISABLE_THE_RDA_INTE
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RDA_INTER`"]
    #[inline]
    pub fn is_enable_the_rda_inter(&self) -> bool {
        *self == RBRIER::ENABLE_THE_RDA_INTER
    }
}
#[doc = "Possible values of the field `THREIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THREIER {
    #[doc = "Disable the THRE interrupts."]
    DISABLE_THE_THRE_INT,
    #[doc = "Enable the THRE interrupts."]
    ENABLE_THE_THRE_INTE,
}
impl THREIER {
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
            THREIER::DISABLE_THE_THRE_INT => false,
            THREIER::ENABLE_THE_THRE_INTE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> THREIER {
        match value {
            false => THREIER::DISABLE_THE_THRE_INT,
            true => THREIER::ENABLE_THE_THRE_INTE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_THRE_INT`"]
    #[inline]
    pub fn is_disable_the_thre_int(&self) -> bool {
        *self == THREIER::DISABLE_THE_THRE_INT
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_THRE_INTE`"]
    #[inline]
    pub fn is_enable_the_thre_inte(&self) -> bool {
        *self == THREIER::ENABLE_THE_THRE_INTE
    }
}
#[doc = "Possible values of the field `RXIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIER {
    #[doc = "Disable the RX line status interrupts."]
    DISABLE_THE_RX_LINE_,
    #[doc = "Enable the RX line status interrupts."]
    ENABLE_THE_RX_LINE_S,
}
impl RXIER {
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
            RXIER::DISABLE_THE_RX_LINE_ => false,
            RXIER::ENABLE_THE_RX_LINE_S => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXIER {
        match value {
            false => RXIER::DISABLE_THE_RX_LINE_,
            true => RXIER::ENABLE_THE_RX_LINE_S,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_RX_LINE_`"]
    #[inline]
    pub fn is_disable_the_rx_line_(&self) -> bool {
        *self == RXIER::DISABLE_THE_RX_LINE_
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_RX_LINE_S`"]
    #[inline]
    pub fn is_enable_the_rx_line_s(&self) -> bool {
        *self == RXIER::ENABLE_THE_RX_LINE_S
    }
}
#[doc = "Possible values of the field `ABEOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABEOINTENR {
    #[doc = "Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO_,
    #[doc = "Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B,
}
impl ABEOINTENR {
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
            ABEOINTENR::DISABLE_END_OF_AUTO_ => false,
            ABEOINTENR::ENABLE_END_OF_AUTO_B => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABEOINTENR {
        match value {
            false => ABEOINTENR::DISABLE_END_OF_AUTO_,
            true => ABEOINTENR::ENABLE_END_OF_AUTO_B,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_END_OF_AUTO_`"]
    #[inline]
    pub fn is_disable_end_of_auto_(&self) -> bool {
        *self == ABEOINTENR::DISABLE_END_OF_AUTO_
    }
    #[doc = "Checks if the value of the field is `ENABLE_END_OF_AUTO_B`"]
    #[inline]
    pub fn is_enable_end_of_auto_b(&self) -> bool {
        *self == ABEOINTENR::ENABLE_END_OF_AUTO_B
    }
}
#[doc = "Possible values of the field `ABTOINTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABTOINTENR {
    #[doc = "Disable auto-baud time-out Interrupt."]
    DISABLE_AUTO_BAUD_TI,
    #[doc = "Enable auto-baud time-out Interrupt."]
    ENABLE_AUTO_BAUD_TIM,
}
impl ABTOINTENR {
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
            ABTOINTENR::DISABLE_AUTO_BAUD_TI => false,
            ABTOINTENR::ENABLE_AUTO_BAUD_TIM => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ABTOINTENR {
        match value {
            false => ABTOINTENR::DISABLE_AUTO_BAUD_TI,
            true => ABTOINTENR::ENABLE_AUTO_BAUD_TIM,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_AUTO_BAUD_TI`"]
    #[inline]
    pub fn is_disable_auto_baud_ti(&self) -> bool {
        *self == ABTOINTENR::DISABLE_AUTO_BAUD_TI
    }
    #[doc = "Checks if the value of the field is `ENABLE_AUTO_BAUD_TIM`"]
    #[inline]
    pub fn is_enable_auto_baud_tim(&self) -> bool {
        *self == ABTOINTENR::ENABLE_AUTO_BAUD_TIM
    }
}
#[doc = "Values that can be written to the field `RBRIE`"]
pub enum RBRIEW {
    #[doc = "Disable the RDA interrupts."]
    DISABLE_THE_RDA_INTE,
    #[doc = "Enable the RDA interrupts."]
    ENABLE_THE_RDA_INTER,
}
impl RBRIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RBRIEW::DISABLE_THE_RDA_INTE => false,
            RBRIEW::ENABLE_THE_RDA_INTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RBRIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RBRIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RBRIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RDA interrupts."]
    #[inline]
    pub fn disable_the_rda_inte(self) -> &'a mut W {
        self.variant(RBRIEW::DISABLE_THE_RDA_INTE)
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline]
    pub fn enable_the_rda_inter(self) -> &'a mut W {
        self.variant(RBRIEW::ENABLE_THE_RDA_INTER)
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
#[doc = "Values that can be written to the field `THREIE`"]
pub enum THREIEW {
    #[doc = "Disable the THRE interrupts."]
    DISABLE_THE_THRE_INT,
    #[doc = "Enable the THRE interrupts."]
    ENABLE_THE_THRE_INTE,
}
impl THREIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            THREIEW::DISABLE_THE_THRE_INT => false,
            THREIEW::ENABLE_THE_THRE_INTE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _THREIEW<'a> {
    w: &'a mut W,
}
impl<'a> _THREIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: THREIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the THRE interrupts."]
    #[inline]
    pub fn disable_the_thre_int(self) -> &'a mut W {
        self.variant(THREIEW::DISABLE_THE_THRE_INT)
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline]
    pub fn enable_the_thre_inte(self) -> &'a mut W {
        self.variant(THREIEW::ENABLE_THE_THRE_INTE)
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
#[doc = "Values that can be written to the field `RXIE`"]
pub enum RXIEW {
    #[doc = "Disable the RX line status interrupts."]
    DISABLE_THE_RX_LINE_,
    #[doc = "Enable the RX line status interrupts."]
    ENABLE_THE_RX_LINE_S,
}
impl RXIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXIEW::DISABLE_THE_RX_LINE_ => false,
            RXIEW::ENABLE_THE_RX_LINE_S => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline]
    pub fn disable_the_rx_line_(self) -> &'a mut W {
        self.variant(RXIEW::DISABLE_THE_RX_LINE_)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline]
    pub fn enable_the_rx_line_s(self) -> &'a mut W {
        self.variant(RXIEW::ENABLE_THE_RX_LINE_S)
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
#[doc = "Values that can be written to the field `ABEOINTEN`"]
pub enum ABEOINTENW {
    #[doc = "Disable end of auto-baud Interrupt."]
    DISABLE_END_OF_AUTO_,
    #[doc = "Enable end of auto-baud Interrupt."]
    ENABLE_END_OF_AUTO_B,
}
impl ABEOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABEOINTENW::DISABLE_END_OF_AUTO_ => false,
            ABEOINTENW::ENABLE_END_OF_AUTO_B => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABEOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABEOINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABEOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline]
    pub fn disable_end_of_auto_(self) -> &'a mut W {
        self.variant(ABEOINTENW::DISABLE_END_OF_AUTO_)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline]
    pub fn enable_end_of_auto_b(self) -> &'a mut W {
        self.variant(ABEOINTENW::ENABLE_END_OF_AUTO_B)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ABTOINTEN`"]
pub enum ABTOINTENW {
    #[doc = "Disable auto-baud time-out Interrupt."]
    DISABLE_AUTO_BAUD_TI,
    #[doc = "Enable auto-baud time-out Interrupt."]
    ENABLE_AUTO_BAUD_TIM,
}
impl ABTOINTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ABTOINTENW::DISABLE_AUTO_BAUD_TI => false,
            ABTOINTENW::ENABLE_AUTO_BAUD_TIM => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ABTOINTENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABTOINTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ABTOINTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline]
    pub fn disable_auto_baud_ti(self) -> &'a mut W {
        self.variant(ABTOINTENW::DISABLE_AUTO_BAUD_TI)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline]
    pub fn enable_auto_baud_tim(self) -> &'a mut W {
        self.variant(ABTOINTENW::ENABLE_AUTO_BAUD_TIM)
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
        const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&self) -> RBRIER {
        RBRIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
    #[inline]
    pub fn threie(&self) -> THREIER {
        THREIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
    #[inline]
    pub fn rxie(&self) -> RXIER {
        RXIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeointen(&self) -> ABEOINTENR {
        ABEOINTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtointen(&self) -> ABTOINTENR {
        ABTOINTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
    #[inline]
    pub fn rbrie(&mut self) -> _RBRIEW {
        _RBRIEW { w: self }
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
    #[inline]
    pub fn threie(&mut self) -> _THREIEW {
        _THREIEW { w: self }
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
    #[inline]
    pub fn rxie(&mut self) -> _RXIEW {
        _RXIEW { w: self }
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline]
    pub fn abeointen(&mut self) -> _ABEOINTENW {
        _ABEOINTENW { w: self }
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline]
    pub fn abtointen(&mut self) -> _ABTOINTENW {
        _ABTOINTENW { w: self }
    }
}
