use std::{ffi::OsString, fmt::Display};

use serde::Serialize;

/// This struct is used to store the title of a change
/// and provide functions to convert into an OsString that
/// can be used as filename.
#[derive(Debug, Clone, PartialEq, Serialize, Hash)]
pub struct Title(pub String);

impl Title {
	/// Convert a title to an OsString
	pub fn as_os_string(&self) -> OsString {
		OsString::from(self.0.replace(' ', "_"))
	}
}

impl Display for Title {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.0)
	}
}

impl From<OsString> for Title {
	fn from(s: OsString) -> Self {
		Self(s.to_string_lossy().replace('_', " "))
	}
}

impl From<&str> for Title {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}

impl AsRef<str> for Title {
	#[inline(always)]
	fn as_ref(&self) -> &str {
		self.0.as_str()
	}
}

#[cfg(test)]
mod test_super {
	use super::*;

	#[test]
	fn test_from_str_with_spaces() {
		assert_eq!(OsString::from("foo_bar"), Title::from("foo bar").as_os_string());
	}

	#[test]
	fn test_from_str_with_emojis() {
		assert_eq!(OsString::from("foo_bar_😀"), Title::from("foo bar 😀").as_os_string());
	}

	#[test]
	fn test_from_os_string() {
		assert_eq!(
			"Original title".to_string(),
			Title::from(OsString::from("Original_title")).to_string()
		);
	}
}
