use sub60_endpoints_common::{PathParams, PathParamsError};

#[derive(Debug, PartialEq, Eq, PathParams)]
#[route("/{owner}/{name}")]
struct RepoPath {
    owner: String,
    name: String,
}

#[derive(Debug, PartialEq, Eq, PathParams)]
#[route("/lines/{line}")]
struct LinesPath {
    line: u32,
}

#[derive(Debug, PartialEq, Eq, PathParams)]
#[route(RepoPath + "/files/{file_id}" + LinesPath)]
struct FilePath<Id = String> {
    repo: RepoPath,
    file_id: Id,
    lines: LinesPath,
}

#[derive(Debug, PartialEq, Eq, PathParams)]
#[route("/-/static")]
struct StaticPath;

fn repo() -> RepoPath {
    RepoPath { owner: "octocat".to_owned(), name: "Hello-World".to_owned() }
}

fn file(file_id: &str) -> FilePath {
    FilePath {
        repo: repo(),
        file_id: file_id.to_owned(),
        lines: LinesPath { line: 3 },
    }
}

#[test]
fn path_joins_the_segments() {
    assert_eq!(
        FilePath::<String>::PATH,
        "/{owner}/{name}/files/{file_id}/lines/{line}"
    );
    assert_eq!(StaticPath::PATH, "/-/static");
}

#[test]
fn to_path_fills_in_the_parameters() {
    assert_eq!(file("abc").to_path(), "/octocat/Hello-World/files/abc/lines/3");
    assert_eq!(StaticPath.to_path(), "/-/static");
}

#[test]
fn to_path_percent_encodes_all_but_unreserved_characters() {
    assert_eq!(
        file("a/b?c d%-._~").to_path(),
        "/octocat/Hello-World/files/a%2Fb%3Fc%20d%25-._~/lines/3"
    );
}

#[test]
fn parse_reads_the_parameters_in_order() {
    let params = [
        ("owner", "octocat"),
        ("name", "Hello-World"),
        ("file_id", "a/b"),
        ("line", "3"),
    ];

    assert_eq!(
        FilePath::<String>::parse(&mut params.into_iter()),
        Ok(file("a/b"))
    );
}

#[test]
fn parse_leaves_the_parameters_it_doesnt_need() {
    let mut params =
        [("owner", "octocat"), ("name", "Hello-World"), ("file_id", "abc")]
            .into_iter();

    assert_eq!(RepoPath::parse(&mut params), Ok(repo()));
    assert_eq!(params.next(), Some(("file_id", "abc")));
}

#[test]
fn parse_rejects_missing_parameters() {
    let params = [("owner", "octocat")];

    assert_eq!(
        RepoPath::parse(&mut params.into_iter()),
        Err(PathParamsError::Missing("name"))
    );
}

#[test]
fn parse_rejects_unexpected_parameters() {
    let params = [("owner", "octocat"), ("file_id", "abc")];

    assert_eq!(
        RepoPath::parse(&mut params.into_iter()),
        Err(PathParamsError::UnexpectedName {
            expected: "name",
            found: "file_id".to_owned(),
        })
    );
}

#[test]
fn parse_rejects_invalid_values() {
    let params = [("line", "third")];

    assert!(matches!(
        LinesPath::parse(&mut params.into_iter()),
        Err(PathParamsError::Invalid { name: "line", .. })
    ));
}

#[test]
fn param_schemas_are_the_fields_schemas_in_order() {
    let mut generator = schemars::SchemaGenerator::default();
    let mut schemas = Vec::new();
    FilePath::<u64>::param_schemas(&mut generator, &mut schemas);

    let names = schemas.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    assert_eq!(names, ["owner", "name", "file_id", "line"]);

    assert_eq!(schemas[2].1.get("type"), Some(&"integer".into()));
    assert_eq!(schemas[3].1.get("type"), Some(&"integer".into()));
}
