#[doc = "Reader of register IR"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR `reset()`'s with value 0"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MR0INT`"]
pub type MR0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR0INT`"]
pub struct MR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR0INT_W<'a> {
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
#[doc = "Reader of field `MR1INT`"]
pub type MR1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR1INT`"]
pub struct MR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR1INT_W<'a> {
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
#[doc = "Reader of field `MR2INT`"]
pub type MR2INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR2INT`"]
pub struct MR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR2INT_W<'a> {
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
#[doc = "Reader of field `MR3INT`"]
pub type MR3INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MR3INT`"]
pub struct MR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> MR3INT_W<'a> {
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
#[doc = "Reader of field `CR0INT`"]
pub type CR0INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR0INT`"]
pub struct CR0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR0INT_W<'a> {
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
#[doc = "Reader of field `CR1INT`"]
pub type CR1INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR1INT`"]
pub struct CR1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR1INT_W<'a> {
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
#[doc = "Reader of field `CR2INT`"]
pub type CR2INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR2INT`"]
pub struct CR2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR2INT_W<'a> {
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
#[doc = "Reader of field `CR3INT`"]
pub type CR3INT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CR3INT`"]
pub struct CR3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR3INT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&self) -> MR0INT_R {
        MR0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&self) -> MR1INT_R {
        MR1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&self) -> MR2INT_R {
        MR2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&self) -> MR3INT_R {
        MR3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&self) -> CR0INT_R {
        CR0INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&self) -> CR1INT_R {
        CR1INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&self) -> CR2INT_R {
        CR2INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub fn cr3int(&self) -> CR3INT_R {
        CR3INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt flag for match channel 0."]
    #[inline(always)]
    pub fn mr0int(&mut self) -> MR0INT_W {
        MR0INT_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt flag for match channel 1."]
    #[inline(always)]
    pub fn mr1int(&mut self) -> MR1INT_W {
        MR1INT_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt flag for match channel 2."]
    #[inline(always)]
    pub fn mr2int(&mut self) -> MR2INT_W {
        MR2INT_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt flag for match channel 3."]
    #[inline(always)]
    pub fn mr3int(&mut self) -> MR3INT_W {
        MR3INT_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt flag for capture channel 0 event."]
    #[inline(always)]
    pub fn cr0int(&mut self) -> CR0INT_W {
        CR0INT_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt flag for capture channel 1 event."]
    #[inline(always)]
    pub fn cr1int(&mut self) -> CR1INT_W {
        CR1INT_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt flag for capture channel 2 event."]
    #[inline(always)]
    pub fn cr2int(&mut self) -> CR2INT_W {
        CR2INT_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt flag for capture channel 3 event."]
    #[inline(always)]
    pub fn cr3int(&mut self) -> CR3INT_W {
        CR3INT_W { w: self }
    }
}
