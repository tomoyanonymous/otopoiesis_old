use iced::{
    button, Align, Application, Button, Clipboard, Column, Command, Container, Element,
    HorizontalAlignment, Length, Row, Settings, Text, VerticalAlignment,
};
use crate::otopoiesis::core;

struct WaveRegion{
    content: &core::Region,
    zoommode: bool
}
