#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STCTRL {
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
#[doc = r" Value of the field"]
pub struct PORT_FUNCR {
    bits: u8,
}
impl PORT_FUNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TMR_SCALER {
    bits: u8,
}
impl TMR_SCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TMR_MODER {
    bits: bool,
}
impl TMR_MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TMR_ENR {
    bits: bool,
}
impl TMR_ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TMR_RSTR {
    bits: bool,
}
impl TMR_RSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct B_HNP_TRACKR {
    bits: bool,
}
impl B_HNP_TRACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct A_HNP_TRACKR {
    bits: bool,
}
impl A_HNP_TRACKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct PU_REMOVEDR {
    bits: bool,
}
impl PU_REMOVEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct TMR_CNTR {
    bits: u16,
}
impl TMR_CNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PORT_FUNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PORT_FUNCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMR_SCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_SCALEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMR_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_MODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _TMR_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_ENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TMR_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_RSTW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _B_HNP_TRACKW<'a> {
    w: &'a mut W,
}
impl<'a> _B_HNP_TRACKW<'a> {
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
#[doc = r" Proxy"]
pub struct _A_HNP_TRACKW<'a> {
    w: &'a mut W,
}
impl<'a> _A_HNP_TRACKW<'a> {
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
#[doc = r" Proxy"]
pub struct _PU_REMOVEDW<'a> {
    w: &'a mut W,
}
impl<'a> _PU_REMOVEDW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TMR_CNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TMR_CNTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline]
    pub fn port_func(&self) -> PORT_FUNCR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PORT_FUNCR { bits }
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline]
    pub fn tmr_scale(&self) -> TMR_SCALER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMR_SCALER { bits }
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline]
    pub fn tmr_mode(&self) -> TMR_MODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR_MODER { bits }
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline]
    pub fn tmr_en(&self) -> TMR_ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR_ENR { bits }
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline]
    pub fn tmr_rst(&self) -> TMR_RSTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMR_RSTR { bits }
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn b_hnp_track(&self) -> B_HNP_TRACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        B_HNP_TRACKR { bits }
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn a_hnp_track(&self) -> A_HNP_TRACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        A_HNP_TRACKR { bits }
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn pu_removed(&self) -> PU_REMOVEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PU_REMOVEDR { bits }
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline]
    pub fn tmr_cnt(&self) -> TMR_CNTR {
        let bits = {
            const MASK: u16 = 65535;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TMR_CNTR { bits }
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
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline]
    pub fn port_func(&mut self) -> _PORT_FUNCW {
        _PORT_FUNCW { w: self }
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline]
    pub fn tmr_scale(&mut self) -> _TMR_SCALEW {
        _TMR_SCALEW { w: self }
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline]
    pub fn tmr_mode(&mut self) -> _TMR_MODEW {
        _TMR_MODEW { w: self }
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline]
    pub fn tmr_en(&mut self) -> _TMR_ENW {
        _TMR_ENW { w: self }
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline]
    pub fn tmr_rst(&mut self) -> _TMR_RSTW {
        _TMR_RSTW { w: self }
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn b_hnp_track(&mut self) -> _B_HNP_TRACKW {
        _B_HNP_TRACKW { w: self }
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn a_hnp_track(&mut self) -> _A_HNP_TRACKW {
        _A_HNP_TRACKW { w: self }
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline]
    pub fn pu_removed(&mut self) -> _PU_REMOVEDW {
        _PU_REMOVEDW { w: self }
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline]
    pub fn tmr_cnt(&mut self) -> _TMR_CNTW {
        _TMR_CNTW { w: self }
    }
}
