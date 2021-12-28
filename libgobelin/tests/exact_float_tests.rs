use libgobelin::ExactFloat;

mod from_str {
    use super::*;

    #[test]
    fn it_should_parse_positive_integer() {
        assert_eq!(
            "100".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(100, 0))
        );
    }

    #[test]
    fn it_should_parse_negative_integer() {
        assert_eq!(
            "-100".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(-100, 0))
        );
    }

    #[test]
    fn it_should_parse_positive_integer_over_thousand() {
        assert_eq!(
            "1000".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(1000, 0))
        );
    }

    #[test]
    fn it_should_parse_negative_integer_over_thousand() {
        assert_eq!(
            "-1000".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(-1000, 0))
        );
    }

    #[test]
    fn it_should_parse_positive_float() {
        assert_eq!(
            "100.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(100, 57))
        );
    }

    #[test]
    fn it_should_parse_negative_float() {
        assert_eq!(
            "-100.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(-100, 57))
        );
    }

    #[test]
    fn it_should_parse_float_with_zero_decimal() {
        assert_eq!(
            "100.00".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(100, 0))
        );
    }

    #[test]
    fn it_should_parse_formatted_number() {
        assert_eq!(
            "-1 000.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(-1000, 57))
        );
        assert_eq!(
            "12 000.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(12000, 57))
        );
        assert_eq!(
            "+10 000.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(10000, 57))
        );
        assert_eq!(
            "-   1 000.07".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(-1000, 7))
        );
        assert_eq!(
            "+   10 000.57".to_string().parse::<ExactFloat>(),
            Ok(ExactFloat::new(10000, 57))
        );
    }
}

mod fmt {
    use super::*;
    #[test]
    fn it_should_format_correctly_positive_integer() {
        assert_eq!(format!("{}", ExactFloat::new(100, 0)), String::from("100"));
    }
    #[test]
    fn it_should_format_correctly_negative_integer() {
        assert_eq!(
            format!("{}", ExactFloat::new(-100, 0)),
            String::from("-100")
        );
    }
    #[test]
    fn it_should_format_correctly_positive_float() {
        assert_eq!(
            format!("{}", ExactFloat::new(100, 57)),
            String::from("100.57")
        );
        assert_eq!(
            format!("{}", ExactFloat::new(100, 7)),
            String::from("100.07")
        );
    }
    #[test]
    fn it_should_format_correctly_negative_float() {
        assert_eq!(
            format!("{}", ExactFloat::new(-100, 57)),
            String::from("-100.57")
        );
        assert_eq!(
            format!("{}", ExactFloat::new(-100, 7)),
            String::from("-100.07")
        );
    }
    #[test]
    fn it_should_format_correctly_positive_integer_over_thousand() {
        assert_eq!(
            format!("{}", ExactFloat::new(1000, 0)),
            String::from("1 000")
        );
        assert_eq!(
            format!("{}", ExactFloat::new(1000000, 0)),
            String::from("1 000 000")
        );
    }
    #[test]
    fn it_should_format_correctly_negative_integer_over_thousand() {
        assert_eq!(
            format!("{}", ExactFloat::new(-1000000, 0)),
            String::from("-1 000 000")
        );
    }
}

mod numerator_digit_count {
    use super::*;

    #[test]
    fn it_should_compute_correctly_digit_count() {
        assert_eq!(ExactFloat::new(0, 0).numerator_digit_count(), 1);
        assert_eq!(ExactFloat::new(1, 0).numerator_digit_count(), 1);
        assert_eq!(ExactFloat::new(12, 0).numerator_digit_count(), 2);
        assert_eq!(ExactFloat::new(123, 0).numerator_digit_count(), 3);
        assert_eq!(ExactFloat::new(1234, 0).numerator_digit_count(), 4);
        assert_eq!(ExactFloat::new(12345, 0).numerator_digit_count(), 5);
        assert_eq!(ExactFloat::new(123456, 0).numerator_digit_count(), 6);
        assert_eq!(ExactFloat::new(1234567, 0).numerator_digit_count(), 7);
        assert_eq!(ExactFloat::new(12345678, 0).numerator_digit_count(), 8);
        assert_eq!(ExactFloat::new(123456789, 0).numerator_digit_count(), 9);
        assert_eq!(ExactFloat::new(1234567890, 0).numerator_digit_count(), 10);
        assert_eq!(ExactFloat::new(-1, 0).numerator_digit_count(), 1);
        assert_eq!(ExactFloat::new(-12, 0).numerator_digit_count(), 2);
        assert_eq!(ExactFloat::new(-123, 0).numerator_digit_count(), 3);
        assert_eq!(ExactFloat::new(-1234, 0).numerator_digit_count(), 4);
        assert_eq!(ExactFloat::new(-12345, 0).numerator_digit_count(), 5);
        assert_eq!(ExactFloat::new(-123456, 0).numerator_digit_count(), 6);
        assert_eq!(ExactFloat::new(-1234567, 0).numerator_digit_count(), 7);
        assert_eq!(ExactFloat::new(-12345678, 0).numerator_digit_count(), 8);
        assert_eq!(ExactFloat::new(-123456789, 0).numerator_digit_count(), 9);
        assert_eq!(ExactFloat::new(-1234567890, 0).numerator_digit_count(), 10);
    }
}

mod abs {
    use super::*;

    #[test]
    fn it_should_do_nothing_for_positive_number() {
        assert_eq!(ExactFloat::new(1, 0).abs(), ExactFloat::new(1, 0));
        assert_eq!(ExactFloat::new(100, 0).abs(), ExactFloat::new(100, 0));
        assert_eq!(ExactFloat::new(1, 55).abs(), ExactFloat::new(1, 55));
        assert_eq!(ExactFloat::new(1000, 55).abs(), ExactFloat::new(1000, 55));
    }

    #[test]
    fn it_should_drop_convert_to_positive_number_when_negative() {
        assert_eq!(ExactFloat::new(-1, 0).abs(), ExactFloat::new(1, 0));
        assert_eq!(ExactFloat::new(-100, 0).abs(), ExactFloat::new(100, 0));
        assert_eq!(ExactFloat::new(-1, 55).abs(), ExactFloat::new(1, 55));
        assert_eq!(ExactFloat::new(-1000, 55).abs(), ExactFloat::new(1000, 55));
    }
}

mod sign {
    use super::*;

    #[test]
    fn it_should_return_positive_sign() {
        assert_eq!(ExactFloat::new(1, 0).sign(), "+");
    }

    #[test]
    fn it_should_return_negative_sign() {
        assert_eq!(ExactFloat::new(-1, 0).sign(), "-");
    }
}
