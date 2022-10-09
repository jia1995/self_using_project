//! Self implementation for sorting Algorithms.
use std::fmt::Debug;

/// Reversing the Vectors when parameter reverse is `true`.
#[inline]
fn reversed<T>(vecs: &mut [T], reverse:bool) ->&mut [T] 
where T:PartialOrd+Copy+Debug
{
    if reverse {
        vecs.reverse();
    }
    vecs
}

/// Select sorting algorithm for Vectors.
pub fn select_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    let size = vecs.len();
    for i in 0..size {
        let mut index = i;
        for j in i+1..size {
            if vecs[j] < vecs[index] {
                index = j;
            }
        }
        vecs.swap(index, i);
    }
    reversed(vecs, reverse)
}

///Iterators quick sorting algorithm for Vectors by left and right.
#[inline]
fn sub_quick_sort<T>(vecs:&mut [T], left:usize, right:usize)
where T:PartialOrd+Copy+Debug
{
    let mid = vecs[left];
    let (mut l, mut r) = (left, right);
    let mut flag:bool = true;
    while l < r {
        if flag {
            if vecs[r] < mid {
                vecs[l] = vecs[r];
                flag = false;
            }else{
                r -= 1;
            }
        }else{
            if vecs[l] > mid {
                vecs[r] = vecs[l];
                flag = true;
            }else{
                l += 1;
            }
        }
    }
    vecs[l] = mid;
    if left < l-1 {
        sub_quick_sort(vecs, left, l-1);
    }
    if right > l+1{
        sub_quick_sort(vecs, l+1, right);
    }
}

/// Quick sorting algorithm for Vectors.
pub fn quick_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    sub_quick_sort(vecs, 0, vecs.len()-1);
    reversed(vecs, reverse)
}

/// Bubble sorting algorithm for Vectors.
pub fn bubble_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    let len = vecs.len();
    for i in 0..len-1 {
        for j in 0..len-1-i {
            if vecs[j] > vecs[j+1] {
                vecs.swap(j,j+1);
            }
        }
    }
    reversed(vecs, reverse)
}

/// Merging sorting inplace.
#[inline]
fn merge<T>(vecs: &mut [T], left:usize, mid:usize, right:usize)
where T:PartialOrd+Copy+Debug
{
    let mut tmp = vec![];
    let (mut i, mut j) = (left,mid+1);
    while i != mid+1 && j!=right+1 {
        if vecs[i] < vecs[j] {
            tmp.push(vecs[i]);
            i+=1;
        }else{
            tmp.push(vecs[j]);
            j+=1;
        }
    }
    while i != mid+1 {
        tmp.push(vecs[i]);
        i+=1;
    }
    while j != right+1 {
        tmp.push(vecs[j]);
        j+=1;
    }
    for i in 0..right-left+1{
        vecs[left+i] = tmp[i];
    }
}

/// Sub merging sorting algorithm inplace.
#[inline]
fn sub_merge_sort<T>(vecs: &mut [T], left:usize, right:usize)
where T:PartialOrd+Copy+Debug
{
    let mid = (left+right)/2;
    if left  < right {
        sub_merge_sort(vecs, left, mid);
        sub_merge_sort(vecs, mid+1, right);
        merge(vecs, left, mid, right);
    }
}

/// Merge sorting algorithm for Vectors.
pub fn merge_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    sub_merge_sort(vecs, 0, vecs.len()-1);
    reversed(vecs, reverse)
}

/// Insert sorting algorithm for Vectors.
pub fn insert_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    let len = vecs.len();
    for i in 1..len {
        let k = vecs[i];
        let mut j = i;
        while j>0 && vecs[j-1]>k {
            vecs[j] = vecs[j-1];
            j-=1;
        }
        vecs[j] = k;
    }
    reversed(vecs, reverse)
}

/// Sub hill sorting algorithm inplace.
#[inline]
fn sub_hill_sort<T>(vecs: &mut [T], start:usize, gap:usize) 
where T : PartialOrd+Copy+Debug
{
    let len = vecs.len();
    for i in ((start+gap)..len).step_by(gap) {
        let n = vecs[i];
        let mut j = i;
        while j >= gap && vecs[j-gap] > n {
            vecs[j] = vecs[j-gap];
            j = j-gap;
        }
        vecs[j] = n;
    }
}

/// Hill sorting algorithm for Vectors.
pub fn hill_sort<T>(vecs: &mut [T], reverse:bool) -> &mut [T] 
where T:PartialOrd+Copy+Debug
{
    let len = vecs.len();
    let mut t = len/2;
    while t>0 {
        for i in 0..t {
            sub_hill_sort(vecs, i, t);
        }
        t/=2;
    }
    reversed(vecs, reverse)
}

#[test]
pub fn test1(){
    let mut vecs = vec![4,3,2,1];
    let cur = hill_sort(&mut vecs, false).to_vec();
    assert_eq!(cur, [1,2,3,4]);
    let mut vecs = vec![4.1,3.2,2.5,1.2];
    let cur = hill_sort(&mut vecs, false).to_vec();
    assert_eq!(cur, [1.2,2.5,3.2,4.1]);
    let mut vecs:Vec<u8> = vec![4,3,2,1];
    let cur = hill_sort(&mut vecs, false).to_vec();
    assert_eq!(cur, [1,2,3,4]);
    let mut vecs = vec!['a','b','d','c'];
    let cur = hill_sort(&mut vecs, false).to_vec();
    assert_eq!(cur, ['a','b','c','d']);
    let mut vecs = vec!["ads","sab","eqd","qerwc"];
    let cur = hill_sort(&mut vecs, false).to_vec();
    assert_eq!(cur, ["ads","eqd","qerwc","sab"]);
}