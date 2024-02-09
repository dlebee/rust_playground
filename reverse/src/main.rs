fn reverse_array(arr: &mut [i32]) {
    let length = arr.len();
    let middle_of_array =length/2;

    for i in 0..middle_of_array {
        let holder = arr[length - 1 - i];
        arr[length - 1 - i] = arr[i];
        arr[i] = holder;
    }
}

fn main() {
    let mut my_array = [1, 2, 3, 4, 5];
    
    println!("Original array: {:?}", my_array);
    
    reverse_array(&mut my_array);
    
    println!("Reversed array: {:?}", my_array);
}