//! This module implements the global `Int8Array` object.

use super::allocate_typed_array;
use crate::{
    builtins::typed_array::TypedArrayName, builtins::BuiltIn, object::ConstructorBuilder,
    property::Attribute, symbol::WellKnownSymbols, value::JsValue, BoaProfiler, Context, JsResult,
};

/// JavaScript `Array` built-in implementation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Int8Array;

impl BuiltIn for Int8Array {
    const NAME: &'static str = "Int8Array";

    fn attribute() -> Attribute {
        Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::CONFIGURABLE
    }

    fn init(context: &mut Context) -> (&'static str, JsValue, Attribute) {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        let int8_array = ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().typed_array_object().clone(),
        )
        .name(Self::NAME)
        .length(Self::LENGTH)
        .build();

        todo!()
    }
}

impl Int8Array {
    const LENGTH: usize = 3;

    /// <https://tc39.es/ecma262/#sec-typedarray>
    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        // 1. If NewTarget is undefined, throw a TypeError exception.
        if new_target.is_undefined() {
            return context
                .throw_type_error("new target was undefined when constructing an Int8Array");
        }

        // 2. Let constructorName be the String value of the Constructor Name value specified in Table 72 for this TypedArray constructor.
        let constructor_name = TypedArrayName::Int8Array;

        // 3. Let proto be "%TypedArray.prototype%".
        let proto = || context.standard_objects().typed_array_object().prototype();

        // 4. Let numberOfArgs be the number of elements in args.
        let number_of_args = args.len();

        // 5. If numberOfArgs = 0, then
        if number_of_args == 0 {
            // a. Return ? AllocateTypedArray(constructorName, NewTarget, proto, 0).
            return allocate_typed_array(constructor_name, new_target, proto, Some(0), context);
        }
        // 6. Else,

        // a. Let firstArgument be args[0].
        let first_argument = args[0];

        // b. If Type(firstArgument) is Object, then
        if let Some(first_argument) = first_argument.as_object() {
            // i. Let O be ? AllocateTypedArray(constructorName, NewTarget, proto).
            let o = allocate_typed_array(constructor_name, new_target, proto, None, context)?;

            // ii. If firstArgument has a [[TypedArrayName]] internal slot, then
            if first_argument.is_typed_array() {
                //     1. Perform ? InitializeTypedArrayFromTypedArray(O, firstArgument).
                // TODO: InitializeTypedArrayFromTypedArray
            } else if first_argument.is_array_buffer() {
                // iii. Else if firstArgument has an [[ArrayBufferData]] internal slot, then

                // 1. If numberOfArgs > 1, let byteOffset be args[1]; else let byteOffset be undefined.
                // 2. If numberOfArgs > 2, let length be args[2]; else let length be undefined.
                // 3. Perform ? InitializeTypedArrayFromArrayBuffer(O, firstArgument, byteOffset, length).
            } else {
                // iv. Else,

                // 1. Assert: Type(firstArgument) is Object and firstArgument does not have either a [[TypedArrayName]] or an [[ArrayBufferData]] internal slot.

                // 2. Let usingIterator be ? GetMethod(firstArgument, @@iterator).
                let using_iterator = JsValue::from(first_argument)
                    .get_method(context, WellKnownSymbols::replace())?;

                // 3. If usingIterator is not undefined, then
                if !using_iterator.is_undefined() {
                    // a. Let values be ? IterableToList(firstArgument, usingIterator).
                    // b. Perform ? InitializeTypedArrayFromList(O, values).
                    // TODO: InitializeTypedArrayFromList
                } else {
                    // 4. Else,

                    // a. NOTE: firstArgument is not an Iterable so assume it is already an array-like object.
                    // b. Perform ? InitializeTypedArrayFromArrayLike(O, firstArgument).
                    // TODO: InitializeTypedArrayFromArrayLike
                }
            }

            // v. Return O.
            Ok(o)
        } else {
            // c. Else,

            // i. Assert: firstArgument is not an Object.
            assert!(!first_argument.is_object(), "firstArgument was an object");

            // ii. Let elementLength be ? ToIndex(firstArgument).
            let element_length = first_argument.to_index(context)?;

            // iii. Return ? AllocateTypedArray(constructorName, NewTarget, proto, elementLength).
            allocate_typed_array(
                constructor_name,
                new_target,
                proto,
                Some(element_length),
                context,
            )
        }
    }
}
