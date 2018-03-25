use std::cmp::Ordering;
use std::collections::HashMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::distances::hamming;

lazy_static! {
    static ref EN_FREQS: HashMap<u8, f64> =
        [(b' ', 0.15208646052236566),
        (b'e', 0.10135094566196337),
        (b't', 0.07481236865806064),
        (b'i', 0.06154308015610928),
        (b'a', 0.06004202942059442),
        (b'o', 0.05601921344941459),
        (b'r', 0.0548784148904233),
        (b'n', 0.05427799459621735),
        (b's', 0.053737616331432),
        (b'h', 0.03860702491744221),
        (b'c', 0.03644551185830081),
        (b'l', 0.029960972680876614),
        (b'm', 0.02611828279795857),
        (b'd', 0.025938156709696788),
        (b'u', 0.024256979885920144),
        (b'p', 0.0172320624437106),
        (b'f', 0.015731011708195738),
        (b'g', 0.015250675472830981),
        (b'y', 0.014229960972680877),
        (b'w', 0.012969078354848394),
        (b'k', 0.011708195737015911),
        (b'b', 0.01086760732512759),
        (b'v', 0.00870609426598619),
        (b'x', 0.00204142900030021),
        (b'j', 0.000720504353047133),
        (b'z', 0.0005403782647853498),
        (b'q', 0.00024016811768237767)].iter().cloned().collect();
}

fn char_count(buf: &[u8]) -> HashMap<u8, usize> {
    let mut vec = buf.to_vec();
    vec.sort();
    vec.dedup();
    vec.iter().map(|&b| {
        (b, buf.iter().filter(|&c| *c == b).count())
    }).collect()
}


pub fn break_single_xor(buf: &[u8]) -> Option<(u8, f64)> {
    let total = buf.len() as f64;
    let count: HashMap<u8, usize> = char_count(buf);
    (0u8..255).into_par_iter().map(|key| {
        let chisq = EN_FREQS.iter().map(|(b, f)| match count.get(&(b ^ key)) {
            Some(&c) => (f - (c as f64 / total)).powi(2) / f,
            None => 1.0
        }).sum::<f64>();
        (key, chisq)
    }).min_by(|x, y| {
        if x.1.is_nan() {
            Ordering::Greater
        } else if y.1.is_nan() {
            Ordering::Less
        } else {
            x.1.partial_cmp(&y.1).unwrap()
        }
    })
}

pub fn break_repeating_xor(buf: &[u8], n_bufs: usize) -> Vec<u8> {
    // this should not break, keysize > 0 and there are several keysizes
    let (keysize, _) = (2..=40).map(|keysize| {
        let bufs = (0..n_bufs).map(|n| &buf[n*keysize..(n+1)*keysize])
            .collect::<Vec<_>>();
        let mut dist = 0.0;
        for i in 0..n_bufs {
            for j in i..n_bufs {
                dist += hamming(bufs[i], bufs[j]) as f64
            }
        }
        (keysize as usize, (dist / ((4 * keysize) as f64)))
    }).min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap();

    let bufsize = buf.len();

    (0..keysize).map(|i| {
        let buf_k: Vec<u8> = (0..bufsize/keysize)
            .filter_map(|j| buf.get(i + j * keysize).map(|&x| x))
            .collect();
        break_single_xor(&buf_k).unwrap().0
    }).collect::<Vec<u8>>()
}
