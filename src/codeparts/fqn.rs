use ::std::fmt;
use ::std::fmt::Formatter;

use ::lazy_static::lazy_static;
use ::regex::Regex;

use crate::codeparts::name::Name;

lazy_static! {
    pub static ref FQN_RE: Regex =
        Regex::new(r"^(?:[a-zA-Z][_a-zA-Z0-9]*\.)*(?:_*[a-zA-Z][_a-zA-Z0-9]*|_\b)").unwrap();
}

//TODO @mark: maybe cache hashcode and make comparisons (and hash) faster
/// Fully-qualified name path, e.g. 'package.module1.module2.Type'.
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Fqn {
    names: Vec<Name>,
}

impl PartialEq<Fqn> for Name {
    fn eq(&self, other: &Fqn) -> bool {
        other.names.len() == 1 && &other.names[0] == self
    }
}

impl fmt::Debug for Fqn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "FQN '{}'", self.as_string())
    }
}

impl fmt::Display for Fqn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl Fqn {
    pub fn new(name: impl AsRef<str>) -> Result<Self, String> {
        let mut parts = vec![];
        for part in name.as_ref().split('.') {
            let name = Name::new(part)?;
            parts.push(name);
        }
        debug_assert!(!parts.is_empty());
        Ok(Fqn { names: parts })
    }

    pub fn from_name(name: Name) -> Self {
        Fqn { names: vec![name] }
    }

    pub fn push(&mut self, addition: Name) {
        self.names.push(addition);
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn parts(&self) -> &[Name] {
        &self.names
    }

    pub fn as_string(&self) -> String {
        self.names
            .iter()
            .map(|name| name.as_str())
            // This collect seems useless, but for now it doesn't work without.
            .collect::<Vec<&str>>()
            .join(".")
    }

    pub fn is_simple(&self) -> bool {
        self.names.len() == 1
    }

    pub fn as_simple_name(&self) -> Option<Name> {
        if self.names.len() == 1 {
            return Some(self.names[0]);
        }
        None
    }

    pub fn leaf(&self) -> &Name {
        self.names.last().unwrap()
    }
}

#[cfg(test)]
mod technical {
    use super::*;

    #[test]
    fn new_simple() {
        let fqn = Fqn::new("TheName1").unwrap();
        assert_eq!(fqn.as_string(), "TheName1".to_owned());
        assert_eq!(fqn.parts(), &[Fqn::new("TheName1").unwrap()]);
        assert_eq!(fqn.as_simple_name(), Some(Name::new("TheName1").unwrap()));
    }

    #[test]
    fn new_complex() {
        let fqn = Fqn::new("package.module1.module2.Class").unwrap();
        assert_eq!(fqn.as_string(), "package.module1.module2.Class".to_owned());
        assert_eq!(
            fqn.parts(),
            &[
                Fqn::new("package").unwrap(),
                Fqn::new("module1").unwrap(),
                Fqn::new("module2").unwrap(),
                Fqn::new("Class").unwrap()
            ]
        );
        assert_eq!(fqn.as_simple_name(), None);
    }

    #[test]
    fn equality() {
        assert_eq!(Fqn::new("Hello").unwrap(), Fqn::new("Hello").unwrap());
        assert_eq!(
            Fqn::new("a.b.c.Hello").unwrap(),
            Fqn::new("a.b.c.Hello").unwrap()
        );
        assert_ne!(Fqn::new("Hello").unwrap(), Fqn::new("Goodbye").unwrap());
        assert_ne!(
            Fqn::new("a.b.c.Hello").unwrap(),
            Fqn::new("a.b.d.Hello").unwrap()
        );
    }

    #[test]
    fn pushing() {
        let mut name = Fqn::new("alpha").unwrap();
        name.push(Name::from_valid("beta"));
        assert_eq!(name.as_string(), "alpha.beta");
    }

    #[test]
    fn leaf() {
        let name = Fqn::new("alpha.beta").unwrap();
        assert_eq!(name.leaf(), "beta");
    }
}
