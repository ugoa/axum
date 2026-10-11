//! Rejection response types.

use crate::__composite_rejection as composite_rejection;
use crate::__define_rejection as define_rejection;

use crate::{BoxError, Error};

// composite_rejection! {
//     /// Rejection type for extractors that buffer the request body. Used if the
//     /// request body cannot be buffered due to an error.
//     pub enum FailedToBufferBody {
//         LengthLimitError,
//         UnknownBodyError,
//     }
// }

// =====================================================
// Recursive expansion of the composite_rejection! macro
// =====================================================

#[doc = r" Rejection type for extractors that buffer the request body. Used if the"]
#[doc = r" request body cannot be buffered due to an error."]
#[derive(Debug)]
#[non_exhaustive]
pub enum FailedToBufferBody {
    #[allow(missing_docs)]
    LengthLimitError(LengthLimitError),
    #[allow(missing_docs)]
    UnknownBodyError(UnknownBodyError),
}
impl crate::response::IntoResponse for FailedToBufferBody {
    fn into_response(self) -> crate::response::Response {
        match self {
            Self::LengthLimitError(inner) => inner.into_response(),
            Self::UnknownBodyError(inner) => inner.into_response(),
        }
    }
}
impl FailedToBufferBody {
    #[doc = r" Get the response body text used for this rejection."]
    #[must_use]
    pub fn body_text(&self) -> String {
        match self {
            Self::LengthLimitError(inner) => inner.body_text(),
            Self::UnknownBodyError(inner) => inner.body_text(),
        }
    }
    #[doc = r" Get the status code used for this rejection."]
    #[must_use]
    pub fn status(&self) -> http::StatusCode {
        match self {
            Self::LengthLimitError(inner) => inner.status(),
            Self::UnknownBodyError(inner) => inner.status(),
        }
    }
}
impl From<LengthLimitError> for FailedToBufferBody {
    fn from(inner: LengthLimitError) -> Self {
        Self::LengthLimitError(inner)
    }
}
impl From<UnknownBodyError> for FailedToBufferBody {
    fn from(inner: UnknownBodyError) -> Self {
        Self::UnknownBodyError(inner)
    }
}
impl std::fmt::Display for FailedToBufferBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LengthLimitError(inner) => f.write_fmt(core::format_args!("{inner}")),
            Self::UnknownBodyError(inner) => f.write_fmt(core::format_args!("{inner}")),
        }
    }
}
impl std::error::Error for FailedToBufferBody {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::LengthLimitError(inner) => inner.source(),
            Self::UnknownBodyError(inner) => inner.source(),
        }
    }
}

impl FailedToBufferBody {
    pub(crate) fn from_err<E>(err: E) -> Self
    where
        E: Into<BoxError>,
    {
        // two layers of boxes here because `with_limited_body`
        // wraps the `http_body_util::Limited` in a `axum_core::Body`
        // which also wraps the error type
        let box_error = match err.into().downcast::<Error>() {
            Ok(err) => err.into_inner(),
            Err(err) => err,
        };
        let box_error = match box_error.downcast::<Error>() {
            Ok(err) => err.into_inner(),
            Err(err) => err,
        };
        match box_error.downcast::<http_body_util::LengthLimitError>() {
            Ok(err) => Self::LengthLimitError(LengthLimitError::from_err(err)),
            Err(err) => Self::UnknownBodyError(UnknownBodyError::from_err(err)),
        }
    }
}

define_rejection! {
    #[status = PAYLOAD_TOO_LARGE]
    #[body = "Failed to buffer the request body"]
    /// Encountered some other error when buffering the body.
    ///
    /// This can _only_ happen when you're using [`tower_http::limit::RequestBodyLimitLayer`] or
    /// otherwise wrapping request bodies in [`http_body_util::Limited`].
    pub struct LengthLimitError(Error);
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Failed to buffer the request body"]
    /// Encountered an unknown error when buffering the body.
    pub struct UnknownBodyError(Error);
}

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "Request body didn't contain valid UTF-8"]
    /// Rejection type used when buffering the request into a [`String`] if the
    /// body doesn't contain valid UTF-8.
    pub struct InvalidUtf8(Error);
}

// composite_rejection! {
//     /// Rejection used for [`Bytes`](bytes::Bytes).
//     ///
//     /// Contains one variant for each way the [`Bytes`](bytes::Bytes) extractor
//     /// can fail.
//     pub enum BytesRejection {
//         FailedToBufferBody,
//     }
// }

// =====================================================
// Recursive expansion of the composite_rejection! macro
// =====================================================

#[doc = r" Rejection used for [`Bytes`](bytes::Bytes)."]
#[doc = r""]
#[doc = r" Contains one variant for each way the [`Bytes`](bytes::Bytes) extractor"]
#[doc = r" can fail."]
#[derive(Debug)]
#[non_exhaustive]
pub enum BytesRejection {
    #[allow(missing_docs)]
    FailedToBufferBody(FailedToBufferBody),
}
impl crate::response::IntoResponse for BytesRejection {
    fn into_response(self) -> crate::response::Response {
        match self {
            Self::FailedToBufferBody(inner) => inner.into_response(),
        }
    }
}
impl BytesRejection {
    #[doc = r" Get the response body text used for this rejection."]
    #[must_use]
    pub fn body_text(&self) -> String {
        match self {
            Self::FailedToBufferBody(inner) => inner.body_text(),
        }
    }
    #[doc = r" Get the status code used for this rejection."]
    #[must_use]
    pub fn status(&self) -> http::StatusCode {
        match self {
            Self::FailedToBufferBody(inner) => inner.status(),
        }
    }
}
impl From<FailedToBufferBody> for BytesRejection {
    fn from(inner: FailedToBufferBody) -> Self {
        Self::FailedToBufferBody(inner)
    }
}
impl std::fmt::Display for BytesRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToBufferBody(inner) => f.write_fmt(core::format_args!("{inner}")),
        }
    }
}
impl std::error::Error for BytesRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FailedToBufferBody(inner) => inner.source(),
        }
    }
}

composite_rejection! {
    /// Rejection used for [`String`].
    ///
    /// Contains one variant for each way the [`String`] extractor can fail.
    pub enum StringRejection {
        FailedToBufferBody,
        InvalidUtf8,
    }
}
