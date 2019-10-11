#[doc = "Reader of register PIN%s"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Writer for register PIN%s"]
pub type W = crate::W<u32, super::PIN>;
#[doc = "Register PIN%s `reset()`'s with value 0"]
impl crate::ResetValue for super::PIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORT0`"]
pub type PORT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT0`"]
pub struct PORT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT0_W<'a> {
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
#[doc = "Reader of field `PORT1`"]
pub type PORT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT1`"]
pub struct PORT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT1_W<'a> {
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
#[doc = "Reader of field `PORT2`"]
pub type PORT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT2`"]
pub struct PORT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT2_W<'a> {
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
#[doc = "Reader of field `PORT3`"]
pub type PORT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT3`"]
pub struct PORT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT3_W<'a> {
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
#[doc = "Reader of field `PORT4`"]
pub type PORT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT4`"]
pub struct PORT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT4_W<'a> {
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
#[doc = "Reader of field `PORT5`"]
pub type PORT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT5`"]
pub struct PORT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT5_W<'a> {
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
#[doc = "Reader of field `PORT6`"]
pub type PORT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT6`"]
pub struct PORT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT6_W<'a> {
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
#[doc = "Reader of field `PORT7`"]
pub type PORT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT7`"]
pub struct PORT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT7_W<'a> {
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
#[doc = "Reader of field `PORT8`"]
pub type PORT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT8`"]
pub struct PORT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT8_W<'a> {
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
#[doc = "Reader of field `PORT9`"]
pub type PORT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT9`"]
pub struct PORT9_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT9_W<'a> {
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
#[doc = "Reader of field `PORT10`"]
pub type PORT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT10`"]
pub struct PORT10_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT10_W<'a> {
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
#[doc = "Reader of field `PORT11`"]
pub type PORT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT11`"]
pub struct PORT11_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT11_W<'a> {
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
#[doc = "Reader of field `PORT12`"]
pub type PORT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT12`"]
pub struct PORT12_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT12_W<'a> {
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
#[doc = "Reader of field `PORT13`"]
pub type PORT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT13`"]
pub struct PORT13_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT13_W<'a> {
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
#[doc = "Reader of field `PORT14`"]
pub type PORT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT14`"]
pub struct PORT14_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT14_W<'a> {
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
#[doc = "Reader of field `PORT15`"]
pub type PORT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT15`"]
pub struct PORT15_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT15_W<'a> {
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
#[doc = "Reader of field `PORT16`"]
pub type PORT16_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT16`"]
pub struct PORT16_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT16_W<'a> {
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
#[doc = "Reader of field `PORT17`"]
pub type PORT17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT17`"]
pub struct PORT17_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT17_W<'a> {
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
#[doc = "Reader of field `PORT18`"]
pub type PORT18_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT18`"]
pub struct PORT18_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT18_W<'a> {
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
#[doc = "Reader of field `PORT19`"]
pub type PORT19_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT19`"]
pub struct PORT19_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT19_W<'a> {
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
#[doc = "Reader of field `PORT20`"]
pub type PORT20_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT20`"]
pub struct PORT20_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT20_W<'a> {
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
#[doc = "Reader of field `PORT21`"]
pub type PORT21_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT21`"]
pub struct PORT21_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT21_W<'a> {
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
#[doc = "Reader of field `PORT22`"]
pub type PORT22_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT22`"]
pub struct PORT22_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT22_W<'a> {
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
#[doc = "Reader of field `PORT23`"]
pub type PORT23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT23`"]
pub struct PORT23_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT23_W<'a> {
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
#[doc = "Reader of field `PORT24`"]
pub type PORT24_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT24`"]
pub struct PORT24_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT24_W<'a> {
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
#[doc = "Reader of field `PORT25`"]
pub type PORT25_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT25`"]
pub struct PORT25_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT25_W<'a> {
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
#[doc = "Reader of field `PORT26`"]
pub type PORT26_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT26`"]
pub struct PORT26_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT26_W<'a> {
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
#[doc = "Reader of field `PORT27`"]
pub type PORT27_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT27`"]
pub struct PORT27_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT27_W<'a> {
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
#[doc = "Reader of field `PORT28`"]
pub type PORT28_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT28`"]
pub struct PORT28_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT28_W<'a> {
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
#[doc = "Reader of field `PORT29`"]
pub type PORT29_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT29`"]
pub struct PORT29_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT29_W<'a> {
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
#[doc = "Reader of field `PORT30`"]
pub type PORT30_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT30`"]
pub struct PORT30_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT30_W<'a> {
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
#[doc = "Reader of field `PORT31`"]
pub type PORT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PORT31`"]
pub struct PORT31_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT31_W<'a> {
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
    #[doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port0(&self) -> PORT0_R {
        PORT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port1(&self) -> PORT1_R {
        PORT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port2(&self) -> PORT2_R {
        PORT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port3(&self) -> PORT3_R {
        PORT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port4(&self) -> PORT4_R {
        PORT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port5(&self) -> PORT5_R {
        PORT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port6(&self) -> PORT6_R {
        PORT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port7(&self) -> PORT7_R {
        PORT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port8(&self) -> PORT8_R {
        PORT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port9(&self) -> PORT9_R {
        PORT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port10(&self) -> PORT10_R {
        PORT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port11(&self) -> PORT11_R {
        PORT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port12(&self) -> PORT12_R {
        PORT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port13(&self) -> PORT13_R {
        PORT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port14(&self) -> PORT14_R {
        PORT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port15(&self) -> PORT15_R {
        PORT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port16(&self) -> PORT16_R {
        PORT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port17(&self) -> PORT17_R {
        PORT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port18(&self) -> PORT18_R {
        PORT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port19(&self) -> PORT19_R {
        PORT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port20(&self) -> PORT20_R {
        PORT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port21(&self) -> PORT21_R {
        PORT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port22(&self) -> PORT22_R {
        PORT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port23(&self) -> PORT23_R {
        PORT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port24(&self) -> PORT24_R {
        PORT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port25(&self) -> PORT25_R {
        PORT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port26(&self) -> PORT26_R {
        PORT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port27(&self) -> PORT27_R {
        PORT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port28(&self) -> PORT28_R {
        PORT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port29(&self) -> PORT29_R {
        PORT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port30(&self) -> PORT30_R {
        PORT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port31(&self) -> PORT31_R {
        PORT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port0(&mut self) -> PORT0_W {
        PORT0_W { w: self }
    }
    #[doc = "Bit 1 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port1(&mut self) -> PORT1_W {
        PORT1_W { w: self }
    }
    #[doc = "Bit 2 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port2(&mut self) -> PORT2_W {
        PORT2_W { w: self }
    }
    #[doc = "Bit 3 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port3(&mut self) -> PORT3_W {
        PORT3_W { w: self }
    }
    #[doc = "Bit 4 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port4(&mut self) -> PORT4_W {
        PORT4_W { w: self }
    }
    #[doc = "Bit 5 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port5(&mut self) -> PORT5_W {
        PORT5_W { w: self }
    }
    #[doc = "Bit 6 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port6(&mut self) -> PORT6_W {
        PORT6_W { w: self }
    }
    #[doc = "Bit 7 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port7(&mut self) -> PORT7_W {
        PORT7_W { w: self }
    }
    #[doc = "Bit 8 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port8(&mut self) -> PORT8_W {
        PORT8_W { w: self }
    }
    #[doc = "Bit 9 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port9(&mut self) -> PORT9_W {
        PORT9_W { w: self }
    }
    #[doc = "Bit 10 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port10(&mut self) -> PORT10_W {
        PORT10_W { w: self }
    }
    #[doc = "Bit 11 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port11(&mut self) -> PORT11_W {
        PORT11_W { w: self }
    }
    #[doc = "Bit 12 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port12(&mut self) -> PORT12_W {
        PORT12_W { w: self }
    }
    #[doc = "Bit 13 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port13(&mut self) -> PORT13_W {
        PORT13_W { w: self }
    }
    #[doc = "Bit 14 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port14(&mut self) -> PORT14_W {
        PORT14_W { w: self }
    }
    #[doc = "Bit 15 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port15(&mut self) -> PORT15_W {
        PORT15_W { w: self }
    }
    #[doc = "Bit 16 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port16(&mut self) -> PORT16_W {
        PORT16_W { w: self }
    }
    #[doc = "Bit 17 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port17(&mut self) -> PORT17_W {
        PORT17_W { w: self }
    }
    #[doc = "Bit 18 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port18(&mut self) -> PORT18_W {
        PORT18_W { w: self }
    }
    #[doc = "Bit 19 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port19(&mut self) -> PORT19_W {
        PORT19_W { w: self }
    }
    #[doc = "Bit 20 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port20(&mut self) -> PORT20_W {
        PORT20_W { w: self }
    }
    #[doc = "Bit 21 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port21(&mut self) -> PORT21_W {
        PORT21_W { w: self }
    }
    #[doc = "Bit 22 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port22(&mut self) -> PORT22_W {
        PORT22_W { w: self }
    }
    #[doc = "Bit 23 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port23(&mut self) -> PORT23_W {
        PORT23_W { w: self }
    }
    #[doc = "Bit 24 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port24(&mut self) -> PORT24_W {
        PORT24_W { w: self }
    }
    #[doc = "Bit 25 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port25(&mut self) -> PORT25_W {
        PORT25_W { w: self }
    }
    #[doc = "Bit 26 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port26(&mut self) -> PORT26_W {
        PORT26_W { w: self }
    }
    #[doc = "Bit 27 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port27(&mut self) -> PORT27_W {
        PORT27_W { w: self }
    }
    #[doc = "Bit 28 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port28(&mut self) -> PORT28_W {
        PORT28_W { w: self }
    }
    #[doc = "Bit 29 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port29(&mut self) -> PORT29_W {
        PORT29_W { w: self }
    }
    #[doc = "Bit 30 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port30(&mut self) -> PORT30_W {
        PORT30_W { w: self }
    }
    #[doc = "Bit 31 - Reads pin states or loads output bits (bit 0 = P0/1_0, bit 1 = P0/1_1, ..., bit 31 = P0/1_31). 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn port31(&mut self) -> PORT31_W {
        PORT31_W { w: self }
    }
}
