#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TouchKind { Press, Move, Release }

impl TouchKind {
    pub fn from_str(string: &str) -> Option<Self> {
        match string {
            "Press" => Some(TouchKind::Press),
            "Move" => Some(TouchKind::Move),
            "Release" => Some(TouchKind::Release),
            _ => None
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Touch {
    pub kind: TouchKind,
    pub slot: u32,
    pub position: [u32; 2]
}

impl Touch {
    pub fn from_str(string: &str) -> Option<Self> {
        let parts: Vec<&str> = string.split_whitespace().collect();

        if parts.len() < 4 {
            return None
        }
        
        let kind = TouchKind::from_str(parts[0])?;
        let slot = parts[1].parse::<u32>().ok()?;
        let x = parts[2].parse::<u32>().ok()?;
        let y = parts[3].parse::<u32>().ok()?;

        Some(Touch {
            kind: kind,
            slot: slot,
            position: [x, y]
        })    
    }
}
