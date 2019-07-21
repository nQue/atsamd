#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct XOSC32KRDYR {
    bits: bool,
}
impl XOSC32KRDYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XOSC32KFAILR {
    bits: bool,
}
impl XOSC32KFAILR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct XOSC32KSWR {
    bits: bool,
}
impl XOSC32KSWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - XOSC32K Ready"]
    #[inline]
    pub fn xosc32krdy(&self) -> XOSC32KRDYR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        XOSC32KRDYR { bits }
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector"]
    #[inline]
    pub fn xosc32kfail(&self) -> XOSC32KFAILR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        XOSC32KFAILR { bits }
    }
    #[doc = "Bit 3 - XOSC32K Clock switch"]
    #[inline]
    pub fn xosc32ksw(&self) -> XOSC32KSWR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        XOSC32KSWR { bits }
    }
}
