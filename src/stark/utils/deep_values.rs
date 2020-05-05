use serde::{ Serialize, Deserialize };
use crate::math::{ field };
use crate::stark::{ TraceTable, ConstraintPoly };

// TYPES AND INTERFACES
// ================================================================================================
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeepValues {
    pub trace_at_z      : Vec<u64>,
    pub trace_at_next_z : Vec<u64>,
    pub constraints_at_z: u64,
}

// DEEP VALUES IMPLEMENTATION
// ================================================================================================
impl DeepValues {

    pub fn new(z: u64, trace: &TraceTable, c_poly: &ConstraintPoly) -> DeepValues {

        // compute mask offset for z
        let g = field::get_root_of_unity(trace.unextended_length() as u64);
        let next_z = field::mul(z, g);

        // compute trace states at z and offset z
        let trace_at_z = trace.eval_polys_at(z);
        let trace_at_next_z = trace.eval_polys_at(next_z);

        // compute combined constraint evaluations at z
        let constraints_at_z = c_poly.eval_at(z);

        return DeepValues { trace_at_z, trace_at_next_z, constraints_at_z };
    }

}