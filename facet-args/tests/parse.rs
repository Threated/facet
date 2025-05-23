#![cfg(test)]

use eyre::{Ok, Result};
use facet::Facet;
use facet_args::parse::{parse_named_arg, parse_positional_arg, parse_short_arg};
use facet_reflect::Wip;

#[test]
fn test_parse_named_arg() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct {
        #[facet(named)]
        text: String,
        #[facet(named)]
        flag: bool,
        #[facet(named)]
        count: u32,
    }

    // Test parsing a named string argument
    let wip = Wip::alloc::<TestStruct>()?;
    let mut args = &["value_for_text"][..];
    let wip = parse_named_arg(wip, "text", &mut args)?;

    // Test parsing a named boolean argument
    let wip = parse_named_arg(wip, "flag", &mut args)?;

    // Test parsing a named numeric argument
    let mut args = &["42"][..];
    let wip = parse_named_arg(wip, "count", &mut args)?;

    // Build and verify
    let heap_value = wip.build()?;
    let test_struct: TestStruct = heap_value.materialize()?;

    assert_eq!(test_struct.text, "value_for_text");
    assert!(test_struct.flag);
    assert_eq!(test_struct.count, 42);

    Ok(())
}

#[test]
fn test_parse_named_arg_errors() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug)]
    struct TestStruct {
        #[facet(named)]
        value: String,
    }

    // Test unknown argument error
    let wip = Wip::alloc::<TestStruct>()?;
    let mut args = &["some_value"][..];
    let result = parse_named_arg(wip, "unknown_field", &mut args);
    assert!(result.is_err());

    // Check the error message without using unwrap_err()
    if let Err(err) = result {
        assert_eq!(
            err.message(),
            "Args error: Unknown argument `unknown_field`"
        );
    } else {
        panic!("Expected an error but got Ok");
    }

    // Test missing value error
    let wip = Wip::alloc::<TestStruct>()?;
    let mut args = &[][..]; // Empty args
    let result = parse_named_arg(wip, "value", &mut args);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(
            err.message(),
            "Args error: expected value after argument `value`"
        );
    } else {
        panic!("Expected an error but got Ok");
    }

    Ok(())
}

#[test]
fn test_parse_short_arg() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct {
        #[facet(named, short = 'v')]
        verbose: bool,
        #[facet(named, short = 'c')]
        count: u32,
    }

    // Get the struct type for testing
    let wip = Wip::alloc::<TestStruct>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Test parsing a short boolean flag
    let wip = parse_short_arg(wip, "v", &mut &[][..], &st)?;

    // Test parsing a short numeric argument
    let mut args = &["42"][..];
    let wip = parse_short_arg(wip, "c", &mut args, &st)?;

    // Build and verify
    let heap_value = wip.build()?;
    let test_struct: TestStruct = heap_value.materialize()?;

    assert!(test_struct.verbose);
    assert_eq!(test_struct.count, 42);

    Ok(())
}

#[test]
fn test_parse_short_arg_errors() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug)]
    struct TestStruct {
        #[facet(named, short = 'c')]
        count: u32,
    }

    // Get the struct type for testing
    let wip = Wip::alloc::<TestStruct>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Test unknown short argument
    let result = parse_short_arg(wip, "x", &mut &[][..], &st);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(err.message(), "Args error: Unknown short argument `-x`");
    } else {
        panic!("Expected an error but got Ok");
    }

    // Test missing value for short argument
    let wip = Wip::alloc::<TestStruct>()?;
    let mut args = &[][..]; // Empty args
    let result = parse_short_arg(wip, "c", &mut args, &st);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(
            err.message(),
            "Args error: expected value after argument `c`"
        );
    } else {
        panic!("Expected an error but got Ok");
    }

    Ok(())
}

#[test]
fn test_parse_positional_arg() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct {
        #[facet(positional)]
        path: String,
        #[facet(positional)]
        count: u32,
    }

    // Get the struct type for testing
    let wip = Wip::alloc::<TestStruct>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Test parsing first positional argument
    let wip = parse_positional_arg(wip, "test.rs", &st)?;

    // Test parsing second positional argument
    let wip = parse_positional_arg(wip, "42", &st)?;

    // Build and verify
    let heap_value = wip.build()?;
    let test_struct: TestStruct = heap_value.materialize()?;

    assert_eq!(test_struct.path, "test.rs");
    assert_eq!(test_struct.count, 42);

    Ok(())
}

#[test]
fn test_parse_positional_arg_errors() -> Result<()> {
    facet_testhelpers::setup();

    // Struct without positional args
    #[derive(Facet, Debug)]
    struct TestStructNoPositional {
        #[facet(named)]
        value: String,
    }

    // Get the struct type for testing
    let wip = Wip::alloc::<TestStructNoPositional>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Test no positional argument available
    let result = parse_positional_arg(wip, "test.rs", &st);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(
            err.message(),
            "Args error: No positional argument field available for token `test.rs`"
        );
    } else {
        panic!("Expected an error but got Ok");
    }

    // Struct with one positional arg already set
    #[derive(Facet, Debug)]
    struct TestStructOnePositional {
        #[facet(positional)]
        path: String,
    }

    // Create and set the positional field
    let wip = Wip::alloc::<TestStructOnePositional>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Set the positional field
    let wip = parse_positional_arg(wip, "first.rs", &st)?;

    // Now try to add another positional argument which should fail
    let result = parse_positional_arg(wip, "second.rs", &st);
    assert!(result.is_err());

    if let Err(err) = result {
        assert_eq!(
            err.message(),
            "Args error: No positional argument field available for token `second.rs`"
        );
    } else {
        panic!("Expected an error but got Ok");
    }

    Ok(())
}

#[test]
fn test_parse_multiple_positional_args() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct<'a> {
        #[facet(positional)]
        path: String,
        #[facet(positional)]
        path_borrow: &'a str,
    }

    // Get the struct type for testing
    let wip = Wip::alloc::<TestStruct>()?;
    let facet_core::Type::User(facet_core::UserType::Struct(st)) = wip.shape().ty else {
        panic!("Expected struct type");
    };

    // Parse first positional arg
    let wip = parse_positional_arg(wip, "example.rs", &st)?;

    // Parse second positional arg
    let wip = parse_positional_arg(wip, "test.rs", &st)?;

    // Build and verify
    let heap_value = wip.build()?;
    let test_struct: TestStruct = heap_value.materialize()?;

    assert_eq!(test_struct.path, "example.rs");
    assert_eq!(test_struct.path_borrow, "test.rs");

    Ok(())
}

#[test]
fn test_parse_different_arg_types() -> Result<()> {
    facet_testhelpers::setup();

    #[derive(Facet, Debug, PartialEq)]
    struct TestStruct {
        #[facet(named)]
        string: String,
        #[facet(named)]
        number: u32,
        #[facet(named)]
        flag: bool,
    }

    // Test with different argument types
    let wip = Wip::alloc::<TestStruct>()?;

    // Parse string arg
    let mut args = &["hello"][..];
    let wip = parse_named_arg(wip, "string", &mut args)?;

    // Parse numeric arg
    let mut args = &["42"][..];
    let wip = parse_named_arg(wip, "number", &mut args)?;

    // Parse boolean arg
    let wip = parse_named_arg(wip, "flag", &mut &[][..])?;

    // Build and verify
    let heap_value = wip.build()?;
    let test_struct: TestStruct = heap_value.materialize()?;

    assert_eq!(test_struct.string, "hello");
    assert_eq!(test_struct.number, 42);
    assert!(test_struct.flag);

    Ok(())
}
