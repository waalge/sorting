use std::time::Duration;
use std::thread;

pub fn run(vec: Vec<usize>) -> Vec<usize> {
    let mut vec = vec;
    sort(&mut vec[..]);
    vec
}


fn sort(vec : &mut [usize]){
    if vec.len() > 1{
        let (left, right) = partition(vec);
        sort(left);
        sort(right);
    }
}

fn partition(vec : &mut [usize]) -> 
(&mut [usize], &mut [usize]) {
    let pivot = vec[vec.len()/2];
    //println!("piv {}", pivot);
    //println!("sta {:?}", vec);
    let mut lo : isize = -1;
    let mut hi = vec.len() as isize;
    let (left, right) = loop{
        loop {
            lo = lo + 1;
            if lo >= hi || vec[lo as usize] >= pivot {
                break;
            }
        }
        loop {
            hi = hi - 1;
            if vec[hi as usize] <= pivot {
                break;
            }
        }
        //println!("lohi, {}, {}", lo, hi);
        if hi < lo {
            let (left, right) = vec.split_at_mut((hi+1) as usize);
            //println!("left : {:?}", left);
            //println!("right : {:?}", right);

            if left.iter().max().unwrap_or(&0) > &pivot ||
                right.iter().min().unwrap_or(&99999999) < &pivot 
                {
                println!("CK {}, {}, {}", 
                         left.iter().max().unwrap_or(&0), 
                         pivot, 
                         right.iter().min().unwrap_or(&99999999999))  ;
            }
            break (left, right);
        }
        vec.swap(hi as usize,lo as usize);
        //println!("end {:?}", vec);
    };
    (left, right)
}
/*
algorithm quicksort(A, lo, hi) is
    if lo < hi then
        p := partition(A, lo, hi)
        quicksort(A, lo, p)
        quicksort(A, p + 1, hi)

algorithm partition(A, lo, hi) is
    pivot := A[⌊(hi + lo) / 2⌋]
    i := lo - 1
    j := hi + 1
    loop forever
        do
            i := i + 1
        while A[i] < pivot
        do
            j := j - 1
        while A[j] > pivot
        if i ≥ j then
            return j
        swap A[i] with A[j]
    Hoare-Partition(A, p, r)
    x = A[p]
    i = p - 1
    j = r + 1
    while true
        repeat
                j = j - 1
                    until A[j] <= x
                        repeat
                                i = i + 1
                                    until A[i] >= x
                                        if i < j
                                                swap( A[i], A[j] )
                                                    else
                                                            return j
*/
