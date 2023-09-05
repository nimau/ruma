use std::time::Duration;

use js_int::UInt;
use ruma_common::OwnedMxcUri;
use serde::{Deserialize, Serialize};
use crate::audio::Amplitude;

use crate::room::{EncryptedFile, MediaSource};

/// The payload for an audio message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[serde(tag = "msgtype", rename = "m.audio")]
pub struct AudioMessageEventContent {
    /// The textual representation of this message.
    pub body: String,

    /// The source of the audio clip.
    #[serde(flatten)]
    pub source: MediaSource,

    /// Metadata for the audio clip referred to in `source`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<AudioInfo>>,

    /// The audio details of the message, if any.
    #[serde(rename = "org.matrix.msc1767.audio", skip_serializing_if = "Option::is_none")]
    pub audio_details: Option<Box<AudioDetails>>,

    /// Whether this is a voice message
    #[serde(rename = "org.matrix.msc3245.voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<Box<VoiceInfo>>,
}

impl AudioMessageEventContent {
    /// Creates a new `AudioMessageEventContent` with the given body and source.
    pub fn new(body: String, source: MediaSource, voice: bool) -> Self {
        let voice_info = if voice { Some(Box::new(VoiceInfo::new())) } else { None };
        Self { body, source, info: None, audio_details: None, voice: voice_info }
    }

    /// Creates a new non-encrypted `AudioMessageEventContent` with the given bod and url.
    pub fn plain(body: String, url: OwnedMxcUri, voice: bool) -> Self {
        Self::new(body, MediaSource::Plain(url), voice)
    }

    /// Creates a new encrypted `AudioMessageEventContent` with the given body and encrypted
    /// file.
    pub fn encrypted(body: String, file: EncryptedFile, voice: bool) -> Self {
        Self::new(body, MediaSource::Encrypted(Box::new(file)), voice)
    }

    /// Creates a new `AudioMessageEventContent` from `self` with the `info` field set to the given
    /// value.
    ///
    /// Since the field is public, you can also assign to it directly. This method merely acts
    /// as a shorthand for that, because it is very common to set this field.
    pub fn info(self, info: impl Into<Option<Box<AudioInfo>>>) -> Self {
        Self { info: info.into(), ..self }
    }

    pub fn audio_details(self, audio_details: impl Into<Option<Box<AudioDetails>>>) -> Self {
        Self { audio_details: audio_details.into(), ..self}
    }

    pub fn is_voice(&self) -> bool {
        self.voice.is_some()
    }
}

/// Metadata about an audio clip.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct AudioInfo {
    /// The duration of the audio in milliseconds.
    #[serde(
        with = "ruma_common::serde::duration::opt_ms",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration: Option<Duration>,

    /// The mimetype of the audio, e.g. "audio/aac".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,

    /// The size of the audio clip in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<UInt>,
}

impl AudioInfo {
    /// Creates an empty `AudioInfo`.
    pub fn new() -> Self {
        Self::default()
    }
}

/// A block for details of audio content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct AudioDetails {
    /// The duration of the audio in seconds.
    #[serde(with = "ruma_common::serde::duration::secs")]
    pub duration: Duration,

    /// The waveform representation of the audio content, if any.
    ///
    /// This is optional and defaults to an empty array.
    #[cfg(feature = "unstable-msc3246")]
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub waveform: Vec<Amplitude>,
}

impl AudioDetails {
    /// Creates a new `AudioDetailsContentBlock` with the given duration.
    pub fn new(duration: Duration, waveform: Vec<Amplitude>) -> Self {
        Self {
            duration,
            #[cfg(feature = "unstable-msc3246")]
            waveform,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct VoiceInfo { }

impl VoiceInfo {
    /// Creates an empty `VoiceInfo`.
    pub fn new() -> Self {
        Self::default()
    }
}
