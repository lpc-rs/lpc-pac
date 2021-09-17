#[doc = "Register `CLR%s` writer"]
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRP00` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP00_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP00_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CLRP01` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP01_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP01_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CLRP02` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP02_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP02_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CLRP03` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP03_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP03_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CLRP04` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP04_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP04_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CLRP05` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP05_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP05_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CLRP06` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP06_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP06_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CLRP07` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP07_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP07_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CLRP08` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP08_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP08_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CLRP09` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP09_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP09_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CLRP010` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP010_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP010_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CLRP011` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP011_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP011_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CLRP012` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP012_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP012_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CLRP013` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP013_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP013_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CLRP014` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP014_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP014_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CLRP015` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP015_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP015_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CLRP016` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP016_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP016_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CLRP017` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP017_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP017_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CLRP018` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP018_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP018_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CLRP019` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP019_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP019_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CLRP020` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP020_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP020_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CLRP021` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP021_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP021_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CLRP022` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP022_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP022_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CLRP023` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP023_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP023_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CLRP024` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP024_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP024_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CLRP025` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP025_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP025_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CLRP026` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP026_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP026_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CLRP027` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP027_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP027_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CLRP028` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP028_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP028_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CLRP029` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP029_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP029_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CLRP030` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP030_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP030_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CLRP031` writer - Clear output bits: 0 = No operation. 1 = Clear output bit."]
pub struct CLRP031_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRP031_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp00(&mut self) -> CLRP00_W {
        CLRP00_W { w: self }
    }
    #[doc = "Bit 1 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp01(&mut self) -> CLRP01_W {
        CLRP01_W { w: self }
    }
    #[doc = "Bit 2 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp02(&mut self) -> CLRP02_W {
        CLRP02_W { w: self }
    }
    #[doc = "Bit 3 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp03(&mut self) -> CLRP03_W {
        CLRP03_W { w: self }
    }
    #[doc = "Bit 4 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp04(&mut self) -> CLRP04_W {
        CLRP04_W { w: self }
    }
    #[doc = "Bit 5 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp05(&mut self) -> CLRP05_W {
        CLRP05_W { w: self }
    }
    #[doc = "Bit 6 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp06(&mut self) -> CLRP06_W {
        CLRP06_W { w: self }
    }
    #[doc = "Bit 7 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp07(&mut self) -> CLRP07_W {
        CLRP07_W { w: self }
    }
    #[doc = "Bit 8 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp08(&mut self) -> CLRP08_W {
        CLRP08_W { w: self }
    }
    #[doc = "Bit 9 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp09(&mut self) -> CLRP09_W {
        CLRP09_W { w: self }
    }
    #[doc = "Bit 10 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp010(&mut self) -> CLRP010_W {
        CLRP010_W { w: self }
    }
    #[doc = "Bit 11 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp011(&mut self) -> CLRP011_W {
        CLRP011_W { w: self }
    }
    #[doc = "Bit 12 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp012(&mut self) -> CLRP012_W {
        CLRP012_W { w: self }
    }
    #[doc = "Bit 13 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp013(&mut self) -> CLRP013_W {
        CLRP013_W { w: self }
    }
    #[doc = "Bit 14 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp014(&mut self) -> CLRP014_W {
        CLRP014_W { w: self }
    }
    #[doc = "Bit 15 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp015(&mut self) -> CLRP015_W {
        CLRP015_W { w: self }
    }
    #[doc = "Bit 16 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp016(&mut self) -> CLRP016_W {
        CLRP016_W { w: self }
    }
    #[doc = "Bit 17 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp017(&mut self) -> CLRP017_W {
        CLRP017_W { w: self }
    }
    #[doc = "Bit 18 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp018(&mut self) -> CLRP018_W {
        CLRP018_W { w: self }
    }
    #[doc = "Bit 19 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp019(&mut self) -> CLRP019_W {
        CLRP019_W { w: self }
    }
    #[doc = "Bit 20 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp020(&mut self) -> CLRP020_W {
        CLRP020_W { w: self }
    }
    #[doc = "Bit 21 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp021(&mut self) -> CLRP021_W {
        CLRP021_W { w: self }
    }
    #[doc = "Bit 22 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp022(&mut self) -> CLRP022_W {
        CLRP022_W { w: self }
    }
    #[doc = "Bit 23 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp023(&mut self) -> CLRP023_W {
        CLRP023_W { w: self }
    }
    #[doc = "Bit 24 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp024(&mut self) -> CLRP024_W {
        CLRP024_W { w: self }
    }
    #[doc = "Bit 25 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp025(&mut self) -> CLRP025_W {
        CLRP025_W { w: self }
    }
    #[doc = "Bit 26 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp026(&mut self) -> CLRP026_W {
        CLRP026_W { w: self }
    }
    #[doc = "Bit 27 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp027(&mut self) -> CLRP027_W {
        CLRP027_W { w: self }
    }
    #[doc = "Bit 28 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp028(&mut self) -> CLRP028_W {
        CLRP028_W { w: self }
    }
    #[doc = "Bit 29 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp029(&mut self) -> CLRP029_W {
        CLRP029_W { w: self }
    }
    #[doc = "Bit 30 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp030(&mut self) -> CLRP030_W {
        CLRP030_W { w: self }
    }
    #[doc = "Bit 31 - Clear output bits: 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn clrp031(&mut self) -> CLRP031_W {
        CLRP031_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear port 0/1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](index.html) module"]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clr::W](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLR%s to value 0"]
impl crate::Resettable for CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
