#[aoc_generator(day19)]
fn parse_input(input: &str) -> Vec<Scanner> {
    input.split("\n\n")
        .map(|scanner| {
            let mut iter = scanner.lines();
            let number = iter.next().unwrap();
            let number = number.replace("--- scanner ", "");
            let number = number.replace(" ---", "");
            let number = number.parse().unwrap();

            let beacons = iter.map(|line| {
                let mut coordinates = line.split(',')
                    .map(|coordinate| coordinate.parse().unwrap());
                Beacon {
                    x: coordinates.next().unwrap(),
                    y: coordinates.next().unwrap(),
                    z: coordinates.next().unwrap(),
                }
            }).collect();
            Scanner {
                number,
                beacons,
            }
        }).collect()
}

#[derive(Debug, Clone)]
struct Scanner {
    number: u8,
    beacons: Vec<Beacon>,
}
impl Scanner {
    fn with_offset(&self, dx:i64, dy:i64, dz:i64) -> Scanner {
        let beacons = self.beacons.iter()
            .map(|Beacon {x,y,z}|Beacon {
                x: x + dx,
                y: y + dy,
                z: z + dz,
            }).collect();
        Scanner {
            number: self.number,
            beacons
        }
    }
    fn overlaps(&self, other: &Scanner) -> i64 {
        let mut overlaps = 0;
        for beacon in &self.beacons {
            if other.beacons.contains(beacon) {
                overlaps += 1;
            }
        }
        overlaps
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Beacon {
    x: i64,
    y: i64,
    z: i64,
}

#[aoc(day19, part1)]
fn part1(scanners: &[Scanner]) -> usize {
    println!("Input: {:?}", scanners);

    todo!()
}

fn overlaps(scanners: Vec<Scanner>) {
    let zero = scanners.get(0).unwrap().clone();
    let one = scanners.get(1).unwrap().clone();

    let mut max_overlaps = 0;
    for x in 0..=1000 {
        for y in 0..=1000 {
            for z in 0..=1000 {
                let one = one.with_offset(x,y,z);
                let overlaps = zero.overlaps(&one);
                max_overlaps = max_overlaps.max(overlaps);
            }
        }
    }
    panic!("There were at max {} overlaps", max_overlaps);
}

// #[aoc(day19, part2)]
// fn part2(input: &[Line]) -> usize {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn verify_part1() {
    //     let input = include_str!("../input/2021/day19.txt");
    //     let input = parse_input(input);
    //     assert_eq!(part1(input.as_slice()), 6666);
    // }

    // #[test]
    // fn verify_part2() {
    //     let input = include_str!("../input/2021/day19.txt");
    //     assert_eq!(part2(parse_input(input).as_slice()), 19081);
    // }

    #[test]
    fn part1_provided_example() {
        let result = part1(&parse_input(INPUT));

        assert_eq!(result, 5)
    }

    #[test]
    fn part2_provided_example() {}

    #[test]
    fn two_scanners() {
        let input = r#"--- scanner 0 ---
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
553,889,-390"#;

        let parsed = parse_input(input);
        println!("figure out overlaps: {:?}", overlaps(parsed));
    }

    const INPUT: &str = r#"--- scanner 0 ---
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
30,-46,-14"#;
}
