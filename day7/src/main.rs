use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, digit1},
    combinator::{map, map_res, recognize, value},
    multi::separated_list1,
    sequence::{separated_pair, terminated, tuple},
    IResult,
};
use petgraph::{
    graphmap::DiGraphMap,
    visit::{Dfs, Reversed, Walker},
};
use std::{
    cell::RefCell,
    error::Error,
    fmt::{self, Display},
    io::{stdin, BufRead},
    str::FromStr,
    string::ToString,
};
use string_interner::{DefaultSymbol, StringInterner};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
struct BagName(DefaultSymbol);

impl BagName {
    fn new(i: &str) -> Self {
        Self(INTERNED_BAG_NAMES.with(|c| c.borrow_mut().get_or_intern(i)))
    }
}

impl Display for BagName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        INTERNED_BAG_NAMES.with(|c| write!(f, "{}", c.borrow().resolve(self.0).unwrap()))
    }
}

#[derive(Debug)]
struct Rule {
    bag_name: BagName,
    elements: Vec<(u32, BagName)>,
}

thread_local! {
    static INTERNED_BAG_NAMES: RefCell<StringInterner> = Default::default();
}

fn bag_name(input: &str) -> IResult<&str, BagName> {
    map(
        recognize(separated_pair(alpha1, char(' '), alpha1)),
        BagName::new,
    )(input)
}

fn element(input: &str) -> IResult<&str, (u32, BagName)> {
    terminated(
        tuple((
            terminated(map_res(digit1, FromStr::from_str), char(' ')),
            bag_name,
        )),
        alt((tag(" bags"), tag(" bag"))),
    )(input)
}

fn elements(input: &str) -> IResult<&str, Vec<(u32, BagName)>> {
    terminated(
        alt((
            value(Vec::new(), tag("no other bags")),
            separated_list1(tag(", "), element),
        )),
        char('.'),
    )(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(bag_name, tag(" bags contain "), elements),
        |(bag_name, elements)| Rule { bag_name, elements },
    )(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut graph = DiGraphMap::new();
    for line in stdin.lines() {
        let line = line?;
        let rule = rule(&line).map_err(|e| e.to_string())?.1;

        for (n, bag_name) in rule.elements {
            graph.add_edge(rule.bag_name, bag_name, n);
        }
    }

    let target_bag = BagName::new("shiny gold");

    {
        let graph = Reversed(&graph);
        let res = Dfs::new(graph, target_bag).iter(graph).count();

        println!(
            "bags that transitively contain {}: {}",
            target_bag,
            res - 1 /* shiny gold itself not counted*/
        );
    }

    fn transitive_children(graph: &DiGraphMap<BagName, u32>, node: BagName) -> u32 {
        graph
            .edges(node)
            .map(|(_, next, n)| n * (1 + transitive_children(graph, next)))
            .sum::<u32>()
    }

    println!(
        "number of bags transitively inside a {}: {}",
        target_bag,
        transitive_children(&graph, target_bag)
    );

    Ok(())
}
