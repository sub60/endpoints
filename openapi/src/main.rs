//! Prints the OpenAPI 3.1 document describing the sub60 domain named by its
//! first argument.

use core::any::{self, TypeId};
use std::{env, process};

use schemars::generate::{Contract, SchemaGenerator, SchemaSettings};
use schemars::{JsonSchema, Schema};
use serde_json::map::Entry;
use serde_json::{Map, Value, json};
use sub60_endpoints_common::{EndpointVisitor, JsonEndpoint, RequestError};

const DOMAINS: &[Domain] = &[Domain {
    name: "api",
    title: "Sub60 API",
    visit_endpoints: sub60_endpoints_api::visit_endpoints,
}];

struct Domain {
    /// The domain's name on the command line.
    name: &'static str,

    /// The title of the domain's OpenAPI document.
    title: &'static str,

    /// Calls the visitor on each of the domain's endpoints.
    visit_endpoints: fn(&mut OpenApiBuilder),
}

fn main() {
    let name = env::args().nth(1).unwrap_or_default();

    let Some(domain) = DOMAINS.iter().find(|domain| domain.name == name) else {
        let names =
            DOMAINS.iter().map(|domain| domain.name).collect::<Vec<_>>();
        eprintln!("usage: sub60-endpoints-openapi <{}>", names.join("|"));
        process::exit(2);
    };

    let schema_generator = |contract| {
        SchemaSettings::draft2020_12()
            .with(|settings| {
                settings.definitions_path = "/components/schemas".into();
                settings.contract = contract;
            })
            .into_generator()
    };

    let mut builder = OpenApiBuilder {
        requests: schema_generator(Contract::Deserialize),
        responses: schema_generator(Contract::Serialize),
        paths: Map::new(),
    };

    (domain.visit_endpoints)(&mut builder);

    let mut schemas = builder.requests.take_definitions(true);

    for (name, schema) in builder.responses.take_definitions(true) {
        match schemas.entry(name) {
            Entry::Vacant(entry) => {
                entry.insert(schema);
            },
            Entry::Occupied(entry) => assert_eq!(
                *entry.get(),
                schema,
                "`{}` is serialized differently than it's deserialized, so \
                 its schemas in requests and responses need different names",
                entry.key(),
            ),
        }
    }

    let document = json!({
        "openapi": "3.1.0",
        "info": {
            "title": domain.title,
            "version": env!("CARGO_PKG_VERSION"),
        },
        "paths": builder.paths,
        "components": {
            "schemas": schemas,
        },
    });

    println!("{document:#}");
}

struct OpenApiBuilder {
    requests: SchemaGenerator,
    responses: SchemaGenerator,
    paths: Map<String, Value>,
}

impl OpenApiBuilder {
    /// Returns the schema of the error type.
    fn error_schema<T: JsonSchema>(&mut self) -> Schema {
        let schema = self.responses.subschema_for::<T>();

        let definitions = self.responses.definitions();

        // The schema is usually a reference to the type's definition.
        let resolved = schema
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|path| path.strip_prefix("#/components/schemas/"))
            .and_then(|name| definitions.get(name))
            .unwrap_or_else(|| schema.as_value());

        let variants = match resolved.get("oneOf") {
            Some(Value::Array(variants)) => variants.iter().collect(),
            _ => vec![resolved],
        };

        for variant in variants {
            let has_code = variant
                .pointer("/properties/code/const")
                .is_some_and(Value::is_string)
                && variant
                    .get("required")
                    .and_then(Value::as_array)
                    .is_some_and(|required| required.contains(&json!("code")));

            assert!(
                has_code,
                "a variant of `{}` isn't serialized with a constant `code` \
                 string, add `#[serde(tag = \"code\", content = \
                 \"details\")]` to it",
                any::type_name::<T>(),
            );
        }

        schema
    }
}

impl EndpointVisitor for OpenApiBuilder {
    #[expect(clippy::too_many_lines)]
    fn visit<E: JsonEndpoint>(&mut self) {
        let mut operation = Map::new();

        // Each `{name}` segment of the path's template is a parameter.
        let parameters = E::PATH
            .split('/')
            .filter_map(|segment| segment.strip_prefix('{')?.strip_suffix('}'))
            .map(|name| {
                json!({
                    "name": name,
                    "in": "path",
                    "required": true,
                    "schema": { "type": "string" },
                })
            })
            .collect::<Vec<_>>();

        if !parameters.is_empty() {
            operation.insert("parameters".to_owned(), parameters.into());
        }

        if TypeId::of::<E::Request>() != TypeId::of::<()>() {
            operation.insert(
                "requestBody".to_owned(),
                json!({
                    "required": true,
                    "content": {
                        "application/json": {
                            "schema": self.requests.subschema_for::<E::Request>(),
                        },
                    },
                }),
            );
        }

        operation.insert(
            "responses".to_owned(),
            json!({
                "200": {
                    "description": "The request succeeded.",
                    "content": {
                        "application/json": {
                            "schema": self.responses.subschema_for::<E::Response>(),
                        },
                    },
                },
                "default": {
                    "description": "The request failed.",
                    "content": {
                        "application/json": {
                            "schema": {
                                "allOf": [
                                    {
                                        "anyOf": [
                                            self.error_schema::<E::Error>(),
                                            self.error_schema::<RequestError>(),
                                        ],
                                    },
                                    {
                                        "type": "object",
                                        "properties": {
                                            "message": {
                                                "description": "What went wrong, as a sentence for people.",
                                                "type": "string",
                                            },
                                        },
                                        "required": ["message"],
                                    },
                                ],
                            },
                        },
                    },
                },
            }),
        );

        let path_item = self
            .paths
            .entry(E::PATH)
            .or_insert_with(|| Value::Object(Map::new()));

        if let Value::Object(path_item) = path_item {
            path_item.insert(
                E::METHOD.as_str().to_ascii_lowercase(),
                operation.into(),
            );
        }
    }
}
