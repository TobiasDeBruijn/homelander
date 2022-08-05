use crate::CombinedDeviceError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CameraStreamProtocol {
    /// HTTP Live Streaming.
    /// <https://en.wikipedia.org/wiki/HTTP_Live_Streaming>
    Hls,
    /// Dynamic Adaptive Streaming over HTTP.
    /// <https://en.wikipedia.org/wiki/Dynamic_Adaptive_Streaming_over_HTTP>
    Dash,
    /// Smooth Streaming.
    /// <https://en.wikipedia.org/wiki/Adaptive_bitrate_streaming#Apple_HTTP_Live_Streaming>
    SmoothStream,
    /// Progressive MP4 (mostly used for clips).
    ProgressiveMp4,
    /// WebRTC.
    /// <https://webrtc.org/>
    #[serde(rename = "webRTC")]
    WebRtc,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CameraStreamDescriptor {
    /// An auth token for the specific receiver to authorize access to the stream.
    /// If cameraStreamNeedAuthToken is true and this value is not provided, the user's OAuth credentials will be used as the auth token.
    pub camera_stream_auth_token: Option<String>,
    /// The media format that the stream url points to. It should be one of the protocols listed in the SupportedStreamProtocols command parameter.
    pub camera_stream_protocol: CameraStreamProtocol,
    #[serde(flatten)]
    pub access_descriptor: CameraStreamAccess,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CameraStreamAccess {
    WebRtc {
        /// URL endpoint for retrieving and exchanging camera and client [session description protocols](https://en.wikipedia.org/wiki/Session_Description_Protocol) (SDPs).
        /// The client should return the signaling URL which uses the cameraStreamAuthToken as the authentication token in the request header.
        camera_stream_signaling_url: String,
        /// Offer session description protocol (SDP).
        camera_stream_offer: Option<String>,
        /// Represents the Interactive Connectivity Establishment (ICE) servers
        /// using an encoded JSON string with the description of a RTCIceServer.
        /// If you do not specify STUN (Session Traversal Utilities for NAT) servers,
        /// the platform defaults to Google's public STUN servers. TURN (Traversal Using Relays around NAT)
        /// servers are only required if you cannot guarantee the IPs / ICE candidates
        /// provided will be publicly accessible (e.g. via a media server, public host ICE candidate,
        /// relay ICE candidate, etc).
        camera_stream_ice_server: Option<String>,
    },
    NonWebRtc {
        /// URL endpoint for retrieving the real-time stream in the format specified by cameraStreamProtocol.
        camera_stream_access_url: String,
        /// Cast receiver ID to process the camera stream when the StreamToChromecast parameter is true;
        /// default receiver will be used if not provided.
        camera_stream_receiver_app_id: Option<String>,
    },
}

/// This trait belongs to devices which have the capability to stream video feeds to third party screens,
/// Chromecast-connected screens, or smartphones. By and large, these are security cameras or baby cameras.
/// But this trait also applies to more complex devices which have a camera on them
/// (for example, video-conferencing devices or a vacuum robot with a camera on it).
///
/// Note: CameraStream is not currently supported by [local fulfillment](https://developers.google.com/assistant/smarthome/concepts/local).
///
/// # WebRTC protocol specification
/// The benefits of using WebRTC are low latency and 1-way talk. WebRTC uses a POST method with a POST body and response in JSON format.
///
/// This section describes the requirements when using the WebRTC stream protocol.
/// <table>
///     <thead>
///         <th> Data Type </th>
///         <th> Parameters/Definitions </th>
///     </thead>
///     <tbody>
///         <tr>
///             <td> Signaling request header </td>
///             <td>
///                 The header should meet these requirements:
///                 <ul>
///                     <li> <strong>Authentication</strong>: The authentication header should use the auth token from the GetCameraStream return value for cameraStreamAuthToken with token type Bearer.
///                     <li> <strong>Content-Type</strong>: application/json.
///                 </ul>
///             </td>
///         </tr>
///         <tr>
///             <td> Signaling request parameters </td>
///             <td>
///                 The request can include these parameters:
///                 <ul>
///                     <li>
///                         <strong>action</strong>: String. The valid values are:
///                         <ul>
///                             <li> <strong>offer</strong>: Offer SDP message from provider.
///                             <li> <strong>answer</strong>: Answer SDP message from provider.
///                             <li> <strong>end</strong>: Close the current session.
///                         </ul>
///                     </li>
///                     <li> <strong>deviceId</strong>: String. The device ID as reported in a SYNC or EXECUTE request.
///                     <li> <strong>sdp</strong>: String. Contains the Session Description Protocol message for the peer connection. The content is based on the value of the action parameter. If the action is "end", this parameter can be empty.
///                 </ul>
///             </td>
///         </tr>
///         <tr>
///             <td> Signaling response parameters </td>
///             <td>
///                 <ul>
///                     <li> <strong>action</strong>: String. Response value must be of type answer.
///                     <li> <strong>sdp</strong>: String. SDP message for the responding answer.
///                 </ul>
///             </td>
///         </tr
///     </tbody>
/// </table>
///
/// # WebRTC Requirments and Recommendations
/// - Google currently supports 1-way (half duplex) communication.
/// - You must support bundling and rtcp-mux.
/// - You must use (D)TLS 1.2 or later.
/// - Trickle ICE is not supported. All ICE candidates must be gathered first before sending the SDP.
/// - It is strongly recommended that you include UDP/IPv4, TCP/IPv4, UDP/IPv6 and TCP/IPv6 ICE candidates to increase the probability of a successful connection.
///
/// ## Supported video resolutions
/// - Minimum: 480p
/// - Maximum: 1080p
///
/// ## Supported video codecs
/// - VP8
/// - H.264
///
/// ## Supported audio codecs
/// - Opus (preferred codec)
/// - G.711/PCMU
/// - G.722
///
/// # Cross-Origin Resource Sharing
/// Cross-Origin Resource Sharing (CORS) is a mechanism that uses additional HTTP Headers
/// to tell browsers to allow a web application running at one origin to access selected resources from a different origin.
/// The server hosting cameraStreamSignalingUrl should respond with the following header:
/// ```txt
/// Access-Control-Allow-Origin: https://www.gstatic.com
/// ```
///
/// # Sample signaling request and response
/// The following example shows a request that Google sends to your signaling service and the corresponding response to Google.
///
/// ## Request
/// ```txt
/// Header:
///
/// Authentication: Bearer <cameraStreamAuthToken>
/// Content-Type: application/json
///
/// ```
/// POST body:
/// ```jsonc
/// // When camera offer SDP is provided in the execution response, Google provides an answer SDP.
/// {
///   "action": "answer",
///   "deviceId": "123",
///   "sdp": "o=- 4611731400430051336 2 IN IP4 127.0.0.1..."
/// }
///
/// // When camera offer SDP is not provided in execution response, Google generates and provides an offer SDP.
/// {
///   "action": "offer",
///   "deviceId": "123",
///   "sdp": "o=- 4611731400430051336 2 IN IP4 127.0.0.1..."
/// }
///
/// // Close the current stream session.
/// {
///   "action": "end"
///   "deviceId": "123"
/// }
/// ```
///
/// ## Response
/// Response to accept the answer SDP in the request.
/// ```txt
/// Response Code : 200
/// ```
/// ```jsonc
/// {}
/// ```
///
/// Response to provide the answer SDP from the service provider.
/// ```txt
/// Response Code : 200
/// ```
/// ```jsonc
/// {
///   // When the camera offer SDP is not provided in the execution response,
///   // Google provides the answer SDP via the signaling response.
///   "action": "answer"
///   "sdp": "o=- 4611731400430051336 2 IN IP4 127.0.0.1..."
/// }
/// ```
///
/// Response to close current session
/// ```txt
/// Response Code : 200
/// ```
/// ```jsonc
/// {}
/// ```
///
pub trait CameraStream {
    fn get_supported_camera_stream_protocols(&self) -> Result<Vec<CameraStreamProtocol>, CombinedDeviceError>;

    /// Whether an auth token will be provided via cameraStreamAuthToken for the target surface to stream the camera feed.
    ///
    /// Note: Auth tokens are only supported when using a custom Cast receiver. The generic Cast receiver doesn’t support authentication.
    fn need_auth_token(&self) -> Result<bool, CombinedDeviceError>;

    /// Get the camera stream
    /// - `to_chromecast` Whether the stream will be played on a Chromecast device.
    /// - `supported_stream_protocols` Media types/formats supported by the desired destination.
    fn get_camera_stream(&mut self, to_chromecast: bool, supported_protocols: Vec<CameraStreamProtocol>)
        -> Result<CameraStreamDescriptor, CombinedDeviceError>;
}
