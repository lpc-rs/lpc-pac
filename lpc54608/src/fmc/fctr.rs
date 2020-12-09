#[doc = "Reader of register FCTR"]
pub type R = crate::R<u32, super::FCTR>;
#[doc = "Writer for register FCTR"]
pub type W = crate::W<u32, super::FCTR>;
#[doc = "Register FCTR `reset()`'s with value 0x0020_0005"]
impl crate::ResetValue for super::FCTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0020_0005
    }
}
#[doc = "Reader of field `FS_RD0`"]
pub type FS_RD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FS_RD0`"]
pub struct FS_RD0_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_RD0_W<'a> {
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
#[doc = "Reader of field `FS_RD1`"]
pub type FS_RD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FS_RD1`"]
pub struct FS_RD1_W<'a> {
    w: &'a mut W,
}
impl<'a> FS_RD1_W<'a> {
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
impl R {
    #[doc = "Bit 3 - Value must be 0 for signature generation."]
    #[inline(always)]
    pub fn fs_rd0(&self) -> FS_RD0_R {
        FS_RD0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Value must be 1 for signature generation."]
    #[inline(always)]
    pub fn fs_rd1(&self) -> FS_RD1_R {
        FS_RD1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Value must be 0 for signature generation."]
    #[inline(always)]
    pub fn fs_rd0(&mut self) -> FS_RD0_W {
        FS_RD0_W { w: self }
    }
    #[doc = "Bit 4 - Value must be 1 for signature generation."]
    #[inline(always)]
    pub fn fs_rd1(&mut self) -> FS_RD1_W {
        FS_RD1_W { w: self }
    }
}
