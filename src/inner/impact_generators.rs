//! The five `UIImpactFeedbackGenerator`s, one per style, kept alive for the
//! lifetime of the app.
//!
//! `objc2-ui-kit` marks `UIFeedbackGenerator` `MainThreadOnly`, which makes the
//! generators neither `Send` nor `Sync` — hence the non-send resource wrapping
//! them. Earlier versions hid that behind a hand-written `objc2` binding and an
//! `unsafe impl Send`, which let a system fire a haptic from a worker thread.

use objc2::{MainThreadMarker, MainThreadOnly, rc::Retained};
use objc2_ui_kit::{UIImpactFeedbackGenerator, UIImpactFeedbackStyle};

use super::feedback_style::FeedbackStyle;

#[derive(Clone, Debug)]
pub struct ImpactGenerators {
    light: Retained<UIImpactFeedbackGenerator>,
    medium: Retained<UIImpactFeedbackGenerator>,
    heavy: Retained<UIImpactFeedbackGenerator>,
    soft: Retained<UIImpactFeedbackGenerator>,
    rigid: Retained<UIImpactFeedbackGenerator>,
}

impl Default for ImpactGenerators {
    fn default() -> Self {
        // Bevy panics on non-send access off the main thread anyway, so this only ever
        // fires ahead of a panic it would have raised itself.
        let mtm =
            MainThreadMarker::new().expect("ImpactResource must be created on the main thread");

        Self {
            light: generator(mtm, UIImpactFeedbackStyle::Light),
            medium: generator(mtm, UIImpactFeedbackStyle::Medium),
            heavy: generator(mtm, UIImpactFeedbackStyle::Heavy),
            soft: generator(mtm, UIImpactFeedbackStyle::Soft),
            rigid: generator(mtm, UIImpactFeedbackStyle::Rigid),
        }
    }
}

impl ImpactGenerators {
    pub fn prepare(&self) {
        self.light.prepare();
    }

    pub fn impact(&self, style: FeedbackStyle) {
        self.get(style).impactOccurred();
    }

    pub fn impact_with_intensity(&self, style: FeedbackStyle, intensity: f64) {
        self.get(style).impactOccurredWithIntensity(intensity);
    }

    fn get(&self, style: FeedbackStyle) -> &UIImpactFeedbackGenerator {
        match style {
            FeedbackStyle::Light => &self.light,
            FeedbackStyle::Medium => &self.medium,
            FeedbackStyle::Heavy => &self.heavy,
            FeedbackStyle::Soft => &self.soft,
            FeedbackStyle::Rigid => &self.rigid,
        }
    }
}

// Deprecated in favour of `feedbackGeneratorWithStyle:forView:`, which wants a `UIView`
// this crate never sees.
#[allow(deprecated)]
fn generator(
    mtm: MainThreadMarker,
    style: UIImpactFeedbackStyle,
) -> Retained<UIImpactFeedbackGenerator> {
    UIImpactFeedbackGenerator::initWithStyle(UIImpactFeedbackGenerator::alloc(mtm), style)
}
