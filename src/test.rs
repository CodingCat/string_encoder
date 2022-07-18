use crate::encode_string_with_base_36;

#[test]
fn test_base36() {
    let input = "emr_on_eks_maps_job_execution_role_on_prod";
    assert_eq!(encode_string_with_base_36(input), "dtnxhomu4kl05pp2fmoz7lvcgxtscvxr7uhzqi5ld1dxwx73ujah8nljiwk9txysk");
}