use std::io::Read;
use std::io::Write;
use std::io::{Seek, SeekFrom};
use std::net::TcpStream;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "typograf-client",
    about = "Yet another Artemy Lebedev Studio Typograf console client"
)]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    /// Edit the file inplace
    #[structopt(short, long)]
    inplace: bool,

    /// *Have no idea how it works*: switches xml, mixed or something
    #[structopt(long, default_value = "4")]
    entity_type: u8,

    /// Use <br /> for multiline text
    #[structopt(long, default_value = "0")]
    use_br: u8,

    /// Use <p> for multiline text: 1 is "yes"
    #[structopt(long, default_value = "0")]
    use_p: u8,

    /// *Don't know what it is*, but default is 3
    #[structopt(long, default_value = "3")]
    max_no_br: u8,

    /// Input encoding
    #[structopt(long, default_value = "UTF-8")]
    encoding: String,

    /// Skip front matter header
    #[structopt(short, long)]
    skip_front_matter: bool,
}

const HOST: &str = "typograf.artlebedev.ru";

fn make_soap_request_header_and_body(text: &str, options: &Opt) -> String {
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
        encoding = options.encoding,
        host = HOST,
        text = cleaned_text,
        entity_type = options.entity_type,
        use_br = options.use_br,
        use_p = options.use_p,
        max_no_br = options.max_no_br
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

fn talk_to_webservice(text: &str, options: &Opt) -> std::io::Result<String> {
    let r = make_soap_request_header_and_body(text, options);

    let mut stream = TcpStream::connect(format!("{host}:80", host = HOST))?;
    stream.write_all(r.as_bytes())?;

    let mut output_string = String::new();
    stream.read_to_string(&mut output_string)?;

    output_string = output_string
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">");

    const RESULT_OPEN_TAG: &str = "<ProcessTextResult>";
    const RESULT_CLOSE_TAG: &str = "</ProcessTextResult>";

    let start_at = RESULT_OPEN_TAG.len()
        + output_string
            .find(RESULT_OPEN_TAG)
            .expect("ProcessTextResult opening tag not found");
    let end_at = output_string
        .find(RESULT_CLOSE_TAG)
        .expect("ProcessTextResult closing tag not found");

    // TODO: use drain with outut_string and try to avoid additional allocation
    Ok(String::from(&output_string[start_at..end_at]))
}

fn find_nth_starting_from(haystack: &str, needle: &str, n: usize, offset: usize) -> Option<usize> {
    match haystack[offset..].find(needle) {
        Some(i) => {
            if n == 1 {
                Some(offset + i)
            } else {
                find_nth_starting_from(haystack, needle, n - 1, offset + i + needle.len())
            }
        }
        None => None,
    }
}

fn find_nth(haystack: &str, needle: &str, n: usize) -> Option<usize> {
    find_nth_starting_from(haystack, needle, n, 0)
}

#[cfg(test)]
#[test]
fn find_nth_simple_case() {
    assert_eq!(Some(8), find_nth("foo bar baz", "baz", 1 as usize));
    assert_eq!(None, find_nth("foo bar baz", "baz", 2 as usize));
}

#[cfg(test)]
#[test]
fn find_nth_should_find_nothing_when_n_is_more_than_number_of_needles() {
    assert_eq!(
        None,
        find_nth("foobarbaz", "some-non-existing-needle", 1 as usize)
    );
}

#[cfg(test)]
#[test]
fn find_nth_should_find_a_needle_if_n_is_less_or_eq_than_number_of_needles() {
    assert_eq!(Some(0), find_nth("simple search", "s", 1 as usize));
    assert_eq!(Some(7), find_nth("simple search", "s", 2 as usize));
    assert_eq!(Some(12), find_nth("foo bar BAZ BAZ BAZ", "BAZ", 2 as usize));
    assert_eq!(Some(16), find_nth("foo bar BAZ BAZ BAZ", "BAZ", 3 as usize));
    assert_eq!(None, find_nth("foo bar BAZ BAZ BAZ", "BAZ", 4 as usize));
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&opt.input)?;

    let mut file_contents = String::new();
    f.read_to_string(&mut file_contents)?;
    let file_contents_offset = {
        if !opt.skip_front_matter {
            0
        } else {
            match find_nth(&file_contents, "+++", 2) {
                None => 0,
                Some(i) => i,
            }
        }
    };

    let output_string = talk_to_webservice(&file_contents[file_contents_offset..], &opt)?;
    if !opt.inplace {
        println!(
            "{}{}",
            &file_contents[..file_contents_offset],
            output_string
        );
    } else {
        f.seek(SeekFrom::Start(file_contents_offset as u64))?;
        let output_bytes = output_string.as_bytes();
        f.write_all(output_bytes)?;
        f.set_len((file_contents_offset + output_bytes.len()) as u64)?;
    }
    Ok(())
}
