#[doc = "Reader of register DLY"]
pub type R = crate::R<u32, super::DLY>;
#[doc = "Writer for register DLY"]
pub type W = crate::W<u32, super::DLY>;
#[doc = "Register DLY `reset()`'s with value 0"]
impl crate::ResetValue for super::DLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRE_DELAY`"]
pub type PRE_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRE_DELAY`"]
pub struct PRE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `POST_DELAY`"]
pub type POST_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POST_DELAY`"]
pub struct POST_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> POST_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `FRAME_DELAY`"]
pub type FRAME_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRAME_DELAY`"]
pub struct FRAME_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TRANSFER_DELAY`"]
pub type TRANSFER_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TRANSFER_DELAY`"]
pub struct TRANSFER_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&self) -> PRE_DELAY_R {
        PRE_DELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&self) -> POST_DELAY_R {
        POST_DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&self) -> FRAME_DELAY_R {
        FRAME_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&self) -> TRANSFER_DELAY_R {
        TRANSFER_DELAY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&mut self) -> PRE_DELAY_W {
        PRE_DELAY_W { w: self }
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&mut self) -> POST_DELAY_W {
        POST_DELAY_W { w: self }
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&mut self) -> FRAME_DELAY_W {
        FRAME_DELAY_W { w: self }
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&mut self) -> TRANSFER_DELAY_W {
        TRANSFER_DELAY_W { w: self }
    }
}
