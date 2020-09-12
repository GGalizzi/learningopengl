use vek::{Aabb, Vec3};

use crate::component::BoundingBox;

const STR_MAP: [&'static str; 4] = [
    r#"
    #################### 
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
    ####################
"#,
    r#"
    ###############.#### 
    #.......######..####
    #..............#####
    #..............#####
    #.......#####.######
    #.......####..##.###
    #.......###.......##
    ####################
"#,
    r#"
    #################### 
    #.................##
    #.................##
    #.................##
    #..................#
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

    pub fn blocks_at(
        &self,
        point: Vec3<i32>,
    ) -> Option<Vec3<i32>> {
        if point.x < 0 || point.y < 0 || point.z < 0 {
            return None;
        }
        if let Some(tile) = self.tiles.get(
            (20 * point.z as usize + point.x as usize) +
                point.y as usize * 20 * 8,
        ) {
            if tile.is_wall() {
                return Some(point);
            }
        }
        None
    }

    pub fn blocks_around(
        &self,
        point: Vec3<f32>,
    ) -> Option<(Aabb<f32>, Aabb<f32>, Vec3<i32>)> {
        let aabb = Aabb {
            min: Vec3::new(
                point.x - 0.25,
                point.y - 0.35,
                point.z - 0.25,
            ),
            max: Vec3::new(
                point.x + 0.25,
                point.y + 0.15,
                point.z + 0.25,
            ),
        };

        let area_rect = aabb.map(|e| e.round() as i32);

        for my in area_rect.min.y..=area_rect.max.y {
            for mx in area_rect.min.x..=area_rect.max.x {
                for mz in area_rect.min.z..=area_rect.max.z {
                    let thisblocks =
                        self.blocks_at((mx, my, mz).into());
                    if let Some(block) = thisblocks {
                        let block_aabb = Aabb {
                            min: Vec3::new(
                                mx as f32 - 0.5,
                                my as f32 - 0.5,
                                mz as f32 - 0.5,
                            ),
                            max: Vec3::new(
                                mx as f32 + 0.5,
                                my as f32 + 0.5,
                                mz as f32 + 0.5,
                            ),
                        };
                        if aabb.collides_with_aabb(block_aabb)
                        {
                            return Some((
                                block_aabb,
                                aabb,
                                Vec3::new(mx, my, mz),
                            ));
                        }
                    }
                }
            }
        }

        None
    }
}
