#[cfg(target_os = "ios")]
use crate::{UIImpactFeedbackGenerator, UIImpactFeedbackStyle};
#[cfg(target_os = "ios")]
use std::sync::{Arc, Mutex};

/// threadsafe abstraction over `UIImpactFeedbackGenerator`.
///
/// [`UIImpactFeedbackGenerator`]: crate::UIImpactFeedbackGenerator
#[cfg(target_os = "ios")]
#[derive(Clone, Debug)]
pub struct ImpactFeedbackGenerator {
    internal: Arc<Mutex<objc2::rc::Id<UIImpactFeedbackGenerator>>>,
}

#[cfg(not(target_os = "ios"))]
#[derive(Clone, Debug)]
pub struct ImpactFeedbackGenerator;

unsafe impl Send for ImpactFeedbackGenerator {}
unsafe impl Sync for ImpactFeedbackGenerator {}

#[cfg(target_os = "ios")]
impl ImpactFeedbackGenerator {
    /// creates `UIImpactFeedbackGenerator` with the given style.
    ///
    /// implicitly prepares the generator too
    pub fn new(style: UIImpactFeedbackStyle) -> Self {
        let generator = UIImpactFeedbackGenerator::initWithStyle(style).unwrap();
        generator.prepare();
        Self {
            internal: Arc::new(Mutex::new(generator)),
        }
    }

    /// see `prepare` on `UIImpactFeedbackGenerator`.
    ///
    /// [`prepare`]: crate:UIImpactFeedbackGenerator::prepare
    pub fn prepare(&self) {
        if let Ok(internal) = self.internal.lock() {
            internal.prepare();
        }
    }

    /// see `impactOccurred` on `UIImpactFeedbackGenerator`.
    ///
    /// [`impactOccurred`]: crate:UIImpactFeedbackGenerator::impactOccurred
    pub fn impact(&self) {
        if let Ok(internal) = self.internal.lock() {
            internal.impactOccurred();
        }
    }

    /// see `impactOccurredWithIntensity` on `UIImpactFeedbackGenerator`.
    ///
    /// [`impactOccurredWithIntensity`]: crate:UIImpactFeedbackGenerator::impactOccurredWithIntensity
    pub fn impact_with_intensity(&self, intensity: f64) {
        if let Ok(internal) = self.internal.lock() {
            internal.impactOccurredWithIntensity(intensity);
        }
    }
}
