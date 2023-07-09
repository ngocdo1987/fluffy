/// factorial
#[inline]
pub fn fac(num: usize) -> usize { 
    let mut total = 1;
    for i in 1..=num { 
        total = total * i;
    }
    total
}

/// Arranging algorithm
/// Select K from n to arrange K for arrangement
#[inline]
pub fn perm(n: usize, k: usize) -> usize { 
    if k > n {  //K cannot be greater than n
        return 0;
    }
    fac(n) / fac(n - k)
}

/// Combined algorithm
/// Select K from n elements to combine combination
#[inline]
pub fn comb(n: usize, k: usize) -> usize { 
    if k > n { 
        return 0; //K cannot be greater than n
    }
    fac(n) / (fac(k) * fac(n-k))
}
