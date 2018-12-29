#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CMDCODE {
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
}
#[doc = "Values that can be written to the field `CMD_PHASE`"]
pub enum CMD_PHASEW {
    #[doc = "Read"]
    READ,
    #[doc = "Write"]
    WRITE,
    #[doc = "Command"]
    COMMAND,
}
impl CMD_PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMD_PHASEW::READ => 2,
            CMD_PHASEW::WRITE => 1,
            CMD_PHASEW::COMMAND => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_PHASEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD_PHASEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Read"]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_PHASEW::READ)
    }
    #[doc = "Write"]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_PHASEW::WRITE)
    }
    #[doc = "Command"]
    #[inline]
    pub fn command(self) -> &'a mut W {
        self.variant(CMD_PHASEW::COMMAND)
    }
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
#[doc = r" Proxy"]
pub struct _CMD_CODE_WDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD_CODE_WDATAW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline]
    pub fn cmd_phase(&mut self) -> _CMD_PHASEW {
        _CMD_PHASEW { w: self }
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline]
    pub fn cmd_code_wdata(&mut self) -> _CMD_CODE_WDATAW {
        _CMD_CODE_WDATAW { w: self }
    }
}
