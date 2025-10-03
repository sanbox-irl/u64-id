use core::fmt;

/// An ID for simply applications, implemented as a wrapper around [`u64`]s.
///
/// ## Valid Range
///
/// The valid range of ids is given by [`U64Id::VALID_RANGE`]. Essentially,
/// this range is `..(u64::MAX - 128)`.
///
/// ## Collisions
///
/// Ids are made entirely randomly, so a collision is possible, but unbelievably
/// unlikely, since just a simply u64 has enough bitspace in it to prevent a collision.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct U64Id(pub u64);

impl U64Id {
    /// Returns the forbidden `null` id. Applications can use this as an Id
    /// which will never be made.
    pub const NULL: U64Id = U64Id(u64::MAX);

    /// Returns the valid range for [`U64Id`]s. Nothing prevents an Id from
    /// being generated outside of this range, so it should not be relied upon
    /// for safety.
    ///
    /// The amount of space at the top was randomly determined to be useful.
    pub const VALID_RANGE: core::ops::Range<u64> = 0..(u64::MAX - 128);

    /// Creates a new, random AssetId, seeded cheaply from thread_rng.
    ///
    /// To avoid calling this internal function repeatedly, consider making
    /// a `U64Id` directly.
    #[cfg(feature = "rand")]
    pub fn new() -> Self {
        use rand::Rng;

        Self(rand::rng().random_range(Self::VALID_RANGE))
    }

    /// Checks if the asset is the `null` ID.
    pub const fn is_null(self) -> bool {
        self.0 == u64::MAX
    }
}

#[cfg(feature = "rand")]
impl Default for U64Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for U64Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}
impl fmt::LowerHex for U64Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::LowerHex::fmt(&val, f)
    }
}
impl fmt::UpperHex for U64Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = self.0;

        fmt::UpperHex::fmt(&val, f)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for U64Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // we serialize the number as a string with lowercase hex formatting by default
        serializer.serialize_str(&format!("{:x}", self.0))
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for U64Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct AssetIdVisitor;
        impl<'de> serde::de::Visitor<'de> for AssetIdVisitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("a hex-encoded integer between 0 and 2^64 - 1")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::from_str_radix(v, 16).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(v), &self)
                })
            }

            // we can also deserialize a u64! This can be nice. Yes. It is nice.
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::from_str_radix(&v.to_string(), 16).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                })
            }

            // we can also deserialize an i64! This can be nice. Yes. It is nice.
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                u64::from_str_radix(&v.to_string(), 16).map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                })
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_any(AssetIdVisitor).map(U64Id)
        } else {
            deserializer.deserialize_str(AssetIdVisitor).map(U64Id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn null_tests() {
        let asset = U64Id::NULL;
        assert!(asset.is_null());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn basic_serde() {
        assert_eq!(
            serde_json::from_str::<U64Id>("12345").unwrap(),
            U64Id(74565)
        );

        assert_eq!(
            serde_json::from_str::<U64Id>("\"a12b345\"").unwrap(),
            U64Id(168997701)
        );
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_cycle() {
        let input = "\"123454321\"";
        let output = serde_json::from_str::<U64Id>(input).unwrap();
        let input_again = serde_json::to_string(&output).unwrap();

        assert_eq!(input, input_again);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_cycle_around() {
        let input = U64Id(12345321);
        let output = serde_json::to_string::<U64Id>(&input).unwrap();
        let input_again = serde_json::from_str(&output).unwrap();

        assert_eq!(input, input_again);
    }
}
