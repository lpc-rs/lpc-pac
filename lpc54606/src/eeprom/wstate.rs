#[doc = "Reader of register WSTATE"]
pub type R = crate::R<u32, super::WSTATE>;
#[doc = "Writer for register WSTATE"]
pub type W = crate::W<u32, super::WSTATE>;
#[doc = "Register WSTATE `reset()`'s with value 0x0004_0802"]
impl crate::ResetValue for super::WSTATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0802
    }
}
#[doc = "Reader of field `PHASE3`"]
pub type PHASE3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE3`"]
pub struct PHASE3_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `PHASE2`"]
pub type PHASE2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE2`"]
pub struct PHASE2_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `PHASE1`"]
pub type PHASE1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHASE1`"]
pub struct PHASE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `LCK_PARWEP`"]
pub type LCK_PARWEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCK_PARWEP`"]
pub struct LCK_PARWEP_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK_PARWEP_W<'a> {
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
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&self) -> PHASE3_R {
        PHASE3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&self) -> PHASE2_R {
        PHASE2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&self) -> PHASE1_R {
        PHASE1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&self) -> LCK_PARWEP_R {
        LCK_PARWEP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait states for phase 3 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase3(&mut self) -> PHASE3_W {
        PHASE3_W { w: self }
    }
    #[doc = "Bits 8:15 - Wait states for phase 2 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase2(&mut self) -> PHASE2_W {
        PHASE2_W { w: self }
    }
    #[doc = "Bits 16:23 - Wait states for phase 1 (minus 1 encoded)."]
    #[inline(always)]
    pub fn phase1(&mut self) -> PHASE1_W {
        PHASE1_W { w: self }
    }
    #[doc = "Bit 31 - Lock timing parameters for write, erase and program operation 0 = WSTATE and CLKDIV registers have R/W access 1 = WSTATE and CLKDIV registers have R only access."]
    #[inline(always)]
    pub fn lck_parwep(&mut self) -> LCK_PARWEP_W {
        LCK_PARWEP_W { w: self }
    }
}
