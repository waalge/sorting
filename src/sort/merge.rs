pub fn run(vec: Vec<usize>) -> Vec<usize> {
    let mut vec0 = vec.clone();
    let mut vec = vec;
    merge(&mut vec, &mut vec0);
    vec 
}

fn merge<'a>(src : &'a mut [usize], trg : &'a mut [usize]) -> () {
    // TODO : tidy this up

    if src.len() > 1 {
        let (trg_l, trg_r) = trg.split_at_mut(src.len()/2);
        let (src_l, src_r) = src.split_at_mut(src.len()/2);
        merge(src_r, trg_r); 
        merge(src_l, trg_l);
        let mut src_l = src_l.iter();
        let mut src_r = src_r.iter();
        let mut l = src_l.next();
        let mut r = src_r.next();
        let mut cnt = 0;
        loop {
            if l.is_none() && r.is_none() {
                break;
            }
            else if l.is_none() {
                trg[cnt] = *r.unwrap();
                r = src_r.next();
            }
            else if r.is_none() {
                trg[cnt] = *l.unwrap();
                l = src_l.next();
            }
            else if l < r {
                trg[cnt] = *l.unwrap();
                l = src_l.next();
            } 
            else {
                trg[cnt] = *r.unwrap();
                r = src_r.next();
            }
            cnt += 1;
        }
        src.clone_from_slice(&trg);
    }
}
