fn main() {
    // println!("Hello, world!");
    let image = vec![
        vec![0,0,0],
        vec![1,0,0]
    ];

    let sr = 1;
    let sc = 0;
    let new_color = 2;

    let result = Solution::flood_fill(image, sr, sc, new_color);
    println!("{:?}", result);
}

struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        let old_color = image[sr as usize][sc as usize];
        if old_color == new_color {
            return image;
        }

        let mut image = image.clone();

        Self::dfs(&mut image, sc, sr, old_color, &new_color);

        image
    }

    fn dfs(image: &mut Vec<Vec<i32>>, x: i32, y: i32, old_color: i32, new_color: &i32) {        
        let ux = x as usize;
        let uy = y as usize;
        
        if image[uy][ux] == old_color {
            image[uy][ux] = *new_color;

            if x > 0 {
                Self::dfs(image, x - 1, y, old_color, &new_color);
            } 
            if x < image[0].len() as i32 - 1 {
                Self::dfs(image, x + 1, y, old_color, &new_color);
            }
            if y > 0 {
                Self::dfs(image, x, y - 1, old_color, &new_color);
            }
            if y < image.len() as i32 - 1 {
                Self::dfs(image, x, y + 1, old_color, &new_color);
            }
        }
    }
}