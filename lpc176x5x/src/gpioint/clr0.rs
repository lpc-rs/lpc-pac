#[doc = "Writer for register CLR0"]
pub type W = crate::W<u32, super::CLR0>;
#[doc = "Register CLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `P0_0CI`"]
pub struct P0_0CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_0CI_W<'a> {
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
#[doc = "Write proxy for field `P0_1CI`"]
pub struct P0_1CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_1CI_W<'a> {
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
#[doc = "Write proxy for field `P0_2CI`"]
pub struct P0_2CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_2CI_W<'a> {
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
#[doc = "Write proxy for field `P0_3CI`"]
pub struct P0_3CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_3CI_W<'a> {
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
#[doc = "Write proxy for field `P0_4CI`"]
pub struct P0_4CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_4CI_W<'a> {
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
#[doc = "Write proxy for field `P0_5CI`"]
pub struct P0_5CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_5CI_W<'a> {
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
#[doc = "Write proxy for field `P0_6CI`"]
pub struct P0_6CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_6CI_W<'a> {
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
#[doc = "Write proxy for field `P0_7CI`"]
pub struct P0_7CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_7CI_W<'a> {
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
#[doc = "Write proxy for field `P0_8CI`"]
pub struct P0_8CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_8CI_W<'a> {
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
#[doc = "Write proxy for field `P0_9CI`"]
pub struct P0_9CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_9CI_W<'a> {
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
#[doc = "Write proxy for field `P0_10CI`"]
pub struct P0_10CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_10CI_W<'a> {
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
#[doc = "Write proxy for field `P0_11CI`"]
pub struct P0_11CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_11CI_W<'a> {
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
#[doc = "Write proxy for field `P0_12CI`"]
pub struct P0_12CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_12CI_W<'a> {
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
#[doc = "Write proxy for field `P0_13CI`"]
pub struct P0_13CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_13CI_W<'a> {
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
#[doc = "Write proxy for field `P0_14CI`"]
pub struct P0_14CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_14CI_W<'a> {
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
#[doc = "Write proxy for field `P0_15CI`"]
pub struct P0_15CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_15CI_W<'a> {
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
#[doc = "Write proxy for field `P0_16CI`"]
pub struct P0_16CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_16CI_W<'a> {
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
#[doc = "Write proxy for field `P0_17CI`"]
pub struct P0_17CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_17CI_W<'a> {
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
#[doc = "Write proxy for field `P0_18CI`"]
pub struct P0_18CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_18CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `P0_19CI`"]
pub struct P0_19CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_19CI_W<'a> {
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
#[doc = "Write proxy for field `P0_20CI`"]
pub struct P0_20CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_20CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `P0_21CI`"]
pub struct P0_21CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_21CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `P0_22CI`"]
pub struct P0_22CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_22CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `P0_23CI`"]
pub struct P0_23CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_23CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Write proxy for field `P0_24CI`"]
pub struct P0_24CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_24CI_W<'a> {
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
#[doc = "Write proxy for field `P0_25CI`"]
pub struct P0_25CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_25CI_W<'a> {
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
#[doc = "Write proxy for field `P0_26CI`"]
pub struct P0_26CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_26CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `P0_27CI`"]
pub struct P0_27CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_27CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Write proxy for field `P0_28CI`"]
pub struct P0_28CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_28CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `P0_29CI`"]
pub struct P0_29CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_29CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `P0_30CI`"]
pub struct P0_30CI_W<'a> {
    w: &'a mut W,
}
impl<'a> P0_30CI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear GPIO port Interrupts for P0\\[0\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_0ci(&mut self) -> P0_0CI_W {
        P0_0CI_W { w: self }
    }
    #[doc = "Bit 1 - Clear GPIO port Interrupts for P0\\[1\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_1ci(&mut self) -> P0_1CI_W {
        P0_1CI_W { w: self }
    }
    #[doc = "Bit 2 - Clear GPIO port Interrupts for P0\\[2\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_2ci(&mut self) -> P0_2CI_W {
        P0_2CI_W { w: self }
    }
    #[doc = "Bit 3 - Clear GPIO port Interrupts for P0\\[3\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_3ci(&mut self) -> P0_3CI_W {
        P0_3CI_W { w: self }
    }
    #[doc = "Bit 4 - Clear GPIO port Interrupts for P0\\[4\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_4ci(&mut self) -> P0_4CI_W {
        P0_4CI_W { w: self }
    }
    #[doc = "Bit 5 - Clear GPIO port Interrupts for P0\\[5\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_5ci(&mut self) -> P0_5CI_W {
        P0_5CI_W { w: self }
    }
    #[doc = "Bit 6 - Clear GPIO port Interrupts for P0\\[6\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_6ci(&mut self) -> P0_6CI_W {
        P0_6CI_W { w: self }
    }
    #[doc = "Bit 7 - Clear GPIO port Interrupts for P0\\[7\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_7ci(&mut self) -> P0_7CI_W {
        P0_7CI_W { w: self }
    }
    #[doc = "Bit 8 - Clear GPIO port Interrupts for P0\\[8\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_8ci(&mut self) -> P0_8CI_W {
        P0_8CI_W { w: self }
    }
    #[doc = "Bit 9 - Clear GPIO port Interrupts for P0\\[9\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_9ci(&mut self) -> P0_9CI_W {
        P0_9CI_W { w: self }
    }
    #[doc = "Bit 10 - Clear GPIO port Interrupts for P0\\[10\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_10ci(&mut self) -> P0_10CI_W {
        P0_10CI_W { w: self }
    }
    #[doc = "Bit 11 - Clear GPIO port Interrupts for P0\\[11\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_11ci(&mut self) -> P0_11CI_W {
        P0_11CI_W { w: self }
    }
    #[doc = "Bit 12 - Clear GPIO port Interrupts for P0\\[12\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_12ci(&mut self) -> P0_12CI_W {
        P0_12CI_W { w: self }
    }
    #[doc = "Bit 13 - Clear GPIO port Interrupts for P0\\[13\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_13ci(&mut self) -> P0_13CI_W {
        P0_13CI_W { w: self }
    }
    #[doc = "Bit 14 - Clear GPIO port Interrupts for P0\\[14\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_14ci(&mut self) -> P0_14CI_W {
        P0_14CI_W { w: self }
    }
    #[doc = "Bit 15 - Clear GPIO port Interrupts for P0\\[15\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_15ci(&mut self) -> P0_15CI_W {
        P0_15CI_W { w: self }
    }
    #[doc = "Bit 16 - Clear GPIO port Interrupts for P0\\[16\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_16ci(&mut self) -> P0_16CI_W {
        P0_16CI_W { w: self }
    }
    #[doc = "Bit 17 - Clear GPIO port Interrupts for P0\\[17\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_17ci(&mut self) -> P0_17CI_W {
        P0_17CI_W { w: self }
    }
    #[doc = "Bit 18 - Clear GPIO port Interrupts for P0\\[18\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_18ci(&mut self) -> P0_18CI_W {
        P0_18CI_W { w: self }
    }
    #[doc = "Bit 19 - Clear GPIO port Interrupts for P0\\[19\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_19ci(&mut self) -> P0_19CI_W {
        P0_19CI_W { w: self }
    }
    #[doc = "Bit 20 - Clear GPIO port Interrupts for P0\\[20\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_20ci(&mut self) -> P0_20CI_W {
        P0_20CI_W { w: self }
    }
    #[doc = "Bit 21 - Clear GPIO port Interrupts for P0\\[21\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_21ci(&mut self) -> P0_21CI_W {
        P0_21CI_W { w: self }
    }
    #[doc = "Bit 22 - Clear GPIO port Interrupts for P0\\[22\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_22ci(&mut self) -> P0_22CI_W {
        P0_22CI_W { w: self }
    }
    #[doc = "Bit 23 - Clear GPIO port Interrupts for P0\\[23\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_23ci(&mut self) -> P0_23CI_W {
        P0_23CI_W { w: self }
    }
    #[doc = "Bit 24 - Clear GPIO port Interrupts for P0\\[24\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_24ci(&mut self) -> P0_24CI_W {
        P0_24CI_W { w: self }
    }
    #[doc = "Bit 25 - Clear GPIO port Interrupts for P0\\[25\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_25ci(&mut self) -> P0_25CI_W {
        P0_25CI_W { w: self }
    }
    #[doc = "Bit 26 - Clear GPIO port Interrupts for P0\\[26\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_26ci(&mut self) -> P0_26CI_W {
        P0_26CI_W { w: self }
    }
    #[doc = "Bit 27 - Clear GPIO port Interrupts for P0\\[27\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_27ci(&mut self) -> P0_27CI_W {
        P0_27CI_W { w: self }
    }
    #[doc = "Bit 28 - Clear GPIO port Interrupts for P0\\[28\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_28ci(&mut self) -> P0_28CI_W {
        P0_28CI_W { w: self }
    }
    #[doc = "Bit 29 - Clear GPIO port Interrupts for P0\\[29\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_29ci(&mut self) -> P0_29CI_W {
        P0_29CI_W { w: self }
    }
    #[doc = "Bit 30 - Clear GPIO port Interrupts for P0\\[30\\]. 0 = No effect. 1 = Clear corresponding bits in IOnINTSTATR and IOnSTATF."]
    #[inline(always)]
    pub fn p0_30ci(&mut self) -> P0_30CI_W {
        P0_30CI_W { w: self }
    }
}
