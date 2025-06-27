use crate::adcom::enums::{CategoryTaxonomy, DoohVenueTaxonomy, PodSequence, SlotPositionInPod, UserAgentSource, AutoRefreshTrigger, ClickType, SizeUnit};

pub fn default_zero() -> Option<i64> {
    Some(0)
}

pub fn default_one() -> Option<i64> {
    Some(1)
}

pub fn default_two() -> Option<i64> {
    Some(2)
}

pub fn default_zero_f64() -> Option<f64> {
    Some(0.0)
}

pub fn default_usd() -> Option<String> {
    Some("USD".to_string())
}

pub fn default_one_cattax() -> Option<CategoryTaxonomy> {
    Some(CategoryTaxonomy::IABTechLabContentCategoryTaxonomy1_0)
}

pub fn default_two_cattax() -> CategoryTaxonomy {
    CategoryTaxonomy::IABTechLabContentCategoryTaxonomy2_0
}

pub fn default_nine_cattax() -> Option<CategoryTaxonomy> {
    Some(CategoryTaxonomy::IABTechLabContentTaxonomy3_1)
}

pub fn default_one_venuetax() -> Option<DoohVenueTaxonomy> {
    Some(DoohVenueTaxonomy::OpenOOH1_0)
}

pub fn default_zero_podseq() -> Option<PodSequence> {
    Some(PodSequence::AnyPosition)
}

pub fn default_zero_slotinpod() -> Option<SlotPositionInPod> {
    Some(SlotPositionInPod::NotApplicable)
}

pub fn default_zero_uasource() -> Option<UserAgentSource> {
    Some(UserAgentSource::Unknown(0))
}

pub fn default_zero_reftype() -> Option<AutoRefreshTrigger> {
    Some(AutoRefreshTrigger::Unknown(0))
}

pub fn default_one_i64() -> i64 {
    1
}

pub fn default_size_unit() -> SizeUnit {
    SizeUnit::DeviceIndependentPixels
}

pub fn default_clktype() -> ClickType {
    ClickType::ClickableUnknown
}
