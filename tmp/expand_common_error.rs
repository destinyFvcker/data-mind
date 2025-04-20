#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use common_error::mock::MockError;
use common_macro::stack_trace_debug;
use serde::{Deserialize, Serialize};
use snafu::{Location, ResultExt, Snafu};
enum SimpleError {
    #[snafu(display("Failed to deserialize value"))]
    ValueDeserialize {
        #[snafu(source)]
        error: serde_json::error::Error,
        #[snafu(implicit)]
        location: Location,
    },
    #[allow(unused)]
    #[snafu(display("Table engine not found: {}", engine_name))]
    TableEngineNotFound {
        engine_name: String,
        #[snafu(implicit)]
        location: Location,
        source: common_error::mock::MockError,
    },
}
impl ::common_error::ext::StackError for SimpleError {
    fn debug_fmt(&self, layer: usize, buf: &mut Vec<String>) {
        use SimpleError::*;
        match self {
            #[allow(unused_variables)]
            ValueDeserialize { error, location } => {
                buf.push(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{2}: {0}, at {1}",
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to deserialize value"),
                                    );
                                    res
                                }),
                                location,
                                layer,
                            ),
                        );
                        res
                    }),
                );
                buf.push(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}: {1:?}", layer + 1, error),
                        );
                        res
                    }),
                );
            }
            #[allow(unused_variables)]
            TableEngineNotFound { engine_name, location, source } => {
                buf.push(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{2}: {0}, at {1}",
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Table engine not found: {0}", engine_name),
                                    );
                                    res
                                }),
                                location,
                                layer,
                            ),
                        );
                        res
                    }),
                );
                source.debug_fmt(layer + 1, buf);
            }
        }
    }
    fn next(&self) -> Option<&dyn ::common_error::ext::StackError> {
        use SimpleError::*;
        match self {
            #[allow(unused_variables)]
            ValueDeserialize { error, location } => None,
            #[allow(unused_variables)]
            TableEngineNotFound { engine_name, location, source } => Some(source),
        }
    }
    fn transparent(&self) -> bool {
        use SimpleError::*;
        match self {
            #[allow(unused_variables)]
            ValueDeserialize { error, location } => false,
            #[allow(unused_variables)]
            TableEngineNotFound { engine_name, location, source } => false,
        }
    }
}
impl std::fmt::Debug for SimpleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ::common_error::ext::StackError;
        let mut buf = ::alloc::vec::Vec::new();
        self.debug_fmt(0, &mut buf);
        f.write_fmt(format_args!("{0}", buf.join("\n")))
    }
}
///SNAFU context selector for the `SimpleError::ValueDeserialize` variant
struct ValueDeserializeSnafu;
#[automatically_derived]
impl ::core::fmt::Debug for ValueDeserializeSnafu {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(f, "ValueDeserializeSnafu")
    }
}
#[automatically_derived]
impl ::core::marker::Copy for ValueDeserializeSnafu {}
#[automatically_derived]
impl ::core::clone::Clone for ValueDeserializeSnafu {
    #[inline]
    fn clone(&self) -> ValueDeserializeSnafu {
        *self
    }
}
impl ::snafu::IntoError<SimpleError> for ValueDeserializeSnafu
where
    SimpleError: ::snafu::Error + ::snafu::ErrorCompat,
{
    type Source = serde_json::error::Error;
    #[track_caller]
    fn into_error(self, error: Self::Source) -> SimpleError {
        let error: serde_json::error::Error = (|v| v)(error);
        SimpleError::ValueDeserialize {
            location: {
                use ::snafu::AsErrorSource;
                let error = error.as_error_source();
                ::snafu::GenerateImplicitData::generate_with_source(error)
            },
            error: error,
        }
    }
}
///SNAFU context selector for the `SimpleError::TableEngineNotFound` variant
struct TableEngineNotFoundSnafu<__T0> {
    #[allow(missing_docs)]
    engine_name: __T0,
}
#[automatically_derived]
impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for TableEngineNotFoundSnafu<__T0> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "TableEngineNotFoundSnafu",
            "engine_name",
            &&self.engine_name,
        )
    }
}
#[automatically_derived]
impl<__T0: ::core::marker::Copy> ::core::marker::Copy
for TableEngineNotFoundSnafu<__T0> {}
#[automatically_derived]
impl<__T0: ::core::clone::Clone> ::core::clone::Clone
for TableEngineNotFoundSnafu<__T0> {
    #[inline]
    fn clone(&self) -> TableEngineNotFoundSnafu<__T0> {
        TableEngineNotFoundSnafu {
            engine_name: ::core::clone::Clone::clone(&self.engine_name),
        }
    }
}
impl<__T0> ::snafu::IntoError<SimpleError> for TableEngineNotFoundSnafu<__T0>
where
    SimpleError: ::snafu::Error + ::snafu::ErrorCompat,
    __T0: ::core::convert::Into<String>,
{
    type Source = common_error::mock::MockError;
    #[track_caller]
    fn into_error(self, error: Self::Source) -> SimpleError {
        let error: common_error::mock::MockError = (|v| v)(error);
        SimpleError::TableEngineNotFound {
            location: {
                use ::snafu::AsErrorSource;
                let error = error.as_error_source();
                ::snafu::GenerateImplicitData::generate_with_source(error)
            },
            source: error,
            engine_name: ::core::convert::Into::into(self.engine_name),
        }
    }
}
#[allow(single_use_lifetimes)]
impl ::core::fmt::Display for SimpleError {
    fn fmt(
        &self,
        __snafu_display_formatter: &mut ::core::fmt::Formatter,
    ) -> ::core::fmt::Result {
        #[allow(unused_variables)]
        match *self {
            SimpleError::ValueDeserialize { ref error, ref location } => {
                __snafu_display_formatter
                    .write_fmt(format_args!("Failed to deserialize value"))
            }
            SimpleError::TableEngineNotFound {
                ref engine_name,
                ref location,
                ref source,
            } => {
                __snafu_display_formatter
                    .write_fmt(format_args!("Table engine not found: {0}", engine_name))
            }
        }
    }
}
#[allow(single_use_lifetimes)]
impl ::snafu::Error for SimpleError
where
    Self: ::core::fmt::Debug + ::core::fmt::Display,
{
    fn description(&self) -> &str {
        match *self {
            SimpleError::ValueDeserialize { .. } => "SimpleError :: ValueDeserialize",
            SimpleError::TableEngineNotFound { .. } => {
                "SimpleError :: TableEngineNotFound"
            }
        }
    }
    fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
        use ::snafu::AsErrorSource;
        match *self {
            SimpleError::ValueDeserialize { ref error, .. } => {
                ::core::option::Option::Some(error.as_error_source())
            }
            SimpleError::TableEngineNotFound { ref source, .. } => {
                ::core::option::Option::Some(source.as_error_source())
            }
        }
    }
    fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
        use ::snafu::AsErrorSource;
        match *self {
            SimpleError::ValueDeserialize { ref error, .. } => {
                ::core::option::Option::Some(error.as_error_source())
            }
            SimpleError::TableEngineNotFound { ref source, .. } => {
                ::core::option::Option::Some(source.as_error_source())
            }
        }
    }
}
#[allow(single_use_lifetimes)]
impl ::snafu::ErrorCompat for SimpleError {
    fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
        match *self {
            SimpleError::ValueDeserialize { .. } => ::core::option::Option::None,
            SimpleError::TableEngineNotFound { .. } => ::core::option::Option::None,
        }
    }
}
struct SimpleStruct {
    filed: String,
}
#[automatically_derived]
impl ::core::fmt::Debug for SimpleStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "SimpleStruct",
            "filed",
            &&self.filed,
        )
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SimpleStruct {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "filed" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"filed" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SimpleStruct>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimpleStruct;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SimpleStruct",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SimpleStruct with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(SimpleStruct { filed: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("filed"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("filed")?
                        }
                    };
                    _serde::__private::Ok(SimpleStruct { filed: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["filed"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SimpleStruct",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimpleStruct>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SimpleStruct {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SimpleStruct",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "filed",
                &self.filed,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
async fn decode_msg(msg: &[u8]) -> Result<SimpleStruct, SimpleError> {
    serde_json::from_slice(&msg).context(ValueDeserializeSnafu)
}
fn internal_fail() -> Result<(), SimpleError> {
    let error = MockError::new(common_error::status_code::StatusCode::InvalidArguments);
    Err(error)
        .context(TableEngineNotFoundSnafu {
            engine_name: "engine name".to_owned(),
        })
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_common_macro_external"]
#[doc(hidden)]
pub const test_common_macro_external: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_common_macro_external"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "_web-server/tests/test_error.rs",
        start_line: 44usize,
        start_col: 10usize,
        end_line: 44usize,
        end_col: 36usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_common_macro_external()),
    ),
};
fn test_common_macro_external() {
    <::actix_web::rt::System>::new()
        .block_on(async {
            {
                let simple1 = SimpleStruct {
                    filed: "simple struct 1".to_string(),
                };
                let simple1_json = serde_json::to_string(&simple1).unwrap();
                let simple1_bytes = simple1_json.bytes().collect::<Vec<u8>>();
                let result1 = decode_msg(&simple1_bytes).await;
                match result1 {
                    Ok(struc) => {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", struc));
                        };
                    }
                    Err(err) => {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", err));
                        };
                    }
                }
                let result2 = decode_msg(&simple1_bytes[1..]).await;
                match result2 {
                    Ok(struc) => {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", struc));
                        };
                    }
                    Err(err) => {
                        {
                            ::std::io::_print(format_args!("{0:?}\n", err));
                        };
                    }
                }
            }
        })
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "test_common_macro_internal"]
#[doc(hidden)]
pub const test_common_macro_internal: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_common_macro_internal"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "_web-server/tests/test_error.rs",
        start_line: 75usize,
        start_col: 4usize,
        end_line: 75usize,
        end_col: 30usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        #[coverage(off)]
        || test::assert_test_result(test_common_macro_internal()),
    ),
};
fn test_common_macro_internal() {
    let result = internal_fail();
    if let Err(err) = result {
        {
            ::std::io::_print(format_args!("{0:?}\n", err));
        };
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_common_macro_external, &test_common_macro_internal])
}
