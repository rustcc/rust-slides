fn main() {
    let mut v = vec![10,20,30];
    let tmp0 = &v;
    let tmp1 = &mut v;
    let tmp2 = Vec::len(tmp0); //v.len()

    Vec::push(tmp1, tmp2);// v.push(tmp2)
}
