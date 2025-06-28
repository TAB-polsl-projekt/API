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
                let mut obj = ::schemars::schema::ObjectValidation::default();

                $(
                    obj.properties.insert(
                        stringify!($field_name).to_string(),
                        r#gen.subschema_for::<$field_ty>(),
                    );
                )*

                // Optional: add required field tracking
                $(
                    obj.required.insert(stringify!($field_name).to_string());
                )*

                let schema = ::schemars::schema::SchemaObject {
                    instance_type: Some(::schemars::schema::SingleOrVec::Single(Box::new(::schemars::schema::InstanceType::Object))),
                    object: Some(Box::new(obj)),
                    metadata: Some(Box::new(::schemars::schema::Metadata {
                        title: Some(Self::schema_name()),
                        ..Default::default()
                    })),
                    ..Default::default()
                };

                Schema::Object(schema)
            }
        }
    };
}
