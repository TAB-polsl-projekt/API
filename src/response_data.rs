#[macro_export]
macro_rules! define_response_data {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $($field_vis:vis $field_name:ident : $field_ty:ty),* $(,)?
        }
    ) => {
        use serde::{Serialize, Deserialize};
        use schemars::{JsonSchema, r#gen::SchemaGenerator, schema::Schema};

        $(#[$meta])*
        #[derive(Serialize, Deserialize, Debug)]
        $vis struct $name {
            $($field_vis $field_name : $field_ty),*
        }

        impl JsonSchema for $name {
            fn schema_name() -> String {
                format!("{}::{}", module_path!(), stringify!($name))
            }

            fn json_schema(r#gen: &mut SchemaGenerator) -> Schema {
                #[derive(JsonSchema)]
                #[allow(dead_code)]
                struct Helper {
                    $($field_vis $field_name : $field_ty),*
                }

                r#gen.subschema_for::<Helper>()
            }
        }
    };
}
