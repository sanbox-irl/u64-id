use core::fmt;

/// An ID for Assets. Currently, it is implemented as a wrapper around u64s.
/// If a collision occurs in your own code, please file an issue -- this will
/// likely require several billion assets for that to happen though.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(transparent)]
pub struct U64Id(u64);

impl U64Id {
    /// Returns the forbidden `null` id.
    pub const NULL: U64Id = U64Id(0);

    /// Creates a new, random AssetId, seeded cheaply from thread_rng.
    ///
    /// To avoid calling this internal function repeatedly, consider using [id]
    /// and directly constructing your own rng handler.
    ///
    /// Internally, we use a u64 for random numbers. These have been, generally,
    /// large enough.
    #[cfg(feature = "rand")]
    pub fn new() -> Self {
        use rand::Rng;

        Self(rand::thread_rng().gen())
    }

    /// Creates a new AssetId with the given Id.
    pub const fn id(id: u64) -> Self {
        Self(id)
    }

    /// Checks if the asset is the `null` ID.
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }

    /// Returns the inner value.
    pub const fn inner(self) -> u64 {
        self.0
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
        write!(f, "*{:x}", self.0)
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
        }

        deserializer.deserialize_str(AssetIdVisitor).map(U64Id)
    }
}
