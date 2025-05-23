#![cfg(test)]

use facet_core::{Type, UserType};
use facet_reflect::Wip;

use facet::Facet;
use facet_args::{defaults::apply_field_defaults, parse_field};

#[test]
fn test_apply_field_defaults() {
    facet_testhelpers::setup();

    // Test with a struct with default values
    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct {
        #[facet(default = 42)]
        number: u32,
        flag: bool, // No default, should be untouched
    }

    // Create a Wip instance for TestStruct
    let wip = Wip::alloc::<TestStruct>().unwrap();

    // Apply the defaults
    let wip = apply_field_defaults(wip).unwrap();

    // Check if the default was applied to 'number' field
    let Type::User(UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Find the index of the 'number' field
    let number_index = st.fields.iter().position(|f| f.name == "number").unwrap();

    // Check if the 'number' field was initialized
    assert!(
        wip.is_field_set(number_index).unwrap(),
        "number field should be initialized"
    );

    // Check if the 'flag' field was left uninitialized (we handle booleans separately)
    let flag_index = st.fields.iter().position(|f| f.name == "flag").unwrap();
    assert!(
        !wip.is_field_set(flag_index).unwrap(),
        "flag field should not be initialized yet"
    );

    // Complete the boolean field initialization as done in from_slice
    let field = wip.field(flag_index).unwrap();
    let wip = parse_field(field, "false").unwrap();

    // Build the final value and verify
    let heap_value = wip.build().unwrap();
    let test_struct: TestStruct = heap_value.materialize().unwrap();

    assert_eq!(
        test_struct.number, 42,
        "number field should have default value 42"
    );
    assert!(!test_struct.flag, "flag field should be false");
}

#[test]
fn test_apply_field_defaults_with_custom_default() {
    facet_testhelpers::setup();

    fn get_custom_default() -> String {
        "custom default".to_string()
    }

    #[derive(Facet, Debug, PartialEq)]
    struct TestStructWithCustomDefault {
        #[facet(default = 100)]
        number: u32,
        #[facet(default = get_custom_default())]
        text: String,
        flag: bool, // No default, should be untouched
    }

    // Create a Wip instance
    let wip = Wip::alloc::<TestStructWithCustomDefault>().unwrap();

    // Apply the defaults
    let wip = apply_field_defaults(wip).unwrap();

    // Check if the defaults were applied to non-boolean fields
    let Type::User(UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Find field indices
    let number_index = st.fields.iter().position(|f| f.name == "number").unwrap();
    let text_index = st.fields.iter().position(|f| f.name == "text").unwrap();
    let flag_index = st.fields.iter().position(|f| f.name == "flag").unwrap();

    // Verify fields were initialized correctly
    assert!(
        wip.is_field_set(number_index).unwrap(),
        "number field should be initialized"
    );
    assert!(
        wip.is_field_set(text_index).unwrap(),
        "text field should be initialized"
    );
    assert!(
        !wip.is_field_set(flag_index).unwrap(),
        "flag field should not be initialized yet"
    );

    // Complete the boolean field initialization
    let field = wip.field(flag_index).unwrap();
    let wip = parse_field(field, "false").unwrap();

    // Build the final value and verify
    let heap_value = wip.build().unwrap();
    let test_struct: TestStructWithCustomDefault = heap_value.materialize().unwrap();

    assert_eq!(
        test_struct.number, 100,
        "number field should have default value 100"
    );
    assert_eq!(
        test_struct.text, "custom default",
        "text field should have custom default value"
    );
    assert!(!test_struct.flag, "flag field should be false");
}
