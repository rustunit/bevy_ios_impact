# bevy_ios_impact

[![crates.io][sh_crates]][lk_crates]
[![docs.rs][sh_docs]][lk_docs]
[![discord][sh_discord]][lk_discord]

[sh_crates]: https://img.shields.io/crates/v/bevy_ios_impact.svg
[lk_crates]: https://crates.io/crates/bevy_ios_impact
[sh_docs]: https://img.shields.io/docsrs/bevy_ios_impact
[lk_docs]: https://docs.rs/bevy_ios_impact/latest/bevy_ios_impact/
[sh_discord]: https://img.shields.io/discord/1176858176897953872?label=discord&color=5561E6
[lk_discord]: https://discord.gg/rQNeEnMhus

Exposes [UIImpactFeedbackGenerator](https://developer.apple.com/documentation/uikit/uiimpactfeedbackgenerator?language=objc) API via Bevy Resource.

On non-ios platforms this has no effect and will compile.

**See also:** [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap), [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications), [bevy_ios_gamecenter](https://github.com/rustunit/bevy_ios_gamecenter), [bevy_ios_review](https://github.com/rustunit/bevy_ios_review) & [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts)

Currently build for `bevy="0.14"`.

usage:
```
bevy_ios_impact = { version = "0.2" }
```

Example:
```rust
app.add_plugins(bevy_ios_impact::ImpactPlugin);

fn my_system(mut impacts: ResMut<bevy_ios_impact::ImpactResource>) {
  // optional: haptic engine might be asleep if not prepared.
  // in practice i never felt a delay, but see apple docs on this:
  // https://developer.apple.com/documentation/uikit/uifeedbackgenerator?language=objc
  impacts.prepare();

  // triggere the impact with different impact strengths (or 'style')
  impacts.impact(bevy_ios_impact::UIImpactFeedbackStyle::UIImpactFeedbackStyleHeavy);
}
```

## Bevy version support

|bevy|bevy\_ios\_impact|
|----|---|
|0.14|0.2,main|
|0.13|0.1|

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
