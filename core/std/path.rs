use foundations::case_convert::*;

pub fn parts_to_path(s: &str, p: &str, n: &str) -> String {
    foundations::concat_string!(s, ":", p, ":", n)
}

pub fn to_rust_name(n: &str) -> String {
    to_pascal_case(n)
}

pub fn to_rust_path(p: &str) -> String {
    // TODO prevent keywords
    to_snake_case(p).replace(':', "_")
}

pub fn parts_to_rust_path(root: &str, p: &str, n: &str) -> String {
    foundations::concat_string!(root, "::", to_rust_path(p), "::", to_rust_name(n))
}

pub fn parts_to_rust_self_path(p: &str, n: &str) -> String {
    parts_to_rust_path("super", p, n)
}

#[derive(Clone, Copy, Debug)]
pub struct StdPath {
    pub path: &'static str,
    pub name: &'static str,
    // pub ver: &'static str,
}

impl StdPath {
    pub fn to_path(&self) -> String {
        parts_to_path("std", self.path, self.name)
    }

    pub fn to_rust_name(&self) -> String {
        to_rust_name(self.name)
    }

    pub fn to_rust_path(&self) -> String {
        to_rust_path(self.path)
    }

    pub fn to_rust_self_path(&self) -> String {
        parts_to_rust_self_path(self.path, self.name)
    }

    pub fn to_rust_foreign_path(&self) -> String {
        parts_to_rust_path("zeon::std::codegen", self.path, self.name)
    }
}

#[derive(Clone, Debug)]
pub struct Path {
    pub namespace: String,
    pub path: String,
    pub name: String,
    // pub ver: String,
}

impl Path {
    pub fn to_path(&self) -> String {
        parts_to_path("std", &self.path, &self.name)
    }

    pub fn to_rust_name(&self) -> String {
        to_rust_name(&self.name)
    }

    pub fn to_rust_path(&self) -> String {
        to_rust_path(&self.name)
    }

    pub fn to_rust_self_path(&self) -> String {
        parts_to_rust_self_path(&self.path, &self.name)
    }

    pub fn to_rust_foreign_path(&self) -> String {
        parts_to_rust_path("zeon::std::codegen", &self.path, &self.name)
    }
}
