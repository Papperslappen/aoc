use std::collections::HashMap;

use pom::{utf8::*, char_class::alpha};
use petgraph::*;
use util::{raw_to_strings, parser::utf8::space};

type NodeName = String; 
enum pulse{
    High,
    Low,
}

#[derive(Clone)]
enum NodeType{
    Broadcast,
    FlipFlop,
    Conjuction, 
    Output,
}

fn node_name_parser<'a>()->Parser<'a,NodeName>{
    is_a(|c| c.is_alphabetic()).repeat(1..).collect().map(|s|s.to_string())
}

fn node_parser<'a>()->Parser<'a,(NodeName,NodeType)>{
    (sym('&')*node_name_parser()).map(|name|(name,NodeType::Conjuction)) 
    | (sym('%')*node_name_parser()).map(|name|(name,NodeType::FlipFlop))
    | seq("broadcast").map(|_| ("broadcast".to_string(),NodeType::Broadcast)) 
}

fn edge_parser<'a>()->Parser<'a,((NodeName,NodeType),Vec<NodeName>)>{
    node_parser() + space() * seq("->") * space() * list::<char, _, _>(node_name_parser(),space()) 
}

fn solution_a(input: &[String]) -> u64 {
    let parser = edge_parser();
    let edges = input.iter().map(|s| parser.parse(s.as_bytes()).unwrap()).collect::<Vec<_>>();
    let nodetypes = edges.iter().map(|s| s.0.clone()).collect::<HashMap<_,_>>();
    let l = edges.into_iter()
    .flat_map(|s| s.1.into_iter()
    .map(|nn| (s.0.0.clone(),nn))).collect::<Vec<_>>();

    let graph = Graph::<NodeName,()>::from_edges(
        l);
    
    0
}

fn solution_b(input: &str) -> u64 {
    0
}

#[test]
fn test_solutions() {
    let input1 = raw_to_strings(r"broadcaster -> a, b, c
    %a -> b
    %b -> c
    %c -> inv
    &inv -> a");

    let input2 = raw_to_strings(r"broadcaster -> a
    %a -> inv, con
    &inv -> b
    %b -> con
    &con -> output");

    assert_eq!(solution_a(&input1), 19114);
    //assert_eq!(solution_b(input1), 167409079868000);
}

fn main() {
    println!("input:");
    let input = util::get_input_rows();
    println!("Answer puzzle A: {}", solution_a(&input));
    //println!("Answer puzzle B: {}", solution_b(&input));
}
