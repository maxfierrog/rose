// Max fierro
// maxfierro@berkeley.edu
// 2023 Mar 9

pub mod build_tree;
pub mod build_file;


use build_tree::{
    docs_from_canon_check, 
    docs_from_project_check
};
use build_file::{
    docfile_from_code_check, 
    docfile_from_both_check, 
    docfile_from_template_check
};

