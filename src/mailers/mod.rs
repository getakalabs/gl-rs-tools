pub mod mutations;

use arraygen::Arraygen;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::traits::prelude::*;
use crate::Cipher;
use crate::DBClient;
use crate::Errors;
use crate::Module;
use crate::Settings;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Arraygen)]
#[gen_array(fn get_ciphers: &mut Option<Cipher>)]
pub struct Mailer {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub sender: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub username: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub password: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub smtp_host: Option<Cipher>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[in_array(get_ciphers)]
    pub service: Option<Cipher>,
}

impl IsEmpty for Mailer {
    fn is_empty(&self) -> bool {
        self.clone() == Self::default()
    }
}

impl Decrypt for Mailer {
    fn decrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.decrypt_master()
                }
            });
        }

        data
    }
}

impl Encrypt for Mailer {
    fn encrypt(&self) -> Self {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = cipher.clone().and_then(|d| {
                match d.is_empty() {
                    true => None,
                    false => d.encrypt_master()
                }
            });
        }

        data
    }
}

impl ToBson for Mailer {
    fn to_bson(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => data.encrypt_master()
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl ToJson for Mailer {
    fn to_json(&self) -> Option<Self> {
        let mut data = self.clone();

        for cipher in data.get_ciphers() {
            *cipher = match cipher {
                None => None,
                Some(data) => {
                    match data.is_empty() {
                        true => None,
                        false => {
                            let data = data.set_to_string();
                            match data.is_empty() {
                                true => None,
                                false => Some(data)
                            }
                        }
                    }
                }
            };
        }

        match data.is_empty() {
            true => None,
            false => Some(data)
        }
    }
}

impl Mailer {
    pub fn new(form: &Settings) -> Self {
        Self {
            sender: Cipher::new(form.get_sender()),
            username: Cipher::new(form.get_username()),
            password: Cipher::new(form.get_password()),
            smtp_host: Cipher::new(form.get_smtp_host()),
            service: Cipher::new(form.get_service()),
        }
    }

    pub async fn stage(client: &DBClient) -> Arc<RwLock<Mailer>> {
        let db = match client.get_db() {
            None => return Arc::new(RwLock::new(Mailer::default())),
            Some(client) => client
        };

        let settings = match Settings::read_from_module(&db, &Module::Mailer).await {
            Ok(settings) => settings,
            Err(_) => return Arc::new(RwLock::new(Mailer::default()))
        };

        let data = settings
            .mailer
            .map_or(Mailer::default(), |d| d.decrypt());

        Arc::new(RwLock::new(data))
    }

    pub fn send_mail<T, S, B>(&self, to: T, subject: S, body: B) -> Result<String, Errors>
        where T: ToString,
              S: ToString,
              B: ToString
    {
        // Set bindings
        let to = to.to_string();
        let subject = subject.to_string();
        let body = body.to_string();

        // Retrieve values
        let data = match self.to_json() {
            None => return Err(Errors::new("Your platform's email configuration is invalid. Please contact your administrator")),
            Some(value) => value
        };

        let sender = data.sender.map_or(String::default(), |d| d.to_string().unwrap_or(String::default()));
        let username = data.username.map_or(String::default(), |d| d.to_string().unwrap_or(String::default()));
        let password = data.password.map_or(String::default(), |d| d.to_string().unwrap_or(String::default()));
        let smtp_host = data.smtp_host.map_or(String::default(), |d| d.to_string().unwrap_or(String::default()));

        // Check if self has data
        let is_empty = sender.is_empty() || to.is_empty() || subject.is_empty() || body.is_empty();
        if is_empty {
            return Err(Errors::new("Your platform's email configuration is invalid. Please contact your administrator"));
        }

        // Create multipart body
        let multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_HTML)
                    .body(body)
            );

        // Create email builder
        let builder = match Message::builder()
            .from(sender.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart) {
            Ok(builder) => builder,
            Err(error) =>  return Err(Errors::new(&error))
        };

        // Set credentials
        let credentials = Credentials::new(username, password);

        // Set smtp transport relay
        let relay = match SmtpTransport::relay(smtp_host.as_str()) {
            Ok(relay) => relay,
            Err(error) => return Err(Errors::new(&error))
        };

        // Open a remote connection
        let mailer = relay.credentials(credentials).build();

        // Send the email
        match mailer.send(&builder) {
            Ok(_) => Ok(format!("Email send successfully to {to}")),
            Err(e) => Err(Errors::new(&e)),
        }
    }
}