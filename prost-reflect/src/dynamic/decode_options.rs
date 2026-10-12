/// Options to control decoding of messages.
///
/// Used by [`DynamicMessage::decode_with_options()`](crate::DynamicMessage::decode_with_options).
#[derive(Debug, Clone, Default)]
pub struct DecodeOptions {
    pub(super) ignore_unknown_fields: bool,
}

impl DecodeOptions {
    /// Creates a new instance of [`DecodeOptions`], with the default options chosen to conform to
    /// the standard protobuf behavior.
    pub const fn new() -> Self {
        DecodeOptions {
            ignore_unknown_fields: false,
        }
    }

    /// Whether to skip fields which are not present in the message descriptor, instead of
    /// preserving them as [`UnknownField`](crate::UnknownField)s.
    ///
    /// This allows decoding a message using a partial descriptor which only declares the fields of
    /// interest, without paying for the allocation of the others. Skipped fields are lost, so
    /// re-encoding the message will not reproduce the original bytes.
    ///
    /// The option applies to nested messages too.
    ///
    /// The default value is `false`.
    pub const fn ignore_unknown_fields(mut self, yes: bool) -> Self {
        self.ignore_unknown_fields = yes;
        self
    }
}
