use axum::response::IntoResponse;
use http::{StatusCode, header};
use schemars::{Schema, SchemaGenerator};
use sub60_endpoints_common::{
    Body,
    Created,
    Json,
    NotFound,
    Ok,
    Response,
    ResponseVisitor,
    WithStatus,
};

#[derive(Debug, PartialEq, Eq, Response)]
enum PutResponse {
    Created(Created<Json<u32>>),
    Replaced(Ok<Json<u32>>),
    Gone(WithStatus<410>),
}

#[derive(Debug, PartialEq, Eq, Response)]
enum GetResponse<B: Body> {
    Found(Ok<B>),
    NotFound(NotFound),
}

#[derive(Response)]
#[expect(dead_code, reason = "only `Gone` is constructed")]
enum DocumentedResponse {
    /// The value was created.
    ///
    /// It's new.
    Created(Created<Json<u32>>),
    Replaced(Ok<Json<u32>>),
    /// The value is gone.
    #[status(GONE)]
    Gone,
}

/// A [`ResponseVisitor`] that collects the status codes and media types of the
/// visited responses.
#[derive(Default)]
struct Collect(Vec<(StatusCode, Option<&'static str>)>);

/// A [`ResponseVisitor`] that collects the descriptions of the visited
/// responses.
#[derive(Default)]
struct CollectDescriptions(Vec<Option<&'static str>>);

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

struct Text;

impl Body for Text {
    const MEDIA_TYPE: Option<&'static str> = Some("text/plain");

    fn schema(_: &mut SchemaGenerator) -> Schema {
        unreachable!()
    }
}

#[test]
fn status_is_the_variants() {
    assert_eq!(
        PutResponse::Created(WithStatus(Json(1))).status(),
        StatusCode::CREATED
    );
    assert_eq!(
        PutResponse::Replaced(WithStatus(Json(1))).status(),
        StatusCode::OK
    );
    assert_eq!(PutResponse::Gone(WithStatus(())).status(), StatusCode::GONE);
}

#[test]
fn into_response_is_the_variants() {
    let created = PutResponse::Created(WithStatus(Json(1))).into_response();
    assert_eq!(created.status(), StatusCode::CREATED);
    assert_eq!(created.headers()[header::CONTENT_TYPE], "application/json");

    let gone = DocumentedResponse::Gone.into_response();
    assert_eq!(gone.status(), StatusCode::GONE);
    assert!(!gone.headers().contains_key(header::CONTENT_TYPE));
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

    assert_eq!(GetResponse::Found(WithStatus(Text)).status(), StatusCode::OK);
    assert_eq!(
        GetResponse::<Text>::NotFound(WithStatus(())).status(),
        StatusCode::NOT_FOUND
    );
}
