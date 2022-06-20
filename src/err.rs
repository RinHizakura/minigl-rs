use std::fmt;

pub enum MGLError {
    EMEM,
    EFAULT,
}

/* TODO */
impl fmt::Debug for MGLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
