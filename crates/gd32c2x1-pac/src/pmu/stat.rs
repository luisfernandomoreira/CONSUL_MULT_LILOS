#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `EFLASHPS_SLEEP` reader - EFLASH is in sleep state."]
pub type EflashpsSleepR = crate::BitReader;
#[doc = "Field `EFLASHPS_ACTIVE` reader - EFLASH is in active state."]
pub type EflashpsActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - EFLASH is in sleep state."]
    #[inline(always)]
    pub fn eflashps_sleep(&self) -> EflashpsSleepR {
        EflashpsSleepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - EFLASH is in active state."]
    #[inline(always)]
    pub fn eflashps_active(&self) -> EflashpsActiveR {
        EflashpsActiveR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0x00c0_0020"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x00c0_0020;
}
