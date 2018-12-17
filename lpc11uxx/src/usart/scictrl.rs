#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCICTRL {
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
#[doc = "Possible values of the field `SCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCIENR {
    #[doc = "Smart card interface disabled."]
    SMART_CARD_INTERFACE,
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    ASYNCHRONOUS_HALF_DU,
}
impl SCIENR {
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
            SCIENR::SMART_CARD_INTERFACE => false,
            SCIENR::ASYNCHRONOUS_HALF_DU => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SCIENR {
        match value {
            false => SCIENR::SMART_CARD_INTERFACE,
            true => SCIENR::ASYNCHRONOUS_HALF_DU,
        }
    }
    #[doc = "Checks if the value of the field is `SMART_CARD_INTERFACE`"]
    #[inline]
    pub fn is_smart_card_interface(&self) -> bool {
        *self == SCIENR::SMART_CARD_INTERFACE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_HALF_DU`"]
    #[inline]
    pub fn is_asynchronous_half_du(&self) -> bool {
        *self == SCIENR::ASYNCHRONOUS_HALF_DU
    }
}
#[doc = "Possible values of the field `NACKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDISR {
    #[doc = "A NACK response is enabled."]
    ENABLED,
    #[doc = "A NACK response is inhibited."]
    DISABLED,
}
impl NACKDISR {
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
            NACKDISR::ENABLED => false,
            NACKDISR::DISABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NACKDISR {
        match value {
            false => NACKDISR::ENABLED,
            true => NACKDISR::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == NACKDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == NACKDISR::DISABLED
    }
}
#[doc = "Possible values of the field `PROTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSELR {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl PROTSELR {
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
            PROTSELR::T_EQ_0 => false,
            PROTSELR::T_EQ_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PROTSELR {
        match value {
            false => PROTSELR::T_EQ_0,
            true => PROTSELR::T_EQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline]
    pub fn is_t_eq_0(&self) -> bool {
        *self == PROTSELR::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline]
    pub fn is_t_eq_1(&self) -> bool {
        *self == PROTSELR::T_EQ_1
    }
}
#[doc = r" Value of the field"]
pub struct TXRETRYR {
    bits: u8,
}
impl TXRETRYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct XTRAGUARDR {
    bits: u8,
}
impl XTRAGUARDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `SCIEN`"]
pub enum SCIENW {
    #[doc = "Smart card interface disabled."]
    SMART_CARD_INTERFACE,
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    ASYNCHRONOUS_HALF_DU,
}
impl SCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SCIENW::SMART_CARD_INTERFACE => false,
            SCIENW::ASYNCHRONOUS_HALF_DU => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCIENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart card interface disabled."]
    #[inline]
    pub fn smart_card_interface(self) -> &'a mut W {
        self.variant(SCIENW::SMART_CARD_INTERFACE)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline]
    pub fn asynchronous_half_du(self) -> &'a mut W {
        self.variant(SCIENW::ASYNCHRONOUS_HALF_DU)
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
#[doc = "Values that can be written to the field `NACKDIS`"]
pub enum NACKDISW {
    #[doc = "A NACK response is enabled."]
    ENABLED,
    #[doc = "A NACK response is inhibited."]
    DISABLED,
}
impl NACKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKDISW::ENABLED => false,
            NACKDISW::DISABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NACKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NACKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A NACK response is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKDISW::ENABLED)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKDISW::DISABLED)
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
#[doc = "Values that can be written to the field `PROTSEL`"]
pub enum PROTSELW {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl PROTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PROTSELW::T_EQ_0 => false,
            PROTSELW::T_EQ_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PROTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PROTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "T = 0"]
    #[inline]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_1)
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
#[doc = r" Proxy"]
pub struct _TXRETRYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRETRYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _XTRAGUARDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTRAGUARDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline]
    pub fn scien(&self) -> SCIENR {
        SCIENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline]
    pub fn nackdis(&self) -> NACKDISR {
        NACKDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline]
    pub fn protsel(&self) -> PROTSELR {
        PROTSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline]
    pub fn txretry(&self) -> TXRETRYR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXRETRYR { bits }
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline]
    pub fn xtraguard(&self) -> XTRAGUARDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        XTRAGUARDR { bits }
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
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline]
    pub fn scien(&mut self) -> _SCIENW {
        _SCIENW { w: self }
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline]
    pub fn nackdis(&mut self) -> _NACKDISW {
        _NACKDISW { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline]
    pub fn protsel(&mut self) -> _PROTSELW {
        _PROTSELW { w: self }
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline]
    pub fn txretry(&mut self) -> _TXRETRYW {
        _TXRETRYW { w: self }
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline]
    pub fn xtraguard(&mut self) -> _XTRAGUARDW {
        _XTRAGUARDW { w: self }
    }
}
