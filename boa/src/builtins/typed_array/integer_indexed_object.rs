use super::TypedArrayName;
use crate::{object::JsObject, JsResult};

/// Type of the array content.
#[derive(Debug, Clone, Copy)]
pub(super) enum ContentType {
    Number,
    BigInt,
}

/// <https://tc39.es/ecma262/#integer-indexed-exotic-object>
#[derive(Debug, Clone)]
pub struct IntegerIndexedObject {
    pub(super) prototype: JsObject,
    pub(super) viewed_array_buffer: DataBlock,
    pub(super) typed_array_name: TypedArrayName,
    pub(super) content_type: ContentType,
    pub(super) byte_offset: usize,
    pub(super) byte_length: usize,
    pub(super) array_length: usize,
}

impl IntegerIndexedObject {
    /// <https://tc39.es/ecma262/#sec-integerindexedobjectcreate>
    pub(super) fn new(prototype: JsObject, constructor_name: TypedArrayName) -> Self {
        let content_type = match constructor_name {
            // 5. If constructorName is "BigInt64Array" or "BigUint64Array", set obj.[[ContentType]] to BigInt.
            TypedArrayName::BigInt64Array | TypedArrayName::BigUint64Array => ContentType::BigInt,
            // 6. Otherwise, set obj.[[ContentType]] to Number.
            _ => ContentType::Number,
        };

        Self {
            prototype,
            viewed_array_buffer: Default::default(),
            typed_array_name: constructor_name,
            content_type,
            // a. Set obj.[[ByteLength]] to 0.
            byte_length: 0,
            // b. Set obj.[[ByteOffset]] to 0.
            byte_offset: 0,
            // c. Set obj.[[ArrayLength]] to 0.
            array_length: 0,
        }
    }

    /// <https://tc39.es/ecma262/#sec-allocatetypedarraybuffer>
    pub(super) fn allocate_typed_array_buffer(
        prototype: JsObject,
        constructor_name: TypedArrayName,
        length: usize,
    ) -> JsResult<Self> {
        let content_type = match constructor_name {
            // 5. If constructorName is "BigInt64Array" or "BigUint64Array", set obj.[[ContentType]] to BigInt.
            TypedArrayName::BigInt64Array | TypedArrayName::BigUint64Array => ContentType::BigInt,
            // 6. Otherwise, set obj.[[ContentType]] to Number.
            _ => ContentType::Number,
        };

        // 1. Assert: O.[[ViewedArrayBuffer]] is undefined.
        // 2. Let constructorName be the String value of O.[[TypedArrayName]].

        // 3. Let elementSize be the Element Size value specified in Table 72 for constructorName.
        let element_size = constructor_name.element_size();

        // 4. Let byteLength be elementSize × length.
        let byte_length = element_size * length;

        // 5. Let data be ? AllocateArrayBuffer(%ArrayBuffer%, byteLength).
        // TODO: AllocateArrayBuffer
        let data = DataBlock::create_byte_data_block(byte_length)?;

        Ok(Self {
            prototype,
            typed_array_name: constructor_name,
            content_type,
            // 6. Set O.[[ViewedArrayBuffer]] to data.
            viewed_array_buffer: data,
            // 7. Set O.[[ByteLength]] to byteLength.
            byte_length,
            // 8. Set O.[[ByteOffset]] to 0.
            byte_offset: 0,
            // 9. Set O.[[ArrayLength]] to length.
            array_length: length,
        })
    }
}

/// A Data Block
///
/// The Data Block specification type is used to describe a distinct and mutable sequence of
/// byte-sized (8 bit) numeric values. A byte value is an integer value in the range `0` through
/// `255`, inclusive. A Data Block value is created with a fixed number of bytes that each have
/// the initial value `0`.
///
/// For more information, check the [spec][spec].
///
/// [spec]: https://tc39.es/ecma262/#sec-data-blocks
#[derive(Debug, Clone, Default)]
pub struct DataBlock {
    inner: Vec<u8>,
}

impl DataBlock {
    /// `CreateByteDataBlock ( size )` abstract operation.
    ///
    /// The abstract operation `CreateByteDataBlock` takes argument `size` (a non-negative
    /// integer). For more information, check the [spec][spec].
    ///
    /// [spec]: https://tc39.es/ecma262/#sec-createbytedatablock
    pub fn create_byte_data_block(size: usize) -> JsResult<Self> {
        // 1. Let db be a new Data Block value consisting of size bytes. If it is impossible to
        //    create such a Data Block, throw a RangeError exception.
        // 2. Set all of the bytes of db to 0.
        // 3. Return db.
        // TODO: waiting on <https://github.com/rust-lang/rust/issues/48043> for having fallible
        // allocation.
        Ok(Self {
            inner: vec![0u8; size],
        })
    }
}
