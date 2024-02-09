fn reverse(arr: &[i32]) -> Vec<i32> {
    let length = arr.len();
    let mut result: Vec<i32> = Vec::new();

    for i in (0..length).rev() {
        result.push(arr[i]);
    }

    result
}

fn reverse_static_alloc(arr: &[i32]) -> &'static [i32] {
    let result = [0; arr.len()];
    let mut gradual = 0;
    for i in (0..arr.len()).rev() {
        result[gradual] = arr[i];
        gradual++;
    }

    result
}


fn main() {

    let numbers = [1, 2, 3, 4, 5];

    println!("before {:?}", numbers);

    let after = reverse(&numbers);

    println!("after: {:?}", after);
}
