#![allow(non_snake_case)]

extern crate num;
extern crate itertools;


use self::num::Complex;
use std::f64;
use statistic::Stream;
use self::itertools::Itertools;

/// Convert network coordinate to chord value in [0,1) 
/// by projection to a plane.
pub fn old_coord_to_ring(coord: &Vec<u64>) -> f64 {
    let fcoord: Vec<f64> = coord.iter().map(|&a| a as f64).collect();

    let k: f64 = fcoord.len() as f64;
    let S_a:f64 = fcoord.iter().sum();
    let normalize = |a| (a/S_a) - (1.0/k);
    let L_a: f64 = fcoord.iter().
        map(|&a| normalize(a).powi(2) as f64).sum::<f64>().sqrt();

    let numerator: f64 = 
        normalize(fcoord[0]) - 
            (1.0/(k-1.0)) * fcoord.iter().skip(1).map(|&a| normalize(a)).sum::<f64>();

    let denominator: f64 = L_a * ((k/(k-1.0))).sqrt();

    (numerator/denominator).acos() / (f64::consts::PI)
}

pub fn coord_to_ring_all_pairs(coord: &Vec<u64>) -> f64 {
    assert!(coord.len() > 1);
    let fcoord: Vec<f64> = coord.iter().map(|&a| a as f64).collect();

    let k: f64 = fcoord.len() as f64;
    let S_a:f64 = fcoord.iter().sum();
    let normalize = |a| (a/S_a) - (1.0/k);
    let L_a: f64 = fcoord.iter().
        map(|&a| normalize(a).powi(2) as f64).sum::<f64>().sqrt();

    let scoord: Vec<f64> = fcoord.into_iter().map(|a| normalize(a) / L_a).collect();


    let mut sum: f64 = 0.0;
    for i in 0..scoord.len() {
        for j in i+1..scoord.len() {
            let x = scoord[i];
            let y = scoord[j];
            let addition = 0.5 + (y.atan2(x) / (2.0 * f64::consts::PI));
            // println!("Addition = {}",addition);
            sum += addition;
        }
    }

    // let pairs: f64 = k * (k-1.0) / 2.0;
    let f = (sum).fract();
    assert!(f >= 0.0);

    f
}

pub fn coord_to_ring_adj_pairs(coord: &Vec<u64>) -> f64 {
    assert!(coord.len() > 1);
    let fcoord: Vec<f64> = coord.iter().map(|&a| a as f64).collect();

    let k: f64 = fcoord.len() as f64;
    let S_a:f64 = fcoord.iter().sum();
    let normalize = |a| (a/S_a) - (1.0/k);
    let L_a: f64 = fcoord.iter().
        map(|&a| normalize(a).powi(2) as f64).sum::<f64>().sqrt();

    let scoord: Vec<f64> = fcoord.into_iter().map(|a| normalize(a) / L_a).collect();

    let mut sum: f64 = 0.0;
    for i in 0..scoord.len() {
        let x = scoord[i];
        let y = scoord[(i + 1) % scoord.len()];
        let addition = 0.5 + (y.atan2(x) / (2.0 * f64::consts::PI));
        // println!("Addition = {}",addition);
        sum += addition;
    }

    let f = (sum).fract();
    assert!(f >= 0.0);
    f
}

pub fn coord_to_ring(coord: &Vec<u64>) -> f64 {
    let k: f64 = coord.len() as f64;
    let ang_part = (2.0 * f64::consts::PI) / k;

    let sum: Complex<f64> = 
        coord.iter().map(|&a| a as f64).enumerate()
            .fold(Complex::new(0.0,0.0), |acc, (i,x)|
                acc + Complex::from_polar(&((-x*2.0).exp()),&(ang_part * (i as f64))));

    (sum.arg() + f64::consts::PI) / (2.0 * f64::consts::PI)
}


/////////////////////////////////////////////////////////////////////


pub fn dist_u64(a: u64, b: u64) -> u64 {
    if a > b {
        return a - b
    }
    b - a
}

pub fn max_dist(a: &Vec<u64>, b: &Vec<u64>) -> u64 {
    assert!(a.len() == b.len(), "Coordinates have different amount of entries! aborting.");
    a.iter()
        .zip(b)
        .map(|(&u,&v): (&u64, &u64)| dist_u64(u,v))
        .max().unwrap()
}


/// Approximate distance between two nodes in the network using network coordinates
pub fn approx_max_dist(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> u64 {
    let _ = landmarks;
    max_dist(&coords[u], &coords[v])
}

/// Approximate distance between two nodes in the network using network coordinates
pub fn approx_avg_dist(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> f64 {
    let _ = landmarks;
    let u_coord = &coords[u];
    let v_coord = &coords[v];

    u_coord.iter().enumerate()
        .map(|(i , _) | ((u_coord[i] as f64) - (v_coord[i]) as f64).abs())
        .collect::<Vec<_>>().mean()
}


pub fn approx_pairs_dist2(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> f64 {

    // A function to calculate landmarks distance:
    let lm_dist = |i: usize, j:usize| coords[landmarks[i]][j];

    // Get a "signature" for location of node w:
    let sig = |w: usize| (0 .. landmarks.len())
                    .zip(0 .. landmarks.len())
                    .map(|(i,j)| coords[w][i] + coords[w][j] - lm_dist(i,j))
                    .map(|val| val as f64)
                    .collect::<Vec<_>>();


    // Calculate square distance between signatures:
    sig(u).iter().zip(sig(v).iter())
        .map(|(x,y)| (x - y).powi(2))
        .sum()

}

pub fn approx_pairs_dist1(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> f64 {

    // A function to calculate landmarks distance:
    let lm_dist = |i: usize, j:usize| coords[landmarks[i]][j];

    // Get a "signature" for location of node w:
    let sig = |w: usize| (0 .. landmarks.len())
                    .zip(0 .. landmarks.len())
                    .map(|(i,j)| coords[w][i] + coords[w][j] - lm_dist(i,j))
                    .map(|val| val as f64)
                    .collect::<Vec<_>>();


    // Calculate square distance between signatures:
    sig(u).iter().zip(sig(v).iter())
        .map(|(x,y)| (x - y).abs())
        .sum()

}

pub fn approx_pairs_dist2_normalized(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> f64 {

    // A function to calculate landmarks distance:
    let lm_dist = |i: usize, j:usize| coords[landmarks[i]][j];


    // Get a "signature" for location of node w:
    let sig = |w: usize| (0 .. landmarks.len()).tuple_combinations()
                    .map(|(i,j)| ((coords[w][i] + coords[w][j] - lm_dist(i,j)) as f64) / (lm_dist(i,j) as f64))
                    .collect::<Vec<_>>();


    // Calculate square distance between signatures:
    sig(u).iter().zip(sig(v).iter())
        .map(|(x,y)| (x - y).powi(2))
        .sum()

}

pub fn approx_pairs_dist1_normalized(u: usize, v: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) 
    -> f64 {

    // A function to calculate landmarks distance:
    let lm_dist = |i: usize, j:usize| coords[landmarks[i]][j];


    // Get a "signature" for location of node w:
    let sig = |w: usize| (0 .. landmarks.len()).tuple_combinations()
                    .map(|(i,j)| ((coords[w][i] + coords[w][j] - lm_dist(i,j)) as f64) / (lm_dist(i,j) as f64))
                    .collect::<Vec<_>>();


    // Calculate square distance between signatures:
    sig(u).iter().zip(sig(v).iter())
        .map(|(x,y)| (x - y).abs())
        .sum()

}
