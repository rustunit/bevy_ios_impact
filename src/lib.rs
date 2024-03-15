//! Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API
//! via Bevy Resource to be easily accessible in Bevy Projects.
//!
//! # Example:
//! ```
//! app.add_plugins(bevy_ios_impact::ImpactPlugin);
//!
//! fn my_system(mut impacts: ResMut<bevy_ios_impact::ImpactResource>) {
//!   // triggere the impact with different impact strengths (or 'style')
//!   impacts.impact(bevy_ios_impact::UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy);
//! }
//! ```
pub(crate) mod inner;
mod plugin;
mod resource;

pub use inner::FeedbackStyle;
pub use plugin::ImpactPlugin;
pub use resource::ImpactResource;
