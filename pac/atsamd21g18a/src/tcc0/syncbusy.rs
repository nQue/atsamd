#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::SYNCBUSY {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
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
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
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
pub struct CTRLBR {
    bits: bool,
}
impl CTRLBR {
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
pub struct STATUSR {
    bits: bool,
}
impl STATUSR {
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
pub struct COUNTR {
    bits: bool,
}
impl COUNTR {
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
pub struct PATTR {
    bits: bool,
}
impl PATTR {
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
pub struct WAVER {
    bits: bool,
}
impl WAVER {
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
pub struct PERR {
    bits: bool,
}
impl PERR {
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
pub struct CC0R {
    bits: bool,
}
impl CC0R {
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
pub struct CC1R {
    bits: bool,
}
impl CC1R {
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
pub struct CC2R {
    bits: bool,
}
impl CC2R {
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
pub struct CC3R {
    bits: bool,
}
impl CC3R {
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
pub struct PATTBR {
    bits: bool,
}
impl PATTBR {
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
pub struct WAVEBR {
    bits: bool,
}
impl WAVEBR {
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
pub struct PERBR {
    bits: bool,
}
impl PERBR {
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
pub struct CCB0R {
    bits: bool,
}
impl CCB0R {
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
pub struct CCB1R {
    bits: bool,
}
impl CCB1R {
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
pub struct CCB2R {
    bits: bool,
}
impl CCB2R {
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
pub struct CCB3R {
    bits: bool,
}
impl CCB3R {
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
    #[doc = "Bit 0 - Swrst Busy"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Enable Busy"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bit 2 - Ctrlb Busy"]
    #[inline]
    pub fn ctrlb(&self) -> CTRLBR {
        let bits = ((self.bits >> 2) & 0x01) != 0;
        CTRLBR { bits }
    }
    #[doc = "Bit 3 - Status Busy"]
    #[inline]
    pub fn status(&self) -> STATUSR {
        let bits = ((self.bits >> 3) & 0x01) != 0;
        STATUSR { bits }
    }
    #[doc = "Bit 4 - Count Busy"]
    #[inline]
    pub fn count(&self) -> COUNTR {
        let bits = ((self.bits >> 4) & 0x01) != 0;
        COUNTR { bits }
    }
    #[doc = "Bit 5 - Pattern Busy"]
    #[inline]
    pub fn patt(&self) -> PATTR {
        let bits = ((self.bits >> 5) & 0x01) != 0;
        PATTR { bits }
    }
    #[doc = "Bit 6 - Wave Busy"]
    #[inline]
    pub fn wave(&self) -> WAVER {
        let bits = ((self.bits >> 6) & 0x01) != 0;
        WAVER { bits }
    }
    #[doc = "Bit 7 - Period busy"]
    #[inline]
    pub fn per(&self) -> PERR {
        let bits = ((self.bits >> 7) & 0x01) != 0;
        PERR { bits }
    }
    #[doc = "Bit 8 - Compare Channel 0 Busy"]
    #[inline]
    pub fn cc0(&self) -> CC0R {
        let bits = ((self.bits >> 8) & 0x01) != 0;
        CC0R { bits }
    }
    #[doc = "Bit 9 - Compare Channel 1 Busy"]
    #[inline]
    pub fn cc1(&self) -> CC1R {
        let bits = ((self.bits >> 9) & 0x01) != 0;
        CC1R { bits }
    }
    #[doc = "Bit 10 - Compare Channel 2 Busy"]
    #[inline]
    pub fn cc2(&self) -> CC2R {
        let bits = ((self.bits >> 10) & 0x01) != 0;
        CC2R { bits }
    }
    #[doc = "Bit 11 - Compare Channel 3 Busy"]
    #[inline]
    pub fn cc3(&self) -> CC3R {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        CC3R { bits }
    }
    #[doc = "Bit 16 - Pattern Buffer Busy"]
    #[inline]
    pub fn pattb(&self) -> PATTBR {
        let bits = ((self.bits >> 16) & 0x01) != 0;
        PATTBR { bits }
    }
    #[doc = "Bit 17 - Wave Buffer Busy"]
    #[inline]
    pub fn waveb(&self) -> WAVEBR {
        let bits = ((self.bits >> 17) & 0x01) != 0;
        WAVEBR { bits }
    }
    #[doc = "Bit 18 - Period Buffer Busy"]
    #[inline]
    pub fn perb(&self) -> PERBR {
        let bits = ((self.bits >> 18) & 0x01) != 0;
        PERBR { bits }
    }
    #[doc = "Bit 19 - Compare Channel Buffer 0 Busy"]
    #[inline]
    pub fn ccb0(&self) -> CCB0R {
        let bits = ((self.bits >> 19) & 0x01) != 0;
        CCB0R { bits }
    }
    #[doc = "Bit 20 - Compare Channel Buffer 1 Busy"]
    #[inline]
    pub fn ccb1(&self) -> CCB1R {
        let bits = ((self.bits >> 20) & 0x01) != 0;
        CCB1R { bits }
    }
    #[doc = "Bit 21 - Compare Channel Buffer 2 Busy"]
    #[inline]
    pub fn ccb2(&self) -> CCB2R {
        let bits = ((self.bits >> 21) & 0x01) != 0;
        CCB2R { bits }
    }
    #[doc = "Bit 22 - Compare Channel Buffer 3 Busy"]
    #[inline]
    pub fn ccb3(&self) -> CCB3R {
        let bits = ((self.bits >> 22) & 0x01) != 0;
        CCB3R { bits }
    }
}
