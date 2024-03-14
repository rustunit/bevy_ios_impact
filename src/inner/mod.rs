#[cfg(target_os = "ios")]
pub(crate) mod impact_generators;
#[cfg(target_os = "ios")]
mod shared_generators;

#[cfg(target_os = "ios")]
pub use shared_generators::SharedGenerators;

#[cfg(not(target_os = "ios"))]
use ios_impact::UIImpactFeedbackStyle;

#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug, Default)]
pub struct SharedGenerators;

#[cfg(not(target_os = "ios"))]
impl SharedGenerators {
    pub fn prepare(&self) {}
    #[allow(unused_variables)]
    pub fn impact(&self, style: UIImpactFeedbackStyle) {}
    #[allow(unused_variables)]
    pub fn impact_with_intensity(&self, style: UIImpactFeedbackStyle, intensity: f64) {}
}
