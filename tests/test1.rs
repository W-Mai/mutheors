#[cfg(test)]
mod tests {
    use simple_compose::*;

    fn sum_duration(ds: &Vec<Duration>) -> f64 {
        ds.iter().fold(0.0f64, |acc, x| acc + x)
    }

    fn do_a_measure_test(beat: u8) {
        let ds = duration_utils::generate_one_measure(beat);
        println!(
            "{}",
            (&ds)
                .iter()
                .fold("".to_owned(), |acc, x| format!("{} {}", acc, x))
        );
        assert_eq!(sum_duration(&ds), beat as f64);
    }

    #[test]
    fn test_duration_1() {
        assert_eq!(
            Duration::new(DurationBase::Whole) + Duration::new(DurationBase::Half),
            1.5
        );
        assert_eq!(
            <Duration as Into<f32>>::into(Duration::new(DurationBase::Half)),
            0.5
        );
        assert_eq!(Duration::from(0.5).base, DurationBase::Half);
    }

    #[test]
    fn test_duration_2() {
        assert_eq!(Duration::new(DurationBase::Half).to_string(), "𝅗𝅥");
    }

    #[test]
    fn test_measure_1() {
        for i in 1..16 {
            for _ in 0..8 {
                do_a_measure_test(i);
            }
        }
    }
}
