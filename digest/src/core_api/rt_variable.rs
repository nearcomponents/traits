use super::{AlgorithmName, UpdateCore, VariableOutputCore};
use crate::HashMarker;
#[cfg(feature = "mac")]
use crate::MacMarker;
use crate::{InvalidOutputSize, Reset, Update, VariableOutput};
use block_buffer::BlockBuffer;
use core::fmt;
use generic_array::typenum::{IsLess, Le, NonZero, Unsigned, U256};

/// Wrapper around [`VariableOutputCore`] which selects output size
/// at run time.
#[derive(Clone)]
pub struct RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    core: T,
    buffer: BlockBuffer<T::BlockSize, T::BufferKind>,
    output_size: usize,
}

impl<T> HashMarker for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + HashMarker,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
}

#[cfg(feature = "mac")]
#[cfg_attr(docsrs, doc(cfg(feature = "mac")))]
impl<T> MacMarker for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + MacMarker,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
}

impl<T> Reset for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn reset(&mut self) {
        // For correct implementations `new` should always return `Ok`
        // since wrapper can be only created with valid `output_size`
        if let Ok(v) = T::new(self.output_size) {
            self.core = v;
        } else {
            debug_assert!(false);
        }
        self.buffer.reset();
    }
}

impl<T> Update for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn update(&mut self, input: &[u8]) {
        let Self { core, buffer, .. } = self;
        buffer.digest_blocks(input, |blocks| core.update_blocks(blocks));
    }
}

impl<T> VariableOutput for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    const MAX_OUTPUT_SIZE: usize = T::MaxOutputSize::USIZE;

    fn new(output_size: usize) -> Result<Self, InvalidOutputSize> {
        let buffer = Default::default();
        T::new(output_size).map(|core| Self {
            core,
            buffer,
            output_size,
        })
    }

    fn output_size(&self) -> usize {
        self.output_size
    }

    fn finalize_variable(mut self, f: impl FnOnce(&[u8])) {
        let Self {
            core,
            buffer,
            output_size,
        } = &mut self;
        core.finalize_variable_core(buffer, *output_size, f);
    }

    fn finalize_variable_reset(&mut self, f: impl FnOnce(&[u8])) {
        let Self {
            core,
            buffer,
            output_size,
        } = self;
        core.finalize_variable_core(buffer, *output_size, f);
        self.reset()
    }
}

impl<T> fmt::Debug for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore + AlgorithmName,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        T::write_alg_name(f)?;
        f.write_str(" { .. }")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl<T> std::io::Write for RtVariableCoreWrapper<T>
where
    T: VariableOutputCore + UpdateCore,
    T::BlockSize: IsLess<U256>,
    Le<T::BlockSize, U256>: NonZero,
{
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Update::update(self, buf);
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
