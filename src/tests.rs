use learning_rust::{self, Config};

// #[test]
// fn config_length_invalid_path() {
//     let args = vec!(
//         String::from("dummy_directory"),
//         String::from("fake_save"),
//         );
//     let config = Config::build(args);

//     assert_err!(config);
// }

#[test]
fn config_length_default() {
    let args = vec!(
        String::from("dummy_directory"),
        );
    let result = Config::build(&args);
    let config = result.expect("expected default config to not error");

    assert_eq!(config.save, String::from("default"));
}

#[test]
fn config_length_ok() {
    let args = vec!(
        String::from("dummy_directory"),
        String::from("default"),
        );
    let result = Config::build(&args);
    let config = result.expect("expected valid config to not error");

    assert_eq!(config.save, String::from("default"));
}
