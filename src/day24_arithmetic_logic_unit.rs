use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::mem::take;
use std::str::{SplitWhitespace};

pub(crate) fn run() {
    let _input = "inp w
add x 12
eql x w
inp y
add y x
add y 3
eql w x
add y 4
add y w
add z y";
    let _input = _get_input();

    let mut alu: ALU<ExpressionSet> = Default::default();
    let instructions: Vec<Instruction> = _input.split('\n').map(|line| line.try_into().unwrap()).collect();
    let mut input = (0..14).into_iter().map(|i| ExpressionValue::Input(i).into());
    let total = instructions.len();
    for (i, instruction) in instructions.iter().enumerate()/*.take(36)*/ {
        println!("processing instruction {}/{} ({}%) {}", i + 1, total, (i + 1) * 100 / total, instruction._source);
        alu.process(instruction, &mut input);
        // for c in ['w', 'x', 'y', 'z'].iter() {
        //     println!("value {}:\n{:?}", c, alu.value_at(c));
        // }
    }
    alu.try_zero_out();
    alu.optimize();
    let z = alu.value_at(&'z');
    println!("value at z:\n\n{:?}", z);
    let mut dependencies: Vec<_> = Vec::from_iter(z.get_dependencies().into_iter());
    dependencies.sort();
    println!("dependencies of z: {:?}", dependencies);
    if let Some(committed) = alu.possible_input_values.iter().next() {
        let mut committed: Vec<_> = committed.iter().map(|(&i, &val)| (i, val)).collect();
        committed.sort_by(|(a, _), (b, _)| a.cmp(b));
        let committed: Vec<_> = committed.into_iter().map(|(_, val)| val).collect();
        println!("committed values: {:?}", committed);
    }

    let z: Equation = z.try_into().unwrap();
    println!("equation:\n{:?}", z);

    let max_input = z.try_solve(14);
    println!("max input:{:?}", max_input);

    let committed = get_committed_input_values(instructions.iter());
    println!("committed: {:?}", committed);

    let max_input = get_max_input(&committed, &instructions[..], false);
    println!("max input: {:?}", max_input);

    let min_input = get_max_input(&committed, &instructions[..], true);
    println!("min input: {:?}", min_input);
}

fn get_max_input(committed: &HashMap<usize, Expression>, instructions: &[Instruction], get_min: bool) -> u64 {
    let mut missing: Vec<_> = (0..14).filter(|i| !committed.contains_key(i)).collect();
    missing.sort();
    let mut input: Vec<i64> = vec![if get_min { 1 } else { 9 }; missing.len()];
    let get_run_input = |input: &Vec<i64>| {
        let new_input: HashMap<usize, i64> = missing.iter().enumerate().map(|(i, &pos)| (pos, input[i])).collect();
        let full_input = get_full_input_mappings(new_input, committed);
        let run_input: Vec<i64> = (0..=*full_input.keys().max().unwrap()).map(|k| *full_input.get(&k).unwrap()).collect();
        run_input
    };
    let operation = if get_min { add } else { subtract };
    loop {
        let run_input = get_run_input(&input);
        if run_input.iter().all(|&n| n <= 9 && n > 0) {
            println!("trying with {:?}", run_input);
            let mut alu = AluV2::new(run_input);
            for instruction in instructions {
                alu.process(instruction);
            }
            if alu.value_at(&'z') == 0 {
                return get_run_input(&input).into_iter().rev().enumerate().map(|(i, val)| val as u64 * 10u64.pow(i as u32)).sum::<u64>();
            }
        }

        if !operation(&mut input) {
            panic!("ran out of numbers :(");
        }
    }
}

fn get_full_input_mappings(input: HashMap<usize, i64>, committed_values: &HashMap<usize, Expression>) -> HashMap<usize, i64> {
    let mut input = input;
    for i in 0..14 {
        if !input.contains_key(&i) {
            input.insert(i, committed_values.get(&i).unwrap().evaluate(&input));
        }
    }

    input
}

fn get_committed_input_values<'a>(mut instructions: impl Iterator<Item=&'a Instruction<'a>>) -> HashMap<usize, Expression> {
    let mut result: HashMap<usize, Expression> = Default::default();
    let mut stack: Vec<(usize, i64)> = Default::default();
    let mut i = 0usize;
    loop {
        if instructions.next().is_none() {
            break;
        }
        for _ in 0..3 {
            instructions.next();
        }
        let a = instructions.next().unwrap().operation.get_numeric_value();
        let b = instructions.next().unwrap().operation.get_numeric_value();
        for _ in 0..9 {
            instructions.next();
        }
        let c = instructions.next().unwrap().operation.get_numeric_value();

        for _ in 0..2 {
            instructions.next();
        }

        if a == 1 {
            // push
            stack.push((i, c));
        } else {
            // pop
            let (i_old, c) = stack.pop().unwrap();
            let value = Expression {
                value: ExpressionValue::Input(i_old),
                modifier: Some(Box::new((Operation::Add, Expression {
                    value: ExpressionValue::Numeric(c + b),
                    modifier: None,
                }))),
            };
            result.insert(i, value);
        }
        i += 1;
    }

    result
}


enum EquationValue {
    Numeric(i64),
    Var(usize),
    Recursive(Box<EquationExpression>),
}

impl EquationValue {
    pub(crate) fn value_with(&self, input: &[u64]) -> i64 {
        match self {
            EquationValue::Numeric(n) => *n,
            EquationValue::Var(index) => input[*index] as i64,
            EquationValue::Recursive(r) => r.value_with(input)
        }
    }
}

impl Debug for EquationValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            EquationValue::Numeric(val) => write!(f, "{}", *val),
            EquationValue::Var(i) => write!(f, "x_{}", *i),
            EquationValue::Recursive(r) => write!(f, "({:?})", *r)
        }
    }
}

impl Debug for EquationExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operation = match &self.operation {
            Operation::Add => '+',
            Operation::Multiply => '*',
            Operation::Divide => '/',
            Operation::Mod => '%'
        };
        write!(f, "{:?} {} {:?}", self.lhs, operation, self.rhs)
    }
}

impl Default for EquationValue {
    fn default() -> Self {
        EquationValue::Numeric(0)
    }
}

struct EquationExpression {
    lhs: EquationValue,
    operation: Operation,
    rhs: EquationValue,
}

impl EquationExpression {
    #[inline]
    fn operate_with(lhs: i64, operation: &Operation, rhs: i64) -> i64 {
        match operation {
            Operation::Add => lhs + rhs,
            Operation::Multiply => lhs * rhs,
            Operation::Divide => lhs / rhs,
            Operation::Mod => lhs % rhs
        }
    }
    pub(crate) fn value_with(&self, input: &[u64]) -> i64 {
        Self::operate_with(self.lhs.value_with(input), &self.operation, self.rhs.value_with(input))
    }
}

struct Equation(EquationExpression);

impl Equation {
    fn is_valid(&self, input: &[u64]) -> bool {
        let expression = self.clone();
        expression.0.value_with(input) == 0
    }
    pub(crate) fn try_solve(&self, total_input_len: usize) -> Option<u64> {
        if total_input_len > 10 {
            return None;
        }
        let mut input = vec![9; total_input_len];
        loop {
            if self.is_valid(&input) {
                return Some(input.into_iter().rev().enumerate().fold(0, |sum, (i, val)| sum + 10u64.pow(i as u32) * val));
            }

            let mut big_change = false;
            for i in (0..total_input_len).rev() {
                if input[i] == 1 {
                    input[i] = 9;
                    if i < 8 {
                        big_change = true;
                    }
                } else {
                    input[i] -= 1;
                    break;
                }
            }
            if big_change {
                println!("trying with {:?}", input);
            }
        }
    }
}

impl Debug for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug)]
enum ParseEquationError {
    UnknownError,
    InvalidNumberOfValues,
    InvalidInput,
}

impl TryInto<Equation> for ExpressionSet {
    type Error = ParseEquationError;

    fn try_into(self) -> Result<Equation, Self::Error> {
        if self.values.len() != 1 {
            return Err(ParseEquationError::InvalidNumberOfValues);
        }
        let value = self.values.into_iter().next().unwrap().get_as_simple_value_move().ok_or(ParseEquationError::UnknownError)?;
        let expression: Expression;
        if let ExpressionValue::Recursive(val) = value {
            expression = *val;
        } else if let ExpressionValue::Numeric(_) = value {
            return Ok(Equation {
                0: EquationExpression {
                    lhs: value.try_into()?,
                    operation: Operation::Add,
                    rhs: Default::default(),
                }
            });
        } else {
            return Err(ParseEquationError::InvalidInput);
        }

        let operation;
        let inner;
        if let Some(m) = expression.modifier {
            inner = (*m).1;
            operation = (*m).0;
        } else {
            return Err(ParseEquationError::InvalidInput);
        }

        Ok(Equation(EquationExpression {
            lhs: expression.value.try_into()?,
            operation,
            rhs: ExpressionValue::Recursive(Box::new(inner)).try_into()?,
        }))
    }
}

impl TryInto<EquationValue> for ExpressionValue {
    type Error = ParseEquationError;

    fn try_into(self) -> Result<EquationValue, Self::Error> {
        match self {
            ExpressionValue::Numeric(n) => Ok(EquationValue::Numeric(n)),
            ExpressionValue::Input(i) => Ok(EquationValue::Var(i)),
            ExpressionValue::Recursive(r) => {
                let lhs = r.value;
                if let Some(inner) = r.modifier {
                    let operation = (*inner).0;
                    let rhs = ExpressionValue::Recursive(Box::new((*inner).1));
                    Ok(EquationValue::Recursive(Box::new(EquationExpression {
                        lhs: lhs.try_into()?,
                        operation,
                        rhs: rhs.try_into()?,
                    })))
                } else {
                    Ok(lhs.try_into()?)
                }
            }
        }
    }
}

trait Operationable: Default + Clone + Debug {
    fn operate(&mut self, operation: Operation, rhs: Self);
    fn from_numeric(number: i64) -> Self;
    fn operate_equals(&mut self, rhs: Self);
    fn cleanup(&mut self);
    fn try_zero_out(&self, possible_input_values: &Vec<HashMap<usize, i64>>) -> (Option<Box<dyn Fn(Self) -> Self>>, Vec<HashMap<usize, i64>>);
}

impl Operationable for i64 {
    fn operate(&mut self, operation: Operation, rhs: Self) {
        match operation {
            Operation::Add => *self += rhs,
            Operation::Multiply => *self *= rhs,
            Operation::Divide => *self /= rhs,
            Operation::Mod => *self %= rhs
        }
    }

    fn from_numeric(number: i64) -> Self {
        number
    }

    fn operate_equals(&mut self, rhs: Self) {
        *self = if *self == rhs { 1 } else { 0 }
    }

    fn cleanup(&mut self) {}

    fn try_zero_out(&self, _: &Vec<HashMap<usize, i64>>) -> (Option<Box<dyn Fn(Self) -> Self>>, Vec<HashMap<usize, i64>>) {
        (None, Default::default())
    }
}

impl From<ExpressionValue> for ExpressionSet {
    fn from(val: ExpressionValue) -> Self {
        Self {
            values: vec![ExpressionStatement {
                conditions: Default::default(),
                value: val,
            }]
        }
    }
}

impl ExpressionValue {
    pub(crate) fn operate(&mut self, operation: Operation, rhs: Self) {
        if let Operation::Multiply = operation {
            if let ExpressionValue::Numeric(0) = rhs {
                *self = Self::Numeric(0);
                return;
            } else if let ExpressionValue::Numeric(0) = self {
                *self = Self::Numeric(0);
                return;
            } else if let ExpressionValue::Numeric(1) = rhs {
                return;
            } else if let ExpressionValue::Numeric(1) = self {
                *self = rhs;
                return;
            }
        }
        if let Operation::Divide = operation {
            if let ExpressionValue::Numeric(0) = rhs {
                panic!("attempted to divide by zero");
            } else if let ExpressionValue::Numeric(0) = self {
                *self = Self::Numeric(0);
                return;
            } else if let ExpressionValue::Numeric(1) = rhs {
                return;
            }
        }
        if let Operation::Add = operation {
            if let ExpressionValue::Numeric(0) = rhs {
                return;
            } else if let ExpressionValue::Numeric(0) = self {
                *self = rhs;
                return;
            }
        }
        if let ExpressionValue::Numeric(n1) = self {
            if let ExpressionValue::Numeric(n2) = rhs {
                *self = ExpressionValue::Numeric(
                    match operation {
                        Operation::Add => *n1 + n2,
                        Operation::Multiply => *n1 * n2,
                        Operation::Divide => *n1 / n2,
                        Operation::Mod => *n1 % n2,
                    }
                );
                return;
            }
        }
        let temp = std::mem::take(self);
        *self = Self::Recursive(Box::new(Expression {
            value: temp,
            modifier: Some(Box::new((operation, Expression {
                value: rhs,
                modifier: None,
            }))),
        }))
    }
}

impl Operationable for ExpressionSet {
    fn operate(&mut self, operation: Operation, rhs: Self) {
        if let Operation::Multiply = operation {
            if let Some(ExpressionValue::Numeric(0)) = self.try_get_single() {
                self.values = vec![ExpressionStatement {
                    conditions: Default::default(),
                    value: Default::default(),
                }];
                return;
            }
            if let Some(ExpressionValue::Numeric(0)) = rhs.try_get_single() {
                self.values = vec![ExpressionStatement {
                    conditions: Default::default(),
                    value: Default::default(),
                }];
                return;
            }
        }

        if rhs.values.len() == 1 {
            let rhs = rhs.values.into_iter().next().unwrap().get_as_simple_value_move().unwrap();
            if self.values.len() == 1 {
                self.values[0].value.operate(operation, rhs);
            } else {
                for current in self.values.iter_mut() {
                    current.value.operate(operation.clone(), rhs.clone());
                }
            }
        } else {
            let self_values = take(&mut self.values);
            self.values =
                Self::get_statement_combinations(self_values, rhs.values).into_iter().map(|(current, rhs)| {
                    let mut current_value = current.value.clone();
                    current_value.operate(operation.clone(), rhs.value);
                    let mut conditions = rhs.conditions;
                    conditions.extend(current.conditions.clone());
                    ExpressionStatement {
                        conditions,
                        value: current_value,
                    }
                }).collect();
        }
    }

    fn from_numeric(number: i64) -> Self {
        ExpressionValue::Numeric(number).into()
    }

    fn operate_equals(&mut self, rhs: Self) {
        let operate = |current: ExpressionStatement, rhs: ExpressionValue, rhs_conditions: Option<HashSet<Equality>>| {
            let mut conditions_equal = current.conditions;
            if let Some(rhs_conditions) = rhs_conditions {
                conditions_equal.extend(rhs_conditions);
            }

            let condition_equal = Equality {
                left: Expression { value: current.value, modifier: None },
                right: Expression { value: rhs, modifier: None },
                inverse: false,
            };

            if condition_equal.left.modifier.is_none() && condition_equal.right.modifier.is_none() {
                if let ExpressionValue::Numeric(left) = condition_equal.left.value {
                    if let ExpressionValue::Numeric(right) = condition_equal.right.value {
                        let value = if left == right {
                            1
                        } else {
                            0
                        };
                        return vec![ExpressionStatement {
                            conditions: conditions_equal,
                            value: ExpressionValue::Numeric(value),
                        }];
                    }
                }
                let values = [(&condition_equal.left.value, &condition_equal.right.value), (&condition_equal.right.value, &condition_equal.left.value)];
                for (left, right) in values.into_iter() {
                    if let ExpressionValue::Numeric(left) = left {
                        if let ExpressionValue::Input(_) = right {
                            if *left < 1 || *left > 9 {
                                return vec![ExpressionStatement {
                                    conditions: conditions_equal,
                                    value: ExpressionValue::Numeric(0),
                                }];
                            }
                        }
                    }
                }
            }

            let mut condition_not_equal = condition_equal.clone();
            condition_not_equal.inverse = true;

            let mut conditions_not_equal = conditions_equal.clone();
            conditions_equal.insert(condition_equal);
            conditions_not_equal.insert(condition_not_equal);
            vec![
                ExpressionStatement {
                    conditions: conditions_equal,
                    value: ExpressionValue::Numeric(1),
                },
                ExpressionStatement {
                    conditions: conditions_not_equal,
                    value: ExpressionValue::Numeric(0),
                },
            ]
        };
        let self_values = take(&mut self.values);
        self.values = if rhs.values.len() == 1 {
            let rhs = rhs.values.into_iter().next().unwrap().get_as_simple_value_move().unwrap();
            if self_values.len() == 1 {
                operate(self_values.into_iter().next().unwrap(), rhs, None)
            } else {
                self_values.into_iter().flat_map(|current| {
                    operate(current, rhs.clone(), None)
                }).collect()
            }
        } else {
            Self::get_statement_combinations(self_values, rhs.values).into_iter().flat_map(|(current, rhs)| {
                let ExpressionStatement {
                    conditions,
                    value
                } = rhs;
                operate(current.clone(), value, Some(conditions))
            }).collect()
        }
    }

    fn cleanup(&mut self) {
        if self.values.len() == 2 {
            let mut values = self.values.iter();
            let first = values.next().unwrap();
            let second = values.next().unwrap();
            if first.value == second.value {
                // self.values.retain(|val| val != second);
                self.values.truncate(1);
            }
        }
    }

    fn try_zero_out(&self, possible_input_values: &Vec<HashMap<usize, i64>>) -> (Option<Box<dyn Fn(Self) -> Self>>, Vec<HashMap<usize, i64>>) {
        if self.values.iter().any(|value| {
            value.conditions.iter().any(|condition| {
                !condition.is_feasible()
            })
        }) {
            (Some(Box::new(|mut value| {
                value.values.retain(|v| v.conditions.iter().all(|c| c.is_feasible()));
                if value.values.len() == 1 {
                    value.values.iter_mut().next().unwrap().conditions.clear();
                }
                value
            })), possible_input_values.clone())
        } else {
            if self.values.len() == 1 {
                (None, Default::default())
            } else {
                let mut found_possible = Vec::new();
                for value in &self.values {
                    for condition in value.conditions.iter().filter(|c| !c.inverse) {
                        let to_zero = &value.value;
                        let possible_input_values = condition.max_for_condition(to_zero, possible_input_values);
                        if !possible_input_values.is_empty() {
                            found_possible.push((possible_input_values, to_zero));
                        }
                    }
                }
                if found_possible.len() == 1 {
                    let (possible_input_values, to_zero) = found_possible.into_iter().next().unwrap();
                    let fun: Option<Box<(dyn Fn(ExpressionSet) -> ExpressionSet + 'static)>> = if possible_input_values.len() == 1 {
                        let value_z = ExpressionValue::Numeric(to_zero.evaluate(possible_input_values.iter().next().unwrap()));
                        Some(Box::new(move |_| ExpressionSet {
                            values: vec![ExpressionStatement {
                                conditions: Default::default(),
                                value: value_z.clone(),
                            }]
                        }))
                    } else {
                        None
                    };
                    (fun, possible_input_values)
                } else if found_possible.is_empty() {
                    (None, vec![])
                } else {
                    if let Some((possible_input_values, to_zero)) = found_possible.iter().filter(|(f, _)| f.len() == 1).next() {
                        let value_z = ExpressionValue::Numeric(to_zero.evaluate(possible_input_values.iter().next().unwrap()));
                        let fun: Option<Box<(dyn Fn(ExpressionSet) -> ExpressionSet + 'static)>> =
                            Some(Box::new(move |_| ExpressionSet {
                                values: vec![ExpressionStatement {
                                    conditions: Default::default(),
                                    value: value_z.clone(),
                                }]
                            }));
                        (fun, possible_input_values.clone())
                    } else {
                        let possible_input_values: Vec<HashMap<usize, i64>> = found_possible.into_iter().flat_map(|(v, _)| v.into_iter()).collect();
                        (None, possible_input_values)
                    }
                }
            }
        }
    }
}

impl ExpressionSet {
    pub(crate) fn try_get_single(&self) -> Option<&ExpressionValue> {
        if self.values.len() == 1 {
            self.values.iter().next().map(|v| &v.value)
        } else {
            None
        }
    }
    fn get_statement_combinations(left: Vec<ExpressionStatement>, right: Vec<ExpressionStatement>) -> Vec<(ExpressionStatement, ExpressionStatement)> {
        let left: HashMap<u64, ExpressionStatement> = left.into_iter().map(|s| (s.calculate_hash(), s)).collect();
        let right: HashMap<u64, ExpressionStatement> = right.into_iter().map(|s| (s.calculate_hash(), s)).collect();
        let left_keys: HashSet<u64> = left.keys().copied().collect();
        let mut right_keys: HashSet<u64> = right.keys().copied().collect();
        let mut get_right_keys = |for_key: u64| {
            let val = right_keys.clone();
            right_keys.remove(&for_key);
            val
        };
        left_keys.into_iter().flat_map(|left_key| {
            let right_keys = get_right_keys(left_key);
            right_keys.into_iter().filter_map(|right_key| {
                let lhs = left.get(&left_key).unwrap();
                let rhs = right.get(&right_key).unwrap();
                if rhs.conditions.iter().any(|other| lhs.conditions.iter().any(|c| c.is_incompatible(other))) {
                    None
                } else {
                    Some((lhs.clone(), rhs.clone()))
                }
            })
                .collect::<Vec<(ExpressionStatement, ExpressionStatement)>>()
        })
            .collect()
    }
    pub(crate) fn get_dependencies(&self) -> HashSet<usize> {
        HashSet::from_iter(self.values.iter().flat_map(|v| v.get_dependencies()))
    }
}

#[derive(Clone/*, PartialEq, Eq*/)]
struct ExpressionStatement {
    conditions: HashSet<Equality>,
    value: ExpressionValue,
}

// impl Hash for ExpressionStatement {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.value.hash(state);
//     }
// }

impl ExpressionStatement {
    pub(crate) fn calculate_hash(&self) -> u64 {
        let mut vec = Vec::from_iter(self.conditions.iter());
        vec.sort();
        let mut hasher = DefaultHasher::new();
        vec.len().hash(&mut hasher);
        for item in vec {
            item.hash(&mut hasher);
        }
        hasher.finish()
    }
    pub(crate) fn get_as_simple_value_move(self) -> Option<ExpressionValue> {
        if self.conditions.is_empty() {
            Some(self.value)
        } else {
            None
        }
    }
    pub(crate) fn get_dependencies(&self) -> HashSet<usize> {
        let mut set = self.value.get_dependencies();
        set.extend(self.conditions.iter().flat_map(|v| v.get_dependencies()));

        set
    }
}

impl Debug for ExpressionStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.conditions.is_empty() {
            write!(f, "{:?}", self.value)
        } else {
            write!(f, "IF {}\nTHEN {:?}", self.conditions.iter().map(|c| format!("{:?}", *c)).collect::<Vec<String>>().join("\n    AND "), self.value)
        }
    }
}

#[derive(Clone)]
struct ExpressionSet {
    values: Vec<ExpressionStatement>,
}

impl Debug for ExpressionSet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.values.len() == 1 {
            write!(f, "{:?}", self.values.iter().next().unwrap())
        } else {
            write!(f, "{{\n{}\n}}", self.values.iter().map(|v| format!("{:?}", v)).collect::<Vec<String>>().join("\n,\n"))
        }
    }
}

impl Default for ExpressionSet {
    fn default() -> Self {
        Self {
            values: vec![ExpressionStatement {
                conditions: Default::default(),
                value: Default::default(),
            }]
        }
    }
}

fn get_register_alu<'a, T: Operationable>(registers: &'a mut HashMap<char, T>, r: &'a char) -> &'a mut T {
    let current = registers.entry(*r).or_insert(Default::default());
    current
}

fn process_alu<T: Operationable, TF: Fn(&mut HashMap<char, T>, TFV), TFV>(registers: &mut HashMap<char, T>, instruction: &Instruction, input: &mut impl Iterator<Item=T>, on_input: TF, on_input_value: TFV) {
    let get_rhs = |registers: &HashMap<char, T>, num_or_val: &NumberOrValue| match num_or_val {
        NumberOrValue::Number(n) => T::from_numeric(*n),
        NumberOrValue::Value(r) => registers.get(r).map_or(Default::default(), |v| v.clone())
    };
    match &instruction.operation {
        InstructionOperation::Input(r) => {
            registers.insert(*r, input.next().unwrap());
            on_input(registers, on_input_value);
        }
        InstructionOperation::Add(r, num_or_val) => {
            let rhs = get_rhs(registers, num_or_val);
            get_register_alu(registers, r).operate(Operation::Add, rhs);
        }
        InstructionOperation::Multiply(r, num_or_val) => {
            let rhs = get_rhs(registers, num_or_val);
            get_register_alu(registers, r).operate(Operation::Multiply, rhs);
        }
        InstructionOperation::Divide(r, num_or_val) => {
            let rhs = get_rhs(registers, num_or_val);
            get_register_alu(registers, r).operate(Operation::Divide, rhs);
        }
        InstructionOperation::Mod(r, num_or_val) => {
            let rhs = get_rhs(registers, num_or_val);
            get_register_alu(registers, r).operate(Operation::Mod, rhs);
        }
        InstructionOperation::Equals(r, num_or_val) => {
            let rhs = get_rhs(registers, num_or_val);
            get_register_alu(registers, r).operate_equals(rhs);
        }
    };
}

fn value_at_alu<T: Operationable>(registers: &HashMap<char, T>, r: &char) -> T {
    registers.get(r).or(Some(&Default::default())).unwrap().clone()
}

struct AluV2 {
    registers: HashMap<char, i64>,
    input: std::vec::IntoIter<i64>,
}

impl AluV2 {
    pub fn new(input: Vec<i64>) -> Self {
        let input = input.into_iter();
        Self {
            registers: Default::default(),
            input,
        }
    }
    pub fn process(&mut self, instruction: &Instruction) {
        process_alu(&mut self.registers, instruction, &mut self.input, |_, _| {}, ());
    }
    pub fn value_at(&self, register: &char) -> i64 {
        value_at_alu(&self.registers, register)
    }
}

struct ALU<T: Operationable> {
    registers: HashMap<char, T>,
    possible_input_values: Vec<HashMap<usize, i64>>,
}

impl<T: Operationable> Default for ALU<T> {
    fn default() -> Self {
        Self {
            registers: Default::default(),
            possible_input_values: Default::default(),
        }
    }
}

impl<T: Operationable> ALU<T> {
    pub(crate) fn try_zero_out(&mut self) {
        Self::try_zero_out_z(&mut self.registers, &mut self.possible_input_values);
    }
    fn try_zero_out_z(registers: &mut HashMap<char, T>, possible_input_values: &mut Vec<HashMap<usize, i64>>) {
        let z = registers.get(&'z').or(Some(&Default::default())).unwrap().clone();
        let result = z.try_zero_out(possible_input_values);
        *possible_input_values = result.1;
        if let Some(fun) = result.0 {
            let val = registers.get_mut(&'z').unwrap();
            let v = std::mem::take(val);
            *val = fun(v);
        }
        let z = registers.get(&'z').or(Some(&Default::default())).unwrap().clone();
        println!("value z:\n{:?}", z);
        println!();
        println!("possible inputs:\n{:?}", possible_input_values);
        println!();
        println!();
    }
    pub fn process(&mut self, instruction: &Instruction, input: &mut impl Iterator<Item=T>) {
        process_alu(&mut self.registers, instruction, input, |registers, possible_input_values| {
            Self::try_zero_out_z(registers, possible_input_values);
        }, &mut self.possible_input_values);
    }
    pub(crate) fn optimize(&mut self) {
        for register in self.registers.values_mut() {
            register.cleanup();
        }
    }
    pub fn value_at(&self, register: &char) -> T {
        value_at_alu(&self.registers, register)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Expression {
    value: ExpressionValue,
    modifier: Option<Box<(Operation, Expression)>>,
}

impl Debug for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let modifier = match &self.modifier {
            None => "".to_string(),
            Some(val) => format!(" {} {:?}", match (**val).0 {
                Operation::Add => '+',
                Operation::Multiply => '*',
                Operation::Divide => '/',
                Operation::Mod => '%'
            }, (**val).1)
        };
        write!(f, "{:?}{}", self.value, modifier)
    }
}

impl Expression {
    pub(crate) fn evaluate(&self, input_map: &HashMap<usize, i64>) -> i64 {
        let lhs = self.value.evaluate(input_map);
        match &self.modifier {
            None => lhs,
            Some(b) => {
                let rhs = (**b).1.evaluate(input_map);
                match (**b).0 {
                    Operation::Add => lhs + rhs,
                    Operation::Multiply => lhs * rhs,
                    Operation::Divide => lhs / rhs,
                    Operation::Mod => lhs % rhs
                }
            }
        }
    }
    pub(crate) fn any<TF: Fn(&ExpressionValue) -> bool>(&self, fun: &TF) -> bool {
        if self.value.any(fun) {
            return true;
        }
        match &self.modifier {
            None => false,
            Some(r) => (**r).1.any(fun)
        }
    }
    pub(crate) fn get_dependencies(&self) -> HashSet<usize> {
        let mut set: HashSet<usize> = Default::default();
        match &self.value {
            ExpressionValue::Recursive(r) => {
                set.extend(r.get_dependencies());
            }
            ExpressionValue::Input(i) => {
                set.insert(*i);
            }
            _ => {}
        }
        if let Some(expr) = &self.modifier {
            set.extend((**expr).1.get_dependencies());
        }

        set
    }
}

#[derive(Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
struct Equality {
    left: Expression,
    right: Expression,
    inverse: bool,
}

impl Debug for Equality {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} {:?}", self.left, if !self.inverse { "=" } else { "≠" }, self.right)
    }
}

fn add(input: &mut Vec<i64>) -> bool {
    for i in (0..input.len()).rev() {
        if input[i] == 9 {
            input[i] = 1;
        } else {
            input[i] += 1;
            return true;
        }
    }
    false
}

fn subtract(input: &mut Vec<i64>) -> bool {
    for i in (0..input.len()).rev() {
        if input[i] == 1 {
            input[i] = 9;
        } else {
            input[i] -= 1;
            return true;
        }
    }
    false
}

impl Equality {
    #[allow(unused)]
    pub(crate) fn max_for_condition(&self, value_to_zero: &ExpressionValue, possible_input_values: &Vec<HashMap<usize, i64>>) -> Vec<HashMap<usize, i64>> {
        println!("trying to get:\n{:?} = {:?}", self.left, self.right);
        println!("and 0 = {:?}", value_to_zero);
        let mut all_possible = Vec::new();
        let default_input_values: Vec<HashMap<usize, i64>> = vec![Default::default()];
        let possible_input_values = if possible_input_values.is_empty() {
            &default_input_values
        } else {
            possible_input_values
        };
        for possible_input in possible_input_values.iter() {
            let mut input_values_needed = value_to_zero.get_dependencies();
            input_values_needed.extend(self.right.get_dependencies());
            for (i, _) in possible_input.iter() {
                input_values_needed.remove(i);
            }
            let mut input_values_needed: Vec<usize> = input_values_needed.into_iter().collect();
            input_values_needed.sort();
            let map: HashMap<usize, usize> = input_values_needed.iter().copied().enumerate().map(|(i, val)| (val, i)).collect();
            let mut input: Vec<i64> = vec![9; input_values_needed.len()];
            let reset = |input: &mut Vec<i64>| {
                for i in (0..input.len()).rev() {
                    input[i] = 9;
                }
            };
            let get_input_map = |input: &Vec<i64>| {
                let mut input_map: HashMap<usize, i64> = map.iter().map(|(&val, &i)| {
                    (val, input[i])
                }).collect();
                for (&i, &val) in possible_input.iter() {
                    input_map.insert(i, val);
                }
                input_map
            };
            loop {
                let input_map = get_input_map(&input);

                let mut inn: Vec<_> = input_map.iter().map(|(&i, &val)| (i, val)).collect();
                inn.sort_by(|(a, _), (b, _)| a.cmp(b));
                let _inn: Vec<_> = inn.into_iter().map(|(_, val)| val).collect();
                let rhs = self.right.evaluate(&input_map);
                let lhs = self.left.evaluate(&input_map);
                let zero = value_to_zero.evaluate(&input_map);
                // println!("{:?}, {} == {}, {} == 0", _inn, lhs, rhs, zero);
                if lhs == rhs {
                    if zero == 0 {
                        return vec![input_map];
                    }
                }

                if !subtract(&mut input) {
                    // break;
                    return vec![input_map];
                }
            }

            // reset(&mut input);
            //
            // loop {
            //     let input_map = get_input_map(&input);
            //
            //     let mut inn: Vec<_> = input_map.iter().map(|(&i, &val)| (i, val)).collect();
            //     inn.sort_by(|(a, _), (b, _)| a.cmp(b));
            //     let _inn: Vec<_> = inn.into_iter().map(|(_, val)| val).collect();
            //     let rhs = self.right.evaluate(&input_map);
            //     let lhs = self.left.evaluate(&input_map);
            //     // println!("{:?}, {} == {}, {} == 0", _inn, lhs, rhs, zero);
            //     if lhs == rhs {
            //         all_possible.push(input_map);
            //         // return input_map;
            //     }
            //
            //     if !subtract(&mut input) {
            //         break;
            //     }
            // }
        }

        // if all_possible.is_empty() {
        //     panic!("out of numbers");
        // }

        all_possible
    }
    pub(crate) fn is_feasible(&self) -> bool {
        let is_negative = |val: &ExpressionValue| {
            if let ExpressionValue::Numeric(val) = val {
                *val < 0
            } else {
                false
            }
        };
        if !self.inverse && self.right.modifier.is_none() {
            if let ExpressionValue::Input(_) = self.right.value {
                if self.left.any(&is_negative) {
                    return true;
                }
                return false;
            }
        }

        true
    }
    pub(crate) fn is_incompatible(&self, other: &Self) -> bool {
        let left_equal = self.left == other.left;
        let right_equal = self.right == other.right;
        if left_equal || right_equal {
            if left_equal && right_equal {
                self.inverse != other.inverse
            } else {
                self.inverse == other.inverse
            }
        } else {
            false
        }
    }
    pub(crate) fn _new(left: ExpressionValue, right: ExpressionValue, inverse: bool) -> Self {
        Self {
            left: Expression { value: left, modifier: None },
            right: Expression { value: right, modifier: None },
            inverse,
        }
    }
    pub(crate) fn get_dependencies(&self) -> HashSet<usize> {
        let mut set = self.left.get_dependencies();
        set.extend(self.right.get_dependencies());
        set
    }
}

#[derive(Clone)]
struct Statement {
    conditions: Vec<Equality>,
    equality: Equality,
}

impl Debug for Statement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.conditions.is_empty() {
            write!(f, "{:?}", self.equality)
        } else {
            write!(f, "IF {}\nTHEN {:?}", self.conditions.iter().map(|c| format!("{:?}", *c)).collect::<Vec<String>>().join("\n    AND "), self.equality)
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Operation {
    Add,
    Multiply,
    Divide,
    Mod,
}

#[derive(Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum ExpressionValue {
    Numeric(i64),
    Input(usize),
    Recursive(Box<Expression>),
}

impl ExpressionValue {
    pub(crate) fn evaluate(&self, input_map: &HashMap<usize, i64>) -> i64 {
        match self {
            ExpressionValue::Numeric(n) => *n,
            ExpressionValue::Input(i) => *input_map.get(i).unwrap(),
            ExpressionValue::Recursive(r) => r.evaluate(input_map)
        }
    }
    pub(crate) fn any<TF: Fn(&Self) -> bool>(&self, fun: &TF) -> bool {
        if fun(self) {
            return true;
        }
        if let ExpressionValue::Recursive(r) = self {
            if r.any(fun) {
                return true;
            }
        }
        false
    }
    pub(crate) fn get_dependencies(&self) -> HashSet<usize> {
        match self {
            ExpressionValue::Recursive(r) => r.get_dependencies(),
            _ => Default::default()
        }
    }
}

impl Default for ExpressionValue {
    fn default() -> Self {
        Self::Numeric(0)
    }
}

impl Debug for ExpressionValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            ExpressionValue::Numeric(val) => write!(f, "{}", *val),
            ExpressionValue::Input(i) => write!(f, "input_{}", *i),
            ExpressionValue::Recursive(e) => write!(f, "({:?})", *e)
        }
    }
}

#[derive(Clone)]
enum InstructionOperation {
    Input(char),
    Add(char, NumberOrValue),
    Multiply(char, NumberOrValue),
    Divide(char, NumberOrValue),
    Mod(char, NumberOrValue),
    Equals(char, NumberOrValue),
}

impl Into<Operation> for &InstructionOperation {
    fn into(self) -> Operation {
        match self {
            InstructionOperation::Add(_, _) => Operation::Add,
            InstructionOperation::Multiply(_, _) => Operation::Multiply,
            InstructionOperation::Divide(_, _) => Operation::Divide,
            InstructionOperation::Mod(_, _) => Operation::Mod,
            _ => panic!("out of bounds"),
        }
    }
}

impl InstructionOperation {
    pub(crate) fn get_numeric_value(&self) -> i64 {
        let get_value = |v: &NumberOrValue| {
            if let NumberOrValue::Number(n) = v {
                *n
            } else {
                panic!("not a number")
            }
        };
        match self {
            InstructionOperation::Input(_) => panic!("invalid operation"),
            InstructionOperation::Add(_, v) => get_value(v),
            InstructionOperation::Multiply(_, v) => get_value(v),
            InstructionOperation::Divide(_, v) => get_value(v),
            InstructionOperation::Mod(_, v) => get_value(v),
            InstructionOperation::Equals(_, v) => get_value(v),
        }
    }
}

#[derive(Clone)]
struct Instruction<'a> {
    operation: InstructionOperation,
    _source: &'a str,
}

impl<'a> Instruction<'a> {
    const VALID_CHARS: [&'static str; 4] = ["w", "x", "y", "z"];
}

impl<'a> TryFrom<&'a str> for Instruction<'a> {
    type Error = ();

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut parts = s.split_whitespace();
        let get_single_char = |s: &str| if s.len() != 1 { Err(()) } else { Ok(s.chars().next().unwrap()) };
        let get_values = |parts: &mut SplitWhitespace| -> Result<(char, NumberOrValue), ()> {
            let c = get_single_char(parts.next().ok_or(())?)?;
            let s = parts.next().ok_or(())?;
            let n = if Instruction::VALID_CHARS.contains(&s) {
                NumberOrValue::Value(get_single_char(s)?)
            } else {
                NumberOrValue::Number(s.parse().or(Err(()))?)
            };

            Ok((c, n))
        };
        let instruct = parts.next().ok_or(())?;
        let operation = if instruct == "inp" {
            InstructionOperation::Input(get_single_char(parts.next().ok_or(())?)?)
        } else {
            let values = get_values(&mut parts)?;
            match instruct {
                "add" => InstructionOperation::Add(values.0, values.1),
                "mul" => InstructionOperation::Multiply(values.0, values.1),
                "div" => InstructionOperation::Divide(values.0, values.1),
                "mod" => InstructionOperation::Mod(values.0, values.1),
                "eql" => InstructionOperation::Equals(values.0, values.1),
                _ => return Err(())
            }
        };
        Ok(Self {
            operation,
            _source: s,
        })
    }
}

#[derive(Clone)]
enum NumberOrValue {
    Number(i64),
    Value(char),
}

fn _get_input_subsection() -> &'static str {
    "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y"
}

fn _get_input() -> &'static str {
    "inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 13
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 2
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -3
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -16
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 15
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -6
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y"
}