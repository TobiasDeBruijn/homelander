# Homelander
Homelander is a framework to make integration with Google Home as easy as possible.

It provides serialization and deserialization for fulfillment requests.
It also handles translation between Google Home traits and Rust traits. Furthermore it provides error handling and translating between
Rust errors and errors accepted by Google Home.

Homelander does *not* provide an OAuth2 server or a web server.

## State of completion
The following [Traits](https://developers.google.com/assistant/smarthome/traits) are currently implemented:
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

These traits are not yet implemented:
- AppSelector
- CameraStream
- Channel
- OpenClose
- Reboot
- Rotation
- RunCycle
- SensorState
- Scene
- SoftwareUpdate
- StartStop
- StatusReport
- TemperatureControl
- TemperatureSetting
- Timer
- Toggles
- TransportControl
- Volume

They are however planned to be implemented, though. You can help by submitting a pull request with the implementation. 
You will find the traits definition in the `traits` module, they are completely non-functional though.

# License
Homelander is dual licensed under the MIT and Apache-2.0 license