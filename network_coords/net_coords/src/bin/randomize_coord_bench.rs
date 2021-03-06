#![cfg(not(test))]
extern crate net_coords;
extern crate rand;
extern crate ordered_float;

use rand::{StdRng};
// use std::hash::Hash;
use net_coords::landmarks::coords::{build_coords, choose_landmarks};
// use net_coords::landmarks::randomize_coord::randomize_coord_rw;
use net_coords::landmarks::randomize_coord::randomize_coord_rw_mix;
// use net_coords::landmarks::randomize_coord::randomize_coord_rw_sparse;
// use net_coords::landmarks::randomize_coord::randomize_coord_rw_directional;
use net_coords::landmarks::randomize_coord::{calc_upper_constraints /*, is_rw_coord */};
use net_coords::network_gen::{gen_network};


#[cfg(not(test))]
fn main() {
    println!("Randomize coord functions benchmark");
    
    let g = 9;
    let l = 2 * g + 1;
    let net_type = 1;
    print!("g={:2}; ",g);
    match net_type {
        0 => print!("rand    ; "),
        1 => print!("2d      ; "),
        2 => print!("rand+2d ; "),
        3 => print!("planar  ; "),
        _ => unreachable!(),
    }
    // print!("nt={:1}; ",net_type);
    println!("Generating network...");
    /* Generate network */
    let seed: &[_] = &[1,g,net_type];
    let mut network_rng: StdRng = rand::SeedableRng::from_seed(seed);
    let net = gen_network(net_type, g, l, 1000, 2000, &mut network_rng);

    // Generate helper structures for landmarks routing:
    // Calculate landmarks and coordinates for landmarks routing:
    // Amount of landmarks can not be above half of the node count:
    let mut num_landmarks: usize = (((g*g) as u32)) as usize;
    // let mut num_landmarks: usize = 10; // DEBUG
    if num_landmarks as f64 > (net.igraph.node_count() as f64) / 2.0 {
        num_landmarks = net.igraph.node_count() / 2;
    }
    println!();
    println!("Choosing landmarks...");
    let landmarks = choose_landmarks(&net, num_landmarks, &mut network_rng);
    println!("landmarks = {:?}", landmarks);
    println!();
    println!();
    println!("Building coordinates...");
    let coords = match build_coords(&net, &landmarks) {
        Some(coords) => coords,
        None => unreachable!(),
    };
    let upper_constraints = calc_upper_constraints(&landmarks, &coords);

    // Randomize a coordinate:
    for _ in 0 .. 1 {
        // let rcoord = randomize_coord_rw_directional(&upper_constraints, 
        //                                        &landmarks, &coords, &mut network_rng);
        let rcoord = randomize_coord_rw_mix(&upper_constraints, 
                                               &landmarks, &coords, &mut network_rng);
        println!("rcoord = {:?}", rcoord);
        // println!("i = {}",i);
    }
    println!();
    println!();
    println!("Real coordinates:");
    for i in 0 .. 5 {
        println!("{:?}", coords[i]);
        println!();
    }

    // assert!(is_rw_coord(&coords[0], &upper_constraints, &landmarks, &coords));
    // println!("coords[0]");
    // println!("{:?}",coords[0]);

    println!();
}



