//! Prints the OpenAPI 3.1 document describing the sub60 domain named by its
//! first argument.

use std::{env, process};

use http::StatusCode;
use schemars::JsonSchema;
use schemars::generate::{Contract, SchemaGenerator, SchemaSettings};
use serde_json::map::Entry;
use serde_json::{Map, Value, json};
use sub60_endpoints_common::{
    Body,
    Endpoint,
    EndpointVisitor,
    JsonError,
    Request,
    RequestError,
    Response,
    ResponseVisitor,
};

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

struct ResponsesBuilder<'a> {
    generator: &'a mut SchemaGenerator,
    responses: &'a mut Map<String, Value>,
}

impl OpenApiBuilder {
    fn query_parameters<Q: JsonSchema>(&mut self) -> Vec<Value> {
        let schema = Q::json_schema(&mut self.requests);
        let required = schema.get("required").and_then(Value::as_array);
        schema
            .get("properties")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
            .map(|(name, schema)| {
                json!({
                    "name": name,
                    "in": "query",
                    "required": required.is_some_and(|required| {
                        required.contains(&json!(name))
                    }),
                    "schema": schema,
                })
            })
            .collect()
    }
}

impl EndpointVisitor for OpenApiBuilder {
    fn visit<E: Endpoint>(&mut self) {
        let mut operation = Map::new();

        // Each `{name}` segment of the path's template is a parameter.
        let mut parameters = E::PATH
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

        parameters
            .extend(self.query_parameters::<<E::Request as Request>::Query>());

        if !parameters.is_empty() {
            operation.insert("parameters".to_owned(), parameters.into());
        }

        if let Some(content) =
            body_content::<<E::Request as Request>::Body>(&mut self.requests)
        {
            operation.insert(
                "requestBody".to_owned(),
                json!({ "required": true, "content": content }),
            );
        }

        let mut failure = json!({ "description": "The request failed." });

        if let Some(media_type) = E::Error::MEDIA_TYPE {
            let schema = json!({
                "anyOf": [
                    E::Error::schema(&mut self.responses),
                    JsonError::<RequestError>::schema(&mut self.responses),
                ],
            });
            failure["content"] = content(media_type, schema);
        }

        let mut responses = Map::new();

        responses.insert("default".to_owned(), failure);

        E::Response::visit(&mut ResponsesBuilder {
            generator: &mut self.responses,
            responses: &mut responses,
        });

        operation.insert("responses".to_owned(), responses.into());

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

impl ResponseVisitor for ResponsesBuilder<'_> {
    fn visit<B: Body>(&mut self, status: StatusCode) {
        let description = if status.is_success() {
            "The request succeeded."
        } else {
            status.canonical_reason().unwrap_or("The request didn't succeed.")
        };

        let mut response = json!({ "description": description });

        if let Some(content) = body_content::<B>(self.generator) {
            response["content"] = content;
        }

        let previous =
            self.responses.insert(status.as_str().to_owned(), response);

        assert!(
            previous.is_none(),
            "an endpoint has two responses with status code {status}",
        );
    }
}

fn body_content<B: Body>(generator: &mut SchemaGenerator) -> Option<Value> {
    B::MEDIA_TYPE.map(|media_type| content(media_type, B::schema(generator)))
}

fn content(media_type: &str, schema: impl serde::Serialize) -> Value {
    let mut content = Map::new();
    content.insert(media_type.to_owned(), json!({ "schema": schema }));
    content.into()
}
