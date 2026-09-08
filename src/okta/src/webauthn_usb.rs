//! USB CTAP2 implementation for [`crate::webauthn::WebAuthnAuthenticator`].
use crate::webauthn::{
    AssertionCredential, AttestationCredential, WebAuthnAuthenticator, WebAuthnError,
};
use authenticator::{
    authenticatorservice::{AuthenticatorService, RegisterArgs, SignArgs},
    ctap2::{
        client_data::{Challenge, CollectedClientData, WebauthnType},
        server::{
            AuthenticationExtensionsClientInputs, PublicKeyCredentialDescriptor,
            PublicKeyCredentialParameters, PublicKeyCredentialUserEntity, RelyingParty,
            ResidentKeyRequirement, Transport, UserVerificationRequirement,
        },
    },
    statecallback::StateCallback,
    Pin, StatusPinUv, StatusUpdate,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ring::digest;
use serde_json::Value as Json;
use std::{
    sync::mpsc::{channel, Sender},
    time::Duration,
};
use zeroize::Zeroizing;

pub struct UsbAuthenticator {
    service: AuthenticatorService,
    timeout: Duration,
    pin: Option<Zeroizing<String>>,
}
impl std::fmt::Debug for UsbAuthenticator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UsbAuthenticator")
            .field("timeout", &self.timeout)
            .field("pin", &self.pin.as_ref().map(|_| "[redacted]"))
            .finish()
    }
}
impl UsbAuthenticator {
    pub fn new(timeout: Duration) -> Result<Self, WebAuthnError> {
        let mut service = AuthenticatorService::new().map_err(auth_error)?;
        service.add_u2f_usb_hid_platform_transports();
        Ok(Self {
            service,
            timeout,
            pin: None,
        })
    }
    /// Configure a CTAP PIN. The value is redacted and zeroized by this wrapper after use/drop.
    pub fn set_pin(&mut self, pin: Option<Zeroizing<String>>) {
        self.pin = pin;
    }
    fn status(&self) -> Sender<StatusUpdate> {
        let (pin_tx, pin_rx) = channel();
        let pin = self.pin.clone();
        std::thread::spawn(move || {
            while let Ok(status) = pin_rx.recv() {
                match status {
                    StatusUpdate::PinUvError(StatusPinUv::PinRequired(sender))
                    | StatusUpdate::PinUvError(StatusPinUv::InvalidPin(sender, _)) => {
                        if let Some(pin) = pin.as_ref() {
                            let _ = sender.send(Pin::new(pin));
                        }
                    }
                    _ => {}
                }
            }
        });
        pin_tx
    }
}
fn auth_error(error: impl std::fmt::Debug) -> WebAuthnError {
    WebAuthnError::Authenticator(format!("{error:?}"))
}
fn bytes(value: &Json) -> Result<Vec<u8>, WebAuthnError> {
    value
        .as_array()
        .ok_or(WebAuthnError::InvalidRequest)?
        .iter()
        .map(|v| {
            v.as_f64()
                .filter(|n| n.is_finite() && n.fract() == 0.0 && *n >= 0.0 && *n <= 255.0)
                .map(|n| n as u8)
                .ok_or(WebAuthnError::InvalidRequest)
        })
        .collect()
}
fn text(value: &Json) -> Result<String, WebAuthnError> {
    value
        .as_str()
        .map(str::to_owned)
        .ok_or(WebAuthnError::InvalidRequest)
}
fn uv(value: &Json) -> UserVerificationRequirement {
    match value.as_str() {
        Some("required") => UserVerificationRequirement::Required,
        Some("discouraged") => UserVerificationRequirement::Discouraged,
        _ => UserVerificationRequirement::Preferred,
    }
}
fn resident(value: &Json) -> ResidentKeyRequirement {
    match value.as_str() {
        Some("required") => ResidentKeyRequirement::Required,
        Some("preferred") => ResidentKeyRequirement::Preferred,
        _ => ResidentKeyRequirement::Discouraged,
    }
}
fn transport(value: &Json) -> Option<Transport> {
    match value.as_str() {
        Some("usb") => Some(Transport::USB),
        Some("nfc") => Some(Transport::NFC),
        Some("ble") => Some(Transport::BLE),
        Some("internal") => Some(Transport::Internal),
        _ => None,
    }
}
fn descriptors(value: &Json) -> Result<Vec<PublicKeyCredentialDescriptor>, WebAuthnError> {
    value
        .as_array()
        .ok_or(WebAuthnError::InvalidRequest)?
        .iter()
        .map(|item| {
            Ok(PublicKeyCredentialDescriptor {
                id: bytes(&item["id"])?,
                transports: item
                    .get("transports")
                    .and_then(Json::as_array)
                    .map(|items| items.iter().filter_map(transport).collect())
                    .unwrap_or_default(),
            })
        })
        .collect()
}
fn client_data(
    kind: WebauthnType,
    challenge: &Json,
    origin: &str,
) -> Result<(Vec<u8>, [u8; 32]), WebAuthnError> {
    let data = CollectedClientData {
        webauthn_type: kind,
        challenge: Challenge(URL_SAFE_NO_PAD.encode(bytes(challenge)?)),
        origin: origin.to_string(),
        cross_origin: false,
        token_binding: None,
    };
    let json = serde_json::to_vec(&data).map_err(|_| WebAuthnError::InvalidRequest)?;
    let hash = digest::digest(&digest::SHA256, &json);
    let mut out = [0; 32];
    out.copy_from_slice(hash.as_ref());
    Ok((json, out))
}
impl WebAuthnAuthenticator for UsbAuthenticator {
    fn get(&mut self, origin: &str, request: &Json) -> Result<AssertionCredential, WebAuthnError> {
        let public = &request["publicKey"];
        let (client_data_json, client_data_hash) =
            client_data(WebauthnType::Get, &public["challenge"], origin)?;
        let allow_list = descriptors(&public["allowCredentials"])?;
        let rp_id = text(&public["rpId"])?;
        let args = SignArgs {
            client_data_hash,
            origin: origin.to_string(),
            relying_party_id: rp_id,
            allow_list: allow_list.clone(),
            user_verification_req: uv(&public["userVerification"]),
            user_presence_req: true,
            extensions: AuthenticationExtensionsClientInputs::default(),
            pin: self.pin.as_ref().map(|v| Pin::new(v)),
            use_ctap1_fallback: true,
        };
        let (tx, result_rx) = channel();
        let callback = StateCallback::new(Box::new(move |result| {
            let _ = tx.send(result);
        }));
        self.service
            .sign(
                self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
                args,
                self.status(),
                callback,
            )
            .map_err(auth_error)?;
        let result = result_rx
            .recv_timeout(self.timeout + Duration::from_secs(1))
            .map_err(auth_error)?
            .map_err(auth_error)?;
        let assertion = result.assertion;
        let id = assertion
            .credentials
            .as_ref()
            .map(|v| v.id.as_slice())
            .or_else(|| allow_list.first().map(|v| v.id.as_slice()))
            .ok_or(WebAuthnError::InvalidRequest)?;
        Ok(AssertionCredential {
            id: URL_SAFE_NO_PAD.encode(id),
            client_data_json,
            authenticator_data: assertion.auth_data.to_vec(),
            signature: assertion.signature,
        })
    }
    fn create(
        &mut self,
        origin: &str,
        request: &Json,
    ) -> Result<AttestationCredential, WebAuthnError> {
        let public = &request["publicKey"];
        let (client_data_json, client_data_hash) =
            client_data(WebauthnType::Create, &public["challenge"], origin)?;
        let rp = &public["rp"];
        let user = &public["user"];
        let params = public["pubKeyCredParams"]
            .as_array()
            .ok_or(WebAuthnError::InvalidRequest)?
            .iter()
            .map(|v| {
                PublicKeyCredentialParameters::try_from(
                    v["alg"].as_i64().ok_or(WebAuthnError::InvalidRequest)? as i32,
                )
                .map_err(auth_error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let selection = &public["authenticatorSelection"];
        let resident = if selection["requireResidentKey"].as_bool() == Some(true) {
            ResidentKeyRequirement::Required
        } else {
            resident(&selection["residentKey"])
        };
        let args = RegisterArgs {
            client_data_hash,
            relying_party: RelyingParty {
                id: text(&rp["id"])?,
                name: rp["name"].as_str().map(str::to_owned),
            },
            origin: origin.to_string(),
            user: PublicKeyCredentialUserEntity {
                id: bytes(&user["id"])?,
                name: user["name"].as_str().map(str::to_owned),
                display_name: user["displayName"].as_str().map(str::to_owned),
            },
            pub_cred_params: params,
            exclude_list: descriptors(&public["excludeCredentials"])?,
            user_verification_req: uv(&selection["userVerification"]),
            resident_key_req: resident,
            extensions: AuthenticationExtensionsClientInputs::default(),
            pin: self.pin.as_ref().map(|v| Pin::new(v)),
            use_ctap1_fallback: true,
        };
        let (tx, rx) = channel();
        let callback = StateCallback::new(Box::new(move |result| {
            let _ = tx.send(result);
        }));
        self.service
            .register(
                self.timeout.as_millis().try_into().unwrap_or(u64::MAX),
                args,
                self.status(),
                callback,
            )
            .map_err(auth_error)?;
        let result = rx
            .recv_timeout(self.timeout + Duration::from_secs(1))
            .map_err(auth_error)?
            .map_err(auth_error)?;
        let id = result
            .att_obj
            .auth_data
            .credential_data
            .as_ref()
            .map(|v| v.credential_id.as_slice())
            .ok_or(WebAuthnError::InvalidRequest)?;
        let attestation_object = serde_cbor::to_vec(&result.att_obj).map_err(auth_error)?;
        Ok(AttestationCredential {
            id: URL_SAFE_NO_PAD.encode(id),
            client_data_json,
            attestation_object,
            transports: vec!["usb".into()],
        })
    }
}
