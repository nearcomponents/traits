use super::{
    AlgorithmName, Buffer, BufferKindUser, FixedOutputCore, Reset, UpdateCore, VariableOutputCore,
};
use crate::HashMarker;
#[cfg(feature = "mac")]
use crate::MacMarker;
use core::{fmt, marker::PhantomData};
use crypto_common::{Block, BlockSizeUser, OutputSizeUser};
use generic_array::{
    typenum::{IsLess, IsLessOrEqual, Le, LeEq, NonZero, U256},
    ArrayLength, GenericArray,
};

/// Wrapper around [`VariableOutputCore`] which selects output size
/// at compile time.
#[derive(Clone)]
pub struct CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    inner: T,
    _out: PhantomData<OutSize>,
}

impl<T, OutSize> HashMarker for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore + HashMarker,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
}

#[cfg(feature = "mac")]
impl<T, OutSize> MacMarker for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore + MacMarker,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
}

impl<T, OutSize> BlockSizeUser for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    type BlockSize = T::BlockSize;
}

impl<T, OutSize> UpdateCore for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        self.inner.update_blocks(blocks);
    }
}

impl<T, OutSize> OutputSizeUser for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize> + 'static,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    type OutputSize = OutSize;
}

impl<T, OutSize> BufferKindUser for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    type BufferKind = T::BufferKind;
}

impl<T, OutSize> FixedOutputCore for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize> + 'static,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn finalize_fixed_core(
        &mut self,
        buffer: &mut Buffer<Self>,
        out: &mut GenericArray<u8, Self::OutputSize>,
    ) {
        self.inner.finalize_variable_core(buffer, out);
    }
}

impl<T, OutSize> Default for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn default() -> Self {
        Self {
            inner: T::new(OutSize::USIZE).unwrap(),
            _out: PhantomData,
        }
    }
}

impl<T, OutSize> Reset for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn reset(&mut self) {
        *self = Default::default();
    }
}

impl<T, OutSize> AlgorithmName for CtVariableCoreWrapper<T, OutSize>
where
    T: VariableOutputCore + AlgorithmName,
    OutSize: ArrayLength<u8> + IsLessOrEqual<T::MaxOutputSize>,
    LeEq<OutSize, T::MaxOutputSize>: NonZero,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        T::write_alg_name(f)?;
        f.write_str("_")?;
        write!(f, "{}", OutSize::USIZE)
    }
}
