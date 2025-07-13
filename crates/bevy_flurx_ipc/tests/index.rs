//! Test process

#[cfg(test)]
#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/test01_action_command.rs");
    t.pass("tests/test02_async_command.rs");
    t.pass("tests/test03_custom_id.rs");
    t.pass("tests/test04_return_result.rs");
}
