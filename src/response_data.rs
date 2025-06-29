#[macro_export]
macro_rules! define_response_data {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(#[$field_meta:meta])*
            $($field_vis:vis $field_name:ident : $field_ty:ty),* $(,)?
        }
    ) => {
        use schemars::{r#gen::SchemaGenerator, schema::Schema};

        $(#[$meta])*
        #[derive(::serde::Serialize, ::serde::Deserialize, Debug)]
        $vis struct $name {
            $(#[$field_meta])*
            $($field_vis $field_name : $field_ty),*
        }

        $(#[$meta])*
        #[derive(::serde::Serialize, ::serde::Deserialize, Debug, ::schemars::JsonSchema)]
        struct Helper {
            $(#[$field_meta])*
            $($field_vis $field_name : $field_ty),*
        }

        impl ::schemars::JsonSchema for $name {
            fn schema_name() -> String {
                format!("{}::{}", module_path!(), stringify!($name))
            }

            fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
                // This builds the full “components/schemas/MyType” object
                let mut root = r#gen.root_schema_for::<Helper>().schema;
                if let Some(md) = &mut root.metadata {
                    md.title = Some(Self::schema_name());
                }
                Schema::Object(root)
            }
        }
    };
}
