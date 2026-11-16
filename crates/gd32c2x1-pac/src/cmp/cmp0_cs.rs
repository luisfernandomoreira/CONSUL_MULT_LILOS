#[doc = "Register `CMP0_CS` reader"]
pub type R = crate::R<Cmp0CsSpec>;
#[doc = "Register `CMP0_CS` writer"]
pub type W = crate::W<Cmp0CsSpec>;
#[doc = "Field `CMP0EN` reader - CMP0 enable"]
pub type Cmp0enR = crate::BitReader;
#[doc = "Field `CMP0EN` writer - CMP0 enable"]
pub type Cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0M` reader - CMP0 mode"]
pub type Cmp0mR = crate::FieldReader;
#[doc = "Field `CMP0M` writer - CMP0 mode"]
pub type Cmp0mW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP0MSEL` reader - CMP0_IM input selection"]
pub type Cmp0mselR = crate::FieldReader;
#[doc = "Field `CMP0MSEL` writer - CMP0_IM input selection"]
pub type Cmp0mselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP0SW` reader - CMP0 switch"]
pub type Cmp0swR = crate::BitReader;
#[doc = "Field `CMP0SW` writer - CMP0 switch"]
pub type Cmp0swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0PL` reader - Polarity of CMP0 output"]
pub type Cmp0plR = crate::BitReader;
#[doc = "Field `CMP0PL` writer - Polarity of CMP0 output"]
pub type Cmp0plW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0HST` reader - CMP0 hysteresis"]
pub type Cmp0hstR = crate::FieldReader;
#[doc = "Field `CMP0HST` writer - CMP0 hysteresis"]
pub type Cmp0hstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP0BLK` reader - CMP0 output blanking source"]
pub type Cmp0blkR = crate::FieldReader;
#[doc = "Field `CMP0BLK` writer - CMP0 output blanking source"]
pub type Cmp0blkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP0BEN` reader - Scaler bridge enable bit"]
pub type Cmp0benR = crate::BitReader;
#[doc = "Field `CMP0BEN` writer - Scaler bridge enable bit"]
pub type Cmp0benW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0SEN` reader - Voltage scaler enable bit"]
pub type Cmp0senR = crate::BitReader;
#[doc = "Field `CMP0SEN` writer - Voltage scaler enable bit"]
pub type Cmp0senW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0O` reader - CMP0 output state"]
pub type Cmp0oR = crate::BitReader;
#[doc = "Field `CMP0LK` reader - CMP0 lock"]
pub type Cmp0lkR = crate::BitReader;
#[doc = "Field `CMP0LK` writer - CMP0 lock"]
pub type Cmp0lkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> Cmp0enR {
        Cmp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    pub fn cmp0m(&self) -> Cmp0mR {
        Cmp0mR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - CMP0_IM input selection"]
    #[inline(always)]
    pub fn cmp0msel(&self) -> Cmp0mselR {
        Cmp0mselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - CMP0 switch"]
    #[inline(always)]
    pub fn cmp0sw(&self) -> Cmp0swR {
        Cmp0swR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Polarity of CMP0 output"]
    #[inline(always)]
    pub fn cmp0pl(&self) -> Cmp0plR {
        Cmp0plR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - CMP0 hysteresis"]
    #[inline(always)]
    pub fn cmp0hst(&self) -> Cmp0hstR {
        Cmp0hstR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - CMP0 output blanking source"]
    #[inline(always)]
    pub fn cmp0blk(&self) -> Cmp0blkR {
        Cmp0blkR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Scaler bridge enable bit"]
    #[inline(always)]
    pub fn cmp0ben(&self) -> Cmp0benR {
        Cmp0benR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn cmp0sen(&self) -> Cmp0senR {
        Cmp0senR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 30 - CMP0 output state"]
    #[inline(always)]
    pub fn cmp0o(&self) -> Cmp0oR {
        Cmp0oR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMP0 lock"]
    #[inline(always)]
    pub fn cmp0lk(&self) -> Cmp0lkR {
        Cmp0lkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP0 enable"]
    #[inline(always)]
    pub fn cmp0en(&mut self) -> Cmp0enW<'_, Cmp0CsSpec> {
        Cmp0enW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CMP0 mode"]
    #[inline(always)]
    pub fn cmp0m(&mut self) -> Cmp0mW<'_, Cmp0CsSpec> {
        Cmp0mW::new(self, 2)
    }
    #[doc = "Bits 4:6 - CMP0_IM input selection"]
    #[inline(always)]
    pub fn cmp0msel(&mut self) -> Cmp0mselW<'_, Cmp0CsSpec> {
        Cmp0mselW::new(self, 4)
    }
    #[doc = "Bit 11 - CMP0 switch"]
    #[inline(always)]
    pub fn cmp0sw(&mut self) -> Cmp0swW<'_, Cmp0CsSpec> {
        Cmp0swW::new(self, 11)
    }
    #[doc = "Bit 15 - Polarity of CMP0 output"]
    #[inline(always)]
    pub fn cmp0pl(&mut self) -> Cmp0plW<'_, Cmp0CsSpec> {
        Cmp0plW::new(self, 15)
    }
    #[doc = "Bits 16:17 - CMP0 hysteresis"]
    #[inline(always)]
    pub fn cmp0hst(&mut self) -> Cmp0hstW<'_, Cmp0CsSpec> {
        Cmp0hstW::new(self, 16)
    }
    #[doc = "Bits 18:20 - CMP0 output blanking source"]
    #[inline(always)]
    pub fn cmp0blk(&mut self) -> Cmp0blkW<'_, Cmp0CsSpec> {
        Cmp0blkW::new(self, 18)
    }
    #[doc = "Bit 22 - Scaler bridge enable bit"]
    #[inline(always)]
    pub fn cmp0ben(&mut self) -> Cmp0benW<'_, Cmp0CsSpec> {
        Cmp0benW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn cmp0sen(&mut self) -> Cmp0senW<'_, Cmp0CsSpec> {
        Cmp0senW::new(self, 23)
    }
    #[doc = "Bit 31 - CMP0 lock"]
    #[inline(always)]
    pub fn cmp0lk(&mut self) -> Cmp0lkW<'_, Cmp0CsSpec> {
        Cmp0lkW::new(self, 31)
    }
}
#[doc = "CMP0 Control / Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0_cs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0_cs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp0CsSpec;
impl crate::RegisterSpec for Cmp0CsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0_cs::R`](R) reader structure"]
impl crate::Readable for Cmp0CsSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp0_cs::W`](W) writer structure"]
impl crate::Writable for Cmp0CsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP0_CS to value 0"]
impl crate::Resettable for Cmp0CsSpec {}
