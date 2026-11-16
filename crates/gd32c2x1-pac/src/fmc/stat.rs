#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `ENDF` reader - End of operation flag bit"]
pub type EndfR = crate::BitReader;
#[doc = "Field `ENDF` writer - End of operation flag bit"]
pub type EndfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPRERR` reader - Operation error flag bit"]
pub type OprerrR = crate::BitReader;
#[doc = "Field `OPRERR` writer - Operation error flag bit"]
pub type OprerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGERR` reader - Program error flag bit"]
pub type PgerrR = crate::BitReader;
#[doc = "Field `PGERR` writer - Program error flag bit"]
pub type PgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPERR` reader - Erase / program protection error flag bit."]
pub type WperrR = crate::BitReader;
#[doc = "Field `WPERR` writer - Erase / program protection error flag bit."]
pub type WperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGAERR` reader - Program alignment error flag bit"]
pub type PgaerrR = crate::BitReader;
#[doc = "Field `PGAERR` writer - Program alignment error flag bit"]
pub type PgaerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGMERR` reader - Program size not match error flag bit. This bit is set by hardware when program size is a half-word/word access. The only correct programming size is double word."]
pub type PgmerrR = crate::BitReader;
#[doc = "Field `PGMERR` writer - Program size not match error flag bit. This bit is set by hardware when program size is a half-word/word access. The only correct programming size is double word."]
pub type PgmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR` reader - Program sequence error flag bit."]
pub type PgserrR = crate::BitReader;
#[doc = "Field `PGSERR` writer - Program sequence error flag bit."]
pub type PgserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTPERR` reader - Fast programming error flag bit."]
pub type FstperrR = crate::BitReader;
#[doc = "Field `FSTPERR` writer - Fast programming error flag bit."]
pub type FstperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPERR` reader - Read protection error flag bit."]
pub type RperrR = crate::BitReader;
#[doc = "Field `RPERR` writer - Read protection error flag bit."]
pub type RperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBERR` reader - Option byte read error bit."]
pub type OberrR = crate::BitReader;
#[doc = "Field `OBERR` writer - Option byte read error bit."]
pub type OberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - The flash is busy bit"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&self) -> EndfR {
        EndfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Operation error flag bit"]
    #[inline(always)]
    pub fn oprerr(&self) -> OprerrR {
        OprerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&self) -> PgerrR {
        PgerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase / program protection error flag bit."]
    #[inline(always)]
    pub fn wperr(&self) -> WperrR {
        WperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Program alignment error flag bit"]
    #[inline(always)]
    pub fn pgaerr(&self) -> PgaerrR {
        PgaerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Program size not match error flag bit. This bit is set by hardware when program size is a half-word/word access. The only correct programming size is double word."]
    #[inline(always)]
    pub fn pgmerr(&self) -> PgmerrR {
        PgmerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Program sequence error flag bit."]
    #[inline(always)]
    pub fn pgserr(&self) -> PgserrR {
        PgserrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast programming error flag bit."]
    #[inline(always)]
    pub fn fstperr(&self) -> FstperrR {
        FstperrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Read protection error flag bit."]
    #[inline(always)]
    pub fn rperr(&self) -> RperrR {
        RperrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Option byte read error bit."]
    #[inline(always)]
    pub fn oberr(&self) -> OberrR {
        OberrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The flash is busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of operation flag bit"]
    #[inline(always)]
    pub fn endf(&mut self) -> EndfW<'_, StatSpec> {
        EndfW::new(self, 0)
    }
    #[doc = "Bit 1 - Operation error flag bit"]
    #[inline(always)]
    pub fn oprerr(&mut self) -> OprerrW<'_, StatSpec> {
        OprerrW::new(self, 1)
    }
    #[doc = "Bit 3 - Program error flag bit"]
    #[inline(always)]
    pub fn pgerr(&mut self) -> PgerrW<'_, StatSpec> {
        PgerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Erase / program protection error flag bit."]
    #[inline(always)]
    pub fn wperr(&mut self) -> WperrW<'_, StatSpec> {
        WperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Program alignment error flag bit"]
    #[inline(always)]
    pub fn pgaerr(&mut self) -> PgaerrW<'_, StatSpec> {
        PgaerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Program size not match error flag bit. This bit is set by hardware when program size is a half-word/word access. The only correct programming size is double word."]
    #[inline(always)]
    pub fn pgmerr(&mut self) -> PgmerrW<'_, StatSpec> {
        PgmerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Program sequence error flag bit."]
    #[inline(always)]
    pub fn pgserr(&mut self) -> PgserrW<'_, StatSpec> {
        PgserrW::new(self, 7)
    }
    #[doc = "Bit 9 - Fast programming error flag bit."]
    #[inline(always)]
    pub fn fstperr(&mut self) -> FstperrW<'_, StatSpec> {
        FstperrW::new(self, 9)
    }
    #[doc = "Bit 14 - Read protection error flag bit."]
    #[inline(always)]
    pub fn rperr(&mut self) -> RperrW<'_, StatSpec> {
        RperrW::new(self, 14)
    }
    #[doc = "Bit 15 - Option byte read error bit."]
    #[inline(always)]
    pub fn oberr(&mut self) -> OberrW<'_, StatSpec> {
        OberrW::new(self, 15)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
