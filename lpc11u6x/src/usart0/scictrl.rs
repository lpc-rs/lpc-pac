#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCICTRL {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
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
impl crate::ToBits<bool> for SCIENR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SCIENR::SMART_CARD_INTERFACE => false,
            SCIENR::ASYNCHRONOUS_HALF_DU => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SCIEN_R = crate::FR<bool, SCIENR>;
impl SCIEN_R {
    #[doc = "Checks if the value of the field is `SMART_CARD_INTERFACE`"]
    #[inline(always)]
    pub fn is_smart_card_interface(&self) -> bool {
        *self == SCIENR::SMART_CARD_INTERFACE
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_HALF_DU`"]
    #[inline(always)]
    pub fn is_asynchronous_half_du(&self) -> bool {
        *self == SCIENR::ASYNCHRONOUS_HALF_DU
    }
}
#[doc = "Values that can be written to the field `SCIEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCIENW {
    #[doc = "Smart card interface disabled."]
    SMART_CARD_INTERFACE,
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    ASYNCHRONOUS_HALF_DU,
}
impl SCIENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SCIENW::SMART_CARD_INTERFACE => false,
            SCIENW::ASYNCHRONOUS_HALF_DU => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SCIENW<'a> {
    w: &'a mut W,
}
impl<'a> _SCIENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCIENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn smart_card_interface(self) -> &'a mut W {
        self.variant(SCIENW::SMART_CARD_INTERFACE)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn asynchronous_half_du(self) -> &'a mut W {
        self.variant(SCIENW::ASYNCHRONOUS_HALF_DU)
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
#[doc = "Possible values of the field `NACKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDISR {
    #[doc = "A NACK response is enabled."]
    ENABLED,
    #[doc = "A NACK response is inhibited."]
    DISABLED,
}
impl crate::ToBits<bool> for NACKDISR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            NACKDISR::ENABLED => false,
            NACKDISR::DISABLED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NACKDIS_R = crate::FR<bool, NACKDISR>;
impl NACKDIS_R {
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKDISR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKDISR::DISABLED
    }
}
#[doc = "Values that can be written to the field `NACKDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACKDISW {
    #[doc = "A NACK response is enabled."]
    ENABLED,
    #[doc = "A NACK response is inhibited."]
    DISABLED,
}
impl NACKDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            NACKDISW::ENABLED => false,
            NACKDISW::DISABLED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NACKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _NACKDISW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACKDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKDISW::ENABLED)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKDISW::DISABLED)
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
#[doc = "Possible values of the field `PROTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSELR {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl crate::ToBits<bool> for PROTSELR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            PROTSELR::T_EQ_0 => false,
            PROTSELR::T_EQ_1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PROTSEL_R = crate::FR<bool, PROTSELR>;
impl PROTSEL_R {
    #[doc = "Checks if the value of the field is `T_EQ_0`"]
    #[inline(always)]
    pub fn is_t_eq_0(&self) -> bool {
        *self == PROTSELR::T_EQ_0
    }
    #[doc = "Checks if the value of the field is `T_EQ_1`"]
    #[inline(always)]
    pub fn is_t_eq_1(&self) -> bool {
        *self == PROTSELR::T_EQ_1
    }
}
#[doc = "Values that can be written to the field `PROTSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTSELW {
    #[doc = "T = 0"]
    T_EQ_0,
    #[doc = "T = 1"]
    T_EQ_1,
}
impl PROTSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            PROTSELW::T_EQ_0 => false,
            PROTSELW::T_EQ_1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PROTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PROTSELW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn t_eq_0(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_0)
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn t_eq_1(self) -> &'a mut W {
        self.variant(PROTSELW::T_EQ_1)
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
#[doc = r"Reader of the field"]
pub type TXRETRY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _TXRETRYW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRETRYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type XTRAGUARD_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _XTRAGUARDW<'a> {
    w: &'a mut W,
}
impl<'a> _XTRAGUARDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> SCIEN_R {
        SCIEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&self) -> NACKDIS_R {
        NACKDIS_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> PROTSEL_R {
        PROTSEL_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    pub fn txretry(&self) -> TXRETRY_R {
        TXRETRY_R::new(((self.bits() >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    pub fn xtraguard(&self) -> XTRAGUARD_R {
        XTRAGUARD_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> _SCIENW {
        _SCIENW { w: self }
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> _NACKDISW {
        _NACKDISW { w: self }
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> _PROTSELW {
        _PROTSELW { w: self }
    }
    #[doc = "Bits 5:7 - When the protocol selection T bit (above) is 0, the field controls the maximum number of retransmissions that the USART will attempt if the remote device signals NACK. When NACK has occurred this number of times plus one, the Tx Error bit in the LSR is set, an interrupt is requested if enabled, and the USART is locked until the FIFO is cleared."]
    #[inline(always)]
    pub fn txretry(&mut self) -> _TXRETRYW {
        _TXRETRYW { w: self }
    }
    #[doc = "Bits 8:15 - When the protocol selection T bit (above) is 0, this field indicates the number of bit times (ETUs) by which the guard time after a character transmitted by the USART should exceed the nominal 2 bit times. 0xFF in this field may indicate that there is just a single bit after a character and 11 bit times/character"]
    #[inline(always)]
    pub fn xtraguard(&mut self) -> _XTRAGUARDW {
        _XTRAGUARDW { w: self }
    }
}
