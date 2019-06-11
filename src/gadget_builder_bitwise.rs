use crate::gadget_builder::GadgetBuilder;
use crate::linear_combination::LinearCombination;
use crate::wire::Wire;

impl GadgetBuilder {
    /// ~x
    pub fn bitwise_not(&mut self, x: Vec<Wire>) -> Vec<LinearCombination> {
        x.iter().map(|w| LinearCombination::one() - LinearCombination::from(w)).collect()
    }

    /// Rotate bits in the direction of greater significance.
    // TODO: Weird bit order issue...
    pub fn bitwise_rotate_left(&mut self, x: Vec<Wire>, n: usize) -> Vec<Wire> {
        let l = x.len();
        let n_min = n % l;
        (0..l).map(|i| {
            if i >= n_min {
                x[i - n_min]
            } else {
                x[i + l - n_min]
            }
        }).collect()
    }

    pub fn bitwise_and(&mut self, x: Vec<Wire>, y: Vec<Wire>) -> Vec<LinearCombination> {
        assert_eq!(x.len(), y.len());
        let l = x.len();
        (0..l).map(|i| {
            self.and(x[i].into(), y[i].into())
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::gadget_builder::GadgetBuilder;

    #[test]
    fn bitwise_not() {
        let mut builder = GadgetBuilder::new();
        let x = builder.wire();
        builder.bitwise_not(vec![x]);
        let gadget = builder.build();

        let mut values = values!(x => 5.into());
        gadget.execute(&mut values);
    }
}