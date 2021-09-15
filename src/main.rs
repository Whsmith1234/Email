extern crate imap;
extern crate native_tls;
use std::fs;
fn main() {
	 fetch_inbox_top();
}
fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "outlook.office365.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login("email", "password")
        .map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    let f = imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    for x in 0..10{
        let d = (f.exists-x).to_string();
        let messages = imap_session.fetch(d,"RFC822.TEXT")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(None);
        };

        // extract the message's body
         let body = message.text().expect("hey");
         let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        
        println!("{}",body );
        fs::write("bar.txt", body);
    };

    imap_session.logout()?;

    Ok(Some("hey".to_string()))
}
