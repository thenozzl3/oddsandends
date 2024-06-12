//use core::num::dec2flt::parse::parse_inf_nan;
use std::io;
use std::io::prelude::*;
//use std::collections::HashMap;
use std::str::FromStr;
//use petgraph::graph::NodeIndex;
//use petgraph::{Graph, visit::Dfs,visit::Bfs};
//
use std::cmp;


fn main() {
  // .. this was a neat try ..
  //let mut paths = Graph::new_undirected();
  let mut node_list:Vec<String> = Vec::new();
  //  let visited_nodes:Vec<bool> = Vec::new();
  let mut adj_matrix:Vec<Vec<i32>> = vec![];
  //let node_count = 0;
  //read in the routes

  for line in io::stdin().lock().lines() {

    let line_tmp = line.unwrap().clone();
    //current line
    let elts = line_tmp.split(" ").collect::<Vec<&str>>();
     println!("some shit here .. ");

    for elt in &elts {
        let elt_string = String::from_str(elt).unwrap();
        if elt.matches("to").count() > 0 ||
            elt.matches("=").count() > 0 { continue };

        //either we are setting a weight or adding node to the adj. matrix.
        if let Ok(weight) = elt.parse::<i32>() {
            //println!("found a weight.. adding node.. {}", weight);
            //look up the nodes in the node list .. the index is 
            //used in the adj_matrix ...
            let first_index = node_list.iter().position(|r| r == elts[0]).unwrap();
            let second_index = node_list.iter().position(|r| r == elts[2]).unwrap();
            adj_matrix[first_index][second_index] = weight;
            adj_matrix[second_index][first_index] = weight;


            //println!("{} {}", first_index,second_index);

            /*
               paths.add_edge(
             *list.get(&String::from_str(elts[0]).unwrap()).unwrap(),
             *list.get(&String::from_str(elts[2]).unwrap()).unwrap(),
             weight
             );
             */
        } else {
            // don't add if have the key already
            if let Some(_) = node_list.iter().position(|r| r == *elt) {
                continue;
            } else {
                //println!("node not found - extend matrix..");
                //not in adj_matrix yet ..
                node_list.push(elt_string.clone());
                for row in adj_matrix.iter_mut() {
                    if row.len() < node_list.len() {
                        for _ in  0..(node_list.len() - row.len()) {
                            row.push(0);
                        }
                    }
                }
                //add a row
                adj_matrix.push(vec![0;node_list.len()]);
            }
        }
    }
  }
  println!("nodes : {:?}",node_list);
  for row in adj_matrix.iter() {
      println!("{:?}",row);
  }
  let mut ans: i32 = 100000;
  let mut been: Vec<bool> = vec![ false; node_list.len()];
  let node_list_len =  node_list.len() ;
  been[0] = true;

  tsp(&mut adj_matrix,
      &mut been,
      &mut node_list,
      0,
      node_list_len as i32,
      1 ,
      0,
      &mut ans);

  println!("distance : {}", ans);
}

fn tsp(v: &mut Vec<Vec<i32>>,
       visited_nodes: &mut Vec<bool>,
       node_list: &mut Vec<String>,
       curr_pos: usize,
       n: i32,
       count: i32,
       cost: i32,
       ans: &mut i32 )    {

    println!(" pos {}  cost {} count {} n {} ans {}",curr_pos, cost,count,n,ans);
    // If last node is reached and it has a link
    // to the starting node i.e the source then
    // keep the minimum value out of the total cost
    // of traversal and "ans"
    // Finally return to check for more possible values
    /*
    if count == 1 && v[curr_pos][0] == 0 {
        println!("starting at {}", node_list[0]);
        visited_nodes[0] = true;
    }*/
    if count == n && v[curr_pos][0] != 0 {
        println!(" {:?}",visited_nodes);
        println!("reached all nodes - returning...");
        *ans = cmp::min(*ans,  cost + v[curr_pos][0] );
        return;
    }
    // BACKTRACKING STEP
    // Loop to traverse the adjacency list
    // of currPos node and increasing the count
    // by 1 and cost by graph[currPos][i] value
    for i in 0..n as usize {
        if !visited_nodes[i] && v[curr_pos][i] != 0 {
        println!(" {:?}",visited_nodes);
        println!("going to .. {}", node_list[i]);

            // Mark as visited
            visited_nodes[i] = true;
            tsp(v,
                visited_nodes,
                node_list,
                i ,
                n,
                count + 1,
                cost + v[curr_pos][i],
                ans);

            // Mark ith node as unvisited
            visited_nodes[i] = false;
        }
    }

}








    //tsp(graph, v, 0, n, 1, 0, ans);


  // Vec.

  //println!("nodes to visit .. {}", list.keys().count());
  //println!("edges  .. {}",paths.edge_count());
  //let mut  total_travel = 0;
  /*
   * neat try .. but this is gotta be el'manuelz. No access to the adj adj_matrix
   * under the hood ... 
   for v in &list {
   println!("\ndfs from {:?}",v);
   let mut dfs = Dfs::new(&paths,*v.1);

   let mut previous_node = dfs.next(&paths).unwrap();

   while let Some(node) = dfs.next(&paths){
   print!("{} to {} ",paths.node_weight(previous_node).unwrap(),
   paths.node_weight(node).unwrap());
   if let Some(current_edge) = &paths.find_edge(previous_node,node){
   let distance = *&paths.edge_weight(*current_edge).unwrap();
   println!("distance : {} ", distance);
   total_travel += distance;
   }
   previous_node = node;
   }

   println!("total travel dfs  {}", total_travel);

   if minimum_travel == 0 || total_travel < minimum_travel{
   minimum_travel = total_travel;
   }
   total_travel = 0;
   }
   println!("minimum travel dfs : {}", minimum_travel);

   minimum_travel =0 ;
   for v in &list {
   println!("bfs from {:?}",v);
   let mut bfs = Bfs::new(&paths,*v.1);

   let mut previous_node = bfs.next(&paths).unwrap();

   while let Some(node) = bfs.next(&paths){
   println!("nodes: {:?} {:?}",previous_node,node);
   if let Some(current_edge) = &paths.find_edge(previous_node,node){
   let distance = *&paths.edge_weight(*current_edge).unwrap();
   print!("distance : {} ", distance);
   total_travel += distance;
   }
   previous_node = node;
   }

   println!("total travel bfs  {}", total_travel);
   if minimum_travel == 0 || total_travel < minimum_travel{
   minimum_travel = total_travel;
   }
   total_travel = 0;
   }
   println!("minimum travel bfs : {}", minimum_travel);
   */
