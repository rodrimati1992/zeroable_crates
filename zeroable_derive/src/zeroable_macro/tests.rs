use super::derive;

use regex::Regex;

fn derive_from_str(s: &str) -> Result<String, String> {
    match syn::parse_str(s).and_then(derive) {
        Ok(x) => Ok(x.to_string()),
        Err(e) => Err(e.to_compile_error().to_string()),
    }
}

#[derive(Debug)]
struct TestCase {
    code: String,
    has_errors: bool,
    expected: String,
}

#[test]
fn test_compilation() {
    let testcases = vec![
        TestCase {
            code: "
                enum Hello{
                    A
                }
            "
            .to_string(),
            has_errors: true,
            expected: r#"Expected .*#\[repr\("#.into(),
        },
        TestCase {
            code: "
                enum Hello{}
            "
            .to_string(),
            has_errors: true,
            expected: r#"cannot implement Zeroable"#.into(),
        },
        TestCase {
            code: "
                #[repr(C)]
                enum Hello{
                    A=10,
                }
            "
            .to_string(),
            has_errors: true,
            expected: r#"0.*discriminant"#.into(),
        },
        TestCase {
            code: "
                #[zero(nonzero_fields)]
                union Hello{
                    a:u32,
                }
            "
            .to_string(),
            has_errors: true,
            expected: r#"Expected.*zeroable field"#.into(),
        },
        TestCase {
            code: "
                union Hello{}
            "
            .to_string(),
            has_errors: true,
            expected: r#"cannot implement Zeroable"#.into(),
        },
        TestCase {
            code: "
                #[repr(transparent)]
                union Hello{
                    a:()
                }
            "
            .to_string(),
            has_errors: false,
            expected: r#"impl.*Zeroable.*for.*Hello"#.into(),
        },
        TestCase {
            code: "
                struct Hello{}
            "
            .to_string(),
            has_errors: false,
            expected: r#"impl.*Zeroable.*for.*Hello"#.into(),
        },
    ];

    let mut errors = Vec::new();

    for example in testcases.iter() {
        let expected = Regex::new(&example.expected).unwrap();

        let (is_error, output) = match derive_from_str(&example.code) {
            Ok(v) => (false, v),
            Err(e) => (true, e),
        };

        if example.has_errors != is_error || !expected.is_match(&output) {
            errors.push((output, example));
        }
    }

    if !errors.is_empty() {
        panic!("{:#?}", errors);
    }
}
