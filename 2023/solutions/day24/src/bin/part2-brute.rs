use anyhow::Result;
use std::io;

use day24::parse;

const INVERSIONS: [(i64, i64, i64); 8] = [
    (1, 1, 1),
    (1, 1, -1),
    (1, -1, 1),
    (1, -1, -1),
    (-1, 1, 1),
    (-1, 1, -1),
    (-1, -1, 1),
    (-1, -1, -1)
];

const EPSILON: f64 = 0.01_f64;

// #[aoc_test("data/test/24.txt", "47")]
// #[aoc_test("data/24.txt", "976976197397181")]
fn main() -> Result<()> {
    env_logger::init();

    let stdin = io::stdin();
    let input = io::read_to_string(stdin.lock())?;
    let (s, lines) = parse::lines(input.as_str()).unwrap();
    assert!(s.trim().is_empty());

    #[allow(unused_assignments)]
    let mut velocity = None;
    let mut position = None;

    'found_solution: for bound in 0_i64.. {
        log::info!("bound: {}", bound);
        for vx in 0..=bound {
            for vy in 0..=(bound - vx) {
                let vz = bound - vx - vy;
                for (xd, yd, zd) in INVERSIONS {
                    if vx == 0 && xd < 0 { continue; }
                    if vy == 0 && yd < 0 { continue; }
                    if vz == 0 && zd < 0 { continue; }

                    let xd = vx * xd;
                    let yd = vy * yd;
                    let zd = vz * zd;

                    // for any two lines l1 and l2
                    // we want to be at l1 at t2 and at l2 at t2
                    // the time between is t2 - t1 = td

                    // we do not know x0

                    // x0 + xd * t1 = l1.o.x + l1.d.x * t1
                    // x0 + xd * t2 = l2.o.x + l2.d.x * t2
                    
                    // l1.o.x + l1.d.x * t1 - xd * t1 = l2.o.x + l2.d.x * t2 - xd * t2
                    // currently t1 and t2 are unknown
                    // l1.d.x * t1 - xd * t1 = l2.o.x + l2.d.x * t2 - xd * t2 - l1.o.x
                    // t1 * (l1.d.x - xd) = l2.o.x + l2.d.x * t2 - xd * t2 - l1.o.x
                    
                    // t1 = (l2.o.x + l2.d.x * t2 - xd * t2 - l1.o.x) / (l1.d.x - xd)
                    // t1 = (l2.o.y + l2.d.y * t2 - yd * t2 - l1.o.y) / (l1.d.y - yd)

                    // (l2.o.x + l2.d.x * t2 - xd * t2 - l1.o.x) / (l1.d.x - xd) 
                    //      = (l2.o.y + l2.d.y * t2 - yd * t2 - l1.o.y) / (l1.d.y - yd)

                    // (l2.o.x + l2.d.x * t2 - xd * t2 - l1.o.x) * (l1.d.y - yd)
                    //      = (l2.o.y + l2.d.y * t2 - yd * t2 - l1.o.y) * (l1.d.x - xd)

                    // let A = l2.o.x - l1.o.x
                    // let B = l1.d.y - yd
                    // let C = l2.o.y - l1.o.y
                    // let D = l1.d.x - xd

                    // (A + l2.d.x * t2 - xd * t2) * B = (C + l2.d.y * t2 - yd * t2) * D
                    // AB + B l2.d.x t2 - B xd t2 = CD + D l2.d.y t2 - D yd t2
                    // B l2.d.x t2 - B xd t2 - D l2.d.y t2 + D yd t2 = CD - AB
                    // t2 = (CD - AB) / (B l2.d.x - B xd - D l2.d.y + D yd)
                    
                    // To have a potential velocity some one line must be able to get to all of the other lines
                    let valid = lines
                        .iter()
                        .any(|l1| {
                            lines
                                .iter()
                                .filter(move |l2| l1 != *l2)
                                .all(|l2| {
                                    let axy = l2.origin.x - l1.origin.x;
                                    let bxy = l1.direction.y - yd as f64;
                                    let cxy = l2.origin.y - l1.origin.y;
                                    let dxy = l1.direction.x - xd as f64;

                                    let t2xy = (cxy * dxy - axy * bxy) / (bxy * l2.direction.x - bxy * xd as f64 - dxy * l2.direction.y + dxy * yd as f64);
                                    if (t2xy - t2xy.round()).abs() > EPSILON {

                                        return false;
                                    }
                                    let t2xy = t2xy.round() as i32;

                                    let axz = l2.origin.x - l1.origin.x;
                                    let bxz = l1.direction.z - zd as f64;
                                    let cxz = l2.origin.z - l1.origin.z;
                                    let dxz = l1.direction.x - xd as f64;

                                    let t2xz = (cxz * dxz - axz * bxz) / (bxz * l2.direction.x - bxz * xd as f64 - dxz * l2.direction.z + dxz * zd as f64);
                                    if (t2xz - t2xz.round()).abs() > EPSILON {
                                        return false;
                                    }
                                    let t2xz = t2xz.round() as i32;
                                    if t2xy != t2xz {
                                        return false;
                                    }

                                    let ayz = l2.origin.y - l1.origin.y;
                                    let byz = l1.direction.z - zd as f64;
                                    let cyz = l2.origin.z - l1.origin.z;
                                    let dyz = l1.direction.y - yd as f64;

                                    let t2yz = (cyz * dyz - ayz * byz) / (byz * l2.direction.y - byz * yd as f64 - dyz * l2.direction.z + dyz * zd as f64);
                                    if (t2yz - t2yz.round()).abs() > EPSILON {
                                        return false;
                                    }
                                    let t2yz = t2yz.round() as i32;
                                    if t2xy != t2yz {
                                        return false;
                                    }

                                    true
                                })
                        });

                    if valid {
                        velocity = Some((xd, yd, zd));
                        log::info!("found potential velocity: {:?}", velocity);

                        // at t = 0 we're at x0, t0 is l[0], t1 is l[1]
                        
                        // x0 + xd * t0 = l[0].o.x + l[0].d.x * t0
                        // x0 + xd * t1 = l[1].o.x + l[1].d.x * t1

                        // l0ox + l0dx * t0 - xd * t0 = l1ox + l1dx * t1 - xd * t1
                        // t0 = (l1ox + l1dx * t1 - xd * t1 - l0ox) / (l0dx - xd)
                        // t0 = (l1oy + l1dy * t1 - yd * t1 - l0oy) / (l0dy - yd)
                        
                        // (l1ox + l1dx * t1 - xd * t1 - l0ox) / (l0dx - xd) = (l1oy + l1dy * t1 - yd * t1 - l0oy) / (l0dy - yd)
                        // (l1ox + l1dx * t1 - xd * t1 - l0ox) * (l0dy - yd) = (l1oy + l1dy * t1 - yd * t1 - l0oy) * (l0dx - xd)

                        // let A = l0dy - yd
                        // let B = 10dx - xd

                        // (l1ox + l1dx * t1 - xd * t1 - l0ox) * A = (l1oy + l1dy * t1 - yd * t1 - l0oy) * B
                        // l1ox * A + l1dx * t1 * A - xd * t1 * A - l0ox * A = l1oy * B + l1dy * t1 * B - yd * t1 * B - l0oy * B
                        // l1dx * t1 * A - xd * t1 * A - l1dy * t1 * B + yd * t1 * B = l1oy * B - l1ox * A - l0oy * B + l0ox * A
                        // t1 * (l1dx * A - xd * A - l1dy * B + yd * B) = l1oy * B - l1ox * A - l0oy * B + l0ox * A
                        // t1 = (l1oy * B - l1ox * A - l0oy * B + l0ox * A) / (l1dx * A - xd * A - l1dy * B + yd * B)

                        let axy = lines[0].direction.y - yd as f64;
                        let bxy = lines[0].direction.x - xd as f64;

                        let t1xy = -(lines[1].origin.y * bxy - lines[1].origin.x * axy - lines[0].origin.y * bxy + lines[0].origin.x * axy) / (lines[1].direction.y * bxy - yd as f64 * bxy - lines[1].direction.x * axy + xd as f64 * axy);

                        let axz = lines[0].direction.z - zd as f64;
                        let bxz = lines[0].direction.x - xd as f64;

                        let t1xz = -(lines[1].origin.z * bxz - lines[1].origin.x * axz - lines[0].origin.z * bxz + lines[0].origin.x * axz) / (lines[1].direction.z * bxz - zd as f64 * bxz - lines[1].direction.x * axz + xd as f64 * axz);

                        let ayz = lines[0].direction.z - zd as f64;
                        let byz = lines[0].direction.y - yd as f64;

                        let t1yz = -(lines[1].origin.z * byz - lines[1].origin.y * ayz - lines[0].origin.z * byz + lines[0].origin.y * ayz) / (lines[1].direction.z * byz - zd as f64 * byz - lines[1].direction.y * ayz + yd as f64 * ayz);

                        if (t1xy - t1xz).abs() > EPSILON || (t1xy - t1yz).abs() > EPSILON {
                            log::info!("found non-matching t1: {:?} {:?} {:?}", t1xy, t1xz, t1yz);
                            continue;
                        }
                        if t1xy < 0_f64 {
                            log::info!("found negative t1: {:?}", t1xy);
                            continue;
                        }

                        // x0 + xd * t1 = l[1].o.x + l[1].d.x * t1
                        // x0 = l[1].o.x + l[1].d.x * t1 - xd * t1

                        let x0 = lines[1].origin.x + lines[1].direction.x * t1xy - (xd as f64) * t1xy;
                        let y0 = lines[1].origin.y + lines[1].direction.y * t1xy - (yd as f64) * t1xy;
                        let z0 = lines[1].origin.z + lines[1].direction.z * t1xy - (zd as f64) * t1xy;

                        if x0.fract() != 0.0 || y0.fract() != 0.0 || z0.fract() != 0.0 {
                            log::info!("found non-integer position: {:?}", (x0, y0, z0));
                            continue;
                        }

                        let x0 = x0.round() as i128;
                        let y0 = y0.round() as i128;
                        let z0 = z0.round() as i128;

                        position = Some((x0, y0, z0));
                        log::info!("found valid position: {position:?}");
                        
                        break 'found_solution;
                    }
                }                
            }
        }
    }

    let position = position.unwrap();
    let result = position.0 + position.1 + position.2;

    println!("{result:?}");
    Ok(())
}
