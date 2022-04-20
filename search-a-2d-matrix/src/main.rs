
pub fn binary_search(nums: &Vec<i32>, target: &i32) -> bool {
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] > *target {
            right = mid - 1;
        } else if nums[mid as usize] < *target {
            left = mid + 1;
        } else {
            return true;
        }
    }
    false
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let col = matrix[0].len();
    let mut ret = false;
    matrix.into_iter().for_each(|m| {
        if m[0] <= target && m[col - 1] > target {
            ret = binary_search(&m, &target);
        }
    });
    ret
}

fn main() {
    
}

