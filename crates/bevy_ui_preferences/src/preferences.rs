#[cfg(feature = "accent-color")]
use bevy_color::{Color, Srgba};
use bevy_ecs::prelude::*;
use bevy_platform::cfg;
#[cfg(feature = "bevy_reflect")]
use bevy_reflect::prelude::*;

/// A collection of system-level UI preferences.
#[derive(Debug, Default, Clone, Copy, PartialEq, Resource)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
#[non_exhaustive]
pub struct UiPreferences {
    /// The user's preference for either light or dark mode.
    #[cfg(feature = "color-scheme")]
    pub color_scheme: ColorScheme,
    /// The user's preferred contrast level.
    #[cfg(feature = "contrast")]
    pub contrast: Contrast,
    /// The user's reduced motion preference.
    #[cfg(feature = "reduced-motion")]
    pub reduced_motion: ReducedMotion,
    /// The user's reduced transparency preference.
    #[cfg(feature = "reduced-transparency")]
    pub reduced_transparency: ReducedTransparency,
    /// The user's current system wide accent color preference.
    #[cfg(feature = "accent-color")]
    pub accent_color: Option<Color>,
    /// The maximum amount of time that may occur between the first and second click
    /// event for it to count as double click.
    #[cfg(feature = "double-click-interval")]
    pub double_click_interval: Option<core::time::Duration>,
}

cfg::std! {
    impl From<mundy::Preferences> for UiPreferences {
        fn from(value: mundy::Preferences) -> Self {
            UiPreferences {
                #[cfg(feature = "color-scheme")]
                color_scheme: value.color_scheme.into(),
                #[cfg(feature = "contrast")]
                contrast: value.contrast.into(),
                #[cfg(feature = "reduced-motion")]
                reduced_motion: value.reduced_motion.into(),
                #[cfg(feature = "reduced-transparency")]
                reduced_transparency: value.reduced_transparency.into(),
                #[cfg(feature = "accent-color")]
                accent_color: to_bevy_color(value.accent_color),
                #[cfg(feature = "double-click-interval")]
                double_click_interval: value.double_click_interval.0,
            }
        }
    }
}

/// The user's preference for either light or dark mode.
/// This corresponds to the [`prefers-color-scheme`] CSS media feature.
///
/// [`prefers-color-scheme`]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
#[cfg(feature = "color-scheme")]
pub enum ColorScheme {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a color scheme preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with a light appearance.
    Light,
    /// Indicates that the user prefers an interface with a dark appearance.
    Dark,
}

cfg::std! {
    #[cfg(feature = "color-scheme")]
    impl From<mundy::ColorScheme> for ColorScheme {
        fn from(value: mundy::ColorScheme) -> Self {
            match value {
                mundy::ColorScheme::NoPreference => ColorScheme::NoPreference,
                mundy::ColorScheme::Light => ColorScheme::Light,
                mundy::ColorScheme::Dark => ColorScheme::Dark,
            }
        }
    }
}

#[cfg(feature = "color-scheme")]
impl ColorScheme {
    fn is_no_preference(self) -> bool {
        matches!(self, ColorScheme::NoPreference)
    }

    fn is_dark(self) -> bool {
        matches!(self, ColorScheme::Dark)
    }

    fn is_light(self) -> bool {
        matches!(self, ColorScheme::Light)
    }
}

/// The user's preferred contrast level.
/// This corresponds to the [`prefers-contrast`] CSS media feature.
///
/// [`prefers-contrast`]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-contrast
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg(feature = "contrast")]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum Contrast {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a contrast preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with a higher level of contrast.
    More,
    /// Indicates that the user prefers an interface with a lower level of contrast.
    Less,
    /// Indicates that the user has configured a specific set of colors (forced color mode)
    /// and the contrast from these colors neither matches [`Contrast::More`] or [`Contrast::Less`].
    Custom,
}

#[cfg(feature = "contrast")]
impl Contrast {
    fn is_no_preference(self) -> bool {
        matches!(self, Contrast::NoPreference)
    }

    fn is_more(self) -> bool {
        matches!(self, Contrast::More)
    }

    fn is_less(self) -> bool {
        matches!(self, Contrast::Less)
    }

    fn is_custom(self) -> bool {
        matches!(self, Contrast::Custom)
    }
}

cfg::std! {
    #[cfg(feature = "contrast")]
    impl From<mundy::Contrast> for Contrast {
        fn from(value: mundy::Contrast) -> Self {
            match value {
                mundy::Contrast::NoPreference => Contrast::NoPreference,
                mundy::Contrast::More => Contrast::More,
                mundy::Contrast::Less => Contrast::Less,
                mundy::Contrast::Custom => Contrast::Custom,
            }
        }
    }
}

/// The user prefers to have a minimal amount of motion. Especially motion that simulates the third dimension.
/// This corresponds to the [`prefers-reduced-motion`] CSS media feature.
///
/// Such motion can cause discomfort to people with [vestibular disorders](https://www.a11yproject.com/posts/understanding-vestibular-disorders/).
///
/// [`prefers-reduced-motion`]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg(feature = "reduced-motion")]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum ReducedMotion {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a reduced motion preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers a minimal amount of motion.
    Reduce,
}

#[cfg(feature = "reduced-motion")]
cfg::std! {
    impl From<mundy::ReducedMotion> for ReducedMotion {
        fn from(value: mundy::ReducedMotion) -> Self {
            match value {
                mundy::ReducedMotion::NoPreference => ReducedMotion::NoPreference,
                mundy::ReducedMotion::Reduce => ReducedMotion::Reduce,
            }
        }
    }
}

#[cfg(feature = "reduced-motion")]
impl ReducedMotion {
    fn is_no_preference(self) -> bool {
        matches!(self, ReducedMotion::NoPreference)
    }

    fn is_reduce(self) -> bool {
        matches!(self, ReducedMotion::Reduce)
    }
}

/// Indicates that applications should not use transparent or semitransparent backgrounds.
/// This corresponds to the [`prefers-reduced-transparency`] CSS media feature.
///
/// [`prefers-reduced-transparency`]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-transparency
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg(feature = "reduced-transparency")]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(Reflect),
    reflect(Clone, PartialEq, Default)
)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    all(feature = "serialize", feature = "bevy_reflect"),
    reflect(Serialize, Deserialize)
)]
pub enum ReducedTransparency {
    /// Indicates that the user has not expressed an active preference,
    /// that the current platform doesn't support a reduced transparency preference
    /// or that an error occurred while trying to retrieve the preference.
    #[default]
    NoPreference,
    /// Indicates that the user prefers an interface with no transparent
    /// or semitransparent backgrounds.
    Reduce,
}

#[cfg(feature = "reduced-transparency")]
cfg::std! {
    impl From<mundy::ReducedTransparency> for ReducedTransparency {
        fn from(value: mundy::ReducedTransparency) -> Self {
            match value {
                mundy::ReducedTransparency::NoPreference => ReducedTransparency::NoPreference,
                mundy::ReducedTransparency::Reduce => ReducedTransparency::Reduce,
            }
        }
    }
}

#[cfg(feature = "reduced-transparency")]
impl ReducedTransparency {
    fn is_no_preference(self) -> bool {
        matches!(self, ReducedTransparency::NoPreference)
    }

    fn is_reduce(self) -> bool {
        matches!(self, ReducedTransparency::Reduce)
    }
}

#[cfg(feature = "accent-color")]
cfg::std! {
    fn to_bevy_color(color: mundy::AccentColor) -> Option<Color> {
        use bevy_color::ColorToComponents as _;
        color
            .0
            .map(|c| Srgba::from_f32_array(c.to_f64_array().map(|c| c as f32)).into())
    }
}
