use bevy::prelude::*;

// This is not ideal. Originally the levels were in the assets folder and were read in from there.
// But of couse this won't work in the world of WASM, of course! So as a temporary work around, as
// its boxing day 2023 and I should be doing something else, this is all I can think of doing right now.
const LEVEL_1: &str = "
############
#.#.......##
#.#.....o.##
#.##..o....#
#..#o......#
#..#...###.#
#.o........#
#...p.t..o.#
############";

const LEVEL_2: &str = "
############
####.....p.#
###..#o#...#
##....#o...#
#....#o#o..#
#...#...#..#
#.........##
#t.......###
############";

const LEVEL_3: &str = "
############
#........#.#
#..##...##.#
#.o#.......#
#.....##...#
#.....o#.o.#
#.#o.......#
#.##p#t....#
############";

const LEVEL_4: &str = "
############
#..........#
##...#o....#
#...###....#
#.#.o#..#o.#
####p..###.#
#.#o....#o.#
#.....t..#.#
############";

const LEVEL_5: &str = "
############
#........#o#
#....o.....#
#..........#
##o#.#..#.##
#.#o#..#o#.#
#..........#
#..#p.t.#..#
############";

const LEVEL_6: &str = "
############
#.....##...#
#......#o#.#
#.#.#..o#..#
#.o#o..#.#.#
#.#.#......#
#..........#
#...t...p#.#
############";

const LEVEL_7: &str = "
############
#####......#
####......##
###o.#o....#
##o.###..o##
#....#..p###
##......####
#...t..#####
############";

const LEVEL_8: &str = "
############
#..........#
#..#o..##..#
#..###.o#o.#
#....#.....#
####.p.....#
###o..###..#
#.....t....#
############";

const LEVEL_9: &str = "
############
#..........#
#.##o..##..#
#.#.#.#o...#
#.#.#.o#o..#
#.#.#..o#..#
#.##p.##...#
#.....t....#
############";

const LEVEL_10: &str = "
############
#..........#
#..#..##...#
#.#.#..o#..#
#.o#o..#o..#
#.#.#..o#..#
#..#..##...#
#...t.p....#
############";

#[derive(Resource)]
pub struct LevelManager {
    pub maps: [&'static str; 10],
}

impl LevelManager {
    pub fn new() -> Self {
        Self {
            maps: [
                LEVEL_1, LEVEL_2, LEVEL_3, LEVEL_4, LEVEL_5, LEVEL_6, LEVEL_7, LEVEL_8, LEVEL_9,
                LEVEL_10,
            ],
        }
    }
}

#[derive(Component, Debug)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelManager::new());
    }
}
