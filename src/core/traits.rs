pub trait MolWt {
    /// Calculates the anhydrous molecular weight of a biomolecule
    fn mol_wt(&self) -> f64;
}

pub trait Seq {
    /// Calculates the sequence length
    fn length(&self) -> usize;
}