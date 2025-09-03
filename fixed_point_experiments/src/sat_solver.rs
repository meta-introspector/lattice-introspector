// MiniZinc-like DSL for SAT problem
#[derive(Clone)]
pub struct Variable {
    pub id: usize,
}

pub struct Clause {
    pub literals: Vec<(Variable, bool)>, // (var, is_true)
}

#[derive(Clone)] // Added Clone derive
pub struct ConstraintModel {
    pub variables: Vec<Variable>,
    pub clauses: Vec<Clause>,
}

impl ConstraintModel {
    pub fn new(n: usize) -> Self {
        let variables = (0..n).map(|i| Variable { id: i }).collect();
        ConstraintModel {
            variables,
            clauses: Vec::new(),
        }
    }

    // MiniZinc-like: var bool: x
    pub fn add_clause(&mut self, literals: Vec<(usize, bool)>) {
        let clause = Clause {
            literals: literals
                .into_iter()
                .map(|(id, val)| (Variable { id }, val))
                .collect(),
        };
        self.clauses.push(clause);
    }

    // MiniZinc-like: constraint at_most(k, vars)
    pub fn add_cardinality_at_most(&mut self, k: usize) {
        let n = self.variables.len();
        let combinations = generate_combinations(n, k + 1);
        for subset in combinations {
            let negative_clause = subset.into_iter().map(|i| (i, false)).collect();
            self.add_clause(negative_clause);
        }
    }

    // Simple solver: Brute-force for small n
    pub fn solve(&self) -> Option<Vec<bool>> {
        let n = self.variables.len();
        for val in 0..1 << n {
            let valuation: Vec<bool> = (0..n).map(|i| (val >> i) & 1 == 1).collect();
            if self.is_satisfied(&valuation) {
                return Some(valuation);
            }
        }
        None
    }

    fn is_satisfied(&self, valuation: &[bool]) -> bool {
        self.clauses.iter().all(|clause| {
            clause.literals.iter().any(|(var, is_true)| {
                valuation[var.id] == *is_true
            })
        })
    }
}

// Generate combinations (n choose k)
pub fn generate_combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if n < k {
        return vec![];
    }
    let mut result = generate_combinations(n - 1, k);
    let mut with_n = generate_combinations(n - 1, k - 1);
    for subset in with_n.iter_mut() {
        subset.push(n - 1);
    }
    result.extend(with_n);
    result
}

// Vertex cover for C5
pub fn vertex_cover_c5() -> ConstraintModel {
    let mut model = ConstraintModel::new(5);
    // Hard clauses: each edge (i, i+1) or (4, 0) must have at least one true vertex
    model.add_clause(vec![(0, true), (1, true)]); // v0 or v1
    model.add_clause(vec![(1, true), (2, true)]); // v1 or v2
    model.add_clause(vec![(2, true), (3, true)]); // v2 or v3
    model.add_clause(vec![(3, true), (4, true)]); // v3 or v4
    model.add_clause(vec![(4, true), (0, true)]); // v4 or v0
    model
}

// Find minimal vertex cover size
pub fn find_min_cost() -> usize {
    let n = 5;
    let base_model = vertex_cover_c5();
    for k in 0..=n {
        let mut model = base_model.clone();
        model.add_cardinality_at_most(k);
        if model.solve().is_some() {
            return k;
        }
    }
    n
}
