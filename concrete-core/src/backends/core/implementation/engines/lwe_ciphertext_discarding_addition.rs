use crate::backends::core::implementation::engines::CoreEngine;
use crate::backends::core::implementation::entities::{
    LweCiphertext32, LweCiphertext64, LweCiphertextMutView32, LweCiphertextMutView64,
    LweCiphertextView32, LweCiphertextView64,
};
use crate::backends::core::private::math::tensor::{AsMutTensor, AsRefTensor};
use crate::specification::engines::{
    LweCiphertextDiscardingAdditionEngine, LweCiphertextDiscardingAdditionError,
};

/// # Description:
/// Implementation of [`LweCiphertextDiscardingAdditionEngine`] for [`CoreEngine`] that operates on
/// 32 bits integers.
impl LweCiphertextDiscardingAdditionEngine<LweCiphertext32, LweCiphertext32> for CoreEngine {
    /// # Example:
    /// ```
    /// use concrete_commons::dispersion::Variance;
    /// use concrete_commons::parameters::LweDimension;
    /// use concrete_core::prelude::*;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // DISCLAIMER: the parameters used here are only for test purpose, and are not secure.
    /// let lwe_dimension = LweDimension(2);
    /// // Here a hard-set encoding is applied (shift by 20 bits)
    /// let input_1 = 3_u32 << 20;
    /// let input_2 = 7_u32 << 20;
    /// let noise = Variance(2_f64.powf(-25.));
    ///
    /// let mut engine = CoreEngine::new(())?;
    /// let key: LweSecretKey32 = engine.create_lwe_secret_key(lwe_dimension)?;
    /// let plaintext_1 = engine.create_plaintext(&input_1)?;
    /// let plaintext_2 = engine.create_plaintext(&input_2)?;
    /// let ciphertext_1 = engine.encrypt_lwe_ciphertext(&key, &plaintext_1, noise)?;
    /// let ciphertext_2 = engine.encrypt_lwe_ciphertext(&key, &plaintext_2, noise)?;
    /// let mut ciphertext_3 = engine.zero_encrypt_lwe_ciphertext(&key, noise)?;
    ///
    /// engine.discard_add_lwe_ciphertext(&mut ciphertext_3, &ciphertext_1, &ciphertext_2)?;
    /// #
    /// assert_eq!(ciphertext_3.lwe_dimension(), lwe_dimension);
    ///
    /// engine.destroy(key)?;
    /// engine.destroy(plaintext_1)?;
    /// engine.destroy(plaintext_2)?;
    /// engine.destroy(ciphertext_1)?;
    /// engine.destroy(ciphertext_2)?;
    /// engine.destroy(ciphertext_3)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn discard_add_lwe_ciphertext(
        &mut self,
        output: &mut LweCiphertext32,
        input_1: &LweCiphertext32,
        input_2: &LweCiphertext32,
    ) -> Result<(), LweCiphertextDiscardingAdditionError<Self::EngineError>> {
        LweCiphertextDiscardingAdditionError::perform_generic_checks(output, input_1, input_2)?;
        unsafe { self.discard_add_lwe_ciphertext_unchecked(output, input_1, input_2) };
        Ok(())
    }

    unsafe fn discard_add_lwe_ciphertext_unchecked(
        &mut self,
        output: &mut LweCiphertext32,
        input_1: &LweCiphertext32,
        input_2: &LweCiphertext32,
    ) {
        output
            .0
            .as_mut_tensor()
            .fill_with_copy(input_1.0.as_tensor());
        output.0.update_with_add(&input_2.0);
    }
}

/// # Description:
/// Implementation of [`LweCiphertextDiscardingAdditionEngine`] for [`CoreEngine`] that operates on
/// 64 bits integers.
impl LweCiphertextDiscardingAdditionEngine<LweCiphertext64, LweCiphertext64> for CoreEngine {
    /// # Example:
    /// ```
    /// use concrete_commons::dispersion::Variance;
    /// use concrete_commons::parameters::LweDimension;
    /// use concrete_core::prelude::*;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // DISCLAIMER: the parameters used here are only for test purpose, and are not secure.
    /// let lwe_dimension = LweDimension(2);
    /// // Here a hard-set encoding is applied (shift by 50 bits)
    /// let input_1 = 3_u64 << 50;
    /// let input_2 = 7_u64 << 50;
    /// let noise = Variance(2_f64.powf(-25.));
    ///
    /// let mut engine = CoreEngine::new(())?;
    /// let key: LweSecretKey64 = engine.create_lwe_secret_key(lwe_dimension)?;
    /// let plaintext_1 = engine.create_plaintext(&input_1)?;
    /// let plaintext_2 = engine.create_plaintext(&input_2)?;
    /// let ciphertext_1 = engine.encrypt_lwe_ciphertext(&key, &plaintext_1, noise)?;
    /// let ciphertext_2 = engine.encrypt_lwe_ciphertext(&key, &plaintext_2, noise)?;
    /// let mut ciphertext_3 = engine.zero_encrypt_lwe_ciphertext(&key, noise)?;
    ///
    /// engine.discard_add_lwe_ciphertext(&mut ciphertext_3, &ciphertext_1, &ciphertext_2)?;
    /// #
    /// assert_eq!(ciphertext_3.lwe_dimension(), lwe_dimension);
    ///
    /// engine.destroy(key)?;
    /// engine.destroy(plaintext_1)?;
    /// engine.destroy(plaintext_2)?;
    /// engine.destroy(ciphertext_1)?;
    /// engine.destroy(ciphertext_2)?;
    /// engine.destroy(ciphertext_3)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn discard_add_lwe_ciphertext(
        &mut self,
        output: &mut LweCiphertext64,
        input_1: &LweCiphertext64,
        input_2: &LweCiphertext64,
    ) -> Result<(), LweCiphertextDiscardingAdditionError<Self::EngineError>> {
        LweCiphertextDiscardingAdditionError::perform_generic_checks(output, input_1, input_2)?;
        unsafe { self.discard_add_lwe_ciphertext_unchecked(output, input_1, input_2) };
        Ok(())
    }

    unsafe fn discard_add_lwe_ciphertext_unchecked(
        &mut self,
        output: &mut LweCiphertext64,
        input_1: &LweCiphertext64,
        input_2: &LweCiphertext64,
    ) {
        output
            .0
            .as_mut_tensor()
            .fill_with_copy(input_1.0.as_tensor());
        output.0.update_with_add(&input_2.0);
    }
}

/// # Description:
/// Implementation of [`LweCiphertextDiscardingAdditionEngine`] for [`CoreEngine`] that operates on
/// views containing 32 bits integers.
impl LweCiphertextDiscardingAdditionEngine<LweCiphertextView32<'_>, LweCiphertextMutView32<'_>>
    for CoreEngine
{
    /// # Example:
    /// ```
    /// use concrete_commons::dispersion::Variance;
    /// use concrete_commons::parameters::LweDimension;
    /// use concrete_core::prelude::*;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // DISCLAIMER: the parameters used here are only for test purpose, and are not secure.
    /// let lwe_dimension = LweDimension(2);
    /// // Here a hard-set encoding is applied (shift by 20 bits)
    /// let input_1 = 3_u32 << 20;
    /// let input_2 = 7_u32 << 20;
    /// let noise = Variance(2_f64.powf(-25.));
    ///
    /// let mut engine = CoreEngine::new(())?;
    /// let key: LweSecretKey32 = engine.create_lwe_secret_key(lwe_dimension)?;
    /// let plaintext_1 = engine.create_plaintext(&input_1)?;
    /// let plaintext_2 = engine.create_plaintext(&input_2)?;
    ///
    /// let mut ciphertext_1_container = vec![0_u32; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_1: LweCiphertextMutView32 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_1_container[..])?;
    /// engine.discard_encrypt_lwe_ciphertext(&key, &mut ciphertext_1, &plaintext_1, noise)?;
    /// let mut ciphertext_2_container = vec![0_u32; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_2: LweCiphertextMutView32 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_2_container[..])?;
    /// engine.discard_encrypt_lwe_ciphertext(&key, &mut ciphertext_2, &plaintext_2, noise)?;
    ///
    /// // Convert MutView to View
    /// let raw_ciphertext_1 = engine.consume_retrieve_lwe_ciphertext(ciphertext_1)?;
    /// let ciphertext_1: LweCiphertextView32 = engine.create_lwe_ciphertext(&raw_ciphertext_1[..])?;
    /// let raw_ciphertext_2 = engine.consume_retrieve_lwe_ciphertext(ciphertext_2)?;
    /// let ciphertext_2: LweCiphertextView32 = engine.create_lwe_ciphertext(&raw_ciphertext_2[..])?;
    ///
    /// let mut ciphertext_3_container = vec![0_u32; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_3: LweCiphertextMutView32 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_3_container[..])?;
    ///
    /// engine.discard_add_lwe_ciphertext(&mut ciphertext_3, &ciphertext_1, &ciphertext_2)?;
    /// #
    /// assert_eq!(ciphertext_3.lwe_dimension(), lwe_dimension);
    ///
    /// engine.destroy(key)?;
    /// engine.destroy(plaintext_1)?;
    /// engine.destroy(plaintext_2)?;
    /// engine.destroy(ciphertext_1)?;
    /// engine.destroy(ciphertext_2)?;
    /// engine.destroy(ciphertext_3)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn discard_add_lwe_ciphertext(
        &mut self,
        output: &mut LweCiphertextMutView32,
        input_1: &LweCiphertextView32,
        input_2: &LweCiphertextView32,
    ) -> Result<(), LweCiphertextDiscardingAdditionError<Self::EngineError>> {
        LweCiphertextDiscardingAdditionError::perform_generic_checks(output, input_1, input_2)?;
        unsafe { self.discard_add_lwe_ciphertext_unchecked(output, input_1, input_2) };
        Ok(())
    }

    unsafe fn discard_add_lwe_ciphertext_unchecked(
        &mut self,
        output: &mut LweCiphertextMutView32,
        input_1: &LweCiphertextView32,
        input_2: &LweCiphertextView32,
    ) {
        output
            .0
            .as_mut_tensor()
            .fill_with_copy(input_1.0.as_tensor());
        output.0.update_with_add(&input_2.0);
    }
}

/// # Description:
/// Implementation of [`LweCiphertextDiscardingAdditionEngine`] for [`CoreEngine`] that operates on
/// on views containing 64 bits integers.
impl LweCiphertextDiscardingAdditionEngine<LweCiphertextView64<'_>, LweCiphertextMutView64<'_>>
    for CoreEngine
{
    /// # Example:
    /// ```
    /// use concrete_commons::dispersion::Variance;
    /// use concrete_commons::parameters::LweDimension;
    /// use concrete_core::prelude::*;
    /// # use std::error::Error;
    ///
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// // DISCLAIMER: the parameters used here are only for test purpose, and are not secure.
    /// let lwe_dimension = LweDimension(2);
    /// // Here a hard-set encoding is applied (shift by 50 bits)
    /// let input_1 = 3_u64 << 50;
    /// let input_2 = 7_u64 << 50;
    /// let noise = Variance(2_f64.powf(-25.));
    ///
    /// let mut engine = CoreEngine::new(())?;
    /// let key: LweSecretKey64 = engine.create_lwe_secret_key(lwe_dimension)?;
    /// let plaintext_1 = engine.create_plaintext(&input_1)?;
    /// let plaintext_2 = engine.create_plaintext(&input_2)?;
    ///
    /// let mut ciphertext_1_container = vec![0_u64; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_1: LweCiphertextMutView64 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_1_container[..])?;
    /// engine.discard_encrypt_lwe_ciphertext(&key, &mut ciphertext_1, &plaintext_1, noise)?;
    /// let mut ciphertext_2_container = vec![0_u64; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_2: LweCiphertextMutView64 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_2_container[..])?;
    /// engine.discard_encrypt_lwe_ciphertext(&key, &mut ciphertext_2, &plaintext_2, noise)?;
    ///
    /// // Convert MutView to View
    /// let raw_ciphertext_1 = engine.consume_retrieve_lwe_ciphertext(ciphertext_1)?;
    /// let ciphertext_1: LweCiphertextView64 = engine.create_lwe_ciphertext(&raw_ciphertext_1[..])?;
    /// let raw_ciphertext_2 = engine.consume_retrieve_lwe_ciphertext(ciphertext_2)?;
    /// let ciphertext_2: LweCiphertextView64 = engine.create_lwe_ciphertext(&raw_ciphertext_2[..])?;
    ///
    /// let mut ciphertext_3_container = vec![0_u64; key.lwe_dimension().to_lwe_size().0];
    /// let mut ciphertext_3: LweCiphertextMutView64 =
    ///     engine.create_lwe_ciphertext(&mut ciphertext_3_container[..])?;
    ///
    /// engine.discard_add_lwe_ciphertext(&mut ciphertext_3, &ciphertext_1, &ciphertext_2)?;
    /// #
    /// assert_eq!(ciphertext_3.lwe_dimension(), lwe_dimension);
    ///
    /// engine.destroy(key)?;
    /// engine.destroy(plaintext_1)?;
    /// engine.destroy(plaintext_2)?;
    /// engine.destroy(ciphertext_1)?;
    /// engine.destroy(ciphertext_2)?;
    /// engine.destroy(ciphertext_3)?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    fn discard_add_lwe_ciphertext(
        &mut self,
        output: &mut LweCiphertextMutView64,
        input_1: &LweCiphertextView64,
        input_2: &LweCiphertextView64,
    ) -> Result<(), LweCiphertextDiscardingAdditionError<Self::EngineError>> {
        LweCiphertextDiscardingAdditionError::perform_generic_checks(output, input_1, input_2)?;
        unsafe { self.discard_add_lwe_ciphertext_unchecked(output, input_1, input_2) };
        Ok(())
    }

    unsafe fn discard_add_lwe_ciphertext_unchecked(
        &mut self,
        output: &mut LweCiphertextMutView64,
        input_1: &LweCiphertextView64,
        input_2: &LweCiphertextView64,
    ) {
        output
            .0
            .as_mut_tensor()
            .fill_with_copy(input_1.0.as_tensor());
        output.0.update_with_add(&input_2.0);
    }
}
