use crate::CombinedDeviceError;

/// In the case of scenes, the type maps 1:1 to the trait, as scenes don't combine with other traits to form composite devices.
/// See the [Scene type guide](https://developers.google.com/assistant/smarthome/guides/scene) for more information.
///
/// For example, if a device allows users to configure one-touch grouping of commands — setting lights to specific colors, or sequencing various security features,
/// or any other combination of activities — this configuration can be exposed through SYNC as a named scene,
/// and the Assistant will make these scenes available to users through simple activation grammar:
/// - Start Party Mode
/// - Activate midnight scene
/// As virtual objects, scenes can be placed in rooms (if relevant) for disambiguation:
/// - Start party time in the kitchen.
/// - Activate nightlight mode in the bedrooms.
///
/// One difference between scenes and physical targets is that the Assistant will automatically apply plural effects to Scene commands,
/// allowing users to trigger scenes across multiple partners.
/// For example, if a user has a "party mode" scene on two different partners,
/// one for security and one for lights, Activate party mode will trigger both.
///
/// Scenes will interact well with upcoming Personal Actions for custom grammar (for example, Activate Party Mode -> Let's get the party started!).
///
/// Scenes should always have user-provided names versus default "BobCo Scene" naming.
/// Each scene is its own virtual device, with its own name(s). User-provided names may come in from SYNC.
///
/// # Warning
/// The Scene trait does not currently support Report State calls. Actions utilizing only the Scene trait, will not be eligble for certification or release.
///
/// # See also
/// <https://developers.google.com/assistant/smarthome/traits/scene>
pub trait Scene {
    /// Indicates that this scene can be cancelled.
    /// This attribute is only relevant for scenes that modify state and remember previous state.
    /// The device supports the ActivateScene command with the deactivate parameter to true.
    ///
    /// Default: false
    fn is_reversible(&self) -> Result<Option<bool>, CombinedDeviceError> {
        Ok(None)
    }

    /// Activate a scene
    fn activate(&mut self) -> Result<(), CombinedDeviceError>;

    /// Deactivate a scene.
    /// Only called if [Self::is_reversible] returns `true`
    fn deactivate(&mut self) -> Result<(), CombinedDeviceError>;
}
