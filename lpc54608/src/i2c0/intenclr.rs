#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MSTPENDINGCLR`"]
pub struct MSTPENDINGCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTPENDINGCLR_W<'a> {
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
#[doc = "Write proxy for field `MSTARBLOSSCLR`"]
pub struct MSTARBLOSSCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTARBLOSSCLR_W<'a> {
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
#[doc = "Write proxy for field `MSTSTSTPERRCLR`"]
pub struct MSTSTSTPERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTSTSTPERRCLR_W<'a> {
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
#[doc = "Write proxy for field `SLVPENDINGCLR`"]
pub struct SLVPENDINGCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVPENDINGCLR_W<'a> {
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
#[doc = "Write proxy for field `SLVNOTSTRCLR`"]
pub struct SLVNOTSTRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVNOTSTRCLR_W<'a> {
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
#[doc = "Write proxy for field `SLVDESELCLR`"]
pub struct SLVDESELCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLVDESELCLR_W<'a> {
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
#[doc = "Write proxy for field `MONRDYCLR`"]
pub struct MONRDYCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MONRDYCLR_W<'a> {
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
#[doc = "Write proxy for field `MONOVCLR`"]
pub struct MONOVCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MONOVCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `MONIDLECLR`"]
pub struct MONIDLECLR_W<'a> {
    w: &'a mut W,
}
impl<'a> MONIDLECLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `EVENTTIMEOUTCLR`"]
pub struct EVENTTIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTTIMEOUTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `SCLTIMEOUTCLR`"]
pub struct SCLTIMEOUTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLTIMEOUTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub fn mstpendingclr(&mut self) -> MSTPENDINGCLR_W {
        MSTPENDINGCLR_W { w: self }
    }
    #[doc = "Bit 4 - Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub fn mstarblossclr(&mut self) -> MSTARBLOSSCLR_W {
        MSTARBLOSSCLR_W { w: self }
    }
    #[doc = "Bit 6 - Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub fn mstststperrclr(&mut self) -> MSTSTSTPERRCLR_W {
        MSTSTSTPERRCLR_W { w: self }
    }
    #[doc = "Bit 8 - Slave Pending interrupt clear."]
    #[inline(always)]
    pub fn slvpendingclr(&mut self) -> SLVPENDINGCLR_W {
        SLVPENDINGCLR_W { w: self }
    }
    #[doc = "Bit 11 - Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub fn slvnotstrclr(&mut self) -> SLVNOTSTRCLR_W {
        SLVNOTSTRCLR_W { w: self }
    }
    #[doc = "Bit 15 - Slave Deselect interrupt clear."]
    #[inline(always)]
    pub fn slvdeselclr(&mut self) -> SLVDESELCLR_W {
        SLVDESELCLR_W { w: self }
    }
    #[doc = "Bit 16 - Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub fn monrdyclr(&mut self) -> MONRDYCLR_W {
        MONRDYCLR_W { w: self }
    }
    #[doc = "Bit 17 - Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub fn monovclr(&mut self) -> MONOVCLR_W {
        MONOVCLR_W { w: self }
    }
    #[doc = "Bit 19 - Monitor Idle interrupt clear."]
    #[inline(always)]
    pub fn monidleclr(&mut self) -> MONIDLECLR_W {
        MONIDLECLR_W { w: self }
    }
    #[doc = "Bit 24 - Event time-out interrupt clear."]
    #[inline(always)]
    pub fn eventtimeoutclr(&mut self) -> EVENTTIMEOUTCLR_W {
        EVENTTIMEOUTCLR_W { w: self }
    }
    #[doc = "Bit 25 - SCL time-out interrupt clear."]
    #[inline(always)]
    pub fn scltimeoutclr(&mut self) -> SCLTIMEOUTCLR_W {
        SCLTIMEOUTCLR_W { w: self }
    }
}
