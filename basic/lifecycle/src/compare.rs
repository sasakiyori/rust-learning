pub fn longer<'num>(s1: &'num str, s2: &'num str) -> &'num str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}

// static is a reserved lifetime name
// which means that the lifecycle of something is from program start to program end.

// pub fn longer<'static>(s1: &'static str, s2: &'static str) -> &'static str {
//     if s2.len() > s1.len() {
//         s2
//     } else {
//         s1
//     }
// }
