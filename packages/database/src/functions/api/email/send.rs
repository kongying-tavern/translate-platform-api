#[cfg(test)]
mod test {
    use lettre::{
        message::header::ContentType,
        transport::smtp::authentication::Credentials,
        {Message, SmtpTransport, Transport},
    };

    #[test]
    pub fn test() {
        let email = Message::builder()
            .from("langyo <hydrosource@sinap.ac.cn>".parse().unwrap())
            .to("langyo <hydrosource@sinap.ac.cn>".parse().unwrap())
            .subject("lettre 框架验证码测试")
            .header(ContentType::TEXT_HTML)
            .body("<h1>来自 lettre 框架的测试</h1><br /><p>验证码：<b>233333</b></p>".to_string())
            .unwrap();

        let creds = Credentials::new("hydrosource@sinap.ac.cn".to_owned(), "xxx".to_owned());

        let mailer = SmtpTransport::relay("mail.cstnet.cn")
            .unwrap()
            .port(994)
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}
