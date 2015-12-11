//! # Evaluator
//!
//! Machine to evaluate the result of a bunch of wires

use super::wire::Wire;
use super::instruction::Instruction;
use super::parse::Evaluable;

use std::collections::HashMap;

pub struct Evaluator {
    wires: Vec<Wire>,
}

type NamedWiresType = HashMap<String, (Wire, Option<usize>)>;

impl Evaluator {
    pub fn new(wires: Vec<Wire>) -> Evaluator {
        Evaluator { wires: wires }
    }

    /// Sort the owned wires by which ones depend on which others to evaluate
    pub fn sort(&mut self) {
        // &str -> (&Wire, Option<usize>)
        let mut named_wires: NamedWiresType = HashMap::new();
        for wire in &self.wires {
            let name = &wire.get_name();
            named_wires.insert(name.to_string(), (wire.clone(), None));
        }

        for wire in &self.wires {
            if let Some(&(_, None)) = named_wires.get(wire.get_name()) {
                self.sort_by_determinability(&mut named_wires, wire);
            }
        }
        unimplemented!();
    }

    /// Recursively determine how many instructions must be computed before this one can be resolved,
    /// and insert that value into the named_wires table.
    ///
    /// - Direct assignments (raw numbers) will insert `0`: nothing must come before them
    /// - Indirect assignments (var assignments) will insert `1+<that var's number>`, because they
    ///   can be resolved as soon as the previous wires have been resolved
    /// - Unary instructions compute exactly like normal assignments
    /// - Binary instructions will insert `1+<max(x's #, y's #)>` because they may be resolved as
    ///   soon as both of their predicates have been resolved
    fn sort_by_determinability(&self, named_wires: &mut NamedWiresType, wire: &Wire) {

        let my_sort_order = match wire.get_instruction() {
            &Instruction::Store(ref ev) => self.unary_trace(named_wires, ev),
            &Instruction::Not(ref ev) => self.unary_trace(named_wires, ev),
            &Instruction::And{ref x, ref y} => self.binary_trace(named_wires, x, y),
            &Instruction::Or{ ref x, ref y} => self.binary_trace(named_wires, x, y),
            &Instruction::Lshift{ref x, ref y} => self.binary_trace(named_wires, x, y),
            &Instruction::Rshift{ref x, ref y} => self.binary_trace(named_wires, x, y),
        };

        named_wires.insert(wire.get_name().to_string(),
                           (wire.clone(), Some(my_sort_order)));
    }

    fn unary_trace(&self, named_wires: &mut NamedWiresType, ev: &Evaluable) -> usize {
        unimplemented!();
    }

    fn binary_trace(&self,
                    named_wires: &mut NamedWiresType,
                    x: &Evaluable,
                    y: &Evaluable)
                    -> usize {
        unimplemented!();
    }
}
