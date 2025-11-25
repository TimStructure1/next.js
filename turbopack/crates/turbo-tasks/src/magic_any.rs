use std::{
    any::{Any, type_name},
    fmt::Debug,
    hash::Hash,
};

use bincode::error::{DecodeError, EncodeError};
use turbo_bincode::{TurboBincodeDecoder, TurboBincodeEncoder};
use turbo_dyn_eq_hash::{
    DynEq, DynHash, impl_eq_for_dyn, impl_hash_for_dyn, impl_partial_eq_for_dyn,
};

use crate::trace::TraceRawVcs;

pub trait MagicAny: Debug + DynEq + DynHash + TraceRawVcs + Send + Sync + 'static {
    #[cfg(debug_assertions)]
    fn magic_type_name(&self) -> &'static str;
}

impl<T> MagicAny for T
where
    T: Debug + Eq + Hash + Send + Sync + TraceRawVcs + 'static,
{
    #[cfg(debug_assertions)]
    fn magic_type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

impl_partial_eq_for_dyn!(dyn MagicAny);
impl_eq_for_dyn!(dyn MagicAny);
impl_hash_for_dyn!(dyn MagicAny);

pub type AnyEncodeFn = fn(&dyn Any, &mut TurboBincodeEncoder<'_>) -> Result<(), EncodeError>;
pub type AnyDecodeFn<T> = fn(&mut TurboBincodeDecoder<'_>) -> Result<T, DecodeError>;

pub fn any_as_encode<T: Any>(this: &dyn Any) -> &T {
    if let Some(enc) = this.downcast_ref::<T>() {
        return enc;
    }
    unreachable!(
        "any_as_encode::<{}> called with invalid type",
        type_name::<T>()
    );
}
