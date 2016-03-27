pub fn fizz_buzz(x: i32) -> String {

    if x % 3 == 0 {
        if x % 5 == 0 {
            String::from("fizzbuzz")
        }
        else {
            String::from("fizz")
        }
    }
    else if x % 5 == 0 {
        String::from("buzz")
    }
    else {
        x.to_string()
    }
}

#[test]
fn fizz_buzz_returns_fizz_for_3() {

    let output = fizz_buzz(3);

    assert_eq!("fizz", output);
}

#[test]
fn fizz_buzz_returns_buzz_for_5() {

    let output = fizz_buzz(5);

    assert_eq!("buzz", output);
}

#[test]
fn fizz_buzz_returns_number_when_passed_value_not_divisible_by_3_or_5() {

    let output = fizz_buzz(1);

    assert_eq!("1", output);
}

#[test]
fn fizz_buzz_returns_fizzbuzz_when_passed_15() {

    let output = fizz_buzz(15);

    assert_eq!("fizzbuzz", output);
}

#[test]
fn fizz_buzz_returns_correct_values_for_1_to_100() {

    for i in 1..100 {
        let output = fizz_buzz(i);
        let expected = {
            if i % 3 == 0 {
                if i % 5 == 0 {
                    "fizzbuzz".to_string()
                }
                else {
                    "fizz".to_string()
                }
            }
            else if i % 5 == 0 {
                "buzz".to_string()
            }
            else {
                i.to_string()
            }
        };

        assert_eq!(expected, output);
    }
}
