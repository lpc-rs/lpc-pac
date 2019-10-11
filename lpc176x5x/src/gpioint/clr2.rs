#[doc = "Writer for register CLR2"]
pub type W = crate::W<u32, super::CLR2>;
#[doc = "Register CLR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `P2_0CI`"]
pub struct P2_0CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_0CI_W<'a> {
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
#[doc = "Write proxy for field `P2_1CI`"]
pub struct P2_1CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_1CI_W<'a> {
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
#[doc = "Write proxy for field `P2_2CI`"]
pub struct P2_2CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_2CI_W<'a> {
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
#[doc = "Write proxy for field `P2_3CI`"]
pub struct P2_3CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_3CI_W<'a> {
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
#[doc = "Write proxy for field `P2_4CI`"]
pub struct P2_4CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_4CI_W<'a> {
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
#[doc = "Write proxy for field `P2_5CI`"]
pub struct P2_5CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_5CI_W<'a> {
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
#[doc = "Write proxy for field `P2_6CI`"]
pub struct P2_6CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_6CI_W<'a> {
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
#[doc = "Write proxy for field `P2_7CI`"]
pub struct P2_7CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_7CI_W<'a> {
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
#[doc = "Write proxy for field `P2_8CI`"]
pub struct P2_8CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_8CI_W<'a> {
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
#[doc = "Write proxy for field `P2_9CI`"]
pub struct P2_9CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_9CI_W<'a> {
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
#[doc = "Write proxy for field `P2_10CI`"]
pub struct P2_10CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_10CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `P2_11CI`"]
pub struct P2_11CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_11CI_W<'a> {
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
#[doc = "Write proxy for field `P2_12CI`"]
pub struct P2_12CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_12CI_W<'a> {
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
#[doc = "Write proxy for field `P2_13CI`"]
pub struct P2_13CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_13CI_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P2\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_0ci(&mut self) -> P2_0CI_W {
        P2_0CI_W { w: self }
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P2\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_1ci(&mut self) -> P2_1CI_W {
        P2_1CI_W { w: self }
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P2\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_2ci(&mut self) -> P2_2CI_W {
        P2_2CI_W { w: self }
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P2\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_3ci(&mut self) -> P2_3CI_W {
        P2_3CI_W { w: self }
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P2\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_4ci(&mut self) -> P2_4CI_W {
        P2_4CI_W { w: self }
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P2\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_5ci(&mut self) -> P2_5CI_W {
        P2_5CI_W { w: self }
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P2\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_6ci(&mut self) -> P2_6CI_W {
        P2_6CI_W { w: self }
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P2\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_7ci(&mut self) -> P2_7CI_W {
        P2_7CI_W { w: self }
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P2\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_8ci(&mut self) -> P2_8CI_W {
        P2_8CI_W { w: self }
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P2\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_9ci(&mut self) -> P2_9CI_W {
        P2_9CI_W { w: self }
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P2\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_10ci(&mut self) -> P2_10CI_W {
        P2_10CI_W { w: self }
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P2\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_11ci(&mut self) -> P2_11CI_W {
        P2_11CI_W { w: self }
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P2\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_12ci(&mut self) -> P2_12CI_W {
        P2_12CI_W { w: self }
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P2\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p2_13ci(&mut self) -> P2_13CI_W {
        P2_13CI_W { w: self }
    }
}
