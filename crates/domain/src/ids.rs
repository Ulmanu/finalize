//! Stable, client-generatable identifiers.
//!
//! All persisted user entities use UUIDv7 identifiers so that offline clients can create
//! records before synchronizing and so that identifiers sort roughly by creation time. Later
//! roadmap changes extend this module with task/project/area identifiers.

use std::fmt;

use uuid::Uuid;

macro_rules! id_newtype {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(Uuid);

        impl $name {
            /// Creates an identifier from an existing UUID.
            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            /// Returns the underlying UUID.
            #[must_use]
            pub const fn as_uuid(&self) -> Uuid {
                self.0
            }

            /// Generates a new time-ordered (UUIDv7) identifier.
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $name {
            type Err = uuid::Error;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(value)?))
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                Uuid::deserialize(deserializer).map(Self)
            }
        }
    };
}

id_newtype!(
    /// Canonical identity of an account.
    AccountId
);

id_newtype!(
    /// Canonical identity of an authenticated session.
    SessionId
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_ids_are_unique_and_parseable() {
        let first = AccountId::new();
        let second = AccountId::new();
        assert_ne!(first, second);

        let parsed: AccountId = first.to_string().parse().expect("round-trip parse");
        assert_eq!(parsed, first);
    }

    #[test]
    fn ids_serialize_as_uuid_strings() {
        let id = AccountId::new();
        let json = serde_json::to_string(&id).expect("serialize");
        assert!(json.contains('-'), "uuid string expected: {json}");
        let decoded: AccountId = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(decoded, id);
    }
}
