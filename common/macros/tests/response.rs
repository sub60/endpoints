use http::response::Parts;
use http::{StatusCode, header};
use schemars::{Schema, SchemaGenerator};
use sub60_endpoints_common::body::{self, Body, Json, WithMessage};
use sub60_endpoints_common::response::{
    Created,
    Decode,
    DecodeError,
    Encode,
    Encoder,
    NotFound,
    Ok,
    Response,
    ResponseVisitor,
    Status,
    WithStatus,
    WithStatusOf,
};
use sub60_endpoints_common::{Alternatives, Statuses, error, status};

const JSON: Option<&str> = Some("application/json");

#[derive(Debug, PartialEq, Eq, Alternatives)]
enum PutResponse {
    Created(Created<Json<u32>>),
    Replaced(Ok<Json<u32>>),
    Gone(WithStatus<410>),
}

#[derive(Debug, PartialEq, Eq, Alternatives)]
enum GetResponse<B: Body> {
    Found(Ok<B>),
    NotFound(NotFound),
}

/// A response with two variants of the same status code.
#[derive(Debug, PartialEq, Eq, Alternatives)]
enum NumberOrText {
    Number(Ok<Json<u32>>),
    Text(Ok<Json<String>>),
}

/// A response whose body can only be described.
#[derive(Alternatives)]
enum TextResponse {
    Found(Ok<Text>),
    NotFound(NotFound),
}

#[derive(Alternatives)]
enum DocumentedResponse {
    /// The value was created.
    ///
    /// It's new.
    Created(Created<Json<u32>>),
    Replaced(Ok<Json<u32>>),
    /// The value is gone.
    Gone(WithStatus<410>),
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    Statuses,
)]
#[serde(tag = "code", rename_all = "snake_case")]
enum Error {
    /// The name is taken.
    #[display("the name is taken")]
    NameTaken(status::Conflict),

    #[display("the value conflicts with {}", _0.id)]
    Conflict(Existing),

    #[display("{_0}")]
    Internal(error::Internal),
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
struct Existing {
    id: u32,
}

/// A [`ResponseVisitor`] that collects the status codes and media types of the
/// visited responses.
#[derive(Default)]
struct Collect(Vec<(StatusCode, Option<&'static str>)>);

/// A [`ResponseVisitor`] that collects the descriptions of the visited
/// responses.
#[derive(Default)]
struct CollectDescriptions(Vec<Option<&'static str>>);

/// An [`Encoder`] into the status code, media type and body text of a
/// response.
struct ToText;

impl Status for Existing {
    const STATUS: StatusCode = StatusCode::CONFLICT;
}

impl ResponseVisitor for Collect {
    fn visit<B: Body>(&mut self, status: StatusCode, _: Option<&'static str>) {
        self.0.push((status, B::MEDIA_TYPE));
    }
}

impl ResponseVisitor for CollectDescriptions {
    fn visit<B: Body>(
        &mut self,
        _: StatusCode,
        description: Option<&'static str>,
    ) {
        self.0.push(description);
    }
}

impl Encoder for ToText {
    type Output = (StatusCode, Option<&'static str>, String);

    fn encode<B: body::Encode>(
        self,
        status: StatusCode,
        body: &B,
    ) -> Self::Output {
        let mut bytes = Vec::new();
        body.encode(&mut bytes);
        let text = String::from_utf8(bytes).expect("the test bodies are UTF-8");
        (status, B::MEDIA_TYPE, text)
    }
}

struct Text;

impl Body for Text {
    const MEDIA_TYPE: Option<&'static str> = Some("text/plain");

    fn schema(_: &mut SchemaGenerator) -> Schema {
        unreachable!()
    }
}

/// Returns the status code, media type and body `response` encodes into.
fn encode(
    response: &impl Encode,
) -> (StatusCode, Option<&'static str>, String) {
    response.encode(ToText)
}

/// Returns the parts of a response with status code `status` and
/// `Content-Type` `content_type`.
fn parts(status: StatusCode, content_type: Option<&str>) -> Parts {
    let mut response = http::Response::builder().status(status);

    if let Some(content_type) = content_type {
        response = response.header(header::CONTENT_TYPE, content_type);
    }

    let (parts, ()) =
        response.body(()).expect("the response is valid").into_parts();

    parts
}

#[test]
fn encode_is_the_variants() {
    assert_eq!(
        encode(&PutResponse::Created(WithStatus(Json(1)))),
        (StatusCode::CREATED, Some("application/json"), "1".to_owned())
    );
    assert_eq!(
        encode(&DocumentedResponse::Gone(WithStatus(()))),
        (StatusCode::GONE, None, String::new())
    );
}

#[test]
fn visit_visits_the_variants_in_order() {
    let mut collect = Collect::default();
    PutResponse::visit(&mut collect);
    assert_eq!(
        collect.0,
        [
            (StatusCode::CREATED, Some("application/json")),
            (StatusCode::OK, Some("application/json")),
            (StatusCode::GONE, None),
        ]
    );
}

#[test]
fn visit_describes_the_variants_with_their_docs() {
    let mut collect = CollectDescriptions::default();
    DocumentedResponse::visit(&mut collect);
    assert_eq!(
        collect.0,
        [
            Some("The value was created.\n\nIt's new."),
            None,
            Some("The value is gone.")
        ]
    );
}

#[test]
fn generic_enums() {
    let mut collect = Collect::default();
    GetResponse::<Text>::visit(&mut collect);
    assert_eq!(
        collect.0,
        [(StatusCode::OK, Some("text/plain")), (StatusCode::NOT_FOUND, None)]
    );
}

#[test]
fn bodies_that_can_only_be_described() {
    let mut collect = Collect::default();
    TextResponse::visit(&mut collect);
    assert_eq!(
        collect.0,
        [(StatusCode::OK, Some("text/plain")), (StatusCode::NOT_FOUND, None)]
    );
}

#[test]
fn decode_is_the_first_variant_that_decodes() {
    assert_eq!(
        PutResponse::decode(&parts(StatusCode::OK, JSON), b"1").unwrap(),
        PutResponse::Replaced(WithStatus(Json(1)))
    );
    assert_eq!(
        NumberOrText::decode(&parts(StatusCode::OK, JSON), b"\"one\"").unwrap(),
        NumberOrText::Text(WithStatus(Json("one".to_owned())))
    );
    assert!(matches!(
        DocumentedResponse::decode(&parts(StatusCode::GONE, None), b""),
        Ok(DocumentedResponse::Gone(_))
    ));
    assert!(matches!(
        PutResponse::decode(&parts(StatusCode::IM_A_TEAPOT, JSON), b"1"),
        Err(DecodeError::Status(_))
    ));
    assert!(matches!(
        PutResponse::decode(&parts(StatusCode::OK, JSON), b"\"one\""),
        Err(DecodeError::Body(_))
    ));
}

#[test]
fn decode_checks_the_media_type() {
    assert!(matches!(
        PutResponse::decode(&parts(StatusCode::OK, Some("text/plain")), b"1"),
        Err(DecodeError::MediaType { .. })
    ));
    assert!(matches!(
        PutResponse::decode(
            &parts(StatusCode::OK, Some("application/json; charset=utf-8")),
            b"1"
        ),
        Ok(PutResponse::Replaced(_))
    ));
}

#[test]
fn decode_results() {
    assert_eq!(
        Result::<PutResponse, StatusCode>::decode(
            &parts(StatusCode::BAD_GATEWAY, None),
            b""
        )
        .unwrap(),
        Err(StatusCode::BAD_GATEWAY)
    );
}

#[test]
fn with_status_of_encodes_the_status_of_the_variant() {
    assert_eq!(
        encode(&WithStatusOf(Json(WithMessage(Error::Conflict(Existing {
            id: 1
        }))))),
        (
            StatusCode::CONFLICT,
            Some("application/json"),
            r#"{"code":"conflict","id":1,"message":"the value conflicts with 1"}"#
                .to_owned()
        )
    );
    assert_eq!(
        encode(&WithStatusOf(Json(WithMessage(Error::NameTaken(
            status::Conflict
        ))))),
        (
            StatusCode::CONFLICT,
            Some("application/json"),
            r#"{"code":"name_taken","message":"the name is taken"}"#.to_owned()
        )
    );
}

#[test]
fn with_status_of_decode_checks_the_status_of_the_variant() {
    type ErrorResponse = WithStatusOf<Json<WithMessage<Error>>>;

    let conflict = br#"{"code":"conflict","id":1,"message":"-"}"#;

    assert_eq!(
        ErrorResponse::decode(&parts(StatusCode::CONFLICT, JSON), conflict)
            .unwrap(),
        WithStatusOf(Json(WithMessage(Error::Conflict(Existing { id: 1 }))))
    );
    assert!(matches!(
        ErrorResponse::decode(
            &parts(StatusCode::INTERNAL_SERVER_ERROR, JSON),
            conflict
        ),
        Err(DecodeError::Status(_))
    ));
    assert!(matches!(
        ErrorResponse::decode(
            &parts(StatusCode::BAD_GATEWAY, Some("text/html")),
            b"<html></html>"
        ),
        Err(DecodeError::Status(_))
    ));
}

#[test]
fn with_status_of_visits_each_variant() {
    type ErrorResponse = WithStatusOf<Json<WithMessage<Error>>>;

    let mut collect = Collect::default();
    ErrorResponse::visit(&mut collect);
    assert_eq!(
        collect.0,
        [
            (StatusCode::CONFLICT, Some("application/json")),
            (StatusCode::CONFLICT, Some("application/json")),
            (StatusCode::INTERNAL_SERVER_ERROR, Some("application/json")),
        ]
    );

    let mut collect = CollectDescriptions::default();
    ErrorResponse::visit(&mut collect);
    assert_eq!(collect.0, [Some("The name is taken."), None, None]);
}
