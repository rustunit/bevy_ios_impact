mod feedback_style;

#[cfg(target_os = "ios")]
mod impact_generators;

#[cfg(target_os = "ios")]
pub use impact_generators::ImpactGenerators;

pub use feedback_style::FeedbackStyle;

#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug, Default)]
pub struct ImpactGenerators;

#[cfg(not(target_os = "ios"))]
impl ImpactGenerators {
    pub fn prepare(&self) {}
    #[allow(unused_variables)]
    pub fn impact(&self, style: FeedbackStyle) {}
    #[allow(unused_variables)]
    pub fn impact_with_intensity(&self, style: FeedbackStyle, intensity: f64) {}
}
