use std::{cell::RefCell, rc::Rc};

use ark_bn254::{G1Affine, G2Affine};

use crate::ecc::MillerLines;

use super::{
    pippenger_reference_string::Pippenger, ProverReferenceString, ReferenceStringFactory,
    VerifierReferenceString,
};

#[derive(Debug)]
pub(crate) struct VerifierFileReferenceString {
    g2_x: G2Affine,
    precomputed_g2_lines: Rc<Vec<MillerLines>>,
}

impl VerifierFileReferenceString {
    pub(crate) fn new(_path: &str) -> Self {
        // Please replace the actual types and functions with ones that you have in your Rust codebase.
        // let g2_x: G2Affine = read_transcript_g2(path);
        // let precomputed_g2_lines: Vec<MillerLines> = vec![MillerLines::default(); 2];

        // precompute_miller_lines(g2_x, &mut precomputed_g2_lines[1]);

        // Self {
        //     g2_x,
        //     precomputed_g2_lines,
        // }
        unimplemented!()
    }
}

impl VerifierReferenceString for VerifierFileReferenceString {
    fn get_g2x(&self) -> G2Affine {
        self.g2_x
    }

    fn get_precomputed_g2_lines(&self) -> Rc<Vec<MillerLines>> {
        self.precomputed_g2_lines.clone()
    }
}

#[derive(Debug)]
pub(crate) struct FileReferenceString {
    num_points: usize,
    pippenger: Pippenger,
}

impl FileReferenceString {
    pub(crate) fn new(_num_points: usize, _path: &str) -> Self {
        // Implementation depends on your project.
        todo!("FileReferenceString::new")
    }

    pub(crate) fn read_from_path(_path: &str) -> Result<Self, std::io::Error> {
        // Implementation depends on your project.
        todo!("FileReferenceString::read_from_path")
    }
}

impl Default for FileReferenceString {
    fn default() -> Self {
        Self {
            num_points: 0,
            pippenger: Pippenger::default(),
        }
    }
}

impl ProverReferenceString for FileReferenceString {
    fn get_monomial_points(&mut self) -> Rc<Vec<G1Affine>> {
        // Implementation depends on your project.
        todo!()
    }

    fn get_monomial_size(&self) -> usize {
        self.num_points
    }
}

pub(crate) struct FileReferenceStringFactory {
    path: String,
}

impl FileReferenceStringFactory {
    pub(crate) fn new(path: String) -> Self {
        Self { path }
    }
}
impl ReferenceStringFactory for FileReferenceStringFactory {
    fn get_prover_crs(&self, degree: usize) -> Option<Rc<dyn ProverReferenceString>> {
        Some(Rc::new(FileReferenceString::new(degree, &self.path)))
    }

    fn get_verifier_crs(&self) -> Option<Rc<dyn VerifierReferenceString>> {
        Some(Rc::new(VerifierFileReferenceString::new(&self.path)))
    }
}

pub(crate) struct DynamicFileReferenceStringFactory {
    path: String,
    degree: RefCell<usize>,
    prover_crs: RefCell<Rc<FileReferenceString>>,
    verifier_crs: Rc<VerifierFileReferenceString>,
}

impl DynamicFileReferenceStringFactory {
    pub(crate) fn new(path: String, initial_degree: usize) -> Self {
        let verifier_crs = Rc::new(VerifierFileReferenceString::new(&path));
        let prover_crs = RefCell::new(Rc::new(FileReferenceString::new(initial_degree, &path)));
        Self {
            path,
            degree: RefCell::new(initial_degree),
            prover_crs,
            verifier_crs,
        }
    }
}

impl ReferenceStringFactory for DynamicFileReferenceStringFactory {
    fn get_prover_crs(&self, degree: usize) -> Option<Rc<dyn ProverReferenceString>> {
        if degree != *self.degree.borrow() {
            *self.prover_crs.borrow_mut() = Rc::new(FileReferenceString::new(degree, &self.path));
            *self.degree.borrow_mut() = degree;
        }
        Some((self.prover_crs.borrow_mut()).clone())
    }

    fn get_verifier_crs(&self) -> Option<Rc<dyn VerifierReferenceString>> {
        Some(self.verifier_crs.clone())
    }
}
