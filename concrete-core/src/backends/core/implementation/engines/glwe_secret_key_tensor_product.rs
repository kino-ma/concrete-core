use concrete_commons::dispersion::Variance;
use concrete_commons::parameters::{DecompositionBaseLog, DecompositionLevelCount};

use crate::backends::core::implementation::engines::CoreEngine;
use crate::backends::core::implementation::entities::{
    FourierLweBootstrapKey32, FourierLweBootstrapKey64, GlweSecretKey32, GlweSecretKey64,
    LweBootstrapKey32, LweBootstrapKey64, LweSecretKey32, LweSecretKey64,
};
use crate::backends::core::private::crypto::bootstrap::{
    FourierBootstrapKey as ImplFourierBootstrapKey,
    StandardBootstrapKey as ImplStandardBootstrapKey,
};
use crate::backends::core::private::crypto::secret::GlweSecretKey as ImplGlweSecretKey;
use crate::backends::core::private::math::polynomial::PolynomialList;
use crate::backends::core::private::math::fft::Complex64;
use crate::prelude::{LweBootstrapKeyEntity, PolynomialCount, PolynomialSize};
use crate::specification::engines::{TensorProductGlweSecretKeyCreationEngine,
                                    TensorProductGlweSecretKeyCreationError};

/// # Description:
/// Implementation of [`TensorProductGlweSecretKeyCreationEngine`] for [`CoreEngine`] that operates
/// on 32 bits integers. It outputs a tensor product of the input GLWE secret keys in the standard
/// domain.
impl TensorProductGlweSecretKeyCreationEngine<GlweSecretKey32, GlweSecretKey32, GlweSecretKey32>
for CoreEngine
{    fn create_tensor_product_glwe_secret_key(
        &mut self,
        input_key1: &GlweSecretKey32,
        input_key2: &GlweSecretKey32,
    ) -> Result<GlweSecretKey32, TensorProductGlweSecretKeyCreationError<Self::EngineError>> {
        TensorProductGlweSecretKeyCreationError::perform_generic_checks(
            input_key1,
            input_key2,
        )?;

    Ok(unsafe { self.create_tensor_product_glwe_secret_key(input_key1, input_key2)})


    }

    unsafe fn create_tensor_product_glwe_secret_key_unchecked(
        &mut self,
        input_key1: &GlweSecretKey32,
        input_key2: &GlweSecretKey32,
    ) -> GlweSecretKey32 {

        // .0 accesses the inner value, i.e. the underlying key wrapped in the GlweSecretKey32
        let mut input_list_1 = input_key1.0.as_polynomial_list();
        let mut input_list_2 = input_key2.0.as_polynomial_list();
        let mut list = PolynomialList::allocate(1u8,
                                            // TODO: correct the below
                                            PolynomialCount(input_key1.0.as_polynomial_list()),
                                            input_key1.0.polynomial_size());

        // create iterators over the two polynomial lists, as well as an output
        let mut iter_output = list.polynomial_iter();
        let iterator_1 = input_list_1.polynomial_iter_mut();
        let iterator_2 = input_list_2.polynomial_iter_mut();

        // fill the output of the iterator up with the correct product/s
        for (i, polynomial1) in iterator_1.enumerate() {
            for (j, polynomial2) in iterator_2.enumerate(){
                let mut output_poly1 = iter_output.next().unwrap();
                // TODO: correct the below
                output_poly1.fill_with_wrapping_mul(polynomial1, polynomial2);
            }
        }

        // put the tensor product key into a GlweSecretKey object
        // TODO: correct the below
        let tensor_key = GlweSecretKey::binary_from_container(list,
                                                  PolynomialSize(input_key1.0.polynomial_size()));

        tensor_key
    }
}



