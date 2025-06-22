#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use webauthn_rs::prelude::CreationChallengeResponse;
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use webauthn_rs_proto::CreationChallengeResponse;

#[derive(Clone)]
#[allow(unused)]
pub enum StartAuthenticationUIState {
    WaitingForInput {
        error: Option<String>,
    },
    PerformingCCR {
        username: String,
        ccr_url: String,
    },
    RegisteringChallenge {
        passkey_state_id: uuid::Uuid,
        ccr: CreationChallengeResponse,
    },
}
/// NOTE: partialeq does not check if ccr is equal! if the ccr changes, the ID must change too for the state to update.
/// CCR does not implement PartialEq
/// If you want to submit a pull request, or fork and add derive partial eq, or implement partialeq yourself, then go ahead
impl PartialEq for StartAuthenticationUIState {
    /// NOTE: eq does not check if ccr is equal! if the ccr changes, the ID must change too for the state to update.
    /// CCR does not implement PartialEq
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                &Self::WaitingForInput { error: ref error1 },
                &Self::WaitingForInput { error: ref error2 },
            ) => error1 == error2,
            (
                &Self::PerformingCCR {
                    username: ref u1,
                    ccr_url: ref c1,
                },
                &Self::PerformingCCR {
                    username: ref u2,
                    ccr_url: ref c2,
                },
            ) => u1 == u2 && c1 == c2,
            (
                &Self::RegisteringChallenge {
                    passkey_state_id: p1,
                    ccr: _,
                },
                &Self::RegisteringChallenge {
                    passkey_state_id: p2,
                    ccr: _,
                },
            ) => p1 == p2,
            _ => false,
        }
    }
}

// #[derive(Serialize,Deserialize)]
// pub enum FinishWebAuthnRegistrationResponseBody {
//     Ok,
//     Err(FinishWebAuthnRegistrationResponseErrorKind),
// }
