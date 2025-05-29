use hashlink::LinkedHashMap;
use json::JsonValue;
use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use std::str::FromStr;
use yaml_rust2::yaml::Yaml;
pub fn to_json(val: mlua::Value, depth: u8) -> JsonValue {
    match val {
        mlua::Value::String(string) => JsonValue::String(string.to_string_lossy()),
        mlua::Value::Boolean(bool) => JsonValue::Boolean(bool),
        mlua::Value::Number(num) => JsonValue::Number(num.into()),
        mlua::Value::Integer(num) => JsonValue::Number(num.into()),
        mlua::Value::Table(table) => {
            if depth == 0 {
                return JsonValue::Null;
            }
            let mut is_arr = true;
            let mut key;
            let mut values = vec![];
            let mut obj = json::object::Object::new();
            for pair in table.pairs::<mlua::Value, mlua::Value>() {
                (key, _) = pair.unwrap();
                if let mlua::Value::Integer(num) = key {
                    if num == 0 {
                        is_arr = false;
                    }
                } else {
                    is_arr = false;
                }
            }
            for pair in table.pairs::<String, mlua::Value>() {
                if let Ok((key, val)) = pair {
                    if is_arr {
                        values.push(to_json(val, depth - 1))
                    } else {
                        obj.insert(&key, to_json(val, depth - 1));
                    }
                } else {
                    values.push(JsonValue::Null)
                }
            }
            if is_arr {
                JsonValue::Array(values)
            } else {
                JsonValue::Object(obj)
            }
        }
        _ => JsonValue::Null,
    }
}
pub fn from_json(lua: &mlua::Lua, json_val: &JsonValue) -> mlua::Value {
    match json_val {
        JsonValue::Boolean(bool) => mlua::Value::Boolean(*bool),
        JsonValue::Number(num) => mlua::Value::Number((*num).into()),
        JsonValue::String(string) => mlua::Value::String(lua.create_string(string).unwrap()),
        JsonValue::Short(string) => {
            mlua::Value::String(lua.create_string(string.as_str()).unwrap())
        }
        JsonValue::Array(arr) => {
            let mut i = 1;
            let array = lua.create_table().unwrap();
            for val in arr {
                array.set(i, from_json(lua, val)).unwrap();
                i += 1;
            }
            mlua::Value::Table(array)
        }
        JsonValue::Object(obj) => {
            let table = lua.create_table().unwrap();
            for (key, val) in obj.iter() {
                table.set(key, from_json(lua, val)).unwrap();
            }
            mlua::Value::Table(table)
        }
        _ => mlua::Value::Nil,
    }
}
pub fn to_yaml(val: mlua::Value, depth: u8) -> Yaml {
    if depth == 0 {
        return Yaml::Null;
    }
    match val {
        mlua::Value::String(string) => Yaml::String(string.to_string_lossy()),
        mlua::Value::Boolean(bool) => Yaml::Boolean(bool),
        mlua::Value::Number(num) => Yaml::Real(num.to_string()),
        mlua::Value::Integer(num) => Yaml::Integer(num.into()),
        mlua::Value::Table(table) => {
            let mut is_arr = true;
            let mut key;
            let mut values = vec![];
            let mut obj = LinkedHashMap::new();
            for pair in table.pairs::<mlua::Value, mlua::Value>() {
                (key, _) = pair.unwrap();
                if let mlua::Value::Integer(num) = key {
                    if num == 0 {
                        is_arr = false;
                    }
                } else {
                    is_arr = false;
                }
            }
            for pair in table.pairs::<String, mlua::Value>() {
                if let Ok((key, val)) = pair {
                    if is_arr {
                        values.push(to_yaml(val, depth - 1))
                    } else {
                        obj.insert(Yaml::String(key), to_yaml(val, depth - 1));
                    }
                } else {
                    values.push(Yaml::Null)
                }
            }
            if is_arr {
                Yaml::Array(values)
            } else {
                Yaml::Hash(obj)
            }
        }
        _ => Yaml::Null,
    }
}
pub fn from_yaml(lua: &mlua::Lua, yaml_val: &Yaml) -> mlua::Value {
    match yaml_val {
        Yaml::Boolean(bool) => mlua::Value::Boolean(*bool),
        Yaml::Real(num) => mlua::Value::Number((*num).parse::<f64>().unwrap_or(0.0)),
        Yaml::Integer(num) => mlua::Value::Integer((*num).into()),
        Yaml::String(string) => mlua::Value::String(lua.create_string(string).unwrap()),
        Yaml::Array(arr) => {
            let mut i = 1;
            let array = lua.create_table().unwrap();
            for val in arr {
                array.set(i, from_yaml(lua, val)).unwrap();
                i += 1;
            }
            mlua::Value::Table(array)
        }
        Yaml::Hash(obj) => {
            let table = lua.create_table().unwrap();
            for (key, val) in obj.iter() {
                table
                    .set(key.to_owned().into_string(), from_yaml(lua, val))
                    .unwrap();
            }
            mlua::Value::Table(table)
        }
        _ => mlua::Value::Nil,
    }
}
pub fn to_toml(val: mlua::Value, depth: u8) -> toml::Value {
    match val {
        mlua::Value::String(string) => {
            if let Ok(datatime) = toml::value::Datetime::from_str(&string.to_string_lossy()) {
                toml::Value::Datetime(datatime)
            } else {
                toml::Value::String(string.to_string_lossy())
            }
        }
        mlua::Value::Boolean(bool) => toml::Value::Boolean(bool),
        mlua::Value::Number(num) => toml::Value::Float(num.into()),
        mlua::Value::Integer(num) => toml::Value::Integer(num.into()),
        mlua::Value::Table(table) => {
            if depth == 0 {
                return toml::Value::Array(vec![]);
            }
            let mut is_arr = true;
            let mut key;
            let mut values = vec![];
            let mut obj = toml::map::Map::new();
            for pair in table.pairs::<mlua::Value, mlua::Value>() {
                (key, _) = pair.unwrap();
                if let mlua::Value::Integer(num) = key {
                    if num == 0 {
                        is_arr = false;
                    }
                } else {
                    is_arr = false;
                }
            }
            for pair in table.pairs::<String, mlua::Value>() {
                if let Ok((key, val)) = pair {
                    if is_arr {
                        values.push(to_toml(val, depth - 1))
                    } else {
                        obj.insert(key, to_toml(val, depth - 1));
                    }
                } else {
                    values.push(toml::Value::String(String::new()))
                }
            }
            if is_arr {
                toml::Value::Array(values)
            } else {
                toml::Value::Table(obj)
            }
        }
        _ => toml::Value::String(String::new()),
    }
}
pub fn from_toml(lua: &mlua::Lua, yaml_val: &toml::Value) -> mlua::Value {
    match yaml_val {
        toml::Value::Boolean(bool) => mlua::Value::Boolean(*bool),
        toml::Value::Float(num) => mlua::Value::Number(*num),
        toml::Value::Integer(num) => mlua::Value::Integer(*num),
        toml::Value::String(string) => mlua::Value::String(lua.create_string(string).unwrap()),
        toml::Value::Datetime(datatime) => {
            let datetime_table = lua.create_table().unwrap();
            if let Some(date) = datatime.date {
                if let Err(e) = datetime_table.set("date", date.to_string()) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return mlua::Value::Nil;
                }
            }
            if let Some(time) = datatime.time {
                if let Err(e) = datetime_table.set("time", time.to_string()) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return mlua::Value::Nil;
                }
            }
            if let Some(offset) = datatime.offset {
                if let Err(e) = datetime_table.set("offset", offset.to_string()) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return mlua::Value::Nil;
                }
            }
            mlua::Value::Table(datetime_table)
        }
        toml::Value::Array(arr) => {
            let mut i = 1;
            let array = lua.create_table().unwrap();
            for val in arr {
                array.set(i, from_toml(lua, val)).unwrap();
                i += 1;
            }
            mlua::Value::Table(array)
        }
        toml::Value::Table(obj) => {
            let table = lua.create_table().unwrap();
            for (key, val) in obj.iter() {
                table.set(key.to_owned(), from_toml(lua, val)).unwrap();
            }
            mlua::Value::Table(table)
        }
    }
}
pub fn to_xml(
    table: mlua::Table,
    document: &mut Vec<Event>,
    depth: u8,
    max_depth: u8,
    pretty: bool,
    spaces: u8,
) {
    if depth == max_depth {
        return;
    }
    if let Ok(comment) = table.get::<String>("comment") {
        document.push(Event::Comment(BytesText::new(&comment).into_owned()));
    } else {
        let name = table.get::<String>("name").unwrap_or_default();
        let attributes = table.get::<mlua::Table>("attributes");
        let children = table
            .get::<Vec<mlua::Value>>("children")
            .unwrap_or_default();
        let is_self_closed = if children.len() == 0 {
            table.get("self_closed").unwrap_or(false)
        } else {
            false
        };
        let mut start = BytesStart::new(name.clone());
        if let Ok(attributes) = attributes {
            let (mut key, mut value);
            for pair in attributes.pairs::<String, String>() {
                if pair.is_err() {
                    eprintln!("Faild to convert to XML.\nError: {}", pair.err().unwrap());
                    return;
                } else {
                    (key, value) = pair.unwrap();
                    start.push_attribute((key.as_str(), value.as_str()));
                }
            }
        }
        document.push(if is_self_closed {
            Event::Empty(start)
        } else {
            Event::Start(start)
        });
        let mut doc = document;
        let mut child_table;
        if !is_self_closed {
            let mut next_is_start_offset;
            for (i, child) in children.iter().enumerate() {
                if let Some(mlua::Value::Table(tab)) = children.get(i + 1) {
                    if tab.sequence_values::<String>().count() == 1 {
                        next_is_start_offset = 0;
                    } else {
                        next_is_start_offset = 1;
                    }
                } else {
                    next_is_start_offset = 0;
                }
                if child.is_table() {
                    child_table = child.as_table().unwrap().to_owned();
                    to_xml(child_table, &mut doc, depth + 1, max_depth, pretty, spaces);
                } else if child.to_string().is_ok() {
                    if pretty {
                        doc.push(Event::Text(
                            BytesText::new(
                                format!(
                                    "{}\n{}",
                                    child.to_string().ok().unwrap(),
                                    " ".repeat(((depth + next_is_start_offset) * spaces) as usize),
                                )
                                .as_str(),
                            )
                            .into_owned(),
                        ));
                    } else {
                        doc.push(Event::Text(
                            BytesText::new(child.to_string().ok().unwrap().as_str()).into_owned(),
                        ));
                    }
                }
            }
            doc.push(Event::End(BytesEnd::new(name)));
        }
    }
}
pub fn from_xml(
    lua: &mlua::Lua,
    xml_vec: &Vec<Event>,
    index: usize,
    is_empty: bool,
) -> Option<(mlua::Table, usize)> {
    if let Event::Comment(text) = &xml_vec[index] {
        let text = String::from_utf8(text.to_vec());
        if let Err(e) = text {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        let comment_table = lua.create_table();

        if let Err(e) = comment_table {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        let comment_table = comment_table.unwrap();
        if let Err(e) = comment_table.set("comment", text.unwrap()) {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        return Some((comment_table, index));
    }
    let tag_table = lua.create_table();
    if let Err(e) = tag_table {
        eprintln!("Faild to parse XML.\nError: {e}");
        return None;
    }
    let tag_table = tag_table.unwrap();
    if is_empty {
        if let Err(e) = tag_table.set("self_closed", true) {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
    }
    let attributes = lua.create_table();
    if let Err(e) = attributes {
        eprintln!("Faild to parse XML.\nError: {e}");
        return None;
    }
    let attributes = attributes.unwrap();
    let children = lua.create_table();
    if let Err(e) = children {
        eprintln!("Faild to parse XML.\nError: {e}");
        return None;
    }
    let children = children.unwrap();
    let mut ind = index + 1;
    if let Event::Start(event) | Event::Empty(event) = &xml_vec[index] {
        let name = String::from_utf8(event.name().into_inner().to_vec());
        if let Err(e) = name {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        if let Err(e) = tag_table.set("name", name.unwrap()) {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        let mut attribute;
        let (mut key, mut val);
        for attr in event.attributes() {
            if attr.is_err() {
                eprintln!("Faild to parse XML.\nError: {}", attr.err().unwrap());
                return None;
            }
            attribute = attr.unwrap();
            key = String::from_utf8(attribute.key.into_inner().to_vec());
            if key.is_err() {
                eprintln!("Faild to parse XML.\nError: {}", key.err().unwrap());
                return None;
            }
            val = attribute.unescape_value();
            if val.is_err() {
                eprintln!("Faild to parse XML.\nError: {}", val.err().unwrap());
                return None;
            }
            if let Err(e) = attributes.set(key.unwrap(), val.unwrap()) {
                eprintln!("Faild to parse XML.\nError: {e}");
                return None;
            }
        }
        if let Err(e) = tag_table.set("attributes", attributes) {
            eprintln!("Faild to parse XML.\nError: {e}");
            return None;
        }
        if is_empty {
            return Some((tag_table, index));
        }
    }
    let mut tag;
    let mut depth = 0;
    loop {
        if ind >= xml_vec.len() {
            return Some((tag_table, ind));
        }
        match &xml_vec[ind] {
            Event::Start(_) | Event::Empty(_) | Event::Comment(_) => {
                if let Event::Start(_) = &xml_vec[ind] {
                    depth = 1;
                } else {
                    depth = 0;
                }
                tag = from_xml(
                    lua,
                    xml_vec,
                    ind,
                    if let Event::Empty(_) = &xml_vec[ind] {
                        true
                    } else {
                        false
                    },
                );
                if tag.is_none() {
                    return None;
                }
                ind = tag.as_ref().unwrap().1;
                if let Err(e) = children.push(tag.unwrap().0) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return None;
                }
            }
            Event::Text(text) => {
                let text = String::from_utf8(text.to_vec());
                if let Err(e) = text {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return None;
                }
                if let Err(e) = children.push(text.unwrap()) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return None;
                }
            }
            Event::End(_) => {
                if depth == 1 {
                    depth = 0;
                    continue;
                }
                if let Err(e) = tag_table.set("children", children) {
                    eprintln!("Faild to parse XML.\nError: {e}");
                    return None;
                }
                return Some((tag_table, ind));
            }
            _ => return None,
        }
        ind += 1;
    }
}
pub fn to_csv(table: mlua::Table, headers: Option<mlua::Table>) -> String {
    let mut writer = csv::Writer::from_writer(vec![]);
    let mut field_values = vec![];
    if headers.is_some()
        && headers
            .as_ref()
            .unwrap()
            .sequence_values::<mlua::Value>()
            .count()
            != 0
    {
        for header in headers.as_ref().unwrap().sequence_values::<String>() {
            if let Ok(header) = header {
                if let Err(e) = writer.write_field(&header) {
                    eprintln!("Faild to convert header {header} to CSV.\nError: {e}");
                    return String::new();
                }
            }
        }
        if let Err(e) = writer.write_record(None::<&[u8]>) {
            eprintln!("Faild to convert to CSV.\nError: {e}");
            return String::new();
        }
        for (i, record) in table.sequence_values::<mlua::Value>().enumerate() {
            if let Ok(mlua::Value::Table(record)) = record {
                if record.sequence_values::<String>().count() == 0 {
                    for header in headers.as_ref().unwrap().sequence_values::<String>() {
                        if header.is_ok()
                            && record
                                .get::<String>(header.as_ref().unwrap().to_owned())
                                .is_ok()
                        {
                            field_values.push(
                                record
                                    .get::<String>(header.as_ref().unwrap().to_owned())
                                    .unwrap(),
                            )
                        } else {
                            eprintln!("Faild to convert to CSV.\nError: {}", header.err().unwrap());
                            return String::new();
                        }
                    }
                } else {
                    for (j, field) in record.sequence_values::<String>().enumerate() {
                        if let Ok(field) = field {
                            if let Err(e) = writer.write_field(field) {
                                eprintln!(
                                    "Faild to convert field {j} in record {i} to CSV.\nError: {e}"
                                );
                                return String::new();
                            }
                        } else {
                            eprintln!("Faild to convert to CSV.\nError: {}", field.err().unwrap());
                            return String::new();
                        }
                    }
                }
                if let Err(e) = writer.write_record(&field_values) {
                    eprintln!("Faild to convert record number {i} to CSV.\nError: {e}");
                    return String::new();
                }
                field_values.clear();
            } else {
                eprintln!("Faild to convert record number {i} to CSV.\nError: Invalid table");
                continue;
            }
        }
    } else {
        if let Some(Ok(mlua::Value::Table(first_table))) =
            table.sequence_values::<mlua::Value>().next()
        {
            if first_table.sequence_values::<String>().count() != 0 {
                let mut fields_values = vec![];
                for (i, record) in table.sequence_values::<mlua::Value>().enumerate() {
                    if let Ok(mlua::Value::Table(record)) = record {
                        if record.sequence_values::<String>().count()
                            != record.pairs::<String, String>().count()
                        {
                            eprintln!(
                                "Faild to convert to CSV.\nError: Record number {i} is invalid"
                            );
                            return String::new();
                        }
                        if record.sequence_values::<String>().count() != 0 {
                            for (j, field) in record.sequence_values::<String>().enumerate() {
                                match field {
                                    Ok(field) => fields_values.push(field),
                                    Err(e) => {
                                        eprintln!("Faild to convert field {j} in record {i} to CSV.\nError: {e}");
                                        return String::new();
                                    }
                                }
                            }
                            if let Err(e) = writer.write_record(&fields_values) {
                                eprintln!("Faild to convert to CSV.\nError: {e}");
                                return String::new();
                            }
                            fields_values.clear();
                        } else {
                            eprintln!(
                                "Faild to convert to CSV.\nError: Record number {i} is invalid"
                            );
                            return String::new();
                        }
                    } else {
                        eprintln!(
                            "Faild to convert record number {i} to CSV.\nError: Invalid table"
                        );
                        return String::new();
                    }
                }
            } else if first_table.pairs::<String, String>().count() != 0 {
                let mut field_values: Vec<String> = vec![];
                for header in first_table
                    .pairs::<String, String>()
                    .filter(|h| h.is_ok())
                    .map(|h| h.unwrap().0)
                {
                    field_values.push(header);
                }
                if let Err(e) = writer.write_record(&field_values) {
                    eprintln!("Faild to convert to CSV.\nError: {e}");
                    return String::new();
                }
                field_values.clear();
                for (i, record) in table.sequence_values::<mlua::Value>().enumerate() {
                    if let Ok(mlua::Value::Table(record)) = &record {
                        if record.pairs::<String, String>().count() != 0 {
                            for header in first_table
                                .pairs::<String, String>()
                                .filter(|h| h.is_ok())
                                .map(|h| h.unwrap().0)
                            {
                                match record.get::<String>(header.as_str()) {
                                    Ok(field) => field_values.push(field),
                                    Err(e) => {
                                        eprintln!("Faild to convert to CSV.\nError: {e}");
                                        return String::new();
                                    }
                                }
                            }
                            if let Err(e) = writer.write_record(&field_values) {
                                eprintln!("Faild to convert to CSV.\nError: {e}");
                                return String::new();
                            }
                            field_values.clear();
                        }
                    } else {
                        eprintln!("Faild to convert to CSV.\nError: Record number {i} is invalid");
                        return String::new();
                    }
                }
            } else {
                eprintln!("Faild to convert table to CSV.\nError: Invalid values for csv table");
                return String::new();
            }
        } else {
            eprintln!("Faild to convert table to CSV.\nError: Invalid values for csv table");
            return String::new();
        }
    }
    if let Ok(writer) = writer.into_inner() {
        if let Ok(csv_str) = String::from_utf8(writer) {
            return csv_str;
        }
    }
    String::new()
}
pub fn from_csv(lua: &mlua::Lua, csv_str: String) -> Option<mlua::Table> {
    let mut reader = csv::ReaderBuilder::new().from_reader(csv_str.as_bytes());
    let headers = reader.headers();
    let csv_table = lua.create_table();
    let (mut record_table_res, mut record_table);
    if let Err(e) = csv_table {
        eprintln!("Faild to parse CSV\nError:{e}");
        return None;
    }
    let csv_table = csv_table.unwrap();
    let headers = if let Ok(headers) = headers {
        headers
            .iter()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>()
            .clone()
    } else {
        vec![]
    };
    let records = reader.records();
    let mut is_err = false;
    for record in records {
        record_table_res = lua.create_table();
        if let Err(e) = record_table_res {
            eprintln!("Faild to parse CSV\nError:{e}");
            is_err = true;
            continue;
        }
        record_table = record_table_res.unwrap();
        if record.is_err() {
            is_err = true;
            continue;
        }
        for (i, field) in record.unwrap().iter().enumerate() {
            if i >= headers.len() {
                break;
            }
            if let Err(e) = record_table.set(headers[i].as_str(), field) {
                eprintln!("Faild to parse CSV\nError:{e}");
                return None;
            }
        }
        if let Err(e) = csv_table.push(record_table) {
            eprintln!("Faild to parse CSV\nError:{e}");
            is_err = true;
            continue;
        }
    }
    if is_err {
        eprintln!("Some records has been skiped as a result of parsing error with those records");
    }
    Some(csv_table)
}
