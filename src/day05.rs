use crate::solution::Solution;
use nom::character::complete::{digit1, newline};
use nom::multi::{many_m_n, separated_list1};
use nom::sequence::separated_pair;
use nom::IResult;
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;
use std::cmp::Ordering;
use std::ops::Deref;

pub struct AdventPuzzle;

impl Solution for AdventPuzzle {
    fn part1(input: &str) -> String {
        let (_, (page_rules, page_orders)) = parse_page_rules_and_orders(input).unwrap();
        page_orders
            .iter()
            .filter(|order| order.is_valid(&page_rules))
            .map(|order| order.get_middle_page())
            .sum::<usize>()
            .to_string()
    }

    fn part2(input: &str) -> String {
        let (_, (page_rules, mut page_orders)) = parse_page_rules_and_orders(input).unwrap();
        page_orders
            .0
            .iter_mut()
            .filter(|order| !order.is_valid(&page_rules))
            .map(|order| {
                order.fix(&page_rules);
                order.get_middle_page()
            })
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug, PartialEq)]
struct PageRule {
    before: usize,
    after: usize,
}

impl PageRule {
    fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }

    fn get_page_after(&self, page: usize) -> Option<usize> {
        (page == self.before).then(|| self.after)
    }
}

impl PartialOrd for PageRule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.before.partial_cmp(&other.after)
    }
}

fn parse_page_rule(input: &str) -> IResult<&str, PageRule, ErrorTree<&str>> {
    separated_pair(
        digit1.map_res(str::parse),
        tag("|"),
        digit1.map_res(str::parse),
    )(input)
    .map(|(remainder, (before, after))| (remainder, PageRule { before, after }))
}

#[derive(Debug, PartialEq)]
struct PageRules(Vec<PageRule>);

impl PageRules {
    fn get_pages_after(&self, page: usize) -> Vec<usize> {
        self.iter()
            .map(|rule| rule.get_page_after(page))
            .filter_map(|rule| rule)
            .collect()
    }
}

impl Deref for PageRules {
    type Target = Vec<PageRule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_page_rules(input: &str) -> IResult<&str, PageRules, ErrorTree<&str>> {
    separated_list1(newline, parse_page_rule)(input)
        .map(|(remainder, rules)| (remainder, PageRules(rules)))
}

#[derive(Debug, PartialEq)]
struct PageOrder(Vec<usize>);

impl Deref for PageOrder {
    type Target = Vec<usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PageOrder {
    fn get_middle_page(&self) -> usize {
        self[self.0.len() / 2]
    }

    fn is_valid(&self, page_rules: &PageRules) -> bool {
        for i in 0..self.0.len() {
            let current_page = self[i];
            let pages_after_current_page = page_rules.get_pages_after(current_page);
            if self[0..i]
                .iter()
                .any(|previous_page| pages_after_current_page.contains(previous_page))
            {
                return false;
            }
        }
        true
    }

    fn fix(&mut self, page_rules: &PageRules) {
        for i in 0..self.0.len() {
            let current_page = self[i];
            let pages_after_current_page = page_rules.get_pages_after(current_page);
            if let Some((j, _)) = self[0..i]
                .iter()
                .enumerate()
                .find(|(j, previous_page)| pages_after_current_page.contains(previous_page))
            {
                self.0.swap(i, j);
                self.fix(&page_rules);
            }
        }
    }
}

fn parse_page_order(input: &str) -> IResult<&str, PageOrder, ErrorTree<&str>> {
    separated_list1(tag(","), digit1.map_res(str::parse))(input)
        .map(|(remainder, order)| (remainder, PageOrder(order)))
}

#[derive(Debug, PartialEq)]
struct PageOrders(Vec<PageOrder>);

impl Deref for PageOrders {
    type Target = Vec<PageOrder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn parse_page_orders(input: &str) -> IResult<&str, PageOrders, ErrorTree<&str>> {
    separated_list1(newline, parse_page_order)(input)
        .map(|(remainder, orders)| (remainder, PageOrders(orders)))
}

fn parse_page_rules_and_orders(
    input: &str,
) -> IResult<&str, (PageRules, PageOrders), ErrorTree<&str>> {
    separated_pair(parse_page_rules, many_m_n(2, 2, newline), parse_page_orders)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_page_rule() {
        let input = "123|456";
        let rule = parse_page_rule(input).unwrap().1;
        assert_eq!(rule.before, 123);
        assert_eq!(rule.after, 456);
    }

    #[test]
    fn test_parse_page_rules() {
        let input = "1|2
3|4
5|6";
        let rules = parse_page_rules(input).unwrap().1;
        assert_eq!(
            rules,
            PageRules(vec![
                PageRule::new(1, 2),
                PageRule::new(3, 4),
                PageRule::new(5, 6),
            ])
        );
    }

    #[test]
    fn test_page_order_middle_value() {
        let order = PageOrder(vec![75, 47, 61, 53, 29]);
        assert_eq!(order.get_middle_page(), 61);
        let order = PageOrder(vec![75, 29, 13]);
        assert_eq!(order.get_middle_page(), 29);
    }

    #[test]
    fn test_parse_page_order() {
        let input = "75,47,61,53,29";
        let order = parse_page_order(input).unwrap().1;
        assert_eq!(order, PageOrder(vec![75, 47, 61, 53, 29]));
    }

    #[test]
    fn test_parse_page_orders() {
        let input = "75,47,61,53,29
97,61,53,29,13";
        let orders = parse_page_orders(input).unwrap().1;
        assert_eq!(
            orders,
            PageOrders(vec![
                PageOrder(vec![75, 47, 61, 53, 29]),
                PageOrder(vec![97, 61, 53, 29, 13]),
            ])
        );
    }

    #[test]
    fn test_parse_page_order_is_valid() {
        let page_rules = PageRules(vec![
            PageRule::new(47, 53),
            PageRule::new(97, 13),
            PageRule::new(97, 61),
            PageRule::new(97, 47),
            PageRule::new(75, 29),
            PageRule::new(61, 13),
            PageRule::new(75, 53),
            PageRule::new(29, 13),
            PageRule::new(97, 29),
            PageRule::new(53, 29),
            PageRule::new(61, 53),
            PageRule::new(97, 53),
            PageRule::new(61, 29),
            PageRule::new(47, 13),
            PageRule::new(75, 47),
            PageRule::new(97, 75),
            PageRule::new(47, 61),
            PageRule::new(75, 61),
            PageRule::new(47, 29),
            PageRule::new(75, 13),
            PageRule::new(53, 13),
        ]);

        let test_1 = PageOrder(vec![75, 47, 61, 53, 29]);
        let test_2 = PageOrder(vec![97, 61, 53, 29, 13]);
        let test_3 = PageOrder(vec![75, 29, 13]);
        let test_4 = PageOrder(vec![75, 97, 47, 61, 53]);
        let test_5 = PageOrder(vec![61, 13, 29]);
        let test_6 = PageOrder(vec![97, 13, 75, 29, 47]);

        assert!(test_1.is_valid(&page_rules));
        assert!(test_2.is_valid(&page_rules));
        assert!(test_3.is_valid(&page_rules));
        assert!(!test_4.is_valid(&page_rules));
        assert!(!test_5.is_valid(&page_rules));
        assert!(!test_6.is_valid(&page_rules));
    }

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(AdventPuzzle::part1(input), "143");
    }

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(AdventPuzzle::part2(input), "123");
    }
}
