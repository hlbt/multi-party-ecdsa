#![allow(non_snake_case)]

use std::convert::TryFrom;
use std::iter;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use curv::elliptic::curves::{secp256_k1::Secp256k1, Point, Scalar};
use curv::BigInt;
use sha2::Sha256;

use round_based::containers::push::Push;
use round_based::containers::{self, BroadcastMsgs, P2PMsgs, Store};
use round_based::Msg;

use crate::utilities::mta::{MessageA, MessageB};

use crate::protocols::multi_party_ecdsa::gg_2020 as gg20;
use crate::utilities::zk_pdl_with_slack::PDLwSlackProof;
use curv::cryptographic_primitives::proofs::sigma_correct_homomorphic_elgamal_enc::HomoELGamalProof;
use curv::cryptographic_primitives::proofs::sigma_valid_pedersen::PedersenProof;
use gg20::party_i::{
    LocalSignature, SignBroadcastPhase1, SignDecommitPhase1, SignKeys, SignatureRecid,
};
use gg20::state_machine::keygen::LocalKey;
use gg20::ErrorType;

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct GWI(pub Point<Secp256k1>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GammaI(pub MessageB);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WI(pub MessageB);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeltaI(Scalar<Secp256k1>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TI(pub Point<Secp256k1>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TIProof(pub PedersenProof<Secp256k1, Sha256>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RDash(Point<Secp256k1>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SI(pub Point<Secp256k1>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HEGProof(pub HomoELGamalProof<Secp256k1, Sha256>);

// PartialSignature
pub struct Round0 {
    /// Index of this party
    ///
    /// Must be in range `[0; n)` where `n` is number of parties involved in signing.
    pub i: u16,

    /// List of parties' indexes from keygen protocol
    ///
    /// I.e. `s_l[i]` must be an index of party `i` that was used by this party in keygen protocol.
    // s_l.len()` equals to `n` (number of parties involved in signing)
    pub s_l: Vec<u16>,

    /// Party local secret share
    pub local_key: LocalKey<Secp256k1>,

    pub message: BigInt,
}

impl Round0 {
    pub fn proceed<O>(self, mut output: O) -> Result<Round1>
    where
        O: Push<Msg<(MessageA, SignBroadcastPhase1)>>,
    {
        let sign_keys = SignKeys::create(
            &self.local_key.keys_linear.x_i,
            &self.local_key.vss_scheme.clone(),
            usize::from(self.s_l[usize::from(self.i - 1)]) - 1,
            &self
                .s_l
                .iter()
                .map(|&i| usize::from(i) - 1)
                .collect::<Vec<_>>(),
        );
        let (bc1, decom1) = sign_keys.phase1_broadcast();

        let party_ek = self.local_key.paillier_key_vec[usize::from(self.local_key.i - 1)].clone();
        let m_a = MessageA::a(&sign_keys.k_i, &party_ek, &self.local_key.h1_h2_n_tilde_vec);

        output.push(Msg {
            sender: self.i,
            receiver: None,
            body: (m_a.0.clone(), bc1.clone()),
        });

        let round1 = Round1 {
            i: self.i,
            s_l: self.s_l.clone(),
            local_key: self.local_key,
            message: self.message,
            m_a,
            sign_keys,
            phase1_com: bc1,
            phase1_decom: decom1,
        };

        Ok(round1)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

pub struct Round1 {
    i: u16,
    s_l: Vec<u16>,
    local_key: LocalKey<Secp256k1>,
    message: BigInt,
    m_a: (MessageA, BigInt),
    sign_keys: SignKeys,
    phase1_com: SignBroadcastPhase1,
    phase1_decom: SignDecommitPhase1,
}

impl Round1 {
    pub fn proceed<O>(
        self,
        input: BroadcastMsgs<(MessageA, SignBroadcastPhase1)>,
        mut output: O,
    ) -> Result<Round2>
    where
        O: Push<Msg<(GammaI, WI)>>,
    {
        let (m_a_vec, bc_vec): (Vec<_>, Vec<_>) = input
            .into_vec_including_me((self.m_a.0.clone(), self.phase1_com.clone()))
            .into_iter()
            .unzip();

        let mut m_b_gamma_vec = Vec::new();
        let mut beta_vec = Vec::new();
        let mut m_b_w_vec = Vec::new();
        let mut ni_vec = Vec::new();

        let ttag = self.s_l.len();
        let l_s: Vec<_> = self
            .s_l
            .iter()
            .cloned()
            .map(|i| usize::from(i) - 1)
            .collect();
        let i = usize::from(self.i - 1);
        for j in 0..ttag - 1 {
            let ind = if j < i { j } else { j + 1 };

            let (m_b_gamma, beta_gamma, _beta_randomness, _beta_tag) = MessageB::b(
                &self.sign_keys.gamma_i,
                &self.local_key.paillier_key_vec[l_s[ind]],
                m_a_vec[ind].clone(),
                &self.local_key.h1_h2_n_tilde_vec,
            )
            .map_err(|e| {
                Error::Round1(ErrorType {
                    error_type: e.to_string(),
                    bad_actors: vec![],
                })
            })?;

            let (m_b_w, beta_wi, _, _) = MessageB::b(
                &self.sign_keys.w_i,
                &self.local_key.paillier_key_vec[l_s[ind]],
                m_a_vec[ind].clone(),
                &self.local_key.h1_h2_n_tilde_vec,
            )
            .map_err(|e| {
                Error::Round1(ErrorType {
                    error_type: e.to_string(),
                    bad_actors: vec![],
                })
            })?;

            m_b_gamma_vec.push(m_b_gamma);
            beta_vec.push(beta_gamma);
            m_b_w_vec.push(m_b_w);
            ni_vec.push(beta_wi);
        }

        let party_indices = (1..=self.s_l.len())
            .map(|j| u16::try_from(j).unwrap())
            .filter(|&j| j != self.i);
        for ((j, gamma_i), w_i) in party_indices.zip(m_b_gamma_vec).zip(m_b_w_vec) {
            output.push(Msg {
                sender: self.i,
                receiver: Some(j),
                body: (GammaI(gamma_i.clone()), WI(w_i.clone())),
            });
        }

        Ok(Round2 {
            i: self.i,
            s_l: self.s_l,
            local_key: self.local_key,
            message: self.message,
            sign_keys: self.sign_keys,
            m_a: self.m_a,
            beta_vec,
            ni_vec,
            bc_vec,
            m_a_vec,
            phase1_decom: self.phase1_decom,
        })
    }

    pub fn expects_messages(
        i: u16,
        n: u16,
    ) -> Store<BroadcastMsgs<(MessageA, SignBroadcastPhase1)>> {
        containers::BroadcastMsgsStore::new(i, n)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

pub struct Round2 {
    i: u16,
    s_l: Vec<u16>,
    local_key: LocalKey<Secp256k1>,
    message: BigInt,
    sign_keys: SignKeys,
    m_a: (MessageA, BigInt),
    beta_vec: Vec<Scalar<Secp256k1>>,
    ni_vec: Vec<Scalar<Secp256k1>>,
    bc_vec: Vec<SignBroadcastPhase1>,
    m_a_vec: Vec<MessageA>,
    phase1_decom: SignDecommitPhase1,
}

impl Round2 {
    pub fn proceed<O>(self, input_p2p: P2PMsgs<(GammaI, WI)>, mut output: O) -> Result<Round3>
    where
        O: Push<Msg<(DeltaI, TI, TIProof)>>, // TODO: unify TI and TIProof
    {
        let (m_b_gamma_s, m_b_w_s): (Vec<_>, Vec<_>) = input_p2p
            .into_vec()
            .into_iter()
            .map(|(gamma_i, w_i)| (gamma_i.0, w_i.0))
            .unzip();

        let mut alpha_vec = Vec::new();
        let mut miu_vec = Vec::new();

        let ttag = self.s_l.len();
        let index = usize::from(self.i) - 1;
        let l_s: Vec<_> = self
            .s_l
            .iter()
            .cloned()
            .map(|i| usize::from(i) - 1)
            .collect();
        let g_w_vec = SignKeys::g_w_vec(
            &self.local_key.pk_vec[..],
            &l_s[..],
            &self.local_key.vss_scheme,
        );
        for j in 0..ttag - 1 {
            let ind = if j < index { j } else { j + 1 };
            let m_b = m_b_gamma_s[j].clone();

            let alpha_ij_gamma = m_b
                .verify_proofs_get_alpha(&self.local_key.paillier_dk, &self.sign_keys.k_i)
                .map_err(|e| {
                    Error::Round3(ErrorType {
                        error_type: e.to_string(),
                        bad_actors: vec![],
                    })
                })?;
            let m_b = m_b_w_s[j].clone();
            let alpha_ij_wi = m_b
                .verify_proofs_get_alpha(&self.local_key.paillier_dk, &self.sign_keys.k_i)
                .map_err(|e| {
                    Error::Round3(ErrorType {
                        error_type: e.to_string(),
                        bad_actors: vec![],
                    })
                })?;
            assert_eq!(m_b.b_proof.pk, g_w_vec[ind]); //TODO: return error

            alpha_vec.push(alpha_ij_gamma.0);
            miu_vec.push(alpha_ij_wi.0);
        }

        let delta_i = self.sign_keys.phase2_delta_i(&alpha_vec, &self.beta_vec);

        let sigma_i = self.sign_keys.phase2_sigma_i(&miu_vec, &self.ni_vec);
        let (t_i, l_i, t_i_proof) = SignKeys::phase3_compute_t_i(&sigma_i);
        output.push(Msg {
            sender: self.i,
            receiver: None,
            body: (
                DeltaI(delta_i.clone()),
                TI(t_i.clone()),
                TIProof(t_i_proof.clone()),
            ),
        });

        Ok(Round3 {
            i: self.i,
            s_l: self.s_l,
            local_key: self.local_key,
            message: self.message,
            sign_keys: self.sign_keys,
            m_a: self.m_a,
            mb_gamma_s: m_b_gamma_s,
            bc_vec: self.bc_vec,
            m_a_vec: self.m_a_vec,
            delta_i,
            t_i,
            l_i,
            sigma_i,
            t_i_proof,
            phase1_decom: self.phase1_decom,
        })
    }

    pub fn expects_messages(i: u16, n: u16) -> Store<P2PMsgs<(GammaI, WI)>> {
        containers::P2PMsgsStore::new(i, n)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

pub struct Round3 {
    i: u16,
    s_l: Vec<u16>,
    local_key: LocalKey<Secp256k1>,
    message: BigInt,
    sign_keys: SignKeys,
    m_a: (MessageA, BigInt),
    mb_gamma_s: Vec<MessageB>,
    bc_vec: Vec<SignBroadcastPhase1>,
    m_a_vec: Vec<MessageA>,
    delta_i: Scalar<Secp256k1>,
    t_i: Point<Secp256k1>,
    l_i: Scalar<Secp256k1>,
    sigma_i: Scalar<Secp256k1>,
    t_i_proof: PedersenProof<Secp256k1, Sha256>,

    phase1_decom: SignDecommitPhase1,
}

impl Round3 {
    pub fn proceed<O>(
        self,
        input: BroadcastMsgs<(DeltaI, TI, TIProof)>,
        mut output: O,
    ) -> Result<Round4>
    where
        O: Push<Msg<SignDecommitPhase1>>,
    {
        let (delta_vec, t_vec, t_proof_vec) = input
            .into_vec_including_me((
                DeltaI(self.delta_i),
                TI(self.t_i.clone()),
                TIProof(self.t_i_proof),
            ))
            .into_iter()
            .map(|(delta_i, t_i, t_i_proof)| (delta_i.0, t_i.0, t_i_proof.0))
            .unzip3();

        for i in 0..t_vec.len() {
            assert_eq!(t_vec[i], t_proof_vec[i].com);
        }

        let delta_inv = SignKeys::phase3_reconstruct_delta(&delta_vec);
        let ttag = self.s_l.len();
        for proof in t_proof_vec.iter().take(ttag) {
            PedersenProof::verify(proof).map_err(|e| {
                Error::Round3(ErrorType {
                    error_type: e.to_string(),
                    bad_actors: vec![],
                })
            })?;
        }

        output.push(Msg {
            sender: self.i,
            receiver: None,
            body: self.phase1_decom.clone(),
        });

        Ok(Round4 {
            i: self.i,
            s_l: self.s_l,
            local_key: self.local_key,
            message: self.message,
            sign_keys: self.sign_keys,
            m_a: self.m_a,
            mb_gamma_s: self.mb_gamma_s,
            bc_vec: self.bc_vec,
            m_a_vec: self.m_a_vec,
            t_i: self.t_i,
            l_i: self.l_i,
            sigma_i: self.sigma_i,
            phase1_decom: self.phase1_decom,
            delta_inv,
            t_vec,
        })
    }

    pub fn expects_messages(i: u16, n: u16) -> Store<BroadcastMsgs<(DeltaI, TI, TIProof)>> {
        containers::BroadcastMsgsStore::new(i, n)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

pub struct Round4 {
    i: u16,
    s_l: Vec<u16>,
    local_key: LocalKey<Secp256k1>,
    message: BigInt,
    sign_keys: SignKeys,
    m_a: (MessageA, BigInt),
    mb_gamma_s: Vec<MessageB>,
    bc_vec: Vec<SignBroadcastPhase1>,
    m_a_vec: Vec<MessageA>,
    t_i: Point<Secp256k1>,
    l_i: Scalar<Secp256k1>,
    sigma_i: Scalar<Secp256k1>,
    delta_inv: Scalar<Secp256k1>,
    t_vec: Vec<Point<Secp256k1>>,
    phase1_decom: SignDecommitPhase1,
}

impl Round4 {
    pub fn proceed<O>(
        self,
        decommit_round1: BroadcastMsgs<SignDecommitPhase1>,
        mut output: O,
    ) -> Result<Round5>
    where
        O: Push<Msg<PartialSignature>>,
    {
        let decom_vec: Vec<_> = decommit_round1.into_vec_including_me(self.phase1_decom.clone());

        let ttag = self.s_l.len();
        let b_proof_vec: Vec<_> = (0..ttag - 1).map(|i| &self.mb_gamma_s[i].b_proof).collect();
        let R = SignKeys::phase4(
            &self.delta_inv,
            &b_proof_vec[..],
            decom_vec,
            &self.bc_vec,
            usize::from(self.i - 1),
        )
        .map_err(|e| Error::Round5(e))?;

        let R_dash = &R * &self.sign_keys.k_i;

        // each party sends first message to all other parties
        let mut phase5_proofs_vec = Vec::new();
        let l_s: Vec<_> = self
            .s_l
            .iter()
            .cloned()
            .map(|i| usize::from(i) - 1)
            .collect();
        let index = usize::from(self.i - 1);
        for j in 0..ttag - 1 {
            let ind = if j < index { j } else { j + 1 };
            let proof = LocalSignature::phase5_proof_pdl(
                &R_dash,
                &R,
                &self.m_a.0.c,
                &self.local_key.paillier_key_vec[l_s[index]],
                &self.sign_keys.k_i,
                &self.m_a.1,
                &self.local_key.h1_h2_n_tilde_vec[l_s[ind]],
            );

            phase5_proofs_vec.push(proof);
        }

        let local_signature = LocalSignature::phase7_local_sig(
            &self.sign_keys.k_i,
            &self.message,
            &R,
            &self.sigma_i,
            &self.local_key.y_sum_s,
        );
        let partial = PartialSignature(local_signature.s_i.clone());

        output.push(Msg {
            sender: self.i,
            receiver: None,
            body: partial.clone(),
        });

        Ok(Round5 {
            local_signature,
        })
    }

    pub fn expects_messages(i: u16, n: u16) -> Store<BroadcastMsgs<SignDecommitPhase1>> {
        containers::BroadcastMsgsStore::new(i, n)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompletedOfflineStage {
    i: u16,
    local_key: LocalKey<Secp256k1>,
    sign_keys: SignKeys,
    t_vec: Vec<Point<Secp256k1>>,
    R: Point<Secp256k1>,
    sigma_i: Scalar<Secp256k1>,
}

impl CompletedOfflineStage {
    pub fn public_key(&self) -> &Point<Secp256k1> {
        &self.local_key.y_sum_s
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PartialSignature(Scalar<Secp256k1>);

pub struct Round5 {
    local_signature: LocalSignature,
}


impl Round5 {
    pub fn proceed(
        self,
        input: BroadcastMsgs<PartialSignature>,
    ) -> Result<SignatureRecid>
    {
        let sigs = input.into_vec().into_iter().map(|s_i| s_i.0.clone()).collect::<Vec<_>>();
        self.local_signature
            .output_signature(&sigs)
            .map_err(Error::Round7)
    }

    pub fn expects_messages(i: u16, n: u16) -> Store<BroadcastMsgs<PartialSignature>> {
        containers::BroadcastMsgsStore::new(i, n)
    }

    pub fn is_expensive(&self) -> bool {
        true
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("round 1: {0:?}")]
    Round1(ErrorType),
    #[error("round 2 stage 3: {0:?}")]
    Round2Stage3(crate::Error),
    #[error("round 2 stage 4: {0:?}")]
    Round2Stage4(ErrorType),
    #[error("round 3: {0:?}")]
    Round3(ErrorType),
    #[error("round 5: {0:?}")]
    Round5(ErrorType),
    #[error("round 6: verify proof: {0:?}")]
    Round6VerifyProof(ErrorType),
    #[error("round 6: check sig: {0:?}")]
    Round6CheckSig(crate::Error),
    #[error("round 7: {0:?}")]
    Round7(crate::Error),
}

trait IteratorExt: Iterator {
    fn unzip3<A, B, C>(self) -> (Vec<A>, Vec<B>, Vec<C>)
    where
        Self: Iterator<Item = (A, B, C)> + Sized,
    {
        let (mut a, mut b, mut c) = (vec![], vec![], vec![]);
        for (a_i, b_i, c_i) in self {
            a.push(a_i);
            b.push(b_i);
            c.push(c_i);
        }
        (a, b, c)
    }
}

impl<I> IteratorExt for I where I: Iterator {}
