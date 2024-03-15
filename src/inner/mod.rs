mod feedback_style;

#[cfg(target_os = "ios")]
pub(crate) mod impact_generators;
#[cfg(target_os = "ios")]
mod shared_generators;

#[cfg(target_os = "ios")]
pub use shared_generators::SharedGenerators;

pub use feedback_style::FeedbackStyle;

#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug, Default)]
pub struct SharedGenerators;

#[cfg(not(target_os = "ios"))]
impl SharedGenerators {
    pub fn prepare(&self) {}
    #[allow(unused_variables)]
    pub fn impact(&self, style: FeedbackStyle) {}
    #[allow(unused_variables)]
    pub fn impact_with_intensity(&self, style: FeedbackStyle, intensity: f64) {}
}
