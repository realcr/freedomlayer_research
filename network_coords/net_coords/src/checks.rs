extern crate rand;

use network::{random_net, Network};
use coords::{build_coords, choose_landmarks, is_coord_unique};
use coord_mappers::{coord_to_ring_all_pairs, coord_to_ring, 
    approx_max_dist};
use statistic::spearman;
use random_util::choose_k_nums;

use self::rand::{StdRng};

// A trait alias for the distance function:
pub trait DistAFunc: Fn(usize,usize,&Vec<Vec<u64>>,&Vec<usize>) -> f64 {}
impl<T: Fn(usize,usize,&Vec<Vec<u64>>,&Vec<usize>) -> f64> DistAFunc for T {}

pub fn check_unique_coord(l: u32) {

    // Set up graph parameters:
    // let l: u32 = 16;
    let n: usize = ((2 as u64).pow(l)) as usize;
    let num_neighbours: usize = (1.5 * (n as f64).ln()) as usize;
    let num_landmarks: usize = (((l*l) as u32)/3) as usize;

    println!("n = {}",n);
    println!("num_neighbours = {}",num_neighbours);
    println!("num_landmarks = {}",num_landmarks);

    let seed: &[_] = &[1,2,3,4,5];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    println!("Creating the network...");
    let net = random_net(n,num_neighbours,&mut rng);
    let landmarks = choose_landmarks(&net,num_landmarks, &mut rng);
    println!("Iterating through coordinates");
    let coords = build_coords(&net, &landmarks);

    if coords.is_none() {
        println!("graph is not connected! Aborting.");
        return
    }

    let is_unique = is_coord_unique(&(coords.unwrap()));
    println!("is_unique = {}",is_unique);

}

pub fn check_ring_nums() {
    println!("{}",coord_to_ring(&vec![1,2,3]));
    println!("{}",coord_to_ring(&vec![2,3,1]));
    println!("{}",coord_to_ring(&vec![3,1,2]));

    println!("----------");

    println!("{}",coord_to_ring(&vec![5,2,4,8,5,9,4,1,1,5,8,7,3]));
    println!("{}",coord_to_ring(&vec![5,3,4,8,5,9,4,1,1,5,8,7,3]));
    println!("{}",coord_to_ring(&vec![5,4,4,8,5,9,4,1,1,5,8,7,3]));
    println!("{}",coord_to_ring(&vec![5,4,4,8,5,8,3,1,1,5,8,7,3]));
    println!("{}",coord_to_ring(&vec![5,4,4,8,5,8,3,1,1,5,8,55,3]));
    println!("{}",coord_to_ring(&vec![5,3,4,8,5,8,3,1,1,5,8,55,3]));
    println!("{}",coord_to_ring(&vec![5,3,4,8,5,3,3,1,1,5,8,55,3]));
    println!("{}",coord_to_ring(&vec![5,3,4,8,5,3,3,2,1,5,8,55,3]));
    println!("{}",coord_to_ring(&vec![5,4,4,8,6,2,3,2,1,5,7,55,2]));
}

/// Check the quality of distance approximation with the function fdisk using the pearson
/// monotonic correlation coefficient (Comparing to the real network distance)
pub fn check_approx_dist<F>(num_iters: u32, fdist: F, 
    net: &Network<usize>, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>, mut rng: &mut StdRng) 
    where F: DistAFunc {

    let mut dists = Vec::new();
    let mut adists = Vec::new();

    for _ in 0 .. num_iters {
        let node_pair: Vec<usize> = choose_k_nums(2,net.igraph.node_count(),&mut rng)
            .into_iter().collect::<Vec<_>>();

        let (u,v) = (node_pair[0], node_pair[1]);

        // Push real distance between u and v on the network:
        dists.push(net.dist(u,v).unwrap() as f64);

        // Push approximated distance between u and v:
        adists.push(fdist(u,v,&coords,&landmarks));
    }

    println!("spearman: {}",spearman(&dists,&adists).unwrap());
}

/// Try to find a path in the network between src_node and dst_node.
/// Returns None if path was not found, or Some(path_length
fn try_route(src_node: usize, dst_node: usize, coords: &Vec<Vec<u64>>, landmarks: &Vec<usize>) -> Option<usize> {
    // Node distance function:
    let node_dist = |x,y| approx_max_dist(x,y,&coords, &landmarks);
    let num_hops = 0;

    while cur_node != dest_node {
        let new_cur_node: usize = net.get_near_nodes(cur_node,gdist)
                                    .min_by(|i| node_dist(cur_node, i));
        if new_cur_node == cur_node {
            return None;
        }
        num_hops += 1;
        cur_node = new_cur_node;
    }
    Some(num_hops)
}

///
/// Check the success rate of routing in the network.
/// gdist is the greedy distance for hopping.
/// iters is the amount of iterations for this check.
pub fn check_routing(l: u32, gdist: usize, iters: usize) {

    // Set up graph parameters:
    // let l: u32 = 16;
    let n: usize = ((2 as u64).pow(l)) as usize;
    let num_neighbours: usize = (1.5 * (n as f64).ln()) as usize;
    let num_landmarks: usize = (((l*l) as u32)/3) as usize;
    let gdist: usize = 3; // Distance for greedy algorithm

    println!("n = {}",n);
    println!("num_neighbours = {}",num_neighbours);
    println!("num_landmarks = {}",num_landmarks);

    let seed: &[_] = &[1,2,3,4,5];
    let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
    println!("Creating the network...");
    let net = random_net(n,num_neighbours,&mut rng);
    let landmarks = choose_landmarks(&net,num_landmarks, &mut rng);
    println!("Iterating through coordinates");
    let coords = build_coords(&net, &landmarks);

    if coords.is_none() {
        println!("graph is not connected! Aborting.");
        return
    }

    let coords = coords.unwrap();

    // Amount of routing failures:
    let num_route_fails: usize = 0;
    // Sum of path length (Used for average later)
    let sum_route_length: usize = 0;

    for _ in iters {
        let node_pair: Vec<usize> = choose_k_nums(2,n,&mut rng)
            .into_iter().collect::<Vec<_>>();

        let num_hops = try_route(node_pair[0], node_pair[1]);
    }

}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_basic() {
        let seed: &[_] = &[1,2,3,4,5];
        let mut rng: StdRng = rand::SeedableRng::from_seed(seed);
        let net = random_net(60,5,&mut rng);
        let landmarks = choose_landmarks(&net,10,&mut rng);
        let coords = build_coords(&net, &landmarks);

        is_coord_unique(&(coords.unwrap()));
    }
}
