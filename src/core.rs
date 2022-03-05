
type AudioCache = Vec<f64>;
pub trait Ranged {
    fn get_dur(&self) -> f64;
}
pub trait Renderer {
    fn render(&self) -> AudioCache;
}
// Generator is essentially a function that generates audio like an oscillator.
// They will have an internal state like delay thus it should be treated as dyn Trait.
pub trait Generator {
    fn perform(&self, now: u64) -> u64;
}

pub enum Waveform {
    Cache(AudioCache),
    Proj(Box<Project>),
    Generator(Box<dyn Generator>),
}

pub struct Region {
    pub start: f64, //absolute time in seconds
    pub end: f64,
    pub content: Waveform,
}
impl Ranged for Region {
    fn get_dur(&self) -> f64 {
        self.end - self.start
    }
}

pub struct Track {
    pub regions: Vec<Region>,
    pub datatype: (), //dummy
    pub input: (),    //dummy
    pub output: (),   //dummy
}

fn get_size_for_track(tr: &Track) -> f64 {
    let last = tr.regions
        .iter()
        .fold(0.0, |acc, x| if x.end.gt(&acc) {x.end} else {acc});
    let first = tr.regions
        .iter()
        .fold(last,
            |acc, x| if x.start.lt(&acc) {x.start} else {acc}
        );
    last - first
}
impl Renderer for Track {
    fn render(&self) -> AudioCache {
        let size = get_size_for_track(self) as usize;
        self.regions.iter().fold(
            vec![0.0; size],
            |a, b| a, //todo
        )
    }
}
pub struct Project {
    pub tracks: Vec<Track>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn make_test_region() -> Region {
        Region {
            start: 10.0,
            end: 20.0,
            content: Waveform::Cache(vec![1.0, 2.0, 3.0]),
        }
    }
    #[test]
    pub fn make_project() {
        let r = make_test_region();
        assert_eq!(r.get_dur(), 10.0);

        let p = Project {
            tracks: vec![Track {
                regions: vec![
                    make_test_region(),
                    make_test_region(),
                    Region {
                        start: 10.0,
                        end: 20.0,
                        content: Waveform::Proj(Box::new(Project {
                            tracks: vec![Track {
                                regions: vec![make_test_region()],
                                input: {},
                                output: {},
                                datatype: {},
                            }],
                        })),
                    },
                ],
                datatype: {},
                input: {},
                output: {},
            }],
        };
        let c: &Waveform = &p.tracks[0].regions[2].content;
        match c {
            Waveform::Proj(p) => assert_eq!(p.tracks[0].regions[0].get_dur(), 10.0),
            _ => panic!(),
        };
    }
}
