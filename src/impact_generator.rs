use std::sync::{Arc, Mutex};

use crate::{UIImpactFeedbackGenerator, UIImpactFeedbackStyle};

/// threadsafe abstraction over `UIImpactFeedbackGenerator`.
///
/// [`UIImpactFeedbackGenerator`]: crate::UIImpactFeedbackGenerator
#[derive(Clone, Debug)]
pub struct ImpactFeedbackGenerator {
    internal: Arc<Mutex<objc2::rc::Id<UIImpactFeedbackGenerator>>>,
}

unsafe impl Send for ImpactFeedbackGenerator {}
unsafe impl Sync for ImpactFeedbackGenerator {}

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
