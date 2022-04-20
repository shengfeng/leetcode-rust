use std::collections::BinaryHeap;

pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
    use std::cmp::Reverse as rvs;
    let mut requests = vec![0; k as usize];
    let mut busy = BinaryHeap::new();
    let mut idle = BinaryHeap::new();
    (0..k as usize).for_each(|i| {
        idle.push(rvs(i));
    });
    arrival.into_iter().zip(load.into_iter()).enumerate().for_each(|(i, (arv, ld))| {
        while let Some(rvs((time, id))) = busy.pop() {
            if time <=arv {
                let i = i as i32;
                let next = i + ((id as i32 - i) % k + k) % k;
                idle.push(rvs(next as usize));
            } else {
                busy.push(rvs((time, id)));
                break;
            }
        }
        if let Some(rvs(id)) = idle.pop() {
            let id = id % k as usize;
            requests[id] += 1;
            busy.push(rvs((arv + ld, id)));
        }
    });
    let mut max_req = 0;
    let mut ans = vec![];
    for (i, req) in requests.into_iter().enumerate() {
        if req > max_req {
            max_req = req;
            ans.clear();
            ans.push(i as i32);
        } else if req == max_req {
            ans.push(i as i32);
        }
    }
    ans
}

pub fn binary_search(nums: &Vec<i32>, target: &i32, lower: bool) -> i32 {
    let mut left = 0i32;
    let mut right = nums.len() as i32 - 1;
    let mut ret = nums.len() as i32;
    while left <= right {
        let mid = left + (right - left) / 2;
        if nums[mid as usize] > *target || (lower && nums[mid as usize] >= *target) {
            right = mid - 1;
            ret = mid;
        } else {
            left = mid + 1;
        }
    }
    ret
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let left_index = binary_search(&nums, &target, true);
    let right_index = binary_search(&nums, &target, false) - 1;
    if left_index <= right_index && right_index < nums.len() as i32 && nums[left_index as usize] == target && nums[right_index as usize] == target {
        vec![left_index, right_index]
    } else {
        vec![-1, -1]
    }
}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if let Some(i) = nums.iter().position(|&x| x == target) {
        i as i32
    } else {
        -1
    }

}

fn main() {
    // use std::cmp::Reverse as rvs;
    // let mut heap = BinaryHeap::new();
    // println!("{:?}", heap.peek());
    // let x = [11, 31, 9, 87, 90, 61, 23, 69];
    // for i in &x {
    //     heap.push(i);
    // }
    // println!("{:?}", heap.peek());
    // println!("{:?}", heap);
    // let mut idle = BinaryHeap::new();
    // (0..10 as usize).for_each(|i| {
    //     idle.push(i);
    // });
    // println!("{:?}", idle.peek());
    // println!("{:?}", idle);

    // let arrival = vec![1, 2, 3, 4, 5];
    // let load = vec![5, 2, 3, 3, 3];
    // let ret = busiest_servers(3, arrival, load);
    // println!("{:?}", ret);

    let nums = vec![];
    let target = 0;
    let ret = search_range(nums, target);
    println!("{:?}", ret);
}
