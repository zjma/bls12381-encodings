use std::ops::Neg;
use ark_ec::AffineCurve;
use ark_serialize::CanonicalSerialize;
use num_traits::identities::Zero;

fn main() {
}

#[test]
fn blst_fp_element_bendian() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_bendian_from_fp(buf.as_mut_ptr(), &point.x);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_fp_element_lendian() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_lendian_from_fp(buf.as_mut_ptr(), &point.x);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_fr_value_7_lendian() {
    let mut buf = [0_u8; 32];
    let val = 7_u64;
    unsafe {
        let mut fr_element = blst::blst_fr::default();
        blst::blst_fr_from_uint64(&mut fr_element, &val);
        let mut scalar = blst::blst_scalar::default();
        blst::blst_scalar_from_fr(&mut scalar, &fr_element);
        blst::blst_lendian_from_scalar(buf.as_mut_ptr(), &scalar);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_fr_value_7_bendian() {
    let mut buf = [0_u8; 32];
    let val = 7_u64;
    unsafe {
        let mut fr_element: blst::blst_fr = blst::blst_fr::default();
        blst::blst_fr_from_uint64(&mut fr_element, &val);
        let mut scalar: blst::blst_scalar = blst::blst_scalar::default();
        blst::blst_scalar_from_fr(&mut scalar, &fr_element);
        blst::blst_bendian_from_scalar(buf.as_mut_ptr(), &scalar);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_compress(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_serialized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_serialize(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_neg_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point, true);
        blst::blst_p1_compress(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_neg_serialized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point, true);
        blst::blst_p1_serialize(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_inf_serailized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let point = blst::blst_p1_generator().read();
        let mut point_neg = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p1::default();
        blst::blst_p1_add(&mut inf, &point, &point_neg);

        blst::blst_p1_serialize(buf.as_mut_ptr(), &inf);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_inf_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let point = blst::blst_p1_generator().read();
        let mut point_neg = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p1::default();
        blst::blst_p1_add(&mut inf, &point, &point_neg);

        blst::blst_p1_compress(buf.as_mut_ptr(), &inf);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_affine_serialized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        let mut point_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut point_affine, &point);
        blst::blst_p1_affine_serialize(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_affine_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        let mut point_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut point_affine, &point);
        blst::blst_p1_affine_compress(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_neg_affine_serialized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point, true);
        let mut point_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut point_affine, &point);
        blst::blst_p1_affine_serialize(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_generator_neg_affine_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point, true);
        let mut point_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut point_affine, &point);
        blst::blst_p1_affine_compress(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_inf_affine_serailized() {
    let mut buf = [0_u8; 144];
    unsafe {
        let point = blst::blst_p1_generator().read();
        let mut point_neg = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p1::default();
        blst::blst_p1_add(&mut inf, &point, &point_neg);

        let mut inf_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut inf_affine, &inf);
        blst::blst_p1_affine_serialize(buf.as_mut_ptr(), &inf_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g1_inf_affine_compressed() {
    let mut buf = [0_u8; 48];
    unsafe {
        let point = blst::blst_p1_generator().read();
        let mut point_neg = blst::blst_p1_generator().read();
        blst::blst_p1_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p1::default();
        blst::blst_p1_add(&mut inf, &point, &point_neg);

        let mut inf_affine = blst::blst_p1_affine::default();
        blst::blst_p1_to_affine(&mut inf_affine, &inf);
        blst::blst_p1_affine_compress(buf.as_mut_ptr(), &inf_affine);
    }
    println!("{}", hex::encode(buf));
}


#[test]
fn blst_g2_affine_inf_compressed() {
    let mut buf = [0_u8; 96];
    unsafe {
        let point = blst::blst_p2_generator().read();
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p2::default();
        blst::blst_p2_add(&mut inf, &point, &point_neg);

        let mut inf_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut inf_affine, &inf);
        blst::blst_p2_affine_compress(buf.as_mut_ptr(), &inf_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_inf_serialized() {
    let mut buf = [0_u8; 288];
    unsafe {
        let point = blst::blst_p2_generator().read();
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, false);

        let mut inf = blst::blst_p2::default();
        blst::blst_p2_add(&mut inf, &point, &point_neg);

        let mut inf_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut inf_affine, &inf);
        blst::blst_p2_affine_serialize(buf.as_mut_ptr(), &inf_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_generator_compressed() {
    let mut buf = [0_u8; 96];
    unsafe {
        let point = blst::blst_p2_generator().read();
        blst::blst_p2_compress(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_generator_serialized() {
    let mut buf = [0_u8; 192];
    unsafe {
        let point = blst::blst_p2_generator().read();
        blst::blst_p2_serialize(buf.as_mut_ptr(), &point);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_generator_neg_serialized() {
    let mut buf = [0_u8; 192];
    unsafe {
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, true);
        blst::blst_p2_serialize(buf.as_mut_ptr(), &point_neg);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_generator_neg_compressed() {
    let mut buf = [0_u8; 96];
    unsafe {
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, true);
        blst::blst_p2_compress(buf.as_mut_ptr(), &point_neg);
    }
    println!("{}", hex::encode(buf));
}





#[test]
fn blst_g2_affine_generator_compressed() {
    let mut buf = [0_u8; 96];
    unsafe {
        let point = blst::blst_p2_generator().read();
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point);
        blst::blst_p2_affine_compress(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_generator_serialized() {
    let mut buf = [0_u8; 288];
    unsafe {
        let point = blst::blst_p2_generator().read();
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point);
        blst::blst_p2_affine_serialize(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_generator_neg_serialized() {
    let mut buf = [0_u8; 288];
    unsafe {
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, true);
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point_neg);
        blst::blst_p2_affine_serialize(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_generator_neg_compressed() {
    let mut buf = [0_u8; 96];
    unsafe {
        let mut point_neg = blst::blst_p2_generator().read();
        blst::blst_p2_cneg(&mut point_neg, true);
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point_neg);
        blst::blst_p2_affine_compress(buf.as_mut_ptr(), &point_affine);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_generator_x_0_bendian() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p2_generator().read();
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point);
        let x_0 = point_affine.x.fp[0];
        blst::blst_bendian_from_fp(buf.as_mut_ptr(), &x_0);
    }
    println!("{}", hex::encode(buf));
}

#[test]
fn blst_g2_affine_generator_x_1_bendian() {
    let mut buf = [0_u8; 48];
    unsafe {
        let mut point = blst::blst_p2_generator().read();
        let mut point_affine = blst::blst_p2_affine::default();
        blst::blst_p2_to_affine(&mut point_affine, &point);
        let x_1 = point_affine.x.fp[1];
        blst::blst_bendian_from_fp(buf.as_mut_ptr(), &x_1);
    }
    println!("{}", hex::encode(buf));
}


// bls12_381

#[test]
fn bls12_381_scalar_val_7() {
    let scalar = bls12_381::Scalar::from(7);
    let buf = scalar.to_bytes();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_generator_compressed() {
    let buf = bls12_381::G1Affine::generator().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_generator_uncompressed() {
    let buf = bls12_381::G1Affine::generator().to_uncompressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_generator_compressed() {
    let buf = bls12_381::G2Affine::generator().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_generator_uncompressed() {
    let buf = bls12_381::G2Affine::generator().to_uncompressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_generator_neg_compressed() {
    let buf = bls12_381::G1Affine::generator().neg().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_generator_neg_uncompressed() {
    let buf = bls12_381::G1Affine::generator().neg().to_uncompressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_generator_neg_compressed() {
    let buf = bls12_381::G2Affine::generator().neg().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_generator_neg_uncompressed() {
    let buf = bls12_381::G2Affine::generator().neg().to_uncompressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_inf_compressed() {
    let buf = bls12_381::G1Affine::identity().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g1_affine_inf_uncompressed() {
    let buf = bls12_381::G1Affine::identity().to_uncompressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_inf_compressed() {
    let buf = bls12_381::G2Affine::identity().to_compressed();
    println!("{}", hex::encode(buf));
}

#[test]
fn bls12_381_g2_affine_inf_uncompressed() {
    let buf = bls12_381::G2Affine::identity().to_uncompressed();
    println!("{}", hex::encode(buf));
}


// ark_bls12_381

#[test]
fn ark_bls12_381_g1_affine_generator_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_generator_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_generator_neg_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().neg().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_generator_neg_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().neg().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_inf_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::zero().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_inf_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::zero().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_neg_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().neg().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_neg_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().neg().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_inf_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::zero().serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_inf_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::zero().serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_fr_val_7() {
    let mut buf = vec![];
    let val_7 = ark_bls12_381::Fr::from(7);
    val_7.serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_generator_x_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().x.serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g1_affine_generator_x_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G1Affine::prime_subgroup_generator().x.serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_x_0_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().x.c0.serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_x_0_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().x.c0.serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_x_1_compressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().x.c1.serialize(&mut buf);
    println!("{}", hex::encode(buf));
}

#[test]
fn ark_bls12_381_g2_affine_generator_x_1_uncompressed() {
    let mut buf = vec![];
    ark_bls12_381::G2Affine::prime_subgroup_generator().x.c1.serialize_uncompressed(&mut buf);
    println!("{}", hex::encode(buf));
}
