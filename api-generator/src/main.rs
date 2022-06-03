use krpc_api::*;
use convert_case::{Case, Casing};
use std::{error::Error, borrow::Borrow};

const API_GEN_PATH: &'static str = "krpc-api/src/generated";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect_to("127.0.0.1:50000", "api-generator").await?;
    let mut services: Vec<api::Service> = client.get_services().await?.services;
    std::fs::remove_dir_all(API_GEN_PATH)?;
    std::fs::create_dir_all(API_GEN_PATH)?;
    let service_names_iter = services.drain(..).map(|v| handle_service(v));
    let mod_content = service_names_iter.map(|v| v.map(|v| format!("pub mod {};", v))).collect::<Result<Vec<_>, _>>()?;
    std::fs::write(format!("{}/mod.rs", API_GEN_PATH), mod_content.join("\n"))?;
    Ok(())
}

fn handle_service(service: api::Service) -> Result<String, Box<dyn Error>> {
    let documentation = handle_documentation(&service.documentation)?;
    for e in service.enumerations.iter() { handle_enumeration(e); } //TODO
    let service_struct_name = format!("{}Service", service.name.to_case(Case::Pascal));
    let service_code = format!(
r#"/*
{documentation}
*/
pub struct {service_struct_name};
impl {service_struct_name} {{
}}"#, service_struct_name = service_struct_name, documentation = documentation
    );
    let service_filename = service_struct_name.to_case(Case::Snake);
    std::fs::write(format!("{}/{}.rs", API_GEN_PATH, service_filename), service_code)?;
    Ok(service_filename)
}

fn handle_documentation(documentation: &str) -> Result<String, Box<dyn Error>> {
    use quick_xml::{Reader, events::Event};
    let (mut result_lines, mut buffer, mut append_next) = (Vec::new(), Vec::new(), false);
    let mut reader = Reader::from_str(documentation);
    reader.trim_text(true);
    loop {
        match reader.read_event(&mut buffer) {
            Ok(Event::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name());
                match name.borrow() {
                    "doc" => { /*skip this one*/ },
                    "a" => { append_next = true; },
                    v => result_lines.push(format!("\t{}:", v))
                }
            },
            Ok(Event::Text(e)) => {
                for line in e.unescape_and_decode(&reader)?.lines() {
                    if line == "." { continue; }
                    if append_next {
                        append_next = false;
                        result_lines.last_mut().unwrap().push_str(&format!(" {}", line));
                    } else {
                        result_lines.push(format!("\t\t{}", line));
                    }
                }
            },
            Ok(Event::Eof) => break,
            Err(e) => { return Err(format!("Error at position {}: {:?}", reader.buffer_position(), e).into()); },
            _ => { /* nothing to do here */ },
        }
        buffer.clear();
    }
    Ok(result_lines.join("\n"))
}

fn handle_procedure(procedure: api::Procedure) {
}

fn handle_enumeration(enumeration: &api::Enumeration) {
    println!("{:#?}", enumeration);
}

fn handle_exception(exception: api::Exception) {
}