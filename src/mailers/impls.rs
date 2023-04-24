use actix_web::Result;
use infer::Infer;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;

use crate::traits::{GetString, ToJson};
use crate::Mailer;
use crate::Payload;

impl Mailer {
    pub fn send_mail<T, S, B, F, N>(&self, to: T, subject: S, body: B, attachments: &[(F, N)]) -> Result<String>
        where T: ToString,
              S: ToString,
              B: ToString,
              F: ToString,
              N: ToString
    {
        // Retrieve values
        let data = match self.to_json() {
            Some(data) => data,
            None => return Err(Payload::error("Your platform's email configuration is invalid. Please contact your administrator"))
        };

        // Set bindings
        let to = to.to_string();
        let subject = subject.to_string();
        let body = body.to_string();

        let sender = data.sender.map_or(String::default(), |d| d.get_string().unwrap_or(String::default()));
        let username = data.username.map_or(String::default(), |d| d.get_string().unwrap_or(String::default()));
        let password = data.password.map_or(String::default(), |d| d.get_string().unwrap_or(String::default()));
        let smtp_host = data.smtp_host.map_or(String::default(), |d| d.get_string().unwrap_or(String::default()));

        // Create multipart body
        let mut multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body)
            );

        // Check if file exists
        for (f, n) in attachments {
            let filename = f.to_string();
            let name = n.to_string();
            match std::fs::read(&filename) {
                Ok(file) => {
                    // Check out mime type
                    let info = Infer::new();
                    match ContentType::parse(&info
                        .get(&file.clone())
                        .map_or(String::default(), |t| String::from(t.mime_type()))) {
                        Ok(content_type) => {
                            multipart = multipart.singlepart(
                                Attachment::new(name).body(file, content_type)
                            );
                        },
                        Err(_) => continue
                    };
                },
                Err(_) => continue
            };
        }

        // Create email builder
        let builder = match Message::builder()
            .from(sender.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart) {
            Ok(builder) => builder,
            Err(error) => return Err(Payload::error(&error))
        };

        // Set credentials
        let credentials = Credentials::new(username, password);

        // Set smtp transport relay
        let relay = match SmtpTransport::relay(smtp_host.as_str()) {
            Ok(relay) => relay,
            Err(error) => return Err(Payload::error(&error))
        };

        // Open a remote connection
        let mailer = relay.credentials(credentials).build();

        // Send the email
        match mailer.send(&builder) {
            Ok(_) => Ok(format!("Email sent successfully to {to}")),
            Err(error) => Err(Payload::error(&error)),
        }
    }
}