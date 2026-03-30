#![allow(dead_code)]

use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

pub fn generate_workspace(manifest_dir: &Path) -> Result<(), String> {
    let forum = ApiSpec::load(manifest_dir.join("forum.json"), "ForumApi", "forum")?;
    let market = ApiSpec::load(manifest_dir.join("market.json"), "MarketApi", "market")?;

    let forum_generated = ApiGenerator::new(&forum).generate()?;
    let market_generated = ApiGenerator::new(&market).generate()?;

    write_file(
        &manifest_dir.join("src/forum/types.rs"),
        &forum_generated.types_code,
    )?;
    write_file(
        &manifest_dir.join("src/forum/methods.rs"),
        &forum_generated.methods_code,
    )?;
    write_file(
        &manifest_dir.join("src/market/types.rs"),
        &market_generated.types_code,
    )?;
    write_file(
        &manifest_dir.join("src/market/methods.rs"),
        &market_generated.methods_code,
    )?;

    let models = r#"//! Shared models and helpers.
//!
//! This file is intentionally small: operation-specific request and response
//! types are generated into `src/forum/types.rs` and `src/market/types.rs`.

#![allow(clippy::all)]
"#;
    write_file(&manifest_dir.join("src/models.rs"), models)?;

    Ok(())
}

pub fn check_workspace(manifest_dir: &Path) -> Result<(), String> {
    let forum = ApiSpec::load(manifest_dir.join("forum.json"), "ForumApi", "forum")?;
    let market = ApiSpec::load(manifest_dir.join("market.json"), "MarketApi", "market")?;
    let forum_generated = ApiGenerator::new(&forum).generate()?;
    let market_generated = ApiGenerator::new(&market).generate()?;

    check_file(
        &manifest_dir.join("src/forum/types.rs"),
        &forum_generated.types_code,
    )?;
    check_file(
        &manifest_dir.join("src/forum/methods.rs"),
        &forum_generated.methods_code,
    )?;
    check_file(
        &manifest_dir.join("src/market/types.rs"),
        &market_generated.types_code,
    )?;
    check_file(
        &manifest_dir.join("src/market/methods.rs"),
        &market_generated.methods_code,
    )?;
    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|err| format!("failed to write {}: {err}", path.display()))
}

fn check_file(path: &Path, expected: &str) -> Result<(), String> {
    let current = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
    if current != expected {
        return Err(format!(
            "{} is out of date. Run `cargo run --bin lzt-codegen -- generate`.",
            path.display()
        ));
    }
    Ok(())
}

#[derive(Clone)]
struct ApiSpec {
    path: PathBuf,
    client_type: &'static str,
    module_name: &'static str,
    doc_title: String,
    root: Value,
}

impl ApiSpec {
    fn load(
        path: PathBuf,
        client_type: &'static str,
        module_name: &'static str,
    ) -> Result<Self, String> {
        let raw = fs::read_to_string(&path)
            .map_err(|err| format!("failed to read {}: {err}", path.display()))?;
        let root: Value = serde_json::from_str(&raw)
            .map_err(|err| format!("failed to parse {}: {err}", path.display()))?;
        let doc_title = root
            .get("info")
            .and_then(|info| info.get("title"))
            .and_then(Value::as_str)
            .unwrap_or(module_name)
            .to_string();
        Ok(Self {
            path,
            client_type,
            module_name,
            doc_title,
            root,
        })
    }

    fn paths(&self) -> &Map<String, Value> {
        self.root["paths"]
            .as_object()
            .expect("openapi paths must be an object")
    }

    fn components_parameters(&self) -> Option<&Map<String, Value>> {
        self.root
            .get("components")
            .and_then(|value| value.get("parameters"))
            .and_then(Value::as_object)
    }
}

#[derive(Clone)]
struct Parameter {
    name: String,
    location: String,
    required: bool,
    schema: Value,
}

#[derive(Clone)]
enum RequestBody {
    Json { struct_name: String },
    Multipart { struct_name: String },
}

#[derive(Clone)]
struct Operation {
    operation_id: String,
    method_name: String,
    type_prefix: String,
    http_method: String,
    path: String,
    tag: Option<String>,
    path_params: Vec<Parameter>,
    query_params: Vec<Parameter>,
    request_body: Option<RequestBody>,
    response_type: String,
}

#[derive(Clone)]
struct ApiGenerated {
    types_code: String,
    methods_code: String,
}

#[derive(Copy, Clone)]
enum StructMode {
    Output,
    Input,
}

struct ApiGenerator<'a> {
    spec: &'a ApiSpec,
    definitions: BTreeMap<String, String>,
    defined_names: HashSet<String>,
    reference_names: HashMap<String, String>,
}

impl<'a> ApiGenerator<'a> {
    fn new(spec: &'a ApiSpec) -> Self {
        Self {
            spec,
            definitions: BTreeMap::new(),
            defined_names: HashSet::new(),
            reference_names: HashMap::new(),
        }
    }

    fn generate(mut self) -> Result<ApiGenerated, String> {
        let operations = self.collect_operations()?;
        let types_code = self.render_types(&operations);
        let methods_code = self.render_methods(&operations);
        Ok(ApiGenerated {
            types_code,
            methods_code,
        })
    }

    fn collect_operations(&mut self) -> Result<Vec<Operation>, String> {
        let mut operations = Vec::new();
        let mut path_keys: Vec<_> = self.spec.paths().keys().cloned().collect();
        path_keys.sort();

        for path_key in path_keys {
            let path_item = &self.spec.paths()[&path_key];
            let item_parameters = path_item
                .get("parameters")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();

            for method in ["get", "post", "put", "delete", "patch"] {
                let Some(operation) = path_item.get(method) else {
                    continue;
                };

                let operation_id = operation["operationId"]
                    .as_str()
                    .ok_or_else(|| format!("missing operationId for {method} {path_key}"))?
                    .to_string();
                let type_prefix = to_pascal_case(&operation_id);
                let method_name = to_snake_case(&operation_id);

                let mut parameters = item_parameters.clone();
                if let Some(op_parameters) = operation.get("parameters").and_then(Value::as_array) {
                    parameters.extend(op_parameters.iter().cloned());
                }
                let parameters = self.resolve_parameters(&parameters)?;
                let path_param_names = extract_path_param_names(&path_key);
                let mut path_params = Vec::new();
                let mut query_params = Vec::new();
                for param in parameters {
                    match param.location.as_str() {
                        "path" => path_params.push(param),
                        "query" => query_params.push(param),
                        _ => {}
                    }
                }
                path_params.sort_by_key(|param| {
                    path_param_names
                        .iter()
                        .position(|name| name == &param.name)
                        .unwrap_or(usize::MAX)
                });
                query_params.sort_by_key(|param| param.name.clone());

                let request_body = self.generate_request_body(operation, &type_prefix)?;
                let response_type = self.generate_response_type(operation, &type_prefix)?;

                operations.push(Operation {
                    operation_id,
                    method_name,
                    type_prefix,
                    http_method: method.to_string(),
                    path: path_key.clone(),
                    tag: operation
                        .get("tags")
                        .and_then(Value::as_array)
                        .and_then(|tags| tags.first())
                        .and_then(Value::as_str)
                        .map(ToOwned::to_owned),
                    path_params,
                    query_params,
                    request_body,
                    response_type,
                });
            }
        }

        Ok(operations)
    }

    fn resolve_parameters(&self, params: &[Value]) -> Result<Vec<Parameter>, String> {
        let mut resolved = Vec::new();
        let mut seen = BTreeSet::new();

        for raw_param in params {
            let param = self.resolve_parameter(raw_param)?;
            let key = format!("{}:{}", param.location, param.name);
            if seen.insert(key) {
                resolved.push(param);
            }
        }

        Ok(resolved)
    }

    fn resolve_parameter(&self, raw: &Value) -> Result<Parameter, String> {
        let param = if let Some(reference) = raw.get("$ref").and_then(Value::as_str) {
            let name = ref_name(reference)?;
            self.spec
                .components_parameters()
                .and_then(|map| map.get(name))
                .ok_or_else(|| {
                    format!(
                        "unknown parameter ref {reference} in {}",
                        self.spec.path.display()
                    )
                })?
        } else {
            raw
        };

        let schema = param
            .get("schema")
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));

        Ok(Parameter {
            name: param["name"]
                .as_str()
                .ok_or_else(|| "parameter without name".to_string())?
                .to_string(),
            location: param["in"]
                .as_str()
                .ok_or_else(|| "parameter without location".to_string())?
                .to_string(),
            required: param
                .get("required")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            schema,
        })
    }

    fn generate_request_body(
        &mut self,
        operation: &Value,
        type_prefix: &str,
    ) -> Result<Option<RequestBody>, String> {
        let Some(request_body) = operation.get("requestBody") else {
            return Ok(None);
        };
        let body = if let Some(reference) = request_body.get("$ref").and_then(Value::as_str) {
            return Err(format!("unsupported requestBody ref {reference}"));
        } else {
            request_body
        };
        let Some(content) = body.get("content").and_then(Value::as_object) else {
            return Ok(None);
        };

        if let Some(json) = content.get("application/json") {
            let struct_name = format!("{type_prefix}Request");
            let schema = json
                .get("schema")
                .cloned()
                .unwrap_or_else(|| Value::Object(Map::new()));
            self.define_input_shape(&struct_name, &schema);
            return Ok(Some(RequestBody::Json { struct_name }));
        }

        if let Some(multipart) = content.get("multipart/form-data") {
            let struct_name = format!("{type_prefix}Multipart");
            let schema = multipart
                .get("schema")
                .cloned()
                .unwrap_or_else(|| Value::Object(Map::new()));
            self.define_input_shape(&struct_name, &schema);
            return Ok(Some(RequestBody::Multipart { struct_name }));
        }

        Ok(None)
    }

    fn generate_response_type(
        &mut self,
        operation: &Value,
        type_prefix: &str,
    ) -> Result<String, String> {
        let responses = operation
            .get("responses")
            .and_then(Value::as_object)
            .ok_or_else(|| "operation without responses".to_string())?;
        let mut codes: Vec<_> = responses.keys().cloned().collect();
        codes.sort();

        for code in codes {
            if !code.starts_with('2') {
                continue;
            }
            let response = &responses[&code];
            let Some(schema) = response
                .get("content")
                .and_then(|content| content.get("application/json"))
                .and_then(|meta| meta.get("schema"))
            else {
                return Ok("serde_json::Value".to_string());
            };
            let response_name = format!("{type_prefix}Response");
            let rust_type = self.define_output_shape(&response_name, schema);
            return Ok(rust_type);
        }

        Ok("serde_json::Value".to_string())
    }

    fn render_types(&mut self, operations: &[Operation]) -> String {
        let mut code = String::new();
        writeln!(
            code,
            "//! Auto-generated types for {}.\n//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.\n",
            self.spec.doc_title
        )
        .unwrap();
        writeln!(code, "#![allow(clippy::all)]").unwrap();
        writeln!(code).unwrap();
        writeln!(code, "use serde::{{Deserialize, Serialize}};").unwrap();
        writeln!(code, "use std::collections::BTreeMap;").unwrap();
        writeln!(code).unwrap();

        for operation in operations {
            if !operation.query_params.is_empty() {
                let struct_name = format!("{}Query", operation.type_prefix);
                writeln!(
                    code,
                    "/// Query parameters for `{}`.",
                    operation.operation_id
                )
                .unwrap();
                code.push_str(&self.render_param_struct(&struct_name, &operation.query_params));
                writeln!(code).unwrap();
            }
        }

        for definition in self.definitions.values() {
            code.push_str(definition);
            writeln!(code).unwrap();
        }

        code
    }

    fn render_methods(&self, operations: &[Operation]) -> String {
        let mut code = String::new();
        writeln!(
            code,
            "//! Auto-generated methods for {}.\n//! DO NOT EDIT -- run `cargo run --bin lzt-codegen -- generate`.\n",
            self.spec.doc_title
        )
        .unwrap();
        writeln!(code, "#![allow(clippy::all)]").unwrap();
        writeln!(code).unwrap();
        writeln!(code, "use crate::client::HttpMethod;").unwrap();
        writeln!(code, "use crate::error::Result;").unwrap();
        writeln!(code, "use crate::{}::types::*;", self.spec.module_name).unwrap();
        writeln!(code).unwrap();
        writeln!(
            code,
            "impl crate::{}::{} {{",
            self.spec.module_name, self.spec.client_type
        )
        .unwrap();

        let mut current_tag = String::new();
        for operation in operations {
            let tag = operation.tag.clone().unwrap_or_else(|| "Misc".to_string());
            if current_tag != tag {
                current_tag = tag.clone();
                writeln!(code, "    // -- {} --", tag).unwrap();
            }

            let doc_path = format!(
                "{} {}",
                operation.http_method.to_uppercase(),
                operation.path
            );
            writeln!(code, "    /// `{doc_path}`").unwrap();

            let mut args = Vec::new();
            for param in &operation.path_params {
                args.push(format!(
                    "{}: {}",
                    field_ident(&param.name),
                    self.schema_to_rust_type(
                        &format!(
                            "{}{}Path",
                            operation.type_prefix,
                            to_pascal_case(&param.name)
                        ),
                        &param.schema,
                        StructMode::Input,
                    )
                ));
            }

            if !operation.query_params.is_empty() {
                args.push(format!("query: {}Query", operation.type_prefix));
            }

            if let Some(body) = &operation.request_body {
                let body_type = match body {
                    RequestBody::Json { struct_name } | RequestBody::Multipart { struct_name } => {
                        struct_name.clone()
                    }
                };
                args.push(format!("body: {body_type}"));
            }

            let signature = if args.is_empty() {
                String::new()
            } else {
                format!(", {}", args.join(", "))
            };
            writeln!(
                code,
                "    pub async fn {}(&self{signature}) -> Result<{}> {{",
                operation.method_name, operation.response_type
            )
            .unwrap();

            let path_expr = format_path_expression(&operation.path, &operation.path_params);
            writeln!(code, "        let path = {path_expr};").unwrap();

            let query_expr = if operation.query_params.is_empty() {
                "None::<&()>".to_string()
            } else {
                "Some(&query)".to_string()
            };

            match &operation.request_body {
                None => {
                    writeln!(
                        code,
                        "        self.client.request_json(HttpMethod::{}, &path, {query_expr}, None::<&()>).await",
                        to_pascal_case(&operation.http_method)
                    )
                    .unwrap();
                }
                Some(RequestBody::Json { .. }) => {
                    writeln!(
                        code,
                        "        self.client.request_json(HttpMethod::{}, &path, {query_expr}, Some(&body)).await",
                        to_pascal_case(&operation.http_method)
                    )
                    .unwrap();
                }
                Some(RequestBody::Multipart { .. }) => {
                    writeln!(
                        code,
                        "        self.client.request_multipart(HttpMethod::{}, &path, {query_expr}, &body).await",
                        to_pascal_case(&operation.http_method)
                    )
                    .unwrap();
                }
            }
            writeln!(code, "    }}").unwrap();
            writeln!(code).unwrap();
        }

        writeln!(code, "}}").unwrap();
        code
    }

    fn define_input_shape(&mut self, name: &str, schema: &Value) {
        self.define_shape(name, schema, StructMode::Input);
    }

    fn define_output_shape(&mut self, name: &str, schema: &Value) -> String {
        self.define_shape(name, schema, StructMode::Output)
    }

    fn define_shape(&mut self, name: &str, schema: &Value, mode: StructMode) -> String {
        let name = self.unique_name(name);
        self.prepare_nested_types(&name, schema, mode);
        let rust_type = self.schema_to_rust_type(&name, schema, mode);
        if rust_type == name {
            self.realize_type_definition(&name, schema, mode);
        } else if !self.definitions.contains_key(&name) {
            let mut alias = String::new();
            writeln!(alias, "pub type {name} = {rust_type};").unwrap();
            self.definitions.insert(name.clone(), alias);
        }
        name
    }

    fn unique_name(&mut self, name: &str) -> String {
        if self.defined_names.insert(name.to_string()) {
            return name.to_string();
        }
        let mut index = 2;
        loop {
            let candidate = format!("{name}{index}");
            if self.defined_names.insert(candidate.clone()) {
                return candidate;
            }
            index += 1;
        }
    }

    fn schema_to_rust_type(&self, name: &str, schema: &Value, mode: StructMode) -> String {
        let _ = mode;
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            return self.reference_type_name(reference);
        }
        if let Some(any_of) = schema.get("anyOf").and_then(Value::as_array) {
            if let Some(optional_schema) = extract_optional_variant(any_of) {
                return format!(
                    "Option<{}>",
                    self.schema_to_rust_type(name, optional_schema, mode)
                );
            }
            return "serde_json::Value".to_string();
        }
        if let Some(one_of) = schema.get("oneOf").and_then(Value::as_array) {
            if let Some(optional_schema) = extract_optional_variant(one_of) {
                return format!(
                    "Option<{}>",
                    self.schema_to_rust_type(name, optional_schema, mode)
                );
            }
            return "serde_json::Value".to_string();
        }

        if let Some(schema_type) = schema
            .get("type")
            .and_then(Value::as_array)
            .and_then(|types| {
                if types.len() == 2 && types.iter().any(|item| item == "null") {
                    types.iter().find_map(Value::as_str)
                } else {
                    None
                }
            })
        {
            if schema_type != "null" {
                let mut inner = schema.clone();
                inner["type"] = Value::String(schema_type.to_string());
                return format!("Option<{}>", self.schema_to_rust_type(name, &inner, mode));
            }
        }

        match schema.get("type").and_then(Value::as_str) {
            Some("string") => "String".to_string(),
            Some("integer") => "i64".to_string(),
            Some("number") => "f64".to_string(),
            Some("boolean") => "bool".to_string(),
            Some("array") => {
                let item_type = schema
                    .get("items")
                    .map(|items| self.schema_to_rust_type(&format!("{name}Item"), items, mode))
                    .unwrap_or_else(|| "serde_json::Value".to_string());
                format!("Vec<{item_type}>")
            }
            Some("object") => {
                if schema
                    .get("properties")
                    .and_then(Value::as_object)
                    .is_some()
                {
                    name.to_string()
                } else if let Some(additional) = schema.get("additionalProperties") {
                    if additional == &Value::Bool(true) {
                        "BTreeMap<String, serde_json::Value>".to_string()
                    } else {
                        format!(
                            "BTreeMap<String, {}>",
                            self.schema_to_rust_type(&format!("{name}Value"), additional, mode)
                        )
                    }
                } else {
                    "serde_json::Value".to_string()
                }
            }
            _ => {
                if schema
                    .get("properties")
                    .and_then(Value::as_object)
                    .is_some()
                {
                    name.to_string()
                } else {
                    "serde_json::Value".to_string()
                }
            }
        }
    }

    fn reference_type_name(&self, reference: &str) -> String {
        if reference.contains("/components/schemas/")
            || reference.contains("/components/parameters/")
        {
            format!(
                "{}{}",
                to_pascal_case(self.spec.module_name),
                to_pascal_case(reference_leaf_name(reference))
            )
        } else {
            to_pascal_case(reference_leaf_name(reference))
        }
    }

    fn define_reference(&mut self, reference: &str) -> String {
        if let Some(existing) = self.reference_names.get(reference) {
            return existing.clone();
        }
        let rust_name = self.reference_type_name(reference);
        let schema = self
            .resolve_reference(reference)
            .cloned()
            .unwrap_or_else(|| Value::Object(Map::new()));
        self.reference_names
            .insert(reference.to_string(), rust_name.clone());
        self.realize_type_definition(&rust_name, &schema, StructMode::Output);
        rust_name
    }

    fn resolve_reference(&self, reference: &str) -> Option<&Value> {
        let pointer = reference.strip_prefix('#')?;
        self.spec.root.pointer(pointer)
    }

    fn realize_type_definition(&mut self, name: &str, schema: &Value, mode: StructMode) {
        if self.definitions.contains_key(name) {
            return;
        }
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            let target = self.define_reference(reference);
            if target == name {
                if let Some(resolved) = self.resolve_reference(reference).cloned() {
                    self.realize_type_definition(name, &resolved, mode);
                }
            } else {
                self.definitions
                    .insert(name.to_string(), format!("pub type {name} = {target};\n"));
            }
            return;
        }
        if schema.get("properties").is_some()
            || schema.get("type").and_then(Value::as_str) == Some("object")
        {
            let definition = self.render_struct(name, schema, mode);
            self.definitions.insert(name.to_string(), definition);
            return;
        }
        let rust_type = self.schema_to_rust_type(name, schema, mode);
        if rust_type != name {
            self.definitions.insert(
                name.to_string(),
                format!("pub type {name} = {rust_type};\n"),
            );
        }
    }

    fn render_struct(&mut self, name: &str, schema: &Value, mode: StructMode) -> String {
        let properties = schema
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let required: HashSet<String> = schema
            .get("required")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default();

        let mut definition = String::new();
        writeln!(
            definition,
            "#[derive(Debug, Clone, Serialize, Deserialize, Default)]"
        )
        .unwrap();
        writeln!(definition, "#[serde(default)]").unwrap();
        writeln!(definition, "pub struct {name} {{").unwrap();

        let mut property_names: Vec<_> = properties.keys().cloned().collect();
        property_names.sort();
        let mut used_field_names = HashSet::new();
        for property_name in property_names {
            let property_schema = &properties[&property_name];
            if let Some(reference) = property_schema.get("$ref").and_then(Value::as_str) {
                self.define_reference(reference);
            }
            let field_name = unique_field_name(field_ident(&property_name), &mut used_field_names);
            let nested_name = format!("{name}{}", to_pascal_case(&property_name));
            self.prepare_nested_types(&nested_name, property_schema, mode);
            let mut field_type = self.schema_to_rust_type(&nested_name, property_schema, mode);
            let is_required = required.contains(&property_name);
            if !is_required {
                field_type = format!("Option<{field_type}>");
                writeln!(
                    definition,
                    "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                )
                .unwrap();
            }
            if field_name != property_name {
                writeln!(definition, "    #[serde(rename = \"{property_name}\")]").unwrap();
            }
            writeln!(definition, "    pub {field_name}: {field_type},").unwrap();
        }

        writeln!(definition, "}}\n").unwrap();
        definition
    }

    fn prepare_nested_types(&mut self, name: &str, schema: &Value, mode: StructMode) {
        if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
            self.define_reference(reference);
            return;
        }

        if let Some(any_of) = schema.get("anyOf").and_then(Value::as_array) {
            if let Some(optional_schema) = extract_optional_variant(any_of) {
                self.prepare_nested_types(name, optional_schema, mode);
            }
            return;
        }

        if let Some(one_of) = schema.get("oneOf").and_then(Value::as_array) {
            if let Some(optional_schema) = extract_optional_variant(one_of) {
                self.prepare_nested_types(name, optional_schema, mode);
            }
            return;
        }

        if let Some(schema_type) = schema.get("type").and_then(Value::as_str) {
            match schema_type {
                "object" => {
                    if schema
                        .get("properties")
                        .and_then(Value::as_object)
                        .is_some()
                    {
                        self.realize_type_definition(name, schema, mode);
                    } else if let Some(additional) = schema.get("additionalProperties") {
                        self.prepare_nested_types(&format!("{name}Value"), additional, mode);
                    }
                }
                "array" => {
                    if let Some(items) = schema.get("items") {
                        self.prepare_nested_types(&format!("{name}Item"), items, mode);
                    }
                }
                _ => {}
            }
            return;
        }

        if schema
            .get("properties")
            .and_then(Value::as_object)
            .is_some()
        {
            self.realize_type_definition(name, schema, mode);
        }
    }

    fn render_param_struct(&mut self, name: &str, params: &[Parameter]) -> String {
        let mut definition = String::new();
        writeln!(
            definition,
            "#[derive(Debug, Clone, Serialize, Deserialize, Default)]"
        )
        .unwrap();
        writeln!(definition, "#[serde(default)]").unwrap();
        writeln!(definition, "pub struct {name} {{").unwrap();
        let mut used_field_names = HashSet::new();
        for param in params {
            let field_name = unique_field_name(field_ident(&param.name), &mut used_field_names);
            let nested_name = format!("{name}{}", to_pascal_case(&param.name));
            self.prepare_nested_types(&nested_name, &param.schema, StructMode::Input);
            let mut field_type =
                self.schema_to_rust_type(&nested_name, &param.schema, StructMode::Input);
            if !param.required {
                field_type = format!("Option<{field_type}>");
                writeln!(
                    definition,
                    "    #[serde(skip_serializing_if = \"Option::is_none\")]"
                )
                .unwrap();
            }
            if field_name != param.name {
                writeln!(definition, "    #[serde(rename = \"{}\")]", param.name).unwrap();
            }
            writeln!(definition, "    pub {field_name}: {field_type},").unwrap();
        }
        writeln!(definition, "}}\n").unwrap();
        definition
    }
}

fn format_path_expression(path: &str, path_params: &[Parameter]) -> String {
    if path_params.is_empty() {
        return format!("\"{path}\".to_string()");
    }
    let mut expression = format!("format!(\"{path}\"");
    for param in path_params {
        let ident = field_ident(&param.name);
        write!(expression, ", {} = {}", param.name, ident).unwrap();
    }
    expression.push(')');
    expression
}

fn extract_path_param_names(path: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut chars = path.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut name = String::new();
            while let Some(next) = chars.peek() {
                if *next == '}' {
                    chars.next();
                    break;
                }
                name.push(*next);
                chars.next();
            }
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

fn extract_optional_variant(variants: &[Value]) -> Option<&Value> {
    if variants.len() != 2 {
        return None;
    }
    let mut non_null = None;
    for variant in variants {
        if variant.get("type").and_then(Value::as_str) == Some("null") {
            continue;
        }
        non_null = Some(variant);
    }
    non_null
}

fn ref_name(reference: &str) -> Result<&str, String> {
    reference
        .rsplit('/')
        .next()
        .ok_or_else(|| format!("invalid ref {reference}"))
}

fn reference_leaf_name(reference: &str) -> &str {
    let mut segments = reference.split('/').filter(|segment| !segment.is_empty());
    let mut last = segments.next().unwrap_or("Unknown");
    for segment in segments {
        if segment != "schema" {
            last = segment;
        }
    }
    last
}

fn to_pascal_case(input: &str) -> String {
    let words = split_words(input);
    if words.is_empty() {
        return "Generated".to_string();
    }
    words
        .into_iter()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut out = String::new();
                    out.push(first.to_ascii_uppercase());
                    out.push_str(chars.as_str());
                    out
                }
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn to_snake_case(input: &str) -> String {
    let words = split_words(input);
    if words.is_empty() {
        return "generated".to_string();
    }
    let mut out = words.join("_").to_ascii_lowercase();
    if out
        .chars()
        .next()
        .map(|ch| ch.is_ascii_digit())
        .unwrap_or(false)
    {
        out = format!("field_{out}");
    }
    if is_rust_keyword(&out) {
        out.push('_');
    }
    out
}

fn field_ident(input: &str) -> String {
    to_snake_case(input)
}

fn unique_field_name(base: String, used: &mut HashSet<String>) -> String {
    if used.insert(base.clone()) {
        return base;
    }
    let mut index = 2;
    loop {
        let candidate = format!("{base}_{index}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        index += 1;
    }
}

fn split_words(input: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    let mut previous_lowercase = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && previous_lowercase && !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            current.push(ch.to_ascii_lowercase());
            previous_lowercase = ch.is_ascii_lowercase();
        } else if !current.is_empty() {
            words.push(current.clone());
            current.clear();
            previous_lowercase = false;
        } else {
            previous_lowercase = false;
        }
    }

    if !current.is_empty() {
        words.push(current);
    }
    words
}

fn is_rust_keyword(value: &str) -> bool {
    matches!(
        value,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "async"
            | "await"
            | "dyn"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "try"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
    )
}
