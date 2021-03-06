use crate::ascii::Ascii;
use crate::solicit::header::HeaderError;
use bytes::Bytes;
use std::fmt;

/// A convenience struct representing a header value.
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct HeaderValue(Ascii);

impl HeaderValue {
    /// Validate and create header value from bytes.
    pub fn from_bytes(bs: Bytes) -> Result<HeaderValue, (HeaderError, Bytes)> {
        // https://svn.tools.ietf.org/svn/wg/httpbis/specs/rfc7230.html#header.fields
        //
        // field-value    = *( field-content / obs-fold )
        // field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
        // field-vchar    = VCHAR / obs-text
        // VCHAR          = any visible USASCII character
        // obs-fold       = CRLF 1*( SP / HTAB )
        //                ; obsolete line folding
        //                ; see Section 3.2.4
        // obs-text       = %x80-FF

        // https://svn.tools.ietf.org/svn/wg/httpbis/specs/rfc7230.html#field.parsing
        // A sender MUST NOT generate a message that includes line folding
        // (i.e., that has any field-value that contains a match to the obs-fold rule)
        // unless the message is intended for packaging within the message/http media type.

        // https://svn.tools.ietf.org/svn/wg/httpbis/specs/rfc7230.html#field.parsing
        // Historically, HTTP has allowed field content with text in the ISO‑8859‑1
        // charset [ISO-8859-1], supporting other charsets only through use of
        // [RFC2047] encoding. In practice, most HTTP header field values use only
        // a subset of the US-ASCII charset [USASCII]. Newly defined header fields
        // SHOULD limit their field values to US‑ASCII octets. A recipient SHOULD
        // treat other octets in field content (obs‑text) as opaque data.

        for &b in &bs {
            if !b.is_ascii() {
                return Err((HeaderError::HeaderValueNotAscii, bs));
            }

            if (b < b' ' || b > b'~') && b != b'\t' {
                return Err((HeaderError::IncorrectCharInValue, bs));
            }
        }

        Ok(HeaderValue(unsafe { Ascii::from_bytes_unchecked(bs) }))
    }

    /// Into underlying storage object.
    pub fn into_inner(self) -> Bytes {
        self.0.into_bytes()
    }

    /// As bytes.
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Unsafe no-validation `const` constructor.
    pub const unsafe fn from_bytes_unchecked(bytes: Bytes) -> HeaderValue {
        HeaderValue(Ascii::from_bytes_unchecked(bytes))
    }
}

impl fmt::Debug for HeaderValue {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, fmt)
    }
}

impl From<Vec<u8>> for HeaderValue {
    fn from(vec: Vec<u8>) -> HeaderValue {
        HeaderValue::from(Bytes::from(vec))
    }
}

impl From<Bytes> for HeaderValue {
    fn from(bytes: Bytes) -> HeaderValue {
        HeaderValue::from_bytes(bytes).map_err(|(e, _)| e).unwrap()
    }
}

impl<'a> From<&'a [u8]> for HeaderValue {
    fn from(buf: &'a [u8]) -> HeaderValue {
        HeaderValue::from(Bytes::copy_from_slice(buf))
    }
}

impl From<String> for HeaderValue {
    fn from(s: String) -> HeaderValue {
        HeaderValue::from(s.into_bytes())
    }
}

impl<'a> From<&'a str> for HeaderValue {
    fn from(s: &'a str) -> HeaderValue {
        HeaderValue::from(s.as_bytes())
    }
}

impl Into<Bytes> for HeaderValue {
    fn into(self) -> Bytes {
        self.0.into()
    }
}

impl AsRef<[u8]> for HeaderValue {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<str> for HeaderValue {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
