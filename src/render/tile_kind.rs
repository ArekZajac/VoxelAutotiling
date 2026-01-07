#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TileKind {
    Solo,
    End,
    Straight,
    Corner,
    Tee,
    Cross,
}

impl TileKind {
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "solo" => TileKind::Solo,
            "end" => TileKind::End,
            "straight" => TileKind::Straight,
            "corner" => TileKind::Corner,
            "tee" => TileKind::Tee,
            "cross" => TileKind::Cross,
            _ => return None,
        })
    }
}
