use modinverse::modinverse;

const TIMESTAMP: u32 = 1005162;
const INPUT_PART_1: [u32; 9] = [19, 41, 823, 23, 17, 29, 443, 37, 13];
const INPUT_PART_2: [(isize, isize); 9] = [
    (19, 0),
    (41, 9),
    (823, 19),
    (23, 27),
    (17, 36),
    (29, 48),
    (443, 50),
    (37, 56),
    (13, 63),
];

pub fn execute() {
    let (bus, departure) = compute_earliest_bus(TIMESTAMP, &INPUT_PART_1);
    println!(
        "13:1 — Product of earliest timestamp and bus: {}",
        bus * (departure - TIMESTAMP),
    );
    println!(
        "13:2 — Earliest timestamp with departure aligned by position: {}",
        compute_earliest_timestamp_with_departure_aligned_by_position(&INPUT_PART_2),
    );
}

fn compute_earliest_bus(timestamp: u32, buses: &[u32]) -> (u32, u32) {
    buses
        .iter()
        .map(|b| (*b, ((timestamp - 1) / b + 1) * b))
        .min_by_key(|(_, t)| *t)
        .unwrap()
}

fn chinese_remainder(entries: &[(isize, isize)]) -> isize {
    let product: isize = entries.iter().map(|(id, _)| id).product();
    entries
        .iter()
        .flat_map(|(m, r)| {
            let p = product / m;
            modinverse(p, *m).map(|inv| inv * r * p)
        })
        .sum::<isize>()
        % product
}

fn compute_earliest_timestamp_with_departure_aligned_by_position(
    buses: &[(isize, isize)],
) -> isize {
    let entries = buses.iter().map(|(b, o)| (*b, b - o)).collect::<Vec<_>>();
    chinese_remainder(&entries)
}

#[cfg(test)]
mod compute_earliest_timestamp_with_departure_aligned_by_position_should {
    use super::*;

    #[test]
    fn return_3417_on_the_first_sample() {
        assert_eq!(
            compute_earliest_timestamp_with_departure_aligned_by_position(&[
                (17, 0),
                (13, 2),
                (19, 3)
            ]),
            3417,
        )
    }

    #[test]
    fn return_754018_on_the_second_sample() {
        assert_eq!(
            compute_earliest_timestamp_with_departure_aligned_by_position(&[
                (67, 0),
                (7, 1),
                (59, 2),
                (61, 3),
            ]),
            754018,
        )
    }

    #[test]
    fn return_779210_on_the_second_sample() {
        assert_eq!(
            compute_earliest_timestamp_with_departure_aligned_by_position(&[
                (67, 0),
                (7, 2),
                (59, 3),
                (61, 4),
            ]),
            779210,
        )
    }

    #[test]
    fn return_1261476_on_the_second_sample() {
        assert_eq!(
            compute_earliest_timestamp_with_departure_aligned_by_position(&[
                (67, 0),
                (7, 1),
                (59, 3),
                (61, 4),
            ]),
            1261476,
        )
    }

    #[test]
    fn return_1202161486_on_the_second_sample() {
        assert_eq!(
            compute_earliest_timestamp_with_departure_aligned_by_position(&[
                (1789, 0),
                (37, 1),
                (47, 2),
                (1889, 3),
            ]),
            1202161486,
        )
    }
}
