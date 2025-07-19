#![allow(dead_code)]

use std::borrow::Cow;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1, take_while_m_n},
    combinator::{map, opt},
    sequence::{separated_pair, tuple},
    AsChar, IResult, Parser,
};

use anyhow::Result;
use fn_error_context::context;
use tracing::Instrument;

/// This is used by dracut.
pub(crate) const INITRD_ARG_PREFIX: &str = "rd.";
/// The kernel argument for configuring the rootfs flags.
pub(crate) const ROOTFLAGS: &str = "rootflags=";

/// Parse the kernel command line.  This is strictly
/// speaking not a correct parser, as the Linux kernel
/// supports quotes.  However, we don't yet need that here.
///
/// See systemd's code for one userspace parser.
#[context("Reading /proc/cmdline")]
pub(crate) fn parse_cmdline() -> Result<Vec<String>> {
    let cmdline = std::fs::read_to_string("/proc/cmdline")?;
    let r = cmdline
        .split_ascii_whitespace()
        .map(ToOwned::to_owned)
        .collect();
    Ok(r)
}

/// Return the value for the string in the vector which has the form target_key=value
pub(crate) fn find_first_cmdline_arg<'a>(
    args: impl Iterator<Item = &'a str>,
    target_key: &str,
) -> Option<&'a str> {
    args.filter_map(|arg| {
        if let Some((k, v)) = arg.split_once('=') {
            if target_key == k {
                return Some(v);
            }
        }
        None
    })
    .next()
}

/*
pub(crate) struct Kargs<'a> {
    raw: String,
    params: Vec<Parameter<'a>>,
}

impl<'a> TryFrom<&str> for Kargs<'a> {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: String::from(""),
            params: vec![],
        })
    }
}
*/

#[derive(Debug)]
pub(crate) struct Parameter<'a> {
    key: &'a [u8],
    canonical_key: Cow<'a, [u8]>,
    value: Option<&'a [u8]>,
}

fn canonical_key<'a>(k: &'a [u8]) -> Cow<'a, [u8]> {
    if k.contains(&u8::try_from('-').unwrap()) {
        let mut owned = Vec::with_capacity(k.len());
        for &byte in k {
            if byte == u8::try_from('-').unwrap() {
                owned.push(u8::try_from('_').unwrap());
            } else {
                owned.push(byte);
            }
        }
        Cow::Owned(owned)
    } else {
        Cow::Borrowed(k)
    }
}

fn is_valid_key_byte(b: u8) -> bool {
    let ch = b.as_char();
    ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'
}

fn is_valid_value_byte(b: u8) -> bool {
    let ch = b.as_char();
    ch.is_ascii_graphic() && ch != '"'
}

fn is_valid_quoted_value_byte(b: u8) -> bool {
    let ch = b.as_char();
    ch != '"'
}

fn is_valid_unquoted_value_byte(b: u8) -> bool {
    let ch = b.as_char();
    ch.is_ascii_graphic() && ch != '"'
}

impl<'a> Parameter<'a> {
    pub(crate) fn key(&self) -> &[u8] {
        self.key
    }

    pub(crate) fn value(&self) -> Option<&[u8]> {
        self.value
    }

    fn parse(input: &'a [u8]) -> IResult<&'a [u8], Self> {
        map(
            (Self::parse_key, opt((tag("="), Self::parse_value))),
            |(key, value)| {
                let canonical_key = canonical_key(key);
                let value = value.map(|(_, v)| v);
                Self {
                    canonical_key,
                    key,
                    value,
                }
            },
        )
        .parse(input)
    }

    fn parse_key(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while1(is_valid_key_byte)(input)
    }

    fn parse_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
        //take_while1(is_valid_value_byte)(input)
        alt((Self::parse_value_quoted, Self::parse_value_unquoted)).parse(input)
    }

    fn parse_value_unquoted(input: &[u8]) -> IResult<&[u8], &[u8]> {
        take_while(is_valid_unquoted_value_byte)(input)
    }

    fn parse_value_quoted(input: &[u8]) -> IResult<&[u8], &[u8]> {
        map(
            (tag("\""), take_while(is_valid_quoted_value_byte), tag("\"")),
            |(_, val, _)| val,
        )
        .parse(input)
    }
}

impl PartialEq for Parameter<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.canonical_key != other.canonical_key {
            return false;
        }

        match (self.value, other.value) {
            (Some(ours), Some(other)) => ours == other,
            (None, None) => true,
            _ => false,
        }
    }
}

impl Eq for Parameter<'_> {}

impl<'a> TryFrom<&'a [u8]> for Parameter<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        match Self::parse(value) {
            Ok((_, val)) => Ok(val),
            Err(e) => Err("Unable to parse parameter"),
        }
    }
}

/*
impl TryFrom<&str> for Kargs {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first() {
        let kargs = &["foo=bar", "root=/dev/vda", "blah", "root=/dev/other"];
        let kargs = || kargs.iter().copied();
        assert_eq!(find_first_cmdline_arg(kargs(), "root"), Some("/dev/vda"));
        assert_eq!(find_first_cmdline_arg(kargs(), "nonexistent"), None);
    }

    /*
    #[test]
    fn test_find_first_v2() {
        let kargs = Kargs::try_from("foo=bar root=/dev/vda blah root=/dev/other").unwrap();

        assert_eq!(
            kargs.iter().find(|k| k == "root"),
            Some(Parameter::KeyValue("root", "/dev/vda"))
        );

        assert_eq!(
            kargs.iter().find(|k| k == "blah"),
            Some(Parameter::Switch("blah"))
        );

        assert_eq!(kargs.iter().find(|k| k == "nonexistent"), None);
    }
     */

    /*
    #[test]
    fn test_parameter_key() {
        let sw = Parameter::try_from("foo").unwrap();
        assert_eq!(sw.key(), "foo");

        let kv = Parameter::try_from("bar=baz").unwrap();
        assert_eq!(kv.key(), "bar");
    }
    */

    /*
    #[test]
    fn test_parameter_value() {
        let sw = Parameter::Switch("foo");
        assert_eq!(sw.value(), None);

        let kv = Parameter::KeyValue("bar", "baz");
        assert_eq!(kv.value(), Some("baz"));
    }

    #[test]
    fn test_parameter_equal() {
        // canonical keys are equal
        let dashes = Parameter::Switch("a-delimited-param");
        let underscores = Parameter::Switch("a_delimited_param");
        assert_eq!(dashes, underscores);

        // canonical key with same values is equal
        let dashes = Parameter::KeyValue("a-delimited-param", "same_values");
        let underscores = Parameter::KeyValue("a_delimited_param", "same_values");
        assert_eq!(dashes, underscores);

        // canonical key with different values is not equal
        let dashes = Parameter::KeyValue("a-delimited-param", "different_values");
        let underscores = Parameter::KeyValue("a_delimited_param", "DiFfErEnT_valUEZ");
        assert_ne!(dashes, underscores);

        // mixed variants are never equal
        let switch = Parameter::Switch("same_key");
        let keyvalue = Parameter::KeyValue("same_key", "but_with_a_value");
        assert_ne!(switch, keyvalue);
    }
     */

    #[test]
    fn test_canonical_key() {
        assert_eq!(canonical_key(b"plain"), Cow::Borrowed(b"plain"));

        assert_eq!(
            canonical_key(b"underscores_are_unchanged"),
            Cow::Borrowed(b"underscores_are_unchanged")
        );

        assert_eq!(
            canonical_key(b"dashes-are-underscores"),
            Cow::<[u8]>::Owned(b"dashes_are_underscores".to_vec())
        );
    }

    #[test]
    fn test_parameter_try_from() {
        let input = b"basic".as_slice();

        assert_eq!(
            Parameter::try_from(input),
            Ok(Parameter {
                key: b"basic",
                canonical_key: Cow::Borrowed(b"basic"),
                value: None
            })
        );

        let input = b"foo=bar".as_slice();
        assert_eq!(
            Parameter::try_from(input),
            Ok(Parameter {
                key: b"foo",
                canonical_key: Cow::Borrowed(b"foo"),
                value: Some(b"bar"),
            })
        );

        let input = b"quoted=\"with spaces\"".as_slice();
        assert_eq!(
            Parameter::try_from(input),
            Ok(Parameter {
                key: b"quoted",
                canonical_key: Cow::Borrowed(b"quoted"),
                value: Some(b"with spaces"),
            })
        );

        let input = b"key=this_should_\"stop_at_the_quote".as_slice();
        assert_eq!(
            Parameter::try_from(input),
            Ok(Parameter {
                key: b"key",
                canonical_key: Cow::Borrowed(b"key"),
                value: Some(b"this_should_"),
            })
        );

        let input = b"should$stop_before".as_slice();
        assert_eq!(
            Parameter::try_from(input),
            Ok(Parameter {
                key: b"should",
                canonical_key: Cow::Borrowed(b"should"),
                value: None,
            })
        );
    }
}
