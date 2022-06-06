use krpc_api::*;
use convert_case::{Case, Casing};
use std::{error::Error, borrow::Borrow};

const API_GEN_PATH: &'static str = "krpc-api/src/generated";
const SERVICE_MOD_CONTENT: &'static str = "pub mod service;\npub mod enumerations;\npub mod classes;";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect_to("127.0.0.1:50000", "api-generator").await?;
    let mut services: Vec<api::Service> = client.get_services().await?.services;
    std::fs::remove_dir_all(API_GEN_PATH)?;
    std::fs::create_dir_all(API_GEN_PATH)?;
    let mod_entries = services.drain(..).map(|v| handle_service(v).map(|v| format!("pub mod {};", v))).collect::<Result<Vec<_>, _>>()?;
    std::fs::write(format!("{}/mod.rs", API_GEN_PATH), mod_entries.join("\n"))?;
    Ok(())
}

fn handle_service(service: api::Service) -> Result<String, Box<dyn Error>> {
    let service_struct_name = format!("{}Service", service.name.to_case(Case::Pascal));
    let service_mod_name = service_struct_name.to_case(Case::Snake);
    let service_directory = format!("{}/{}", API_GEN_PATH, service_mod_name);
    std::fs::create_dir_all(&service_directory)?;
    handle_enumerations(service.enumerations, &service_directory)?;
    let documentation = handle_documentation(&service.documentation)?;
    let service_code = format!(
r#"/*
{documentation}
*/
pub struct {service_struct_name};
impl {service_struct_name} {{
}}"#, service_struct_name = service_struct_name, documentation = documentation
    );
    std::fs::write(format!("{}/service.rs", service_directory), service_code)?;
    std::fs::write(format!("{}/mod.rs", service_directory), SERVICE_MOD_CONTENT)?;
    Ok(service_mod_name)
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
                    if line == "." || line.starts_with("See") { continue; }
                    if append_next {
                        append_next = false;
                        result_lines.last_mut().unwrap().push_str(&format!(" {}", line));
                    } else {
                        result_lines.push(format!("\t\t{}", line));
                    }
                }
            },
            Ok(Event::Eof) => break,
            Ok(Event::Empty(v)) => result_lines.push(format!("\t\t{}", String::from_utf8_lossy(&v.unescaped()?))),
            Err(e) => { return Err(format!("Error at position {}: {:?}", reader.buffer_position(), e).into()); },
            _ => { /* nothing to do here */ },
        }
        buffer.clear();
    }
    Ok(result_lines.join("\n"))
}

fn handle_class(class: api::Class) -> Result<String, Box<dyn Error>> {
    todo!()
}

fn handle_procedure(procedure: api::Procedure) {
}

fn handle_enumerations(mut enumerations: Vec<api::Enumeration>, output_path: &str) -> Result<(), Box<dyn Error>> {
    let enumeration_directory = format!("{}/enumerations", output_path);
    std::fs::create_dir_all(&enumeration_directory)?;
    let mut enumeration_filenames = Vec::new();
    for enumeration in enumerations.drain(..) {
        let enumeration_name = enumeration.name.to_case(Case::Pascal);
        let enumeration_filename = format!("{}_enumeration", enumeration_name.to_case(Case::Snake));
        let enumeration_entries = enumeration.values.iter().map(|v| handle_documentation(&v.documentation).map(|d| format!("/*\n{}\n*/\n{},", d, v.name))).collect::<Result<Vec<_>, _>>()?;
        let documentation = handle_documentation(&enumeration.documentation)?;
        let enumeration = format!(
r#"/*
{documentation}
*/
pub enum {enumeration_name} {{
    {enumeration_entries}
}}"#, enumeration_name = enumeration_name, documentation = documentation, enumeration_entries = enumeration_entries.join("\n").replace("\n", "\n\t"),
        );
        std::fs::write(format!("{}/{}.rs", enumeration_directory, enumeration_filename), enumeration)?;
        enumeration_filenames.push(enumeration_filename);
    }
    std::fs::write(format!("{}/mod.rs", enumeration_directory), enumeration_filenames.drain(..).map(|v| format!("pub mod {};", v)).collect::<Vec<_>>().join("\n"))?;
    Ok(())
}

fn handle_exception(exception: api::Exception) {
}