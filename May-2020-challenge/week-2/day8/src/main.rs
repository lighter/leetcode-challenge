fn main() {
    // println!("Hello, world!");
    let input:Vec<Vec<i32>> = vec![
        vec![1,2],
        vec![2,3],
        vec![3,4],
        vec![4,5],
        vec![5,6],
        vec![6,7],
    ];

    let result = Solution::check_straight_line(input);

    println!("{}", result);
}


struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x1 = coordinates[0][0];
        let y1 = coordinates[0][1];
        let x2 = coordinates[1][0];
        let y2 = coordinates[1][1];

        for (index, coordinate) in coordinates.iter().enumerate() {
            if index >= 2 {
                let x3 = coordinate[0];
                let y3 = coordinate[1];

                if (x1 - x2) * (y1 - y3) != (x1 - x3) * (y1 - y2) {
                    return false;
                }
            }
        }

        true
    }
}