use crate::fixture::Fixture;
use crate::generation::prototyping::{
    PrototypesLweCiphertext, PrototypesLweSecretKey, PrototypesPlaintext,
};
use crate::generation::synthesizing::{SynthesizesLweCiphertext, SynthesizesPlaintext};
use crate::generation::{IntegerPrecision, Maker};
use crate::raw::generation::RawUnsignedIntegers;
use crate::raw::statistical_test::assert_noise_distribution;
use concrete_commons::dispersion::{DispersionParameter, LogStandardDev, Variance};
use concrete_commons::numeric::UnsignedInteger;
use concrete_commons::parameters::LweDimension;
use concrete_core::prelude::{
    LweCiphertextEntity, LweCiphertextPlaintextFusingSubtractionEngine, PlaintextEntity,
};

/// A fixture for the types implementing the `LweCiphertextPlaintextFusingSubtractionEngine`
/// trait.
pub struct LweCiphertextPlaintextFusingSubtractionFixture;

#[derive(Debug)]
pub struct LweCiphertextPlaintextFusingSubtractionParameters {
    pub noise: Variance,
    pub lwe_dimension: LweDimension,
}

#[allow(clippy::type_complexity)]
impl<Precision, Engine, Plaintext, OutputCiphertext>
    Fixture<Precision, Engine, (Plaintext, OutputCiphertext)>
    for LweCiphertextPlaintextFusingSubtractionFixture
where
    Precision: IntegerPrecision,
    Engine: LweCiphertextPlaintextFusingSubtractionEngine<OutputCiphertext, Plaintext>,
    Plaintext: PlaintextEntity,
    OutputCiphertext: LweCiphertextEntity,
    Maker: SynthesizesPlaintext<Precision, Plaintext>
        + SynthesizesLweCiphertext<Precision, OutputCiphertext>,
{
    type Parameters = LweCiphertextPlaintextFusingSubtractionParameters;
    type RepetitionPrototypes = (
        <Maker as PrototypesLweSecretKey<Precision, OutputCiphertext::KeyDistribution>>::LweSecretKeyProto,
    );
    type SamplePrototypes = (
        <Maker as PrototypesPlaintext<Precision>>::PlaintextProto,
        <Maker as PrototypesPlaintext<Precision>>::PlaintextProto,
        <Maker as PrototypesLweCiphertext<Precision, OutputCiphertext::KeyDistribution>>::LweCiphertextProto,
    );
    type PreExecutionContext = (OutputCiphertext, Plaintext);
    type PostExecutionContext = (OutputCiphertext, Plaintext);
    type Criteria = (Variance,);
    type Outcome = (Precision::Raw, Precision::Raw);

    fn generate_parameters_iterator() -> Box<dyn Iterator<Item = Self::Parameters>> {
        Box::new(
            vec![LweCiphertextPlaintextFusingSubtractionParameters {
                noise: Variance(LogStandardDev::from_log_standard_dev(-15.).get_variance()),
                lwe_dimension: LweDimension(600),
            }]
            .into_iter(),
        )
    }

    fn generate_random_repetition_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
    ) -> Self::RepetitionPrototypes {
        (maker.new_lwe_secret_key(parameters.lwe_dimension),)
    }

    fn generate_random_sample_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::SamplePrototypes {
        let (proto_secret_key,) = repetition_proto;
        let raw_plaintext = Precision::Raw::uniform();
        let proto_plaintext = maker.transform_raw_to_plaintext(&raw_plaintext);
        let proto_output_ciphertext = maker.encrypt_plaintext_to_lwe_ciphertext(
            proto_secret_key,
            &proto_plaintext,
            parameters.noise,
        );

        let raw_plaintext_sub = Precision::Raw::uniform();
        let proto_plaintext_sub = maker.transform_raw_to_plaintext(&raw_plaintext_sub);

        (
            proto_plaintext,
            proto_plaintext_sub,
            proto_output_ciphertext,
        )
    }

    fn prepare_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
    ) -> Self::PreExecutionContext {
        let (_, proto_plaintext, proto_output_ciphertext) = sample_proto;
        let synth_plaintext = maker.synthesize_plaintext(proto_plaintext);
        let synth_output_ciphertext = maker.synthesize_lwe_ciphertext(proto_output_ciphertext);
        (synth_output_ciphertext, synth_plaintext)
    }

    fn execute_engine(
        _parameters: &Self::Parameters,
        engine: &mut Engine,
        context: Self::PreExecutionContext,
    ) -> Self::PostExecutionContext {
        let (mut output_ciphertext, plaintext) = context;
        unsafe {
            engine.fuse_sub_lwe_ciphertext_plaintext_unchecked(&mut output_ciphertext, &plaintext)
        };
        (output_ciphertext, plaintext)
    }

    fn process_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
        context: Self::PostExecutionContext,
    ) -> Self::Outcome {
        let (output_ciphertext, plaintext) = context;
        let (proto_plaintext, proto_plaintext_sub, ..) = sample_proto;
        let (proto_secret_key,) = repetition_proto;
        let raw_plaintext = maker.transform_plaintext_to_raw(proto_plaintext);
        let raw_plaintext_sub = maker.transform_plaintext_to_raw(proto_plaintext_sub);
        let expected_mean = raw_plaintext.wrapping_sub(raw_plaintext_sub);
        let proto_output_ciphertext = maker.unsynthesize_lwe_ciphertext(output_ciphertext);
        let proto_output_plaintext =
            maker.decrypt_lwe_ciphertext_to_plaintext(proto_secret_key, &proto_output_ciphertext);
        maker.destroy_plaintext(plaintext);
        (
            expected_mean,
            maker.transform_plaintext_to_raw(&proto_output_plaintext),
        )
    }

    fn compute_criteria(
        parameters: &Self::Parameters,
        _maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::Criteria {
        (parameters.noise,)
    }

    fn verify(criteria: &Self::Criteria, outputs: &[Self::Outcome]) -> bool {
        let (means, actual): (Vec<_>, Vec<_>) = outputs.iter().cloned().unzip();
        assert_noise_distribution(&actual, means.as_slice(), criteria.0)
    }
}
