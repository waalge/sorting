pub fn run(vec: Vec<usize>) -> Vec<usize> {
    let l = vec.len() - 1;
    let mut vec = vec;
    loop {
        let mut swapped = false;
        for ii in 0..l {
            if vec[ii] > vec[ii + 1] {
                vec.swap(ii, ii + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
    vec
}
