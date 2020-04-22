#[doc = "Reader of register MASTER"]
pub type R = crate::R<u32, super::MASTER>;
#[doc = "Writer for register MASTER"]
pub type W = crate::W<u32, super::MASTER>;
#[doc = "Register MASTER `reset()`'s with value 0x80"]
impl crate::ResetValue for super::MASTER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `MASK`"]
pub type MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MASK`"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `TSTARTEN`"]
pub type TSTARTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTARTEN`"]
pub struct TSTARTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTARTEN_W<'a> {
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
#[doc = "Reader of field `TSTOPEN`"]
pub type TSTOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSTOPEN`"]
pub struct TSTOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTOPEN_W<'a> {
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
#[doc = "Reader of field `SFRWPRIV`"]
pub type SFRWPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFRWPRIV`"]
pub struct SFRWPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFRWPRIV_W<'a> {
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
#[doc = "Reader of field `RAMPRIV`"]
pub type RAMPRIV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMPRIV`"]
pub struct RAMPRIV_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMPRIV_W<'a> {
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
#[doc = "Reader of field `HALTREQ`"]
pub type HALTREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALTREQ`"]
pub struct HALTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HALTREQ_W<'a> {
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
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn sfrwpriv(&self) -> SFRWPRIV_R {
        SFRWPRIV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn rampriv(&self) -> RAMPRIV_R {
        RAMPRIV_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline(always)]
    pub fn haltreq(&self) -> HALTREQ_R {
        HALTREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM. It specifies the most-significant bit of the POSITION.POINTER field that can be updated by automatic increment. If the trace tries to advance past this power of two, the POSITION.WRAP bit is set to 1, the POSITION.POINTER\\[MASK:0\\]
bits are set to zero, and the POSITION.POINTER\\[AWIDTH-4:MASK+1\\]
bits remain unchanged. This field causes the trace packet information to be stored in a circular buffer of size 2(MASK+4) bytes, that can be positioned in memory at multiples of this size. Valid values of this field are zero to AWIDTH-4. Values greater than the maximum have the same effect as the maximum."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Bit 5 - Trace start input enable. If this bit is 1 and the TSTART signal is HIGH, then the EN bit is set to 1. Tracing continues until a stop condition occurs."]
    #[inline(always)]
    pub fn tstarten(&mut self) -> TSTARTEN_W {
        TSTARTEN_W { w: self }
    }
    #[doc = "Bit 6 - Trace stop input enable. If this bit is 1 and the TSTOP signal is HIGH, then the EN bit is set to 0. If a trace packet is being written to memory, the write is completed before tracing is stopped."]
    #[inline(always)]
    pub fn tstopen(&mut self) -> TSTOPEN_W {
        TSTOPEN_W { w: self }
    }
    #[doc = "Bit 7 - Special Function Register Write Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the Special Function Registers are permitted. If this bit is 1, then only Privileged write accesses are permitted and User write accesses are ignored. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn sfrwpriv(&mut self) -> SFRWPRIV_W {
        SFRWPRIV_W { w: self }
    }
    #[doc = "Bit 8 - SRAM Privilege bit. If this bit is 0, then User or Privileged AHB-Lite read and write accesses to the SRAM are permitted. If this bit is 1, then only Privileged AHB-Lite read and write accesses to the SRAM are permitted and User accesses are RAZ/WI. The HPROT\\[1\\]
signal determines if an access is User or Privileged."]
    #[inline(always)]
    pub fn rampriv(&mut self) -> RAMPRIV_W {
        RAMPRIV_W { w: self }
    }
    #[doc = "Bit 9 - Halt request bit. This bit is connected to the halt request signal of the trace logic, EDBGRQ. When HALTREQ is set to 1, EDBGRQ is asserted if DBGEN is also HIGH. The HALTREQ bit can be automatically set to 1 using the FLOW.WATERMARK field."]
    #[inline(always)]
    pub fn haltreq(&mut self) -> HALTREQ_W {
        HALTREQ_W { w: self }
    }
    #[doc = "Bit 31 - Main trace enable bit. When this bit is 1 trace data is written into the SRAM memory location addressed by POSITION.POINTER. The POSITION.POINTER value auto increments after the trace data packet is written. The EN bit can be automatically set to 0 using the FLOW.WATERMARK field and the FLOW.AUTOSTOP bit. The EN bit is automatically set to 1 if the TSTARTEN bit is 1 and the TSTART signal is HIGH. The EN bit is automatically set to 0 if TSTOPEN bit is 1 and the TSTOP signal is HIGH."]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
