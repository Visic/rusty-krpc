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
    let mut result_filenames = Vec::new();
    for ele in services.drain(..) { result_filenames.append(&mut handle_service(ele)?); }
    let mod_content = result_filenames.drain(..).map(|v| format!("pub mod {};", v)).collect::<Vec<_>>();
    std::fs::write(format!("{}/mod.rs", API_GEN_PATH), mod_content.join("\n"))?;
    Ok(())
}

fn handle_service(service: api::Service) -> Result<Vec<String>, Box<dyn Error>> {
    let documentation = handle_documentation(&service.documentation)?;
    let mut result_filenames = service.enumerations.iter().map(|v| handle_enumeration(v)).collect::<Result<Vec<_>, _>>()?;
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
    result_filenames.push(service_filename);
    Ok(result_filenames)
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

fn handle_enumeration(enumeration: &api::Enumeration) -> Result<String, Box<dyn Error>> {
    let documentation = handle_documentation(&enumeration.documentation)?;
    let enumeration_name = enumeration.name.to_case(Case::Pascal);
    let enumeration_entries = enumeration.values.iter().map(|v| handle_documentation(&v.documentation).map(|d| format!("/*\n{}\n*/\n{},", d, v.name))).collect::<Result<Vec<_>, _>>()?;
    let enumeration = format!(
r#"/*
{documentation}
*/
pub enum {enumeration_name} {{
    {enumeration_entries}
}}"#, enumeration_name = enumeration_name, documentation = documentation, enumeration_entries = enumeration_entries.join("\n").replace("\n", "\n\t"),
    );
    let enumeration_filename = format!("{}_enumeration", enumeration_name.to_case(Case::Snake));
    std::fs::write(format!("{}/{}.rs", API_GEN_PATH, enumeration_filename), enumeration)?;
    Ok(enumeration_filename)
}

fn handle_exception(exception: api::Exception) {
}