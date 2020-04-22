#[doc = "Writer for register CMDCODE"]
pub type W = crate::W<u32, super::CMDCODE>;
#[doc = "Register CMDCODE `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDCODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The command phase:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_PHASE_AW {
    #[doc = "2: Read"]
    READ = 2,
    #[doc = "1: Write"]
    WRITE = 1,
    #[doc = "5: Command"]
    COMMAND = 5,
}
impl From<CMD_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_PHASE_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CMD_PHASE`"]
pub struct CMD_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_PHASE_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::READ)
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::WRITE)
    }
    #[doc = "Command"]
    #[inline(always)]
    pub fn command(self) -> &'a mut W {
        self.variant(CMD_PHASE_AW::COMMAND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `CMD_CODE_WDATA`"]
pub struct CMD_CODE_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_CODE_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 8:15 - The command phase:"]
    #[inline(always)]
    pub fn cmd_phase(&mut self) -> CMD_PHASE_W {
        CMD_PHASE_W { w: self }
    }
    #[doc = "Bits 16:23 - This is a multi-purpose field. When CMD_PHASE is Command or Read, this field contains the code for the command (CMD_CODE). When CMD_PHASE is Write, this field contains the command write data (CMD_WDATA)."]
    #[inline(always)]
    pub fn cmd_code_wdata(&mut self) -> CMD_CODE_WDATA_W {
        CMD_CODE_WDATA_W { w: self }
    }
}
