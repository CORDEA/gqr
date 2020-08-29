pub enum Version {
    M1,
}

impl Version {
    pub fn module(&self) -> u16 {
        return match self {
            Self::M1 => 11,
        };
    }
}
