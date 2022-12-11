use std::collections::VecDeque;

pub struct Monkey {
  items: VecDeque<Item>,
  worry_expression: WorryExprNode,
  pub throw_conditional: ThrowCondition,
  pub inspections: u64
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThrowCondition {
  pub divisor: u64,
  if_true: usize,
  if_false: usize
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item { worry: u64 }

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorryExprNode {
  Operator(WorryOperatorNode)
}


#[derive(Debug, Clone, PartialEq, Eq)]
enum WorryOperatorNode {
  Mul(WorryIdentifierNode, WorryIdentifierNode),
  Add(WorryIdentifierNode, WorryIdentifierNode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorryIdentifierNode {
  Old,
  Literal(u32)
}

pub fn run_rounds(monkeys: &mut Vec<Monkey>, rounds: u32, update_worry: impl Fn(u64) -> u64) -> u64 {
  // 20 rounds
  for _ in 0..rounds {
    apply_round(monkeys, &update_worry);
  }
  find_most_active(monkeys)
}

fn find_most_active(monkeys: &Vec<Monkey>) -> u64 {
  let mut most_active: Option<&Monkey> = None;
  let mut second_most: Option<&Monkey> = None;

  for monkey in monkeys {
    if most_active.is_none() || most_active.unwrap().inspections < monkey.inspections {
      second_most = most_active;
      most_active = Some(monkey);
    } else if second_most.is_none() || second_most.unwrap().inspections < monkey.inspections {
      second_most = Some(monkey);
    }
  }

  most_active.map(|m| m.inspections).unwrap_or(1) * second_most.map(|m| m.inspections).unwrap_or(1)
}

pub fn apply_round(monkeys: &mut Vec<Monkey>, update_worry: impl Fn(u64) -> u64) {
  for idx in 0..monkeys.len() {
    while !monkeys[idx].items.is_empty() {
      let item_opt = monkeys[idx].items.pop_front();

      if let Some(mut item) = item_opt {
        let throw_idx = inspect_item(&mut item, &mut monkeys[idx], &update_worry);

        if throw_idx == idx {
          panic!("A MONKEY THROWING TO ITSELF IS AN INFINITE LOOP CRIME");
        }

        monkeys[throw_idx].items.push_back(item);
      }
    }
  }
}

pub fn inspect_item(item: &mut Item, monkey: &mut Monkey, update_worry: impl Fn(u64) -> u64) -> usize{
  // count inspection
  monkey.inspections += 1;

  // apply operation
  apply_worry(item, &monkey.worry_expression);

  // boredom, divide by 3
  // item.worry /= 3;
  // item.worry %= combined_divisors;
  item.worry = update_worry(item.worry);

  let condition = item.worry % monkey.throw_conditional.divisor == 0;

  if condition { monkey.throw_conditional.if_true } else { monkey.throw_conditional.if_false }
}

fn apply_worry(item: &mut Item, expr: &WorryExprNode) {
  let old = item.worry;
  item.worry = match expr {
    WorryExprNode::Operator(op) =>
      match op {
        WorryOperatorNode::Mul(left, right) => resolve(left, old) * resolve(right, old),
        WorryOperatorNode::Add(left, right) => resolve(left, old) + resolve(right, old),
      }
  };

}

fn resolve(identifier: &WorryIdentifierNode, old: u64) -> u64 {
  match identifier {
    WorryIdentifierNode::Old => old,
    WorryIdentifierNode::Literal(val) => *val as u64
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_apply_worry() {
    let mut item = Item { worry: 45 };
    let expr = WorryExprNode::Operator(WorryOperatorNode::Add(WorryIdentifierNode::Old, WorryIdentifierNode::Literal(6)));
    apply_worry(&mut item, &expr);
    assert_eq!(item.worry, 51);
  }
}


pub mod parser {
  use super::*;

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct ParseError {
    msg: String
  }

  pub type ParseResult<T> = std::result::Result<T, ParseError>;

  pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let monkey_inputs = input.split("\n\n");
    monkey_inputs.map(|m_str| parse_monkey(m_str).unwrap()).collect()
  }

  pub fn parse_monkey(monkey_str: &str) -> ParseResult<Monkey> {
    let mut lines = monkey_str.split('\n');

    let monkey_line = lines.next().unwrap_or("");
    assert!(monkey_line.starts_with("Monkey "));

    let starting_items = parse_starting_items(lines.next().unwrap_or(""))?;

    let worry_expr = parse_operation_line(lines.next().unwrap_or(""))?;

    let throw_condition = parse_test_line(lines.next().unwrap_or(""), lines.next().unwrap_or(""), lines.next().unwrap_or(""));

    Ok(Monkey { items: starting_items, worry_expression: worry_expr, throw_conditional: throw_condition, inspections: 0 })
  }

  fn parse_starting_items(items_line: &str) -> ParseResult<VecDeque<Item>> {
    let items_line = items_line.trim();
    assert!(items_line.starts_with("Starting items: "));

    let (_, after) = items_line.split_once(':').unwrap();

    let starting_items: VecDeque<Item> = after.split(',').map(|i| Item { worry: i.trim().parse::<u64>().unwrap() }).collect();
    Ok(starting_items)
  }

  fn parse_operation_line(operation_line: &str) -> ParseResult<WorryExprNode> {
    let operation_line = operation_line.trim();
    assert!(operation_line.starts_with("Operation: "));
    let (_, after) = operation_line.split_once("new = ").unwrap();

    // find the operator
    let mut tokens = after.split(' ');

    let left = parse_identifier(tokens.next().unwrap_or(""))?;
    let op_str = tokens.next().unwrap_or("");
    let right = parse_identifier(tokens.next().unwrap_or(""))?;

    let operator = parse_operator(op_str, left, right)?;
    // let operator = WorryOp::Add(Identifier::Old, Identifier::Old);

    Ok(WorryExprNode::Operator(operator))
  }

  fn parse_operator(op_token: &str, left: WorryIdentifierNode, right: WorryIdentifierNode) -> ParseResult<WorryOperatorNode>{
    match op_token {
      "+" => Ok(WorryOperatorNode::Add(left, right)),
      // "-" => Ok(WorryOperatorNode::Sub(left, right)),
      "*" => Ok(WorryOperatorNode::Mul(left, right)),
      // "/" => Ok(WorryOperatorNode::Div(left, right)),
      _ => Err(ParseError { msg: format!("Unrecognized operator token {}", op_token)})
    }
  }


  fn parse_test_line(test_line: &str, true_line: &str, false_line: &str) -> ThrowCondition {
    let test_line = test_line.trim();
    let true_line = true_line.trim();
    let false_line = false_line.trim();

    assert!(test_line.starts_with("Test:"));
    assert!(true_line.starts_with("If true:"));
    assert!(false_line.starts_with("If false:"));

    let (_, after) = test_line.split_once("divisible by ").unwrap();
    let divisor = after.parse::<u64>().unwrap();

    let (_, after) = true_line.split_once("throw to monkey ").unwrap();
    let if_true = after.parse::<usize>().unwrap();

    let (_, after) = false_line.split_once("throw to monkey ").unwrap();
    let if_false = after.parse::<usize>().unwrap();

    ThrowCondition { divisor, if_true, if_false }
  }


  fn parse_identifier(identifier_token: &str) -> ParseResult<WorryIdentifierNode> {
    if identifier_token == "old" {
      Ok(WorryIdentifierNode::Old)
    } else {
      let literal = identifier_token.parse::<u32>()?;
      Ok(WorryIdentifierNode::Literal(literal))
    }
  }

  impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "An error occurred during parsing: {}", self.msg)
    }
  }

  impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
      ParseError { msg: err.to_string() }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_monkey() {
      let parsed = parse_monkey("Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3").unwrap();
      assert_eq!(parsed.items, vec![Item { worry: 79 }, Item { worry: 60}, Item { worry: 97}]);
      assert_eq!(parsed.worry_expression, WorryExprNode::Operator(WorryOperatorNode::Mul(WorryIdentifierNode::Old, WorryIdentifierNode::Old)));
      assert_eq!(parsed.throw_conditional, ThrowCondition { divisor: 13, if_true: 1, if_false: 3});
    }

    #[test]
    fn parse_operation_line_old_mul_19() {
      let parsed = parse_operation_line("  Operation: new = old * 19").unwrap();
      assert_eq!(parsed, WorryExprNode::Operator(WorryOperatorNode::Mul(WorryIdentifierNode::Old, WorryIdentifierNode::Literal(19))));
    }

    #[test]
    fn parse_operation_line_old_add_old() {
      let parsed = parse_operation_line("Operation: new = old + old").unwrap();
      assert_eq!(parsed, WorryExprNode::Operator(WorryOperatorNode::Add(WorryIdentifierNode::Old, WorryIdentifierNode::Old)));
    }

    #[test]
    fn parse_operation_line_invalid() {
      let parsed = parse_operation_line("Operation: new = old ^ 19");
      assert!(parsed.is_err());
    }

    #[test]
    fn test_parse_operators() {
      assert_eq!(parse_operator("*", WorryIdentifierNode::Old, WorryIdentifierNode::Literal(3)), Ok(WorryOperatorNode::Mul(WorryIdentifierNode::Old, WorryIdentifierNode::Literal(3))));
      assert!(parse_operator("^", WorryIdentifierNode::Old, WorryIdentifierNode::Old).is_err());
    }

    #[test]
    fn parse_identifier_old() {
      assert_eq!(parse_identifier("old"), Ok(WorryIdentifierNode::Old));
    }

    #[test]
    fn parse_identifier_literals() {
      assert_eq!(parse_identifier("3"), Ok(WorryIdentifierNode::Literal(3)));

    }

    #[test]
    fn parse_identifier_unknown() {
      assert!(parse_identifier("3.14").is_err());
      assert!(parse_identifier("foo").is_err());
    }
  }
}