use macroquad::{
    color::Color,
    math::{
        Vec2,
        vec2,
    },
};

use serde::{
    Serialize,
    Deserialize,
};

use super::PostProcessing;

#[derive(Clone, Serialize, Deserialize)]
#[serde(remote = "Color")]
pub struct ColorDef {
    #[serde(rename = "red", alias = "r")]
    pub r: f32,
    #[serde(rename = "green", alias = "g")]
    pub g: f32,
    #[serde(rename = "blue", alias = "b")]
    pub b: f32,
    #[serde(rename = "alpha", alias = "a")]
    pub a: f32,
}

impl From<Color> for ColorDef {
    fn from(other: Color) -> Self {
        ColorDef {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

impl From<ColorDef> for Color {
    fn from(other: ColorDef) -> Self {
        Color {
            r: other.r,
            g: other.g,
            b: other.b,
            a: other.a,
        }
    }
}

pub mod vec2_def {
    use super::{vec2, Vec2};
    use serde::{
        de::{self, MapAccess, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serializer,
    };
    use std::fmt;

    pub fn serialize<S>(value: &Vec2, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct(stringify!(Vec2), 2)?;
        state.serialize_field("x", &value.x)?;
        state.serialize_field("y", &value.y)?;
        state.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec2, D::Error>
        where
            D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            X,
            Y,
        }

        struct Vec2Visitor;

        impl<'de> Visitor<'de> for Vec2Visitor {
            type Value = Vec2;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!("struct ", stringify!(Vec2)))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::X => {
                            if x.is_some() {
                                return Err(de::Error::duplicate_field("x"));
                            }
                            x = Some(map.next_value()?);
                        }
                        Field::Y => {
                            if y.is_some() {
                                return Err(de::Error::duplicate_field("y"));
                            }
                            y = Some(map.next_value()?);
                        }
                    }
                }

                let x = x.ok_or_else(|| de::Error::missing_field("x"))?;
                let y = y.ok_or_else(|| de::Error::missing_field("y"))?;

                Ok(vec2(x, y))
            }
        }

        deserializer.deserialize_struct(stringify!(Vec2), &["x", "y"], Vec2Visitor)
    }
}

pub mod post_processing_def {
    use super::PostProcessing;
    use serde::{
        de::{self, MapAccess, Visitor},
        ser::SerializeStruct,
        Deserialize, Deserializer, Serializer,
    };
    use std::fmt;
    use serde::ser::SerializeMap;

    pub fn serialize<S>(_: &PostProcessing, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_map(Some(0))?;
        state.end()
    }

    pub fn deserialize<'de, D>(_: D) -> Result<PostProcessing, D::Error>
        where
            D: Deserializer<'de>,
    {
        Ok(PostProcessing)
    }
}

pub mod post_processing_opt {
    use super::PostProcessing;
    use serde::{de::{self, MapAccess, Visitor}, ser::SerializeStruct, Deserialize, Deserializer, Serializer, Serialize};
    use std::{
        collections::HashMap,
        fmt,
    };

    use serde::ser::SerializeMap;

    pub fn serialize<S>(value: &Option<PostProcessing>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        value.as_ref().map(|_| HashMap::<String, String>::new()).serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<PostProcessing>, D::Error>
        where
            D: Deserializer<'de>,
    {

        let helper: Option<HashMap<String, String>> = Option::deserialize(deserializer)?;
        Ok(helper.map(|_| PostProcessing))
    }
}