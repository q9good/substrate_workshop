fn bubble_sort<T: PartialOrd + Copy>(slice: &mut [T]){
    for _i in 0..slice.len(){
        for j in 0..slice.len() - 1{
            if slice[j] > slice[j+1] {
                slice.swap(j, j+1);
            }
        }
    }
}

fn main() {
    let mut digital_arr = [1,5,3,8,4,7,6,2];
    bubble_sort(&mut digital_arr);
    println!("{:?}", digital_arr);

    let mut letter_vec = vec!['a', 'z', 'b', 'y', 'c', 'f'];
    bubble_sort(&mut letter_vec);
    println!("{:?}", letter_vec);
}

