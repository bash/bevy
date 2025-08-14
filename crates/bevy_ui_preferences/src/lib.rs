#![no_std]

//! This crate contains a way to read various system-level UI and accessibility preferences.
//!
//! It supports the following preferences:
//! * [`UiPreferences::accent_color`]—The user's current system wide accent color preference.
//! * [`ColorScheme`]—The user's preference for either light or dark mode.
//! * [`Contrast`]—The user's preferred contrast level.
//! * [`UiPreferences::double_click_interval`]—The maximum amount of time allowed between the first and second click.
//! * [`ReducedMotion`]—The user's reduced motion preference.
//! * [`ReducedTransparency`]—The user's reduced transparency preference.
//!
//! ## Basic Usage
//!
//! Retrieve the preferences using the [`UiPreferences`] resource.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_platform::cfg;

cfg::std! {
    extern crate std;
    use bevy_tasks::futures_lite::StreamExt as _;
    use bevy_tasks::IoTaskPool;
}

mod preferences;
pub use preferences::*;

/// The UI preferences prelude.
///
/// This includes the most common types in this crate, re-exported for your convenience.
pub mod prelude {
    pub use crate::preferences::*;
    pub use crate::UiPreferencesPlugin;
}

/// The system in which the [`UiPreferences`] are updated.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, SystemSet)]
pub struct UiPreferencesSystem;

/// The UI preferences plugin.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct UiPreferencesPlugin {}

impl Plugin for UiPreferencesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "bevy_reflect")]
        app.register_type::<UiPreferences>();

        app.init_resource::<UiPreferences>()
            .configure_sets(Startup, UiPreferencesSystem)
            .configure_sets(PreUpdate, UiPreferencesSystem);

        cfg::std! {
            app.add_systems(
                Startup,
                subscribe_to_preferences.in_set(UiPreferencesSystem),
            )
            .add_systems(
                PreUpdate,
                poll_system_preferences.in_set(UiPreferencesSystem),
            );
        }
    }
}

cfg::std! {
    // Note: this function must be called from the main thread.
    fn subscribe_to_preferences(mut commands: Commands) {
        let (tx, rx) = crossbeam_channel::unbounded();
        let stream = mundy::Preferences::stream(mundy::Interest::All);
        IoTaskPool::get()
            .spawn(async move { forward_stream_to_receiver(tx, stream).await })
            .detach();
        commands.insert_resource(Receiver(rx));
    }

    async fn forward_stream_to_receiver(
        sender: crossbeam_channel::Sender<mundy::Preferences>,
        mut stream: mundy::PreferencesStream,
    ) -> Result {
        while let Some(preferences) = stream.next().await {
            sender.send(preferences)?;
        }
        Ok(())
    }

    #[derive(Debug, Resource)]
    struct Receiver(crossbeam_channel::Receiver<mundy::Preferences>);

    fn poll_system_preferences(
        receiver: Res<Receiver>,
        mut preferences_res: ResMut<UiPreferences>,
    ) -> Result {
        let preferences = match receiver.0.try_recv() {
            Ok(preferences) => preferences,
            Err(crossbeam_channel::TryRecvError::Empty) => return Ok(()),
            Err(e) => return Err(e.into()),
        };
        *preferences_res = preferences.into();
        Ok(())
    }
}
