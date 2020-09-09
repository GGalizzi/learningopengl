use vek::Vec3;

use crate::component::BoundingBox;

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

    pub fn blocks_at(&self, point: Vec3<f32>) -> bool {
        println!("point to check {:?}", point);
        let x = point.x.round() as usize;
        let y = point.z.round() as usize;
        let z = point.y.round() as usize;
        println!("rounded to check {:?}", (x,y,z));

        if let Some(tile) =
            self.tiles.get((20 * y + x) + z * 20 * 8)
        {
            return tile.is_wall();
        }
        false
    }

    pub fn blocks_around(
        &self,
        point: Vec3<f32>,
        bound: &BoundingBox,
    ) -> bool {
        let px = point.x + bound.size;
        let x = point.x;
        let mx = point.x - bound.size;

        let py = point.y + bound.height;
        let y = point.y;
        let my = point.y - bound.height;

        let pz = point.z + bound.size;
        let z = point.z;
        let mz = point.z - bound.size;

        return self.blocks_at((x, y, pz).into()) ||
            self.blocks_at((px, y, z).into()) ||
            self.blocks_at((x, y, mz).into()) ||
            self.blocks_at((mx, y, z).into()) ||
            self.blocks_at((x, py, z).into()) ||
            self.blocks_at((x, my, z).into());
    }
}
