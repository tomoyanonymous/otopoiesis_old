pub mod otopoiesis{
    pub trait Duration{
        fn get_dur(&self) -> u64;
    }

    pub enum Waveform{
        Cache(Vec<f64>),
        Proj(Box<Project>)
    }
    pub struct Region{
        pub start: u64,
        pub end: u64,
        pub content:Waveform
    }

    impl Duration for Region{
        fn get_dur(&self) -> u64{
            self.end - self.start
        }
    }
    pub struct Project{
       pub tracks: Vec<Vec<Region>>
    }

}


#[cfg(test)]
mod tests {
    use crate::otopoiesis::*;

    fn make_test_region()->Region{
        Region{start:10,end:20,content:Waveform::Cache(vec![1.0, 2.0, 3.0])}
    }
    #[test]
    pub fn make_project(){
        let r = make_test_region();
        assert_eq!(r.get_dur(),10);

        let p = Project{tracks:vec![vec![
            make_test_region(),
            make_test_region(),
            Region{
                start:10,
                end:20,
                content: Waveform::Proj(Box::new(Project{tracks:vec![vec![make_test_region()]]}))
            }
        ]]};
        let c: &Waveform = &p.tracks[0][2].content;
        match c {
            Waveform::Proj(p) => assert_eq!(p.tracks[0][0].get_dur(),10),
            _ => panic!(),
        };
    }
}