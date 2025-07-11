use crate::Problem;
use crate::math::linalg::{Point, Vector};
use std::f64::consts::PI;

problem!(Problem0102, 102, "Triangle Containment");

impl Problem for Problem0102 {
    fn id(&self) -> usize {
        self.id
    }

    fn title(&self) -> &str {
        self.title
    }

    fn run(&self) -> String {
        let input_str = include_str!("0102_triangles.txt");
        let triangles = parse_input(input_str);

        // for every triangle, generate vectors from origin to each point
        // calculate and sum the angles between each vector
        // if the sum is 2pi, the triangle contains the origin
        // if the point is outside the triangle, the sum will be less than 2pi
        // count the number of triangles that contain the origin

        let mut count = 0;

        for triangle in triangles {
            let mut angle_sum = 0.0;
            let vec1 = Vector::from_points(Point::new([0.0; 2]), triangle[0]);
            let vec2 = Vector::from_points(Point::new([0.0; 2]), triangle[1]);
            let vec3 = Vector::from_points(Point::new([0.0; 2]), triangle[2]);

            angle_sum += vec1.angle_between(&vec2);
            angle_sum += vec2.angle_between(&vec3);
            angle_sum += vec3.angle_between(&vec1);

            if (angle_sum - FULL_ANGLE).abs() < 1e-8 {
                count += 1;
            }
        }

        count.to_string()
    }
}

const FULL_ANGLE: f64 = 2.0 * PI;

fn parse_input(input: &str) -> Vec<[Point<2>; 3]> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_iter = line
                .trim()
                .split(',')
                .map(|num_str| num_str.parse::<f64>().unwrap());
            [
                Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
                Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
                Point::new([line_iter.next().unwrap(), line_iter.next().unwrap()]),
            ]
        })
        .collect()
}
