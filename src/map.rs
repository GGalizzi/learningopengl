use glam::Vec3;

const STR_MAP: [&'static str; 4] = [
    r#"
    #################### 
    #...#..............#
    #................###
    #...#..........#####
    ######...........###
    #######............#
    ####...............#
    ####################
"#,
    r#"
    #################### 
    ####################
    ####################
    ####################
    ####################
    ################.###
    ###########.......##
    ####################
"#,
    r#"
    #################### 
    ####..............##
    #######...........##
    #######...........##
    #######............#
    #######............#
    ####################
    ####################
    "#,
    r#"
    #################### 
    ####...#############
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
    "#,
];

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
        let mut tiles = Vec::new();
        for z in &STR_MAP {
            tiles.append(
                &mut z
                    .to_owned()
                    .replace('\n', "")
                    .replace(' ', "")
                    .chars()
                    .map(|ch| Tile::from_char(ch))
                    .collect(),
            );
        }
        Area { tiles }
    }
    
    pub fn blocks_at(&self, point: Vec3) -> bool {
        let x = point.x().round() as usize;
        let y = point.z().round() as usize;
        let z = point.y().round() as usize;
        
        if let Some(tile) = self.tiles.get((20 * y + x) + z * 20 * 8) {
            return tile.is_wall();
        }
        false
    }
}
