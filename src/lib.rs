#![feature(impl_trait_in_assoc_type)]
#![feature(adt_const_params)]
// #![feature(generic_const_exprs)]

mod impls;
// mod io;
mod operations;
mod traits;
mod types;
mod wrappers;

// pub use io::{random_vector_to_json, sequential_vector_to_json};
pub use traits::{
    ColumnMatrixTrait, MatrixTrait, PermutatorTrait, RandomVectorTrait, RowMatrixTrait, SequentialMatrixTrait,
    SequentialVectorTrait, VectorTrait,
};
pub use types::Direction;
pub use wrappers::{
    ColumnMatrix, CompressedMatrix, CompressedVector, DenseVector, FullPermutator, Permutator, RandomVector, RowMatrix,
    SequentialMatrix, SequentialVector, SparseVector,
};
