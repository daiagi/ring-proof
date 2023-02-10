use ark_ec::short_weierstrass::{Affine, SWCurveConfig};
use ark_ff::PrimeField;
use fflonk::pcs::PCS;

use common::gadgets::sw_cond_add::AffineColumn;
use common::prover::PlonkProver;
use common::setup::Setup;

use crate::piop::params::PiopParams;
use crate::piop::PiopProver;
use crate::{ProverKey, RingProof};

pub struct RingProver<F: PrimeField, CS: PCS<F>, Curve: SWCurveConfig<BaseField=F>> {
    piop_params: PiopParams<F, Curve>,
    points: AffineColumn<F, Affine<Curve>>,
    k: usize,

    plonk_prover: PlonkProver<F, CS, merlin::Transcript>,
}


impl<F: PrimeField, CS: PCS<F>, Curve: SWCurveConfig<BaseField=F>> RingProver<F, CS, Curve> {
    pub fn init(prover_key: ProverKey<F, CS, Affine<Curve>>,
                piop_params: PiopParams<F, Curve>,
                k: usize,
                empty_transcript: merlin::Transcript,
    ) -> Self {
        let ProverKey { pcs_ck, fixed_columns, verifier_key } = prover_key;

        let plonk_prover = PlonkProver::init(pcs_ck, verifier_key, empty_transcript);

        Self {
            piop_params,
            points: fixed_columns.points,
            k,
            plonk_prover,
        }
    }


    pub fn prove(&self, t: Curve::ScalarField) -> RingProof<F, CS> {
        let piop = PiopProver::build(&self.piop_params, self.points.clone(), self.k, t);
        self.plonk_prover.prove(piop)
    }
}

