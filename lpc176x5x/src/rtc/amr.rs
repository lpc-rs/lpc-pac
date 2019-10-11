#[doc = "Reader of register AMR"]
pub type R = crate::R<u32, super::AMR>;
#[doc = "Writer for register AMR"]
pub type W = crate::W<u32, super::AMR>;
#[doc = "Register AMR `reset()`'s with value 0"]
impl crate::ResetValue for super::AMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AMRSEC`"]
pub type AMRSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRSEC`"]
pub struct AMRSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRSEC_W<'a> {
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
#[doc = "Reader of field `AMRMIN`"]
pub type AMRMIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRMIN`"]
pub struct AMRMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMIN_W<'a> {
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
#[doc = "Reader of field `AMRHOUR`"]
pub type AMRHOUR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRHOUR`"]
pub struct AMRHOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRHOUR_W<'a> {
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
#[doc = "Reader of field `AMRDOM`"]
pub type AMRDOM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRDOM`"]
pub struct AMRDOM_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOM_W<'a> {
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
#[doc = "Reader of field `AMRDOW`"]
pub type AMRDOW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRDOW`"]
pub struct AMRDOW_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOW_W<'a> {
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
#[doc = "Reader of field `AMRDOY`"]
pub type AMRDOY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRDOY`"]
pub struct AMRDOY_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRDOY_W<'a> {
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
#[doc = "Reader of field `AMRMON`"]
pub type AMRMON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRMON`"]
pub struct AMRMON_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRMON_W<'a> {
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
#[doc = "Reader of field `AMRYEAR`"]
pub type AMRYEAR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMRYEAR`"]
pub struct AMRYEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMRYEAR_W<'a> {
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
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&self) -> AMRSEC_R {
        AMRSEC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&self) -> AMRMIN_R {
        AMRMIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&self) -> AMRHOUR_R {
        AMRHOUR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&self) -> AMRDOM_R {
        AMRDOM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&self) -> AMRDOW_R {
        AMRDOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&self) -> AMRDOY_R {
        AMRDOY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&self) -> AMRMON_R {
        AMRMON_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&self) -> AMRYEAR_R {
        AMRYEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&mut self) -> AMRSEC_W {
        AMRSEC_W { w: self }
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&mut self) -> AMRMIN_W {
        AMRMIN_W { w: self }
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&mut self) -> AMRHOUR_W {
        AMRHOUR_W { w: self }
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&mut self) -> AMRDOM_W {
        AMRDOM_W { w: self }
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&mut self) -> AMRDOW_W {
        AMRDOW_W { w: self }
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&mut self) -> AMRDOY_W {
        AMRDOY_W { w: self }
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&mut self) -> AMRMON_W {
        AMRMON_W { w: self }
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&mut self) -> AMRYEAR_W {
        AMRYEAR_W { w: self }
    }
}
