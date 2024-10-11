fn main() {
    let mut inum = 160;
    println!("身長は{}です", inum);
    let fnum = inum as f32;
    println!("身長は{}です", fnum);
    inum = fnum as i32;
    println!("身長は{}です", inum);
}
