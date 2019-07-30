#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENSET {
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
#[doc = r"Reader of the field"]
pub type RXRDYEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXRDYENW<'a> {
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
#[doc = r"Reader of the field"]
pub type TXRDYEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXRDYENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXRDYENW<'a> {
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
pub type TXIDLEEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXIDLEENW<'a> {
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
#[doc = r"Reader of the field"]
pub type DELTACTSEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DELTACTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTACTSENW<'a> {
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
#[doc = r"Reader of the field"]
pub type TXDISEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _TXDISENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDISENW<'a> {
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
#[doc = r"Reader of the field"]
pub type OVERRUNEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _OVERRUNENW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNENW<'a> {
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
#[doc = r"Reader of the field"]
pub type DELTARXBRKEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DELTARXBRKENW<'a> {
    w: &'a mut W,
}
impl<'a> _DELTARXBRKENW<'a> {
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
#[doc = r"Reader of the field"]
pub type STARTEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _STARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FRAMERREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FRAMERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMERRENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PARITYERREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PARITYERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYERRENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type RXNOISEEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _RXNOISEENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXNOISEENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ABERREN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ABERRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ABERRENW<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen(&self) -> RXRDYEN_R {
        RXRDYEN_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
    #[inline(always)]
    pub fn txrdyen(&self) -> TXRDYEN_R {
        TXRDYEN_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&self) -> TXIDLEEN_R {
        TXIDLEEN_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&self) -> DELTACTSEN_R {
        DELTACTSEN_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&self) -> TXDISEN_R {
        TXDISEN_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - When 1, enables an interrupt when an overrun error occurred."]
    #[inline(always)]
    pub fn overrunen(&self) -> OVERRUNEN_R {
        OVERRUNEN_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&self) -> DELTARXBRKEN_R {
        DELTARXBRKEN_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&self) -> STARTEN_R {
        STARTEN_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&self) -> FRAMERREN_R {
        FRAMERREN_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&self) -> PARITYERREN_R {
        PARITYERREN_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected."]
    #[inline(always)]
    pub fn rxnoiseen(&self) -> RXNOISEEN_R {
        RXNOISEEN_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto-baud error occurs."]
    #[inline(always)]
    pub fn aberren(&self) -> ABERREN_R {
        ABERREN_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - When 1, enables an interrupt when there is a received character available to be read from the RXDAT register."]
    #[inline(always)]
    pub fn rxrdyen(&mut self) -> _RXRDYENW {
        _RXRDYENW { w: self }
    }
    #[doc = "Bit 2 - When 1, enables an interrupt when the TXDAT register is available to take another character to transmit."]
    #[inline(always)]
    pub fn txrdyen(&mut self) -> _TXRDYENW {
        _TXRDYENW { w: self }
    }
    #[doc = "Bit 3 - When 1, enables an interrupt when the transmitter becomes idle (TXIDLE = 1)."]
    #[inline(always)]
    pub fn txidleen(&mut self) -> _TXIDLEENW {
        _TXIDLEENW { w: self }
    }
    #[doc = "Bit 5 - When 1, enables an interrupt when there is a change in the state of the CTS input."]
    #[inline(always)]
    pub fn deltactsen(&mut self) -> _DELTACTSENW {
        _DELTACTSENW { w: self }
    }
    #[doc = "Bit 6 - When 1, enables an interrupt when the transmitter is fully disabled as indicated by the TXDISINT flag in STAT. See description of the TXDISINT bit for details."]
    #[inline(always)]
    pub fn txdisen(&mut self) -> _TXDISENW {
        _TXDISENW { w: self }
    }
    #[doc = "Bit 8 - When 1, enables an interrupt when an overrun error occurred."]
    #[inline(always)]
    pub fn overrunen(&mut self) -> _OVERRUNENW {
        _OVERRUNENW { w: self }
    }
    #[doc = "Bit 11 - When 1, enables an interrupt when a change of state has occurred in the detection of a received break condition (break condition asserted or deasserted)."]
    #[inline(always)]
    pub fn deltarxbrken(&mut self) -> _DELTARXBRKENW {
        _DELTARXBRKENW { w: self }
    }
    #[doc = "Bit 12 - When 1, enables an interrupt when a received start bit has been detected."]
    #[inline(always)]
    pub fn starten(&mut self) -> _STARTENW {
        _STARTENW { w: self }
    }
    #[doc = "Bit 13 - When 1, enables an interrupt when a framing error has been detected."]
    #[inline(always)]
    pub fn framerren(&mut self) -> _FRAMERRENW {
        _FRAMERRENW { w: self }
    }
    #[doc = "Bit 14 - When 1, enables an interrupt when a parity error has been detected."]
    #[inline(always)]
    pub fn parityerren(&mut self) -> _PARITYERRENW {
        _PARITYERRENW { w: self }
    }
    #[doc = "Bit 15 - When 1, enables an interrupt when noise is detected."]
    #[inline(always)]
    pub fn rxnoiseen(&mut self) -> _RXNOISEENW {
        _RXNOISEENW { w: self }
    }
    #[doc = "Bit 16 - When 1, enables an interrupt when an auto-baud error occurs."]
    #[inline(always)]
    pub fn aberren(&mut self) -> _ABERRENW {
        _ABERRENW { w: self }
    }
}
