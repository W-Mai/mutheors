#[cfg(test)]
mod tests {
    use simple_compose::*;

    #[test]
    fn test_modulation() {
        assert_eq!(Tuning::C.modulation(2), Tuning::E);
        assert_eq!(Tuning::C.modulation(-2), Tuning::A);
        assert_eq!(Tuning::C.modulation(7), Tuning::C);
        assert_eq!(Tuning::C.modulation(-7), Tuning::C);
        assert_eq!(Tuning::C.modulation(0), Tuning::C);
        assert_eq!(Tuning::None.modulation(1), Tuning::None);
    }
}
