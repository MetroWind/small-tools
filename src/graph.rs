use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use xml::reader;

use crate::layout;
use crate::config;
use crate::draw;

pub struct Graph
{
    config: config::Config,
    column: layout::Column,
}

fn makeCell(attrs: &Vec<xml::attribute::OwnedAttribute>) ->
    layout::Cell
{
    let mut cell = layout::Cell::default();
    for attr in attrs
    {
        match attr.name.local_name.as_str()
        {
            "label" => cell.label = attr.value.clone(),
            "address" => cell.address = attr.value.clone(),
            "size" => cell.size = attr.value.parse().unwrap(),
            _ => {},
        }
    }
    cell
}

#[derive(PartialEq)]
enum State
{
    Cell, Meh,
}

impl Graph
{
    // A really ugly function to read XML. But it sorta kinda works...
    pub fn fromFile(filename: &str) -> Self
    {
        let file = File::open(filename).unwrap();
        let file = BufReader::new(file);

        let parser = reader::EventReader::new(file);
        let mut column = layout::Column::new();
        let mut scope = layout::Scope::new("");
        let mut cell = layout::Cell::default();
        let mut state = State::Meh;
        let mut pointer = layout::Pointer::new(("", ""), ("", ""));

        for e in parser
        {
            match e
            {
                Ok(xml::reader::XmlEvent::StartElement { name, attributes, .. }) =>
                {
                    match name.local_name.as_str()
                    {
                        "cell" => {
                            state = State::Cell;
                            cell = makeCell(&attributes);
                        },
                        "scope" => {
                            for attr in &attributes
                            {
                                if attr.name.local_name == "name"
                                {
                                    scope.name = attr.value.clone();
                                }
                            }
                        },
                        "from" => {
                            for attr in &attributes
                            {
                                match attr.name.local_name.as_str()
                                {
                                    "scope" =>
                                        pointer.from.scope = attr.value.clone(),
                                    "cell" =>
                                        pointer.from.cell = attr.value.clone(),
                                    _ => {},
                                }
                            }
                        },
                        "to" => {
                            for attr in &attributes
                            {
                                match attr.name.local_name.as_str()
                                {
                                    "scope" =>
                                        pointer.to.scope = attr.value.clone(),
                                    "cell" =>
                                        pointer.to.cell = attr.value.clone(),
                                    _ => {},
                                }
                            }
                        },
                        _ => {},
                    }
                },

                Ok(xml::reader::XmlEvent::EndElement { name }) =>
                {
                    match name.local_name.as_str()
                    {
                        "cell" => {
                            scope.addCell(cell.clone());
                            cell = layout::Cell::default();
                            state = State::Meh;
                        },
                        "scope" => {
                            column.addScope(scope.clone());
                            scope = layout::Scope::new("");
                        },
                        "pointer" => {
                            column.addPointer(pointer.clone());
                            pointer = layout::Pointer::new(("", ""), ("", ""));
                        },
                        _ => {},
                    }
                },

                Ok(xml::reader::XmlEvent::Characters(s)) => {
                    if state == State::Cell
                    {
                        cell.content = s;
                    }
                },

                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Self {
            config: config::Config::default(),
            column: column,
        }
    }

    pub fn drawToFile(&self, filename: &str)
    {
        let mut canvas = draw::Canvas::newWithConfig(&self.config);
        canvas.drawColumn(&self.column, (0.0, 0.0));
        let mut file = match File::create(filename) {
            Err(_) => panic!("Failed to open file {}", filename),
            Ok(file) => file,
        };

        if file.write_all(canvas.as_string().as_bytes()).is_err()
        {
            panic!("Failed to write file {}", filename);
        }
    }

    pub fn drawToStdout(&self)
    {
        let mut canvas = draw::Canvas::newWithConfig(&self.config);
        canvas.drawColumn(&self.column, (0.0, 0.0));
        canvas.print();
    }
}
