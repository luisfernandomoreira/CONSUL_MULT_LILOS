#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `PUD` reader - Watchdog prescaler value update"]
pub type PudR = crate::BitReader;
#[doc = "Field `RUD` reader - Watchdog counter reload value update"]
pub type RudR = crate::BitReader;
#[doc = "Field `WUD` reader - Watchdog counter window value update"]
pub type WudR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Watchdog prescaler value update"]
    #[inline(always)]
    pub fn pud(&self) -> PudR {
        PudR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog counter reload value update"]
    #[inline(always)]
    pub fn rud(&self) -> RudR {
        RudR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog counter window value update"]
    #[inline(always)]
    pub fn wud(&self) -> WudR {
        WudR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
