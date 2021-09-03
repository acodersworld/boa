//! <https://tc39.es/ecma262/#sec-typedarray-objects>
use crate::{
    object::{ConstructorBuilder, JsObject, PROTOTYPE},
    property::Attribute,
    value::JsValue,
    BoaProfiler, Context, JsResult,
};
use integer_indexed_object::IntegerIndexedObject;
pub use integer_indexed_object::IntegerIndexedObject;

pub mod int8_array;
mod integer_indexed_object;

/// The JavaScript `%TypedArray%` object.
///
/// <https://tc39.es/ecma262/#sec-%typedarray%-intrinsic-object>
#[derive(Debug, Clone, Copy)]
struct TypedArray;

impl TypedArray {
    const NAME: &'static str = "TypedArray";
    const LENGTH: usize = 0;

    fn init(context: &mut Context) -> JsObject {
        let _timer = BoaProfiler::global().start_event(Self::NAME, "init");

        ConstructorBuilder::with_standard_object(
            context,
            Self::constructor,
            context.standard_objects().function_object().clone(),
        )
        .name(Self::NAME)
        .length(Self::LENGTH)
        .property(
            "length",
            0,
            Attribute::WRITABLE | Attribute::NON_ENUMERABLE | Attribute::PERMANENT,
        )
        .build()
    }

    /// <https://tc39.es/ecma262/#sec-%typedarray%>
    fn constructor(
        new_target: &JsValue,
        args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        // 1. Throw a TypeError exception.
        context.throw_type_error("the TypedArray constructor should never be called directly")
    }
}

/// `AllocateTypedArray ( constructorName, newTarget, defaultProto [ , length ] )`
///
/// It is used to validate and create an instance of a `TypedArray` constructor. If the `length`
/// argument is passed, an `ArrayBuffer` of that length is also allocated and associated with the
/// new `TypedArray` instance. `AllocateTypedArray` provides common semantics that is used by
/// `TypedArray`.
///
/// For more information, check the [spec][spec].
///
/// [spec]: https://tc39.es/ecma262/#sec-allocatetypedarray
fn allocate_typed_array<P>(
    constructor_name: TypedArrayName,
    new_target: &JsValue,
    default_proto: P,
    length: Option<usize>,
    context: &mut Context,
) -> JsResult<JsValue>
where
    P: FnOnce() -> JsObject,
{
    // 1. Let proto be ? GetPrototypeFromConstructor(newTarget, defaultProto).
    let proto = new_target
        .as_object()
        .and_then(|obj| {
            obj.__get__(&PROTOTYPE.into(), obj.clone().into(), context)
                .map(|o| o.as_object())
                .transpose()
        })
        .transpose()?
        .unwrap_or_else(default_proto);

    // 2. Let obj be ! IntegerIndexedObjectCreate(proto).
    // 3. Assert: obj.[[ViewedArrayBuffer]] is undefined.
    // 4. Set obj.[[TypedArrayName]] to constructorName.
    let obj = match length {
        // 7. If length is not present, then
        None => IntegerIndexedObject::new(proto, constructor_name),
        // 8. Else,
        Some(length) => {
            // a. Perform ? AllocateTypedArrayBuffer(obj, length).
            IntegerIndexedObject::allocate_typed_array_buffer(proto, constructor_name, length)?
        }
    };

    // 9. Return obj.
    Ok(obj.into())
}

/// Names of all the typed arrays.
#[derive(Debug, Clone, Copy)]
enum TypedArrayName {
    Int8Array,
    Uint8Array,
    Uint8ClampedArray,
    Int16Array,
    Uint16Array,
    Int32Array,
    Uint32Array,
    BigInt64Array,
    BigUint64Array,
    Float32Array,
    Float64Array,
}

impl TypedArrayName {
    /// Gets the element size of the given typed array name, as per the [spec].
    ///
    /// [spec]: https://tc39.es/ecma262/#table-the-typedarray-constructors
    fn element_size(self) -> usize {
        match self {
            Self::Int8Array | Self::Uint8Array | Self::Uint8ClampedArray => 1,
            Self::Int16Array | Self::Uint16Array => 2,
            Self::Int32Array | Self::Uint32Array | Self::Float32Array => 4,
            Self::BigInt64Array | Self::BigUint64Array | Self::Float64Array => 8,
        }
    }
}
