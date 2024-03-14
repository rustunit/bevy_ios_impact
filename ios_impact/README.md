# ios_impact

Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API via [objc2](https://crates.io/crates/objc2) Abstraction.

On non-ios platforms this has no effect and will compile.

Currently build using `objc2="0.4.1"` to match [Bevy](https://crates.io/crates/bevy).

Primary usage of this is in [`bevy_ios_impact`](https://github.com/rustunit/bevy_ios_impact) but should be usable outside of Bevy.
