use crate::utils::{
    derive_builder_attr, serde_as_attr, serde_attr, sqlx_from_row_attr, sqlx_type_attr,
};
use prost_build::Config;

/// provide extra attributes to the generated protobuf code easily
pub trait BuilderAttributes {
    /// add type attributes with `#[derive(serde::Serialize, serde::Deserialize)]`
    fn with_serde(
        &mut self,
        paths: &[&str],
        ser: bool,
        de: bool,
        extra_attrs: Option<&[&str]>,
    ) -> &mut Self;
    fn with_serde_as(&mut self, paths: &str, fields: &[(&[&str], &str)]) -> &mut Self;
    /// add type attributes with `#[derive(sqlx::Type)]`
    fn with_sqlx_type(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self;
    /// add type attributes with `#[derive(sqlx::FromRow)]`
    fn with_sqlx_from_row(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self;
    /// add type attributes with `#[derive(derive_builder::Builder)]`
    fn with_derive_builder(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self;
    /// add type attributes with `#[derive(strum::EnumString)]`
    fn with_strum(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self;
    /// add type attributes
    fn with_type_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self;
    /// add field attributes
    fn with_field_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self;
    /// add optional type attributes
    fn with_optional_type_attributes(
        &mut self,
        paths: &[&str],
        attributes: Option<&[&str]>,
    ) -> &mut Self;
    /// add optional field attributes
    fn with_optional_field_attributes(
        &mut self,
        paths: &[&str],
        attributes: Option<&[&str]>,
    ) -> &mut Self;
}

impl BuilderAttributes for Config {
    fn with_serde(
        &mut self,
        paths: &[&str],
        ser: bool,
        de: bool,
        extra_attrs: Option<&[&str]>,
    ) -> &mut Self {
        let attr = serde_attr(ser, de);

        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, attr)
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_serde_as(&mut self, path: &str, fields: &[(&[&str], &str)]) -> &mut Self {
        let serde_attr = serde_as_attr();
        fields.iter().fold(
            self.type_attribute(path, serde_attr),
            |builder, (paths, attr)| {
                paths.iter().fold(builder, |builder, p| {
                    let p = format!("{}.{}", path, p);
                    builder.field_attribute(p, attr)
                })
            },
        )
    }

    fn with_sqlx_type(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_type_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_sqlx_from_row(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, sqlx_from_row_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_derive_builder(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(ty, derive_builder_attr())
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_strum(&mut self, paths: &[&str], extra_attrs: Option<&[&str]>) -> &mut Self {
        paths.iter().fold(self, |builder, ty| {
            builder
                .type_attribute(
                    ty,
                    "#[derive(strum::EnumString, strum::Display,strum::EnumIter)]",
                )
                .with_optional_type_attributes(&[ty], extra_attrs)
        })
    }

    fn with_type_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self {
        let attr = attributes.join("\n");

        paths.iter().fold(self, |builder, ty| {
            builder.type_attribute(ty, attr.as_str())
        })
    }

    fn with_field_attributes(&mut self, paths: &[&str], attributes: &[&str]) -> &mut Self {
        let attr = attributes.join("\n");
        paths.iter().fold(self, |builder, ty| {
            builder.field_attribute(ty, attr.as_str())
        })
    }

    fn with_optional_type_attributes(
        &mut self,
        paths: &[&str],
        attributes: Option<&[&str]>,
    ) -> &mut Self {
        if let Some(attributes) = attributes {
            self.with_type_attributes(paths, attributes)
        } else {
            self
        }
    }

    fn with_optional_field_attributes(
        &mut self,
        paths: &[&str],
        attributes: Option<&[&str]>,
    ) -> &mut Self {
        if let Some(attributes) = attributes {
            self.with_field_attributes(paths, attributes)
        } else {
            self
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fs;
//     use tempfile::tempdir;

//     #[test]
//     fn test_prost_build_with_extra_attributes_should_work() {
//         let path = tempdir().unwrap();
//         let filename = path.path().join("todo.rs");
//         Config::default()
//             .out_dir(path.path())
//             .with_serde(
//                 &["todo.Todo", "todo.TodoStatus"],
//                 true,
//                 true,
//                 Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
//             )
//             .with_serde_as(
//                 "todo.Todo",
//                 &[(
//                     &["status", "created_at"],
//                     r#"#[serde_as(as = "DisplayFromStr")]"#,
//                 )],
//             )
//             .with_derive_builder(
//                 &["todo.Todo"],
//                 Some(&[r#"#[builder(build_fn(name = "private_build"))]"#]),
//             )
//             .with_sqlx_type(&["todo.TodoStatus"], None)
//             .with_strum(
//                 &["todo.TodoStatus"],
//                 Some(&[r#"#[strum(ascii_case_insensitive, serialize_all = "snake_case")]"#]),
//             )
//             .with_field_attributes(
//                 &["todo.Todo.created_at", "todo.Todo.updated_at"],
//                 &["#[derive(Copy)]"],
//             )
//             .compile_protos(&["fixtures/protos/todo.proto"], &["fixtures/protos"])
//             .unwrap();
//     }
// }
