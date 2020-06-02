use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

const HOST: &str = "typograf.artlebedev.ru";

fn make_soap_request_header_and_body(text: &str) -> String {
    let cleaned_text = text
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;");

    let soap_body = format!(
        "<?xml version=\"1.0\" encoding=\"{encoding}\"?>\n\
        <soap:Envelope xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n\
                        xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\"\n\
                        xmlns:soap=\"http://schemas.xmlsoap.org/soap/envelope/\">\n\
            <soap:Body>\n\
                <ProcessText xmlns=\"http://{host}/webservices/\">\n\
                    <text>{text}</text>\n\
                    <entityType>{entity_type}</entityType>\n\
                    <useBr>{use_br}</useBr>\n\
                    <useP>{use_p}</useP>\n\
                    <maxNobr>{max_no_br}</maxNobr>\n\
                </ProcessText>\n\
            </soap:Body>\n\
        </soap:Envelope>",
        encoding = "UTF-8",
        host = HOST,
        text = cleaned_text,
        entity_type = 4,
        use_br = 0,
        use_p = 0,
        max_no_br = 3
    );

    format!(
        "POST /webservices/typograf.asmx HTTP/1.1\n\
        Host: {host}\n\
        Content-Type: text/xml\n\
        Content-Length: {soap_body_len}\n\
        SOAPAction: \"http://{host}/webservices/ProcessText\"\n\n{soap_body}",
        host = HOST,
        soap_body_len = soap_body.len(),
        soap_body = soap_body
    )
}

fn main() -> std::io::Result<()> {
    let r = make_soap_request_header_and_body(
        "\"Вы все еще кое-как верстаете в \"Ворде\"? - Тогда мы идем к вам!\"",
    );

    println!("{}", r);

    let mut stream = TcpStream::connect(format!("{host}:80", host = HOST))?;
    stream.write_all(r.as_bytes())?;

    let mut output_string = String::new();
    stream.read_to_string(&mut output_string)?;

    output_string = output_string
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">");

    println!("{}", output_string);

    Ok(())
}
