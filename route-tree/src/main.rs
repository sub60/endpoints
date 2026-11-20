use core::fmt;
use std::{env, process};

use http::Method;
use sub60_endpoints_common::{Endpoint, EndpointVisitor};

const DOMAINS: &[Domain] = &[
    Domain {
        name: "api",
        visit_endpoints: sub60_endpoints_api::visit_endpoints,
    },
    Domain {
        name: "cache",
        visit_endpoints: sub60_endpoints_cache::visit_endpoints,
    },
];

const METHOD_ORDER: &[Method] = &[
    Method::GET,
    Method::HEAD,
    Method::POST,
    Method::PUT,
    Method::PATCH,
    Method::DELETE,
];

struct Domain {
    name: &'static str,
    visit_endpoints: fn(&mut Node),
}

#[derive(Default)]
struct Node {
    /// The path's segments after its parent's.
    segments: Vec<&'static str>,

    /// The endpoint's methods in `METHOD_ORDER`.
    methods: Vec<Method>,

    /// Longer paths starting with this one, sorted by their segments.
    children: Vec<Self>,
}

fn main() {
    let name = env::args().nth(1).unwrap_or_default();

    let Some(domain) = DOMAINS.iter().find(|domain| domain.name == name) else {
        let names =
            DOMAINS.iter().map(|domain| domain.name).collect::<Vec<_>>();
        eprintln!("usage: sub60-endpoints-route-tree <{}>", names.join("|"));
        process::exit(2);
    };

    let mut root = Node::default();

    (domain.visit_endpoints)(&mut root);

    root.compress();

    print!("{root}");
}

impl Node {
    /// Returns the child whose only segment is `segment`, inserting it if
    /// there isn't one.
    fn child_or_insert(&mut self, segment: &'static str) -> &mut Self {
        let idx = match self
            .children
            .binary_search_by(|child| child.segments.as_slice().cmp(&[segment]))
        {
            Ok(idx) => idx,
            Err(idx) => {
                let child = Self { segments: vec![segment], ..Self::default() };
                self.children.insert(idx, child);
                idx
            },
        };

        &mut self.children[idx]
    }

    fn compress(&mut self) {
        while self.methods.is_empty() && self.children.len() == 1 {
            let child = self.children.pop().expect("checked length");
            self.segments.extend(child.segments);
            self.methods = child.methods;
            self.children = child.children;
        }

        for child in &mut self.children {
            child.compress();
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let root_line_len = 1 + self.segments.join("/").len();

        // The methods start two spaces after the longest line.
        let methods_column =
            root_line_len.max(longest_descendant_line_len(self, 0)) + 2;

        write_tree(self, &mut String::new(), "/", 0, methods_column, f)
    }
}

impl EndpointVisitor for Node {
    fn visit<E: Endpoint>(&mut self) {
        let node = E::PATH
            .split('/')
            .filter(|segment| !segment.is_empty())
            .fold(self, Self::child_or_insert);

        node.methods.push(E::METHOD);

        node.methods.sort_by_key(|method| {
            METHOD_ORDER
                .iter()
                .position(|known| known == method)
                .unwrap_or(METHOD_ORDER.len())
        });
    }
}

/// Returns the number of characters in the longest line of the node's
/// descendants, when the node is `depth` levels below the root.
fn longest_descendant_line_len(node: &Node, depth: usize) -> usize {
    /// The number of characters in a branch, e.g. `├── ` or `│   `.
    const BRANCH_LEN: usize = 4;

    node.children
        .iter()
        .map(|child| {
            let line_len =
                BRANCH_LEN * (depth + 1) + child.segments.join("/").len();
            line_len.max(longest_descendant_line_len(child, depth + 1))
        })
        .max()
        .unwrap_or_default()
}

#[expect(clippy::too_many_arguments)]
fn write_tree(
    node: &Node,
    shifts: &mut String,
    ident: &str,
    last_shift_byte_len: usize,
    methods_column: usize,
    f: &mut fmt::Formatter,
) -> fmt::Result {
    let line = format!(
        "{}{}{}",
        &shifts[..shifts.len() - last_shift_byte_len],
        ident,
        node.segments.join("/")
    );

    if node.methods.is_empty() {
        writeln!(f, "{line}")?;
    } else {
        let methods = node
            .methods
            .iter()
            .map(Method::as_str)
            .collect::<Vec<_>>()
            .join(" ");

        writeln!(f, "{line:methods_column$}{methods}")?;
    }

    for (i, child) in node.children.iter().enumerate() {
        let is_last = i + 1 == node.children.len();
        let ident = if is_last { "└── " } else { "├── " };
        let shift = if is_last { "    " } else { "│   " };
        shifts.push_str(shift);
        write_tree(child, shifts, ident, shift.len(), methods_column, f)?;
        shifts.truncate(shifts.len() - shift.len());
    }

    Ok(())
}
