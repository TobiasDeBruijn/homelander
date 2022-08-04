# Homelander
Homelander is a framework to make integration with Google Home as easy as possible.

It provides serialization and deserialization for fulfillment requests.
It also handles translation between Google Home traits and Rust traits. Furthermore it provides error handling and translating between
Rust errors and errors accepted by Google Home.

Homelander does *not* provide an OAuth2 server or a web server.

## State of completion
The following [Traits](https://developers.google.com/assistant/smarthome/traits) are currently implemented:
- AppSelector
- ArmDisarm
- Brightness
- ColorSetting
- Cook
- Dispense
- Dock
- EnergyStorage
- FanSpeed
- Fill
- HumiditySetting
- InputSelector
- LightEffects
- Locator
- LockUnlock
- MediaState
- Modes
- NetworkControl
- OnOff
- OpenClose
- Reboot
- Rotation
- RunCycle
- Scene
- SensorState
- SoftwareUpdate
- StartStop
- StatusReport
- TemperatureControl
- TemperatureSetting
- Timer
- Toggles
- TransportControl
- Volume

These traits are not yet implemented:
- CameraStream
- ObjectDetection
- Channel

They are however planned to be implemented, though. You can help by submitting a pull request with the implementation. 
You will find the traits definition in the `traits` module, they are completely non-functional though.

## Things that need love too
- Error handling. It's just not pretty at the moment, and is not always up to spec
- Documentation. Quite often you'll still need to check out Google's docs
- Sending notifications to Google (for follow-up or ObjectDetection)
- Requesting a QUERY from Google

# License
Homelander is dual licensed under the MIT and Apache-2.0 license