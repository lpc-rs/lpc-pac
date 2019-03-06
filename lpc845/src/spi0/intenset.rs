#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = "Possible values of the field `RXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDYENR {
    #[doc = "No interrupt will be generated when receiver data is available."]
    RXRDYEN_0,
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    RXRDYEN_1,
}
impl RXRDYENR {
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
            RXRDYENR::RXRDYEN_0 => false,
            RXRDYENR::RXRDYEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXRDYENR {
        match value {
            false => RXRDYENR::RXRDYEN_0,
            true => RXRDYENR::RXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_0`"]
    #[inline]
    pub fn is_rxrdyen_0(&self) -> bool {
        *self == RXRDYENR::RXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `RXRDYEN_1`"]
    #[inline]
    pub fn is_rxrdyen_1(&self) -> bool {
        *self == RXRDYENR::RXRDYEN_1
    }
}
#[doc = "Possible values of the field `TXRDYEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDYENR {
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    TXRDYEN_0,
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    TXRDYEN_1,
}
impl TXRDYENR {
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
            TXRDYENR::TXRDYEN_0 => false,
            TXRDYENR::TXRDYEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXRDYENR {
        match value {
            false => TXRDYENR::TXRDYEN_0,
            true => TXRDYENR::TXRDYEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_0`"]
    #[inline]
    pub fn is_txrdyen_0(&self) -> bool {
        *self == TXRDYENR::TXRDYEN_0
    }
    #[doc = "Checks if the value of the field is `TXRDYEN_1`"]
    #[inline]
    pub fn is_txrdyen_1(&self) -> bool {
        *self == TXRDYENR::TXRDYEN_1
    }
}
#[doc = "Possible values of the field `RXOVEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOVENR {
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    RXOVEN_0,
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    RXOVEN_1,
}
impl RXOVENR {
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
            RXOVENR::RXOVEN_0 => false,
            RXOVENR::RXOVEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXOVENR {
        match value {
            false => RXOVENR::RXOVEN_0,
            true => RXOVENR::RXOVEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `RXOVEN_0`"]
    #[inline]
    pub fn is_rxoven_0(&self) -> bool {
        *self == RXOVENR::RXOVEN_0
    }
    #[doc = "Checks if the value of the field is `RXOVEN_1`"]
    #[inline]
    pub fn is_rxoven_1(&self) -> bool {
        *self == RXOVENR::RXOVEN_1
    }
}
#[doc = "Possible values of the field `TXUREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXURENR {
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    TXUREN_0,
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    TXUREN_1,
}
impl TXURENR {
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
            TXURENR::TXUREN_0 => false,
            TXURENR::TXUREN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXURENR {
        match value {
            false => TXURENR::TXUREN_0,
            true => TXURENR::TXUREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TXUREN_0`"]
    #[inline]
    pub fn is_txuren_0(&self) -> bool {
        *self == TXURENR::TXUREN_0
    }
    #[doc = "Checks if the value of the field is `TXUREN_1`"]
    #[inline]
    pub fn is_txuren_1(&self) -> bool {
        *self == TXURENR::TXUREN_1
    }
}
#[doc = "Possible values of the field `SSAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSAENR {
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_0,
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_1,
}
impl SSAENR {
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
            SSAENR::SSAEN_0 => false,
            SSAENR::SSAEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSAENR {
        match value {
            false => SSAENR::SSAEN_0,
            true => SSAENR::SSAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSAEN_0`"]
    #[inline]
    pub fn is_ssaen_0(&self) -> bool {
        *self == SSAENR::SSAEN_0
    }
    #[doc = "Checks if the value of the field is `SSAEN_1`"]
    #[inline]
    pub fn is_ssaen_1(&self) -> bool {
        *self == SSAENR::SSAEN_1
    }
}
#[doc = "Possible values of the field `SSDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSDENR {
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_0,
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_1,
}
impl SSDENR {
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
            SSDENR::SSDEN_0 => false,
            SSDENR::SSDEN_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSDENR {
        match value {
            false => SSDENR::SSDEN_0,
            true => SSDENR::SSDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SSDEN_0`"]
    #[inline]
    pub fn is_ssden_0(&self) -> bool {
        *self == SSDENR::SSDEN_0
    }
    #[doc = "Checks if the value of the field is `SSDEN_1`"]
    #[inline]
    pub fn is_ssden_1(&self) -> bool {
        *self == SSDENR::SSDEN_1
    }
}
#[doc = "Values that can be written to the field `RXRDYEN`"]
pub enum RXRDYENW {
    #[doc = "No interrupt will be generated when receiver data is available."]
    RXRDYEN_0,
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    RXRDYEN_1,
}
impl RXRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXRDYENW::RXRDYEN_0 => false,
            RXRDYENW::RXRDYEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when receiver data is available."]
    #[inline]
    pub fn rxrdyen_0(self) -> &'a mut W {
        self.variant(RXRDYENW::RXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when receiver data is available in the RXDAT register."]
    #[inline]
    pub fn rxrdyen_1(self) -> &'a mut W {
        self.variant(RXRDYENW::RXRDYEN_1)
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
#[doc = "Values that can be written to the field `TXRDYEN`"]
pub enum TXRDYENW {
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    TXRDYEN_0,
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    TXRDYEN_1,
}
impl TXRDYENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXRDYENW::TXRDYEN_0 => false,
            TXRDYENW::TXRDYEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXRDYENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter holding register is available."]
    #[inline]
    pub fn txrdyen_0(self) -> &'a mut W {
        self.variant(TXRDYENW::TXRDYEN_0)
    }
    #[doc = "An interrupt will be generated when data may be written to TXDAT."]
    #[inline]
    pub fn txrdyen_1(self) -> &'a mut W {
        self.variant(TXRDYENW::TXRDYEN_1)
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
#[doc = "Values that can be written to the field `RXOVEN`"]
pub enum RXOVENW {
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    RXOVEN_0,
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    RXOVEN_1,
}
impl RXOVENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXOVENW::RXOVEN_0 => false,
            RXOVENW::RXOVEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXOVENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOVENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXOVENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when a receiver overrun occurs."]
    #[inline]
    pub fn rxoven_0(self) -> &'a mut W {
        self.variant(RXOVENW::RXOVEN_0)
    }
    #[doc = "An interrupt will be generated if a receiver overrun occurs."]
    #[inline]
    pub fn rxoven_1(self) -> &'a mut W {
        self.variant(RXOVENW::RXOVEN_1)
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
#[doc = "Values that can be written to the field `TXUREN`"]
pub enum TXURENW {
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    TXUREN_0,
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    TXUREN_1,
}
impl TXURENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXURENW::TXUREN_0 => false,
            TXURENW::TXUREN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXURENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXURENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when the transmitter underruns."]
    #[inline]
    pub fn txuren_0(self) -> &'a mut W {
        self.variant(TXURENW::TXUREN_0)
    }
    #[doc = "An interrupt will be generated if the transmitter underruns."]
    #[inline]
    pub fn txuren_1(self) -> &'a mut W {
        self.variant(TXURENW::TXUREN_1)
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
#[doc = "Values that can be written to the field `SSAEN`"]
pub enum SSAENW {
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_0,
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    SSAEN_1,
}
impl SSAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSAENW::SSAEN_0 => false,
            SSAENW::SSAEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn ssaen_0(self) -> &'a mut W {
        self.variant(SSAENW::SSAEN_0)
    }
    #[doc = "An interrupt will be generated when any Slave Select transitions from deasserted to asserted."]
    #[inline]
    pub fn ssaen_1(self) -> &'a mut W {
        self.variant(SSAENW::SSAEN_1)
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
#[doc = "Values that can be written to the field `SSDEN`"]
pub enum SSDENW {
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_0,
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    SSDEN_1,
}
impl SSDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSDENW::SSDEN_0 => false,
            SSDENW::SSDEN_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSDENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn ssden_0(self) -> &'a mut W {
        self.variant(SSDENW::SSDEN_0)
    }
    #[doc = "An interrupt will be generated when all asserted Slave Selects transition to deasserted."]
    #[inline]
    pub fn ssden_1(self) -> &'a mut W {
        self.variant(SSDENW::SSDEN_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline]
    pub fn rxrdyen(&self) -> RXRDYENR {
        RXRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline]
    pub fn txrdyen(&self) -> TXRDYENR {
        TXRDYENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline]
    pub fn rxoven(&self) -> RXOVENR {
        RXOVENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline]
    pub fn txuren(&self) -> TXURENR {
        TXURENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&self) -> SSAENR {
        SSAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline]
    pub fn ssden(&self) -> SSDENR {
        SSDENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Determines whether an interrupt occurs when receiver data is available."]
    #[inline]
    pub fn rxrdyen(&mut self) -> _RXRDYENW {
        _RXRDYENW { w: self }
    }
    #[doc = "Bit 1 - Determines whether an interrupt occurs when the transmitter holding register is available."]
    #[inline]
    pub fn txrdyen(&mut self) -> _TXRDYENW {
        _TXRDYENW { w: self }
    }
    #[doc = "Bit 2 - Determines whether an interrupt occurs when a receiver overrun occurs. This happens in slave mode when there is a need for the receiver to move newly received data to the RXDAT register when it is already in use. The interface prevents receiver overrun in Master mode by not allowing a new transmission to begin when a receiver overrun would otherwise occur."]
    #[inline]
    pub fn rxoven(&mut self) -> _RXOVENW {
        _RXOVENW { w: self }
    }
    #[doc = "Bit 3 - Determines whether an interrupt occurs when a transmitter underrun occurs. This happens in slave mode when there is a need to transmit data when none is available."]
    #[inline]
    pub fn txuren(&mut self) -> _TXURENW {
        _TXURENW { w: self }
    }
    #[doc = "Bit 4 - Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline]
    pub fn ssaen(&mut self) -> _SSAENW {
        _SSAENW { w: self }
    }
    #[doc = "Bit 5 - Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline]
    pub fn ssden(&mut self) -> _SSDENW {
        _SSDENW { w: self }
    }
}
