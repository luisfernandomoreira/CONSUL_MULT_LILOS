#[doc = "Register `CMP1_CS` reader"]
pub type R = crate::R<Cmp1CsSpec>;
#[doc = "Register `CMP1_CS` writer"]
pub type W = crate::W<Cmp1CsSpec>;
#[doc = "Field `CMP1EN` reader - CMP1 enable."]
pub type Cmp1enR = crate::BitReader;
#[doc = "Field `CMP1EN` writer - CMP1 enable."]
pub type Cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1M` reader - CMP1 mode"]
pub type Cmp1mR = crate::FieldReader;
#[doc = "Field `CMP1M` writer - CMP1 mode"]
pub type Cmp1mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1MSEL` reader - CMP1_IM input selection"]
pub type Cmp1mselR = crate::FieldReader;
#[doc = "Field `CMP1MSEL` writer - CMP1_IM input selection"]
pub type Cmp1mselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1SW` reader - CMP1 switch"]
pub type Cmp1swR = crate::BitReader;
#[doc = "Field `CMP1SW` writer - CMP1 switch"]
pub type Cmp1swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1PL` reader - Polarity of CMP1 output"]
pub type Cmp1plR = crate::BitReader;
#[doc = "Field `CMP1PL` writer - Polarity of CMP1 output"]
pub type Cmp1plW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1HST` reader - CMP1 hysteresis"]
pub type Cmp1hstR = crate::FieldReader;
#[doc = "Field `CMP1HST` writer - CMP1 hysteresis"]
pub type Cmp1hstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1BLK` reader - CMP1 output blanking source"]
pub type Cmp1blkR = crate::FieldReader;
#[doc = "Field `CMP1BLK` writer - CMP1 output blanking source"]
pub type Cmp1blkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1BEN` reader - Scaler bridge enable bit"]
pub type Cmp1benR = crate::BitReader;
#[doc = "Field `CMP1BEN` writer - Scaler bridge enable bit"]
pub type Cmp1benW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1SEN` reader - Voltage scaler enable bit"]
pub type Cmp1senR = crate::BitReader;
#[doc = "Field `CMP1SEN` writer - Voltage scaler enable bit"]
pub type Cmp1senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1O` reader - CMP1 output state"]
pub type Cmp1oR = crate::BitReader;
#[doc = "Field `CMP1LK` reader - CMP1 lock"]
pub type Cmp1lkR = crate::BitReader;
#[doc = "Field `CMP1LK` writer - CMP1 lock"]
pub type Cmp1lkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP1 enable."]
    #[inline(always)]
    pub fn cmp1en(&self) -> Cmp1enR {
        Cmp1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMP1 mode"]
    #[inline(always)]
    pub fn cmp1m(&self) -> Cmp1mR {
        Cmp1mR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - CMP1_IM input selection"]
    #[inline(always)]
    pub fn cmp1msel(&self) -> Cmp1mselR {
        Cmp1mselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - CMP1 switch"]
    #[inline(always)]
    pub fn cmp1sw(&self) -> Cmp1swR {
        Cmp1swR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Polarity of CMP1 output"]
    #[inline(always)]
    pub fn cmp1pl(&self) -> Cmp1plR {
        Cmp1plR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - CMP1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hst(&self) -> Cmp1hstR {
        Cmp1hstR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - CMP1 output blanking source"]
    #[inline(always)]
    pub fn cmp1blk(&self) -> Cmp1blkR {
        Cmp1blkR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable bit"]
    #[inline(always)]
    pub fn cmp1ben(&self) -> Cmp1benR {
        Cmp1benR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn cmp1sen(&self) -> Cmp1senR {
        Cmp1senR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP1 output state"]
    #[inline(always)]
    pub fn cmp1o(&self) -> Cmp1oR {
        Cmp1oR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&self) -> Cmp1lkR {
        Cmp1lkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP1 enable."]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> Cmp1enW<'_, Cmp1CsSpec> {
        Cmp1enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CMP1 mode"]
    #[inline(always)]
    pub fn cmp1m(&mut self) -> Cmp1mW<'_, Cmp1CsSpec> {
        Cmp1mW::new(self, 2)
    }
    #[doc = "Bits 4:6 - CMP1_IM input selection"]
    #[inline(always)]
    pub fn cmp1msel(&mut self) -> Cmp1mselW<'_, Cmp1CsSpec> {
        Cmp1mselW::new(self, 4)
    }
    #[doc = "Bit 11 - CMP1 switch"]
    #[inline(always)]
    pub fn cmp1sw(&mut self) -> Cmp1swW<'_, Cmp1CsSpec> {
        Cmp1swW::new(self, 11)
    }
    #[doc = "Bit 15 - Polarity of CMP1 output"]
    #[inline(always)]
    pub fn cmp1pl(&mut self) -> Cmp1plW<'_, Cmp1CsSpec> {
        Cmp1plW::new(self, 15)
    }
    #[doc = "Bits 16:17 - CMP1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hst(&mut self) -> Cmp1hstW<'_, Cmp1CsSpec> {
        Cmp1hstW::new(self, 16)
    }
    #[doc = "Bits 18:20 - CMP1 output blanking source"]
    #[inline(always)]
    pub fn cmp1blk(&mut self) -> Cmp1blkW<'_, Cmp1CsSpec> {
        Cmp1blkW::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable bit"]
    #[inline(always)]
    pub fn cmp1ben(&mut self) -> Cmp1benW<'_, Cmp1CsSpec> {
        Cmp1benW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn cmp1sen(&mut self) -> Cmp1senW<'_, Cmp1CsSpec> {
        Cmp1senW::new(self, 23)
    }
    #[doc = "Bit 31 - CMP1 lock"]
    #[inline(always)]
    pub fn cmp1lk(&mut self) -> Cmp1lkW<'_, Cmp1CsSpec> {
        Cmp1lkW::new(self, 31)
    }
}
#[doc = "CMP1 Control / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1CsSpec;
impl crate::RegisterSpec for Cmp1CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_cs::R`](R) reader structure"]
impl crate::Readable for Cmp1CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1_cs::W`](W) writer structure"]
impl crate::Writable for Cmp1CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1_CS to value 0"]
impl crate::Resettable for Cmp1CsSpec {}
