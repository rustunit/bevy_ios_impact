//! Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API
//! via a Bevy non-send resource to be easily accessible in Bevy Projects.
//!
//! UIKit's feedback generators are main-thread-only, so [`ImpactResource`] is a *non-send*
//! resource: reach for it as `NonSendMut<ImpactResource>`, which is what keeps the systems
//! using it on the main thread.
//!
//! # Example:
//! ```
//! use bevy_ecs::system::NonSendMut;
//! use bevy_ios_impact::{FeedbackStyle, ImpactPlugin, ImpactResource};
//!
//! let mut app = bevy_app::App::new();
//! app.add_plugins(ImpactPlugin);
//!
//! fn my_system(mut impacts: NonSendMut<ImpactResource>) {
//!   // trigger the impact with different impact strengths (or 'style')
//!   impacts.impact(FeedbackStyle::Light);
//! }
//! ```
pub(crate) mod inner;
mod plugin;
mod resource;

pub use inner::FeedbackStyle;
pub use plugin::ImpactPlugin;
pub use resource::ImpactResource;
