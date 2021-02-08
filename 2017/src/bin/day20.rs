extern crate regex;

mod day20 {
    use std::collections::{HashMap, HashSet};

    #[derive(Clone, Copy, Debug)]
    pub struct Particle {
        pub p: [i64; 3],
        pub v: [i64; 3],
        pub a: [i64; 3],
    }

    #[inline]
    fn norm(a: &[i64; 3]) -> i64 {
        (0..3).map(|i| a[i].abs()).sum()
    }

    pub fn solve_a(particles: &[Particle]) -> usize {
        let mn = particles.iter().enumerate()
            .map(|(i, p)| (norm(&p.a), norm(&p.v), norm(&p.p), i))
            .min().unwrap();

        mn.3
    }

    fn isqrt(x: i64) -> Option<i64> {
        if x < 0 {
            return None;
        }

        let mut x0 = x >> 1;

        if x0 > 0 {
            let mut x1 = (x0 + x/x0) >> 1;

            while x1 < x0 {
                x0 = x1;
                x1 = (x0 + x/x0) >> 1;
            }

            Some(x0)
        }
        else {
            Some(x)
        }
    }

    /// Compute intersection times between the two particles
    fn intersect(p_a: &Particle, p_b: &Particle)
        -> (Option<i64>, Option<i64>)  {
        let t = (0..3).map(|i|
            {
                let a = p_a.a[i] - p_b.a[i];
                let v = 2 * (p_a.v[i] - p_b.v[i]) + a;
                let p = 2 * (p_a.p[i] - p_b.p[i]);

                if a != 0 {
                    let delta = v*v - 4*a*p;
                    Some(
                        if delta > 0 {
                            let sqdelta = isqrt(delta).unwrap();
                            if sqdelta.pow(2) == delta {
                                (
                                 if (-v - sqdelta) % (2*a) == 0 { Some((-v - sqdelta)/(2*a)) }
                                     else { None },
                                 if (-v + sqdelta) % (2*a) == 0 { Some((-v + sqdelta)/(2*a)) }
                                     else { None }
                                )
                            }
                            else { (None, None) }
                        }
                        else if delta == 0 {
                            if v % (2*a) == 0  { (Some(-v/(2*a)), None) }
                            else { (None, None) }
                        }
                        else {
                            (None, None)
                        })
                }
                else if v != 0 {
                    if p % v == 0 { Some((Some(-p/v), None)) }
                    else { Some((None, None)) }
                }
                else {
                    if p == 0 {
                        None
                    }
                    else {
                        Some((None, None))
                    }
                }
            }).collect::<Vec<_>>();

        let t = t.iter().filter_map(|&ti| ti).collect::<Vec<_>>();

        let mut count: HashMap<i64, usize> = HashMap::new();

        for ti in t.iter() {
            if let Some(ti0) = ti.0 {
                *count.entry(ti0).or_insert(0) += 1;
            }
            if let Some(ti1) = ti.1 {
                *count.entry(ti1).or_insert(0) += 1;
            }
        }

        let mut t = count.keys().filter(|k| count[k] == t.len()).collect::<Vec<_>>();

        t.sort_unstable();

        match t.len() {
            0 => (None, None),
            1 => (Some(*t[0]), None),
            2 => (Some(*t[0]), Some(*t[1])),
            _ => unreachable!(),
        }
    }

    pub fn solve_b(particles: &[Particle]) -> usize {
        //Create boolean list of particles left
        let mut p_left = vec![true; particles.len()];

        //Compute intersection time and sort list [(t, a, b)]
        let mut collisions: HashMap<i64, HashSet<usize>> = HashMap::new();
        for i_a in 0..particles.len() {
            for i_b in (i_a + 1)..particles.len() {
                let (t1, t2) = intersect(&particles[i_a], &particles[i_b]);
                for t in vec![t1, t2].into_iter() {
                    if let Some(t) = t{
                        let crash_site = collisions.entry(t).or_insert(HashSet::new());
                        crash_site.insert(i_a);
                        crash_site.insert(i_b);
                    }
                }
            }
        }

        let mut collision_times = collisions.keys().collect::<Vec<_>>();
        collision_times.sort_unstable();

        //Go through the list and eliminate particles in order
        for t in collision_times.into_iter().filter(|&&t| t>= 0) {
            if collisions[&t].iter().filter(|&&i| p_left[i]).count() > 1 {
                for &i in collisions[&t].iter() {
                    p_left[i] = false;
                }
            }
        }

        //Count remaining particles
        p_left.into_iter().filter(|&p| p).count()
    }

    #[cfg(test)]
    mod test_day20 {
        use super::*;

        #[test]
        fn test_solve_a() {
            let particles = vec![
                Particle { p: [3, 0, 0], v: [2, 0, 0], a: [-1, 0, 0] },
                Particle { p: [4, 0, 0], v: [0, 0, 0], a: [-2, 0, 0] },
            ];

            assert_eq!(solve_a(&particles), 0);
        }

        #[test]
        fn test_solve_b() {
            let particles = vec![
                Particle { p: [-6, 0, 0], v: [3, 0, 0], a: [0, 0, 0] },
                Particle { p: [-4, 0, 0], v: [2, 0, 0], a: [0, 0, 0] },
                Particle { p: [-2, 0, 0], v: [1, 0, 0], a: [0, 0, 0] },
                Particle { p: [3, 0, 0], v: [-1, 0, 0], a: [0, 0, 0] },
            ];

            assert_eq!(solve_b(&particles), 1);
        }
    }
}

fn main() {
    use regex::Regex;
    use std::io::{self, Read};
    use std::convert::TryInto;

    let stdin = io::stdin();
    let mut buffer = String::new();
    {
        let mut stdin_lock = stdin.lock();
        stdin_lock.read_to_string(&mut buffer).unwrap();
    }

    let re_particle = Regex::new(r"p=<(?P<p>(-?\d+,?){3})>, v=<(?P<v>(-?\d+,?){3})>, a=<(?P<a>(-?\d+,?){3})>").unwrap();

    let particles: Vec<_> = re_particle.captures_iter(&buffer)
        .map(|capt| {
            let p = capt.name("p").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();
            let v = capt.name("v").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();
            let a = capt.name("a").unwrap().as_str()
                .split(',').map(|v| v.parse().unwrap()).collect::<Vec<_>>()
                .try_into().unwrap();

            day20::Particle { p, v, a }
        }).collect();

    println!("Solution A-part: {}", day20::solve_a(&particles));
    println!("Solution B-part: {}", day20::solve_b(&particles));
}
