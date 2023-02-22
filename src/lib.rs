use std::convert;
use std::fmt;
use std::fs;
use std::path;

use an_rope::Rope;

//use yaml_rust::{
//    parser::{Event, MarkedEventReceiver, Parser},
//    scanner::{Marker, TScalarStyle},
//};

use anyhow::{Context, Error, Result};

#[derive(Default, Debug, Clone)]
pub struct YamlEd {
    content: Rope,
}

impl convert::From<&str> for YamlEd {
    fn from(content: &str) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl convert::From<String> for YamlEd {
    fn from(content: String) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl convert::TryFrom<&path::Path> for YamlEd {
    type Error = Error;
    fn try_from(path: &path::Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("could not read file: {}", path.display()))?;
        Ok(content.into())
    }
}

impl convert::TryFrom<&path::PathBuf> for YamlEd {
    type Error = Error;
    fn try_from(path: &path::PathBuf) -> Result<Self> {
        Self::try_from(path.as_path())
    }
}

impl fmt::Display for YamlEd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl convert::From<YamlEd> for String {
    fn from(src: YamlEd) -> Self {
        src.content.to_string()
    }
}

//impl YamlEd {
//pub fn load_from_str(&mut self, source: &str) {
//    let mut parser = Parser::new(source.chars());
//    parser.load(self, true).unwrap();
//}
//}

//impl<'a> MarkedEventReceiver for YamlLoader<'a> {
//    // event: (StreamStart, Marker { index: 0, line: 1, col: 0 })
//
//    // event: (DocumentStart, Marker { index: 1, line: 1, col: 1 })
//    // event: (MappingStart(0), Marker { index: 1, line: 1, col: 1 })
//    // event: (Scalar("a", Plain, 0, None), Marker { index: 0, line: 1, col: 0 })
//    // event: (Scalar("1", DoubleQuoted, 0, None), Marker { index: 3, line: 1, col: 3 })
//    // event: (Scalar("b", Plain, 0, None), Marker { index: 7, line: 2, col: 0 })
//    // event: (MappingStart(0), Marker { index: 14, line: 3, col: 4 })
//    // event: (Scalar("bb", Plain, 0, None), Marker { index: 12, line: 3, col: 2 })
//    // event: (Scalar("2", DoubleQuoted, 0, None), Marker { index: 16, line: 3, col: 6 })
//    // event: (MappingEnd, Marker { index: 20, line: 4, col: 0 })
//    // event: (Scalar("c", Plain, 0, None), Marker { index: 20, line: 4, col: 0 })
//    // event: (MappingStart(0), Marker { index: 27, line: 5, col: 4 })
//    // event: (Scalar("cc", Plain, 0, None), Marker { index: 25, line: 5, col: 2 })
//    // event: (MappingStart(0), Marker { index: 36, line: 6, col: 7 })
//    // event: (Scalar("ccc", Plain, 0, None), Marker { index: 33, line: 6, col: 4 })
//    // event: (Scalar("3", DoubleQuoted, 0, None), Marker { index: 38, line: 6, col: 9 })
//    // event: (MappingEnd, Marker { index: 42, line: 7, col: 0 })
//    // event: (MappingEnd, Marker { index: 42, line: 7, col: 0 })
//    // event: (MappingEnd, Marker { index: 42, line: 7, col: 0 })
//    // event: (DocumentEnd, Marker { index: 42, line: 7, col: 0 })
//
//    // event: (DocumentStart, Marker { index: 42, line: 7, col: 0 })
//    // event: (MappingStart(0), Marker { index: 47, line: 8, col: 1 })
//    // event: (Scalar("d", Plain, 0, None), Marker { index: 46, line: 8, col: 0 })
//    // event: (Scalar("4", DoubleQuoted, 0, None), Marker { index: 49, line: 8, col: 3 })
//    // event: (Scalar("e", Plain, 0, None), Marker { index: 53, line: 9, col: 0 })
//    // event: (MappingStart(0), Marker { index: 60, line: 10, col: 4 })
//    // event: (Scalar("ee", Plain, 0, None), Marker { index: 58, line: 10, col: 2 })
//    // event: (Scalar("5", DoubleQuoted, 0, None), Marker { index: 62, line: 10, col: 6 })
//    // event: (MappingEnd, Marker { index: 66, line: 11, col: 0 })
//    // event: (Scalar("f", Plain, 0, None), Marker { index: 66, line: 11, col: 0 })
//    // event: (MappingStart(0), Marker { index: 73, line: 12, col: 4 })
//    // event: (Scalar("ff", Plain, 0, None), Marker { index: 71, line: 12, col: 2 })
//    // event: (MappingStart(0), Marker { index: 82, line: 13, col: 7 })
//    // event: (Scalar("fff", Plain, 0, None), Marker { index: 79, line: 13, col: 4 })
//    // event: (Scalar("6", DoubleQuoted, 0, None), Marker { index: 84, line: 13, col: 9 })
//    // event: (MappingEnd, Marker { index: 88, line: 14, col: 0 })
//    // event: (MappingEnd, Marker { index: 88, line: 14, col: 0 })
//    // event: (MappingEnd, Marker { index: 88, line: 14, col: 0 })
//    // event: (DocumentEnd, Marker { index: 88, line: 14, col: 0 })
//
//    // event: (StreamEnd, Marker { index: 88, line: 14, col: 0 })
//    fn on_event(&mut self, ev: Event, marker: Marker) {
//        match ev {
//            Event::StreamStart => {
//                // do nothing
//            }
//            Event::DocumentStart => {
//                // do nothing
//                if marker.line() > 1 {
//                    writeln!(self.writer).unwrap();
//                    writeln!(self.writer, "---").unwrap();
//                    self.line += 2;
//                    self.col = 0;
//                }
//            }
//            Event::MappingStart(_aid) => {
//                // do nothing
//            }
//            Event::Scalar(val, style, _aid, _tag) => {
//                while self.line < marker.line() {
//                    writeln!(self.writer).unwrap();
//                    self.line += 1;
//                    self.col = 0;
//                }
//                while self.col < marker.col() {
//                    write!(self.writer, " ").unwrap();
//                    self.col += 1;
//                }
//                match style {
//                    TScalarStyle::Plain => {
//                        write!(self.writer, "{}:", val).unwrap();
//                        self.col += val.len() + 1;
//                    }
//                    TScalarStyle::SingleQuoted => {
//                        write!(self.writer, "'{}'", val).unwrap();
//                        self.col += val.len() + 2;
//                    }
//                    TScalarStyle::DoubleQuoted => {
//                        write!(self.writer, "\"{}\"", val).unwrap();
//                        self.col += val.len() + 2;
//                    }
//                    TScalarStyle::Foled => {
//                        write!(self.writer, "{}", val).unwrap();
//                        self.col += val.len();
//                    }
//                    TScalarStyle::Literal => {
//                        write!(self.writer, "{}", val).unwrap();
//                        self.col += val.len();
//                    }
//                    TScalarStyle::Any => {
//                        write!(self.writer, "{}", val).unwrap();
//                        self.col += val.len();
//                    }
//                }
//            }
//            Event::MappingEnd => {
//                // do nothing
//            }
//            Event::DocumentEnd => {
//                // do nothing
//            }
//            Event::StreamEnd => {
//                // do nothing
//            }
//            _ => todo!(),
//        }
//    }
//}
