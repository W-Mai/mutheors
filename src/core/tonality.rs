#[derive(Copy, Clone, Debug)]
#[repr(u8)]
#[derive(PartialEq)]
pub enum Tonality {
    Major,
    Minor,
    Diminished,
    Augmented,
    Perfect,
}
