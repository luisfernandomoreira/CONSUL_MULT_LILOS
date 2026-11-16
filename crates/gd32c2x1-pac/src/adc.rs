#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stat: Stat,
    ctl0: Ctl0,
    ctl1: Ctl1,
    sampt0: Sampt0,
    sampt1: Sampt1,
    ioff0: Ioff0,
    ioff1: Ioff1,
    ioff2: Ioff2,
    ioff3: Ioff3,
    wd0ht: Wd0ht,
    wd0lt: Wd0lt,
    rsq0: Rsq0,
    rsq1: Rsq1,
    rsq2: Rsq2,
    isq: Isq,
    idata0: Idata0,
    idata1: Idata1,
    idata2: Idata2,
    idata3: Idata3,
    rdata: Rdata,
    wd1sr: Wd1sr,
    wd2sr: Wd2sr,
    wd1ht: Wd1ht,
    wd1lt: Wd1lt,
    wd2ht: Wd2ht,
    wd2lt: Wd2lt,
    _reserved26: [u8; 0x18],
    ovsampctl: Ovsampctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x04 - Control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x08 - Control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x0c - Sample time register 0"]
    #[inline(always)]
    pub const fn sampt0(&self) -> &Sampt0 {
        &self.sampt0
    }
    #[doc = "0x10 - Sample time register 1"]
    #[inline(always)]
    pub const fn sampt1(&self) -> &Sampt1 {
        &self.sampt1
    }
    #[doc = "0x14 - Inserted channel data offset register 0"]
    #[inline(always)]
    pub const fn ioff0(&self) -> &Ioff0 {
        &self.ioff0
    }
    #[doc = "0x18 - Inserted channel data offset register 1"]
    #[inline(always)]
    pub const fn ioff1(&self) -> &Ioff1 {
        &self.ioff1
    }
    #[doc = "0x1c - Inserted channel data offset register 2"]
    #[inline(always)]
    pub const fn ioff2(&self) -> &Ioff2 {
        &self.ioff2
    }
    #[doc = "0x20 - Inserted channel data offset register 3"]
    #[inline(always)]
    pub const fn ioff3(&self) -> &Ioff3 {
        &self.ioff3
    }
    #[doc = "0x24 - Watchdog 0 high threshold register"]
    #[inline(always)]
    pub const fn wd0ht(&self) -> &Wd0ht {
        &self.wd0ht
    }
    #[doc = "0x28 - Watchdog 0 low threshold register"]
    #[inline(always)]
    pub const fn wd0lt(&self) -> &Wd0lt {
        &self.wd0lt
    }
    #[doc = "0x2c - Routine sequence register 0"]
    #[inline(always)]
    pub const fn rsq0(&self) -> &Rsq0 {
        &self.rsq0
    }
    #[doc = "0x30 - Routine sequence register 1"]
    #[inline(always)]
    pub const fn rsq1(&self) -> &Rsq1 {
        &self.rsq1
    }
    #[doc = "0x34 - Routine sequence register 2"]
    #[inline(always)]
    pub const fn rsq2(&self) -> &Rsq2 {
        &self.rsq2
    }
    #[doc = "0x38 - Inserted sequence register"]
    #[inline(always)]
    pub const fn isq(&self) -> &Isq {
        &self.isq
    }
    #[doc = "0x3c - Inserted data register 0"]
    #[inline(always)]
    pub const fn idata0(&self) -> &Idata0 {
        &self.idata0
    }
    #[doc = "0x40 - Inserted data register 1"]
    #[inline(always)]
    pub const fn idata1(&self) -> &Idata1 {
        &self.idata1
    }
    #[doc = "0x44 - Inserted data register 2"]
    #[inline(always)]
    pub const fn idata2(&self) -> &Idata2 {
        &self.idata2
    }
    #[doc = "0x48 - Inserted data register 3"]
    #[inline(always)]
    pub const fn idata3(&self) -> &Idata3 {
        &self.idata3
    }
    #[doc = "0x4c - Routine data register"]
    #[inline(always)]
    pub const fn rdata(&self) -> &Rdata {
        &self.rdata
    }
    #[doc = "0x50 - Watchdog 1 channel selection register"]
    #[inline(always)]
    pub const fn wd1sr(&self) -> &Wd1sr {
        &self.wd1sr
    }
    #[doc = "0x54 - Watchdog 2 channel selection register"]
    #[inline(always)]
    pub const fn wd2sr(&self) -> &Wd2sr {
        &self.wd2sr
    }
    #[doc = "0x58 - Watchdog 1 high threshold register1"]
    #[inline(always)]
    pub const fn wd1ht(&self) -> &Wd1ht {
        &self.wd1ht
    }
    #[doc = "0x5c - Watchdog 1 low threshold register1"]
    #[inline(always)]
    pub const fn wd1lt(&self) -> &Wd1lt {
        &self.wd1lt
    }
    #[doc = "0x60 - Watchdog 2 high threshold register2"]
    #[inline(always)]
    pub const fn wd2ht(&self) -> &Wd2ht {
        &self.wd2ht
    }
    #[doc = "0x64 - Watchdog 2 low threshold register2"]
    #[inline(always)]
    pub const fn wd2lt(&self) -> &Wd2lt {
        &self.wd2lt
    }
    #[doc = "0x80 - Oversample control register"]
    #[inline(always)]
    pub const fn ovsampctl(&self) -> &Ovsampctl {
        &self.ovsampctl
    }
}
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "CTL0 (rw) register accessor: Control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`] module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`] module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control register 1"]
pub mod ctl1;
#[doc = "SAMPT0 (rw) register accessor: Sample time register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sampt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt0`] module"]
#[doc(alias = "SAMPT0")]
pub type Sampt0 = crate::Reg<sampt0::Sampt0Spec>;
#[doc = "Sample time register 0"]
pub mod sampt0;
#[doc = "SAMPT1 (rw) register accessor: Sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sampt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sampt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sampt1`] module"]
#[doc(alias = "SAMPT1")]
pub type Sampt1 = crate::Reg<sampt1::Sampt1Spec>;
#[doc = "Sample time register 1"]
pub mod sampt1;
#[doc = "IOFF0 (rw) register accessor: Inserted channel data offset register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff0`] module"]
#[doc(alias = "IOFF0")]
pub type Ioff0 = crate::Reg<ioff0::Ioff0Spec>;
#[doc = "Inserted channel data offset register 0"]
pub mod ioff0;
#[doc = "IOFF1 (rw) register accessor: Inserted channel data offset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff1`] module"]
#[doc(alias = "IOFF1")]
pub type Ioff1 = crate::Reg<ioff1::Ioff1Spec>;
#[doc = "Inserted channel data offset register 1"]
pub mod ioff1;
#[doc = "IOFF2 (rw) register accessor: Inserted channel data offset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff2`] module"]
#[doc(alias = "IOFF2")]
pub type Ioff2 = crate::Reg<ioff2::Ioff2Spec>;
#[doc = "Inserted channel data offset register 2"]
pub mod ioff2;
#[doc = "IOFF3 (rw) register accessor: Inserted channel data offset register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ioff3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioff3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioff3`] module"]
#[doc(alias = "IOFF3")]
pub type Ioff3 = crate::Reg<ioff3::Ioff3Spec>;
#[doc = "Inserted channel data offset register 3"]
pub mod ioff3;
#[doc = "WD0HT (rw) register accessor: Watchdog 0 high threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd0ht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd0ht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd0ht`] module"]
#[doc(alias = "WD0HT")]
pub type Wd0ht = crate::Reg<wd0ht::Wd0htSpec>;
#[doc = "Watchdog 0 high threshold register"]
pub mod wd0ht;
#[doc = "WD0LT (rw) register accessor: Watchdog 0 low threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd0lt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd0lt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd0lt`] module"]
#[doc(alias = "WD0LT")]
pub type Wd0lt = crate::Reg<wd0lt::Wd0ltSpec>;
#[doc = "Watchdog 0 low threshold register"]
pub mod wd0lt;
#[doc = "RSQ0 (rw) register accessor: Routine sequence register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rsq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq0`] module"]
#[doc(alias = "RSQ0")]
pub type Rsq0 = crate::Reg<rsq0::Rsq0Spec>;
#[doc = "Routine sequence register 0"]
pub mod rsq0;
#[doc = "RSQ1 (rw) register accessor: Routine sequence register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rsq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq1`] module"]
#[doc(alias = "RSQ1")]
pub type Rsq1 = crate::Reg<rsq1::Rsq1Spec>;
#[doc = "Routine sequence register 1"]
pub mod rsq1;
#[doc = "RSQ2 (rw) register accessor: Routine sequence register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`rsq2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsq2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsq2`] module"]
#[doc(alias = "RSQ2")]
pub type Rsq2 = crate::Reg<rsq2::Rsq2Spec>;
#[doc = "Routine sequence register 2"]
pub mod rsq2;
#[doc = "ISQ (rw) register accessor: Inserted sequence register\n\nYou can [`read`](crate::Reg::read) this register and get [`isq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isq`] module"]
#[doc(alias = "ISQ")]
pub type Isq = crate::Reg<isq::IsqSpec>;
#[doc = "Inserted sequence register"]
pub mod isq;
#[doc = "IDATA0 (r) register accessor: Inserted data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`idata0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata0`] module"]
#[doc(alias = "IDATA0")]
pub type Idata0 = crate::Reg<idata0::Idata0Spec>;
#[doc = "Inserted data register 0"]
pub mod idata0;
#[doc = "IDATA1 (r) register accessor: Inserted data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`idata1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata1`] module"]
#[doc(alias = "IDATA1")]
pub type Idata1 = crate::Reg<idata1::Idata1Spec>;
#[doc = "Inserted data register 1"]
pub mod idata1;
#[doc = "IDATA2 (r) register accessor: Inserted data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`idata2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata2`] module"]
#[doc(alias = "IDATA2")]
pub type Idata2 = crate::Reg<idata2::Idata2Spec>;
#[doc = "Inserted data register 2"]
pub mod idata2;
#[doc = "IDATA3 (r) register accessor: Inserted data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`idata3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idata3`] module"]
#[doc(alias = "IDATA3")]
pub type Idata3 = crate::Reg<idata3::Idata3Spec>;
#[doc = "Inserted data register 3"]
pub mod idata3;
#[doc = "RDATA (r) register accessor: Routine data register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdata`] module"]
#[doc(alias = "RDATA")]
pub type Rdata = crate::Reg<rdata::RdataSpec>;
#[doc = "Routine data register"]
pub mod rdata;
#[doc = "WD1SR (rw) register accessor: Watchdog 1 channel selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd1sr`] module"]
#[doc(alias = "WD1SR")]
pub type Wd1sr = crate::Reg<wd1sr::Wd1srSpec>;
#[doc = "Watchdog 1 channel selection register"]
pub mod wd1sr;
#[doc = "WD2SR (rw) register accessor: Watchdog 2 channel selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`wd2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd2sr`] module"]
#[doc(alias = "WD2SR")]
pub type Wd2sr = crate::Reg<wd2sr::Wd2srSpec>;
#[doc = "Watchdog 2 channel selection register"]
pub mod wd2sr;
#[doc = "WD1HT (rw) register accessor: Watchdog 1 high threshold register1\n\nYou can [`read`](crate::Reg::read) this register and get [`wd1ht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd1ht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd1ht`] module"]
#[doc(alias = "WD1HT")]
pub type Wd1ht = crate::Reg<wd1ht::Wd1htSpec>;
#[doc = "Watchdog 1 high threshold register1"]
pub mod wd1ht;
#[doc = "WD1LT (rw) register accessor: Watchdog 1 low threshold register1\n\nYou can [`read`](crate::Reg::read) this register and get [`wd1lt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd1lt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd1lt`] module"]
#[doc(alias = "WD1LT")]
pub type Wd1lt = crate::Reg<wd1lt::Wd1ltSpec>;
#[doc = "Watchdog 1 low threshold register1"]
pub mod wd1lt;
#[doc = "WD2HT (rw) register accessor: Watchdog 2 high threshold register2\n\nYou can [`read`](crate::Reg::read) this register and get [`wd2ht::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd2ht::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd2ht`] module"]
#[doc(alias = "WD2HT")]
pub type Wd2ht = crate::Reg<wd2ht::Wd2htSpec>;
#[doc = "Watchdog 2 high threshold register2"]
pub mod wd2ht;
#[doc = "WD2LT (rw) register accessor: Watchdog 2 low threshold register2\n\nYou can [`read`](crate::Reg::read) this register and get [`wd2lt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wd2lt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wd2lt`] module"]
#[doc(alias = "WD2LT")]
pub type Wd2lt = crate::Reg<wd2lt::Wd2ltSpec>;
#[doc = "Watchdog 2 low threshold register2"]
pub mod wd2lt;
#[doc = "OVSAMPCTL (rw) register accessor: Oversample control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsampctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovsampctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovsampctl`] module"]
#[doc(alias = "OVSAMPCTL")]
pub type Ovsampctl = crate::Reg<ovsampctl::OvsampctlSpec>;
#[doc = "Oversample control register"]
pub mod ovsampctl;
