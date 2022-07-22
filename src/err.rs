use std::fmt;

pub enum MGLError {
    EPERM,
    EMEM,
    EFAULT,
    EINVALID,
}

/* TODO */
impl fmt::Debug for MGLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MGLError: ?")
    }
}
