#[test]
fn empty() {
    run_test(
        &[],
        &[]
    );
}

#[test]
fn one_frame() {
    run_test(
        &[&[1,2,3]],
        &[SOF,1,2,3,216]
    );
}

#[test]
fn two_frames() {
    run_test(
        &[&[1,2,3], &[4,5,6]],
        &[SOF,1,2,3,216,SOF,4,5,6,188]
    );
}

#[test]
fn sof_in_payload() {
    run_test(
        &[&[1,SOF,2]],
        &[SOF,1,ESC,ESC_SOF,2,ESC,ESC_ESC] // the crc happens to equal ESC
    );
}

#[test]
fn all_sof() {
    run_test(
        &[&[SOF,SOF,SOF]],
        &[SOF,ESC,ESC_SOF,ESC,ESC_SOF,ESC,ESC_SOF,150]
    );
}

#[test]
fn esc_in_payload() {
    run_test(
        &[&[1,ESC,2]],
        &[SOF,1,ESC,ESC_ESC,2,40]
    );
}

#[test]
fn all_esc() {
    run_test(
        &[&[ESC,ESC,ESC]],
        &[SOF,ESC,ESC_ESC,ESC,ESC_ESC,ESC,ESC_ESC,197]
    );
}

#[test]
fn zero_payload() {
    run_test(
        &[&[0,0,0]],
        &[SOF,0,0,0,0]
    );
}
