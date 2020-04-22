#[doc = "Reader of register DAO"]
pub type R = crate::R<u32, super::DAO>;
#[doc = "Writer for register DAO"]
pub type W = crate::W<u32, super::DAO>;
#[doc = "Register DAO `reset()`'s with value 0x87e1"]
impl crate::ResetValue for super::DAO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x87e1
    }
}
#[doc = "Selects the number of bytes in data as follows:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WORDWIDTH_A {
    #[doc = "0: 8-bit data"]
    _8_BIT_DATA = 0,
    #[doc = "1: 16-bit data"]
    _16_BIT_DATA = 1,
    #[doc = "3: 32-bit data"]
    _32_BIT_DATA = 3,
}
impl From<WORDWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: WORDWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WORDWIDTH`"]
pub type WORDWIDTH_R = crate::R<u8, WORDWIDTH_A>;
impl WORDWIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WORDWIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WORDWIDTH_A::_8_BIT_DATA),
            1 => Val(WORDWIDTH_A::_16_BIT_DATA),
            3 => Val(WORDWIDTH_A::_32_BIT_DATA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BIT_DATA`"]
    #[inline(always)]
    pub fn is_8_bit_data(&self) -> bool {
        *self == WORDWIDTH_A::_8_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_16_BIT_DATA`"]
    #[inline(always)]
    pub fn is_16_bit_data(&self) -> bool {
        *self == WORDWIDTH_A::_16_BIT_DATA
    }
    #[doc = "Checks if the value of the field is `_32_BIT_DATA`"]
    #[inline(always)]
    pub fn is_32_bit_data(&self) -> bool {
        *self == WORDWIDTH_A::_32_BIT_DATA
    }
}
#[doc = "Write proxy for field `WORDWIDTH`"]
pub struct WORDWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> WORDWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORDWIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_8_BIT_DATA)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_16_BIT_DATA)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bit_data(self) -> &'a mut W {
        self.variant(WORDWIDTH_A::_32_BIT_DATA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `MONO`"]
pub type MONO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONO`"]
pub struct MONO_W<'a> {
    w: &'a mut W,
}
impl<'a> MONO_W<'a> {
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
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `WS_SEL`"]
pub type WS_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WS_SEL`"]
pub struct WS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_SEL_W<'a> {
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
#[doc = "Reader of field `WS_HALFPERIOD`"]
pub type WS_HALFPERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `WS_HALFPERIOD`"]
pub struct WS_HALFPERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> WS_HALFPERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | (((value as u32) & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Reader of field `MUTE`"]
pub type MUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUTE`"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WORDWIDTH_R {
        WORDWIDTH_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WS_SEL_R {
        WS_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WS_HALFPERIOD_R {
        WS_HALFPERIOD_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> WORDWIDTH_W {
        WORDWIDTH_W { w: self }
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&mut self) -> MONO_W {
        MONO_W { w: self }
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&mut self) -> WS_SEL_W {
        WS_SEL_W { w: self }
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&mut self) -> WS_HALFPERIOD_W {
        WS_HALFPERIOD_W { w: self }
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
}
