pub enum Dimension {
    DIM_NETHER,
    DIM_OVERWORLD,
    DIM_END,
    DIM_UNDEF,
}

impl Dimension {
    pub fn value(&self) -> i32 { 
        match *self {
            Dimension::DIM_NETHER           => -1,
            Dimension::DIM_OVERWORLD        => 0,
            Dimension::DIM_END              => 1,
            Dimension::DIM_UNDEF            => 1000,
        }
    }
}