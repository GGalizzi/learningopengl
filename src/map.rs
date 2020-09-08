const STR_MAP: &'static str = r#"
    #################### 
    #...#..............#
    #................###
    #...#..........#####
    ######...........###
    #######............#
    ####...............#
    ####################
"#;

#[derive(PartialEq)]
pub enum Tile {
    Floor,
    Wall,
}

impl Tile {
    fn from_char(ch: char) -> Tile {
        match ch {
            '#' => Tile::Wall,
            _ => Tile::Floor,
        }
    }

    pub fn is_wall(&self) -> bool {
        *self == Tile::Wall
    }
}

pub struct Area {
    pub tiles: Vec<Tile>,
}

impl Area {
    pub fn debug() -> Area {
        let tiles = STR_MAP
            .to_owned()
            .replace('\n', "")
            .replace(' ', "")
            .chars()
            .map(|ch| Tile::from_char(ch))
            .collect();

        Area { tiles }
    }
}