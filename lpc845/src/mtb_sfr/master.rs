#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MASTER {
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
pub struct MASKR {
    bits: u8,
}
impl MASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TSTARTENR {
    bits: bool,
}
impl TSTARTENR {
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
pub struct TSTOPENR {
    bits: bool,
}
impl TSTOPENR {
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
pub struct SFRWPRIVR {
    bits: bool,
}
impl SFRWPRIVR {
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
pub struct RAMPRIVR {
    bits: bool,
}
impl RAMPRIVR {
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
pub struct HALTREQR {
    bits: bool,
}
impl HALTREQR {
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
pub struct ENR {
    bits: bool,
}
impl ENR {
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
#[doc = r" Proxy"]
pub struct _MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _MASKW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTENW<'a> {
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
pub struct _TSTOPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPENW<'a> {
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
pub struct _SFRWPRIVW<'a> {
    w: &'a mut W,
}
impl<'a> _SFRWPRIVW<'a> {
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
#[doc = r" Proxy"]
pub struct _RAMPRIVW<'a> {
    w: &'a mut W,
}
impl<'a> _RAMPRIVW<'a> {
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
pub struct _HALTREQW<'a> {
    w: &'a mut W,
}
impl<'a> _HALTREQW<'a> {
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
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\] bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\] bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline]
    pub fn mask(&self) -> MASKR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MASKR { bits }
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline]
    pub fn tstarten(&self) -> TSTARTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTARTENR { bits }
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline]
    pub fn tstopen(&self) -> TSTOPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TSTOPENR { bits }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\] signal determines if an access is User or Privileged."]
    #[inline]
    pub fn sfrwpriv(&self) -> SFRWPRIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SFRWPRIVR { bits }
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\] signal determines if an access is User or Privileged."]
    #[inline]
    pub fn rampriv(&self) -> RAMPRIVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RAMPRIVR { bits }
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline]
    pub fn haltreq(&self) -> HALTREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HALTREQR { bits }
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline]
    pub fn en(&self) -> ENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\] bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\] bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline]
    pub fn mask(&mut self) -> _MASKW {
        _MASKW { w: self }
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline]
    pub fn tstarten(&mut self) -> _TSTARTENW {
        _TSTARTENW { w: self }
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline]
    pub fn tstopen(&mut self) -> _TSTOPENW {
        _TSTOPENW { w: self }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\] signal determines if an access is User or Privileged."]
    #[inline]
    pub fn sfrwpriv(&mut self) -> _SFRWPRIVW {
        _SFRWPRIVW { w: self }
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\] signal determines if an access is User or Privileged."]
    #[inline]
    pub fn rampriv(&mut self) -> _RAMPRIVW {
        _RAMPRIVW { w: self }
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline]
    pub fn haltreq(&mut self) -> _HALTREQW {
        _HALTREQW { w: self }
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
}
