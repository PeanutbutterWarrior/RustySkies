use std::collections::HashMap;
use rand::prelude::*;

const POINTS_DENSITY: i64 = 10;
const CELL_SIZE: i64 = 50;

#[derive(Copy, Clone)]
pub struct Point (pub i64, pub i64);

pub struct WorleyNoiseGen {
    points: HashMap<(i64, i64), Vec<Point>>,
    rand_generator: ThreadRng,
}

impl WorleyNoiseGen {
    pub fn new() -> Self {
        WorleyNoiseGen {
            points: HashMap::new(),
            rand_generator: thread_rng(),
        }
    }

    pub fn generate_noise(&mut self, position: Point) -> f64 {
        let x_cell_position = position.0 / CELL_SIZE;
        let y_cell_position = position.1 / CELL_SIZE;

        let mut distance = self.find_distance_squared((x_cell_position, y_cell_position), &position);
        distance = distance.min(self.find_distance_squared((x_cell_position - 1, y_cell_position - 1), &position));
        distance = distance.min(self.find_distance_squared((x_cell_position - 1, y_cell_position), &position));
        distance = distance.min(self.find_distance_squared((x_cell_position - 1, y_cell_position + 1), &position));

        distance = distance.min(self.find_distance_squared((x_cell_position, y_cell_position - 1), &position));
        distance = distance.min(self.find_distance_squared((x_cell_position, y_cell_position + 1), &position));

        distance = distance.min(self.find_distance_squared((x_cell_position + 1, y_cell_position - 1), &position));
        distance = distance.min(self.find_distance_squared((x_cell_position + 1, y_cell_position), &position));
        distance = distance.min(self.find_distance_squared((x_cell_position + 1, y_cell_position + 1), &position));
        distance
    }

    fn find_distance_squared(&mut self,
                            cell_position: (i64, i64),
                            position: &Point) -> f64 {
        let points_vec = match self.points.get_mut(&cell_position) {
            Some(points_vec) => points_vec,
            None => {
                let mut new_points_vec: Vec<Point> = Vec::new();
                for _ in 0..POINTS_DENSITY {
                    let new_x = self.rand_generator.next_u64() as i64 % CELL_SIZE + cell_position.0 * CELL_SIZE;
                    let new_y = self.rand_generator.next_u64() as i64 % CELL_SIZE + cell_position.1 * CELL_SIZE;
                    new_points_vec.push(Point(new_x, new_y));
                }
                self.points.insert(cell_position, new_points_vec);
                self.points.get_mut(&cell_position).unwrap()
            }
        };

        points_vec.iter()
                  .map(|&val| {(val.0 - position.0).pow(2) + (val.1 - position.1).pow(2)})
                  .min()
                  .unwrap() as f64
    }
}