// @generated
impl serde::Serialize for IdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone_number.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("users.client.userspb.IdRequest", len)?;
        if !self.phone_number.is_empty() {
            struct_ser.serialize_field("phoneNumber", &self.phone_number)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phoneNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PhoneNumber,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "phoneNumber" => Ok(GeneratedField::PhoneNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct users.client.userspb.IdRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(IdRequest {
                    phone_number: phone_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("users.client.userspb.IdRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateUserRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone_number.is_empty() {
            len += 1;
        }
        if self.user.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("users.client.userspb.UpdateUserRequest", len)?;
        if !self.phone_number.is_empty() {
            struct_ser.serialize_field("phoneNumber", &self.phone_number)?;
        }
        if let Some(v) = self.user.as_ref() {
            struct_ser.serialize_field("user", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateUserRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phoneNumber",
            "user",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PhoneNumber,
            User,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "phoneNumber" => Ok(GeneratedField::PhoneNumber),
                            "user" => Ok(GeneratedField::User),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateUserRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct users.client.userspb.UpdateUserRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UpdateUserRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone_number__ = None;
                let mut user__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = Some(map.next_value()?);
                        }
                        GeneratedField::User => {
                            if user__.is_some() {
                                return Err(serde::de::Error::duplicate_field("user"));
                            }
                            user__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UpdateUserRequest {
                    phone_number: phone_number__.unwrap_or_default(),
                    user: user__,
                })
            }
        }
        deserializer.deserialize_struct("users.client.userspb.UpdateUserRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.phone_number.is_empty() {
            len += 1;
        }
        if !self.email_address.is_empty() {
            len += 1;
        }
        if self.verified {
            len += 1;
        }
        if self.created_at != 0 {
            len += 1;
        }
        if self.updated_at != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("users.client.userspb.User", len)?;
        if !self.phone_number.is_empty() {
            struct_ser.serialize_field("phoneNumber", &self.phone_number)?;
        }
        if !self.email_address.is_empty() {
            struct_ser.serialize_field("emailAddress", &self.email_address)?;
        }
        if self.verified {
            struct_ser.serialize_field("verified", &self.verified)?;
        }
        if self.created_at != 0 {
            struct_ser.serialize_field("createdAt", &self.created_at)?;
        }
        if self.updated_at != 0 {
            struct_ser.serialize_field("updatedAt", &self.updated_at)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phoneNumber",
            "emailAddress",
            "verified",
            "createdAt",
            "updatedAt",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PhoneNumber,
            EmailAddress,
            Verified,
            CreatedAt,
            UpdatedAt,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "phoneNumber" => Ok(GeneratedField::PhoneNumber),
                            "emailAddress" => Ok(GeneratedField::EmailAddress),
                            "verified" => Ok(GeneratedField::Verified),
                            "createdAt" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" => Ok(GeneratedField::UpdatedAt),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct users.client.userspb.User")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phone_number__ = None;
                let mut email_address__ = None;
                let mut verified__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PhoneNumber => {
                            if phone_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phoneNumber"));
                            }
                            phone_number__ = Some(map.next_value()?);
                        }
                        GeneratedField::EmailAddress => {
                            if email_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("emailAddress"));
                            }
                            email_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Verified => {
                            if verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            verified__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = Some(
                                map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0
                            );
                        }
                    }
                }
                Ok(User {
                    phone_number: phone_number__.unwrap_or_default(),
                    email_address: email_address__.unwrap_or_default(),
                    verified: verified__.unwrap_or_default(),
                    created_at: created_at__.unwrap_or_default(),
                    updated_at: updated_at__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("users.client.userspb.User", FIELDS, GeneratedVisitor)
    }
}
