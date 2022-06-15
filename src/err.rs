use std::fmt;

pub enum MglError {
    MglErrorMem,
}

/* TODO */
impl fmt::Debug for MglError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MglError").finish()
    }
}
