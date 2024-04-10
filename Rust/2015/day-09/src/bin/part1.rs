use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input: &str = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn parse_input(input: &str) -> (Vec<&str>, HashMap<(&str, &str), usize>) {
    let lines = input.lines().map(|f| f.trim()).collect::<Vec<&str>>();

    let mut cities_set: HashSet<&str> = HashSet::new();
    let mut distances: HashMap<(&str, &str), usize> = HashMap::new();

    for line in lines {
        let eq_sing_split = line.split(" = ").collect::<Vec<&str>>();
        let to_split = eq_sing_split
            .first()
            .unwrap()
            .split(" to ")
            .collect::<Vec<&str>>();
        let distance = eq_sing_split.last().unwrap().parse::<usize>().unwrap();

        let from_city = to_split.first().unwrap();
        let to_city = to_split.last().unwrap();
        cities_set.insert(&from_city);
        cities_set.insert(&to_city);

        distances.insert((from_city, to_city), distance);
    }

    let cities = cities_set.iter().map(|f| *f).collect::<Vec<&str>>();

    return (cities, distances);
}

fn make_adj_matrix(cities: Vec<&str>, distances: HashMap<(&str, &str), usize>) -> Vec<Vec<usize>> {
    let mut adjacency_matrix: Vec<Vec<usize>> = vec![vec![0; cities.len()]; cities.len()];

    for (i, &from_city) in cities.iter().enumerate() {
        for (j, &to_city) in cities.iter().enumerate() {
            if let Some(&distance) = distances.get(&(from_city, to_city)) {
                adjacency_matrix[i][j] = distance;
                adjacency_matrix[j][i] = distance;
            }
        }
    }
    return adjacency_matrix;
}

fn min_route(routes: Vec<Vec<&str>>, cities: Vec<&str>, adj_matrix: Vec<Vec<usize>>) -> usize {
    let mut min = usize::max_value();
    for path in &routes {
        let mut total_distance = 0;

        for i in 0..path.len() - 1 {
            let from_city = path[i];
            let to_city = path[i + 1];
            let from_index = cities.iter().position(|r| *r == from_city).unwrap();
            let to_index = cities.iter().position(|r| *r == to_city).unwrap();
            total_distance += adj_matrix[from_index][to_index];
        }
        if total_distance < min {
            min = total_distance;
        }
    }
    min
}

fn process(input: &str) -> usize {
    //? PARSE THE INPUT
    let (cities, distances) = parse_input(input);

    //? CREATE THE ADJACENCY MATRIX
    let adj_matrix = make_adj_matrix(cities.clone(), distances);

    //? CREATE THE ALL ROUTES ARRAY
    let routes = cities
        .iter()
        .permutations(cities.len())
        .unique()
        .map(|i| i.iter().map(|j| **j).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    //? FIND THE SHORTEST ROUTE AND RETURN IT
    min_route(routes, cities, adj_matrix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = process(
            "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        );
        assert_eq!(result, 605);
    }
}
