use crate::{
    versions::testonly::simple_execution::{
        test_create2_deployment_address, test_estimate_fee, test_reusing_create2_salt,
        test_reusing_create_address, test_simple_execute,
    },
    vm_fast::Vm,
};

#[test]
fn estimate_fee() {
    test_estimate_fee::<Vm<_>>();
}

#[test]
fn simple_execute() {
    test_simple_execute::<Vm<_>>();
}

#[test]
fn create2_deployment_address() {
    test_create2_deployment_address::<Vm<_>>();
}

#[test]
fn reusing_create_address() {
    test_reusing_create_address::<Vm<_>>();
}

#[test]
fn reusing_create2_salt() {
    test_reusing_create2_salt::<Vm<_>>();
}
