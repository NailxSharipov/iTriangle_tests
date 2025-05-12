use i_triangle::int::triangulation::IntTriangulation;
use std::fs::File;
use std::io::{BufRead, BufReader};
use i_triangle::i_overlay::i_float::int::point::IntPoint;
use i_triangle::i_overlay::i_float::triangle::Triangle;
use i_triangle::i_overlay::i_shape::int::area::Area;
use i_triangle::i_overlay::i_shape::int::shape::IntContour;
use i_triangle::int::triangulatable::IntTriangulatable;


fn main() {
    run_test("input_0.txt");
    run_test("input_1.txt");
}

fn run_test(path: &str) {
    println!("file {}", path);
    let file = File::open(path).expect("Cannot open input file");
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            continue;
        }

        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();

        let n = nums[0] as usize;
        let coords = &nums[1..];

        assert_eq!(coords.len(), n * 2, "Invalid number of coordinates");

        let points: Vec<IntPoint> = coords
            .chunks(2)
            .map(|pair| IntPoint::new(pair[0], pair[1]))
            .collect();

        if !triangulate(points) {
            println!("failt test {}", i);            
        } 
    }

}

fn triangulate(contour: IntContour) -> bool {
    let s0 = contour.area();
    let triangulation = contour.triangulate().into_triangulation();
    let s1 = calculate_area(triangulation);

    let is_equal = s0 == s1;
    
    if !is_equal {
        println!("unequal found {} and {}", s0, s1);
    }

    is_equal
}

fn calculate_area(triangulation: IntTriangulation<usize>) -> i64 {
    let mut i = 0;
    let mut s = 0;
    while i < triangulation.indices.len() {
        let i0 = triangulation.indices[i + 0];
        let i1 = triangulation.indices[i + 1];
        let i2 = triangulation.indices[i + 2];

        let a= triangulation.points[i0];
        let b= triangulation.points[i1];
        let c= triangulation.points[i2];

        s += Triangle::area_two_point(a, b, c);

        i += 3;
    }

    s / 2
}
