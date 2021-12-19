mod day19 {
    use std::collections::{HashSet, VecDeque};
    use std::iter::FromIterator;

    fn rotate(mut pos: (i32, i32, i32), rot: (usize, usize, usize)) -> (i32, i32, i32) {
        for _ in 0..rot.0 {
            pos = (pos.0, -pos.2, pos.1);
        }

        for _ in 0..rot.1 {
            pos = (-pos.2, pos.1, pos.0);
        }

        for _ in 0..rot.2 {
            pos = (-pos.1, pos.0, pos.2);
        }

        pos
    }

    fn match_scans(scan_a: &[(i32, i32, i32)], scan_b: &[(i32, i32, i32)])
        -> Option<(i32, i32, i32)> {

        for a in scan_a {
            for b in scan_b {
                let trans = (a.0 - b.0, a.1 - b.1, a.2 - b.2);
                if scan_a.iter().map(|p| *p).collect::<HashSet<_>>().intersection(
                    &scan_b.iter()
                    .map(|(x, y, z)| (x + trans.0, y + trans.1, z + trans.2))
                    .collect()).count() >= 12 {
                    return Some(trans);
                }
            }
        }

        None
    }

    fn add(
        set_scanners: &mut Vec<Vec<(i32, i32, i32)>>,
        new_scan: &[(i32, i32, i32)]) -> Result<(i32, i32, i32), ()> {
        for rx in 0..4 {
            for ry in 0..4 {
                for rz in 0..4 {
                    let rot_scan: Vec<(i32, i32, i32)> = new_scan.iter()
                        .map(|pos| rotate(*pos, (rx, ry, rz))).collect();
                    for scan_a in set_scanners.iter() {
                        match match_scans(&scan_a, &rot_scan) {
                            Some((dx, dy, dz)) => {
                                set_scanners.push(
                                    rot_scan.into_iter()
                                    .map(|(x, y, z)| (x + dx, y + dy, z + dz))
                                    .collect::<Vec<(i32, i32, i32)>>());
                                return Ok((dx, dy, dz));
                            },
                            None => {},
                        }
                    }
                }
            }
        }

        Err(())
    }

    fn assemble(scanners: &[Vec<(i32, i32, i32)>]) -> (Vec<(i32, i32, i32)>, Vec<(i32, i32, i32)>) {
        let mut set_scanners = vec![scanners[0].clone()];
        let mut pos_scanners = vec![(0, 0, 0)];

        let mut queue: VecDeque<Vec<(i32, i32, i32)>> = VecDeque::from(
            scanners.into_iter().skip(1).map(|p| p.clone()).collect::<Vec<_>>());



        while let Some(scanner) = queue.pop_front() {
            match add(&mut set_scanners, &scanner) {
                Ok(pos_scan) => pos_scanners.push(pos_scan),
                Err(_) => queue.push_back(scanner),
            }
        }

        (set_scanners.into_iter()
            .map(|scan| HashSet::from_iter(scan.into_iter()))
            .fold(HashSet::new(), |acc, x| acc.union(&x).map(|p| *p).collect())
            .into_iter().collect(),
        pos_scanners)
    }

    fn parse(input: &[String]) -> Vec<Vec<(i32, i32, i32)>> {
        let mut scanners = Vec::new();
        let mut new_scanner = Vec::new();

        for line in input {
            let scan: Vec<_> = line.split(',').map(|v| v.parse::<i32>()).collect();
            if scan.len() != 3 {
                if !new_scanner.is_empty() {
                    scanners.push(new_scanner.drain(..).collect());
                }
            }
            else {
                new_scanner.push((
                        *scan[0].as_ref().unwrap(),
                        *scan[1].as_ref().unwrap(),
                        *scan[2].as_ref().unwrap()));
            }
        }

        scanners.push(new_scanner.drain(..).collect());

        scanners
    }

    pub fn solve_ab(input: &[String]) -> (usize, i32) {
        let scanners = parse(input);

        let (beacons, scanners) = assemble(&scanners);

        (
            beacons.len(),
            scanners.iter()
                .flat_map(|(ax, ay, az)| scanners.iter().map(move |(bx, by, bz)| (ax - bx).abs() + (ay - by).abs() + (az - bz).abs()))
                .max().unwrap())
    }

    #[cfg(test)]
    mod test_day19 {
        use super::*;

        #[test]
        fn test_add() {
            let scan_0 = vec![ (404,-588,-901), (528,-643,409), (-838,591,734), (390,-675,-793), (-537,-823,-458), (-485,-357,347), (-345,-311,381), (-661,-816,-575), (-876,649,763), (-618,-824,-621), (553,345,-567), (474,580,667), (-447,-329,318), (-584,868,-557), (544,-627,-890), (564,392,-477), (455,729,728), (-892,524,684), (-689,845,-530), (423,-701,434), (7,-33,-71), (630,319,-379), (443,580,662), (-789,900,-551), (459,-707,401)];
            let scan_1 = vec![ (686,422,578), (605,423,415), (515,917,-361), (-336,658,858), (95,138,22), (-476,619,847), (-340,-569,-846), (567,-361,727), (-460,603,-452), (669,-402,600), (729,430,532), (-500,-761,534), (-322,571,750), (-466,-666,-811), (-429,-592,574), (-355,545,-477), (703,-491,-529), (-328,-685,520), (413,935,-424), (-391,539,-444), (586,-435,557), (-364,-763,-893), (807,-499,-711), (755,-354,-619), (553,889,-390)];

            let mut scanners = vec![scan_0];

            assert_eq!(add(&mut scanners, &scan_1), Ok((68,-1246,-43)));
        }

        #[test]
        fn test_solve_ab() {
            let input: Vec<String> = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14".lines().map(|s| s.to_string()).collect();
            let (sol_a, sol_b) = solve_ab(&input);
            assert_eq!(sol_a, 79);
            assert_eq!(sol_b, 3621);
        }

    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();

    let input: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();

    let (sol_a, sol_b) = day19::solve_ab(&input);
    println!("Solution A-part: {}", sol_a);
    println!("Solution B-part: {}", sol_b);
}
