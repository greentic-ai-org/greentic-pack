wit_bindgen::generate!({
    path: "wit",
    world: "pack-validator",
});

use exports::greentic::pack_validate::validator::{Diagnostic, Guest, PackInputs};

struct NoopValidator;

impl Guest for NoopValidator {
    fn applies(_inputs: PackInputs) -> bool {
        true
    }

    fn validate(_inputs: PackInputs) -> Vec<Diagnostic> {
        vec![Diagnostic {
            severity: "warn".to_string(),
            code: "PACK_VALIDATOR_NOOP".to_string(),
            message: "noop validator ran".to_string(),
            path: None,
            hint: None,
        }]
    }
}

export!(NoopValidator);
