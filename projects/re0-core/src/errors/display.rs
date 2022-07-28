use super::*;

impl Error for Re0Error {}

impl Display for Re0Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Default for Re0Error {
    fn default() -> Self {
        Self { kind: Box::new(Re0ErrorKind::UnknownError), file: None, position: None }
    }
}
