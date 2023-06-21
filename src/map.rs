use core::slice::{Iter, IterMut};
use smart_leds::RGB8;

pub const INDEX_MAP : [&str; 72] = [
    "DECIN",
    "LIBEREC",
    "JABLONEC_NAD_NISOU",
    "ÚSTI_NAD_LABEM",
    "CESKA_LIPA",
    "SEMILY",
    "TEPLICE",
    "TRUTNOV",
    "LITOMERICE",
    "MOST",
    "CHOMUTOV",
    "JICIN",
    "NACHOD",
    "MLADA_BOLESLAV",
    "MELNIK",
    "LOUNY",
    "KARLOVY_VARY",
    "JESENIK",
    "HRADEC_KRALOVE",
    "SOKOLOV",
    "NYMBURK",
    "RYCHNOV_NAD_KNEZNOU",
    "KLADNO",
    "RAKOVNIK",
    "CHEB",
    "BRUNTAL",
    "PRAHA",
    "PARDUBICE",
    "KOLIN",
    "ÚSTI_NAD_ORLICI",
    "OPAVA",
    "SUMPERK",
    "BEROUN",
    "KUTNA_HORA",
    "CHRUDIM",
    "KARVINA",
    "OSTRAVA",
    "TACHOV",
    "SVITAVY",
    "BENESOV",
    "PLZEN",
    "ROKYCANY",
    "FRYDEK_MISTEK",
    "PRIBRAM",
    "NOVY_JICIN",
    "OLOMOUC",
    "HAVLICKŮV_BROD",
    "ZĎAR_NAD_SAZAVOU",
    "PREROV",
    "PROSTEJOV",
    "DOMAZLICE",
    "PELHRIMOV",
    "TABOR",
    "JIHLAVA",
    "KLATOVY",
    "BLANSKO",
    "VSETIN",
    "KROMERIZ",
    "PISEK",
    "VYSKOV",
    "STRAKONICE",
    "ZLIN",
    "TREBIC",
    "BRNO",
    "JINDRICHUV_HRADEC",
    "UHERSKÉ_HRADISTE",
    "PRACHATICE",
    "CESKE_BUDEJOVICE",
    "HODONIN",
    "ZNOJMO",
    "CESKY_KRUMLOV",
    "BRECLAV",
];

pub struct Map<'d> {
    index_map: &'d [&'d str],
    data: &'d mut [RGB8]
}

pub enum Error {
    NotFound
}

impl<'d> Map<'d> {
    pub fn new(index_map: &'d [&'d str], data: &'d mut [RGB8]) -> Self {
        Map {
            index_map,
            data
        }
    }

    pub fn get_index_by_name(&self, name: &[char]) -> Result<usize, Error> {
        for (i, current) in self.index_map.iter().enumerate() {
            if current.len() != name.len() {
                continue;
            }

            let mut matches = true;
            for (j, c) in current.chars().enumerate() {
                if name[j] != c {
                    matches = false;
                    break;
                }
            }

            if matches {
                return Ok(i);
            }
        }

        return Err(Error::NotFound);
    }

    pub fn set_rgb(&mut self, index: usize, r: Option<u8>, g: Option<u8>, b: Option<u8>) -> Result<(), Error> {
        let original = self.data[index];
        if self.data.len() <= index {
            return Err(Error::NotFound)
        }

        self.data[index] = RGB8 {
            r: r.unwrap_or(original.r),
            g: g.unwrap_or(original.g),
            b: b.unwrap_or(original.b),
        };

        Ok(())
    }

    pub fn set_rgb_by_name(&mut self, name: &[char], r: Option<u8>, g: Option<u8>, b: Option<u8>) -> Result<(), Error> {
        let index = self.get_index_by_name(name)?;
        self.set_rgb(index, r, g, b)
    }

    pub fn get_map(&self) -> Iter<RGB8> {
        return self.data.iter();
    }
    pub fn get_map_mut(&mut self) -> IterMut<RGB8> { return self.data.iter_mut(); }
}